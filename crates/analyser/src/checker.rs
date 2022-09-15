#![allow(unused)]

use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    collections::HashMap,
    fmt::Display,
    hash::Hash,
    ops::Deref,
    string,
};

use ast::{Expression, Identifier, Location, Statement, TextSpan, Visitor};
use errors::SemanticError;
use utils::Stage;

#[derive(Debug, Clone, PartialEq)]
pub struct Atom<'a> {
    source_node: Identifier<'a>,
    given_type: Type,
    is_initialized: bool,
    is_constant: bool,
}
impl<'a> Default for Atom<'a> {
    fn default() -> Self {
        Atom {
            source_node: Identifier {
                value: "",
                span: [[0, 0], [0, 0]],
            },
            given_type: Type::Unknown,
            is_initialized: false,
            is_constant: false,
        }
    }
}

type TypeErrors = Vec<(SemanticError<Type>, TextSpan)>;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Unknown,
    Uninferrable,
    Any,
    Never,
    Nil,
    Generic {
        name: String,
        implements: Vec<Interface>,
    },
    Instance {
        class: TypeClass,
        arguments: Option<Vec<Type>>,
    },
    Function {
        name: String,
        arguments: Vec<Type>,
        returns: Box<Type>,
    },
}

impl Type {
    /// Returns `true` if the type is [`Unknown`].
    ///
    /// [`Unknown`]: Type::Unknown
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Returns `true` if the type is [`Uninferrable`].
    ///
    /// [`Uninferrable`]: Type::Uninferrable
    pub fn is_uninferrable(&self) -> bool {
        matches!(self, Self::Uninferrable)
            || matches!(
                self,
                Self::Instance {
                    arguments: Some(arg),
                    ..
                } if arg.iter().find(|t| t.is_uninferrable()).is_some()
            )
    }

    /// Returns `true` if the type is [`Any`].
    ///
    /// [`Any`]: Type::Any
    pub fn is_any(&self) -> bool {
        matches!(self, Self::Any)
    }

    /// Returns `true` if the type is [`Never`].
    ///
    /// [`Never`]: Type::Never
    pub fn is_never(&self) -> bool {
        matches!(self, Self::Never)
            || matches!(
                self,
                Self::Instance {
                    arguments: Some(arg),
                    ..
                } if arg.iter().find(|t| t.is_never()).is_some()
            )
    }

    /// Returns `true` if the type is [`Nil`].
    ///
    /// [`Nil`]: Type::Nil
    pub fn is_nil(&self) -> bool {
        matches!(self, Self::Nil)
            || matches!(
                self,
                Self::Instance {
                    arguments: Some(arg),
                    ..
                } if arg.iter().find(|t| t.is_nil()).is_some()
            )
    }
}

impl Type {
    pub fn string() -> Self {
        Type::Instance {
            class: TypeClass::string_primitive(),
            arguments: None,
        }
    }
    pub fn number() -> Self {
        Type::Instance {
            class: TypeClass::number_primitive(),
            arguments: None,
        }
    }
    pub fn character() -> Self {
        Type::Instance {
            class: TypeClass::character_primitive(),
            arguments: None,
        }
    }
    pub fn boolean() -> Self {
        Type::Instance {
            class: TypeClass::boolean_primitive(),
            arguments: None,
        }
    }
    pub fn array(arg: Self) -> Self {
        Type::Instance {
            class: TypeClass::array_primitive(),
            arguments: Some(vec![arg]),
        }
    }

    pub fn is_number(&self) -> bool {
        matches!(
            self,
            Type::Instance {
                class: TypeClass { name, .. },
                arguments
            } if name == "Number"
        )
    }

    pub fn is_character(&self) -> bool {
        matches!(
            self,
            Type::Instance {
                class: TypeClass { name, .. },
                arguments
            } if name == "Character"
        )
    }

    pub fn is_string(&self) -> bool {
        matches!(
            self,
            Type::Instance {
                class: TypeClass { name, .. },
                arguments
            } if name == "String"
        )
    }

    pub fn is_boolean(&self) -> bool {
        matches!(
            self,
            Type::Instance {
                class: TypeClass { name, .. },
                arguments
            } if name == "Boolean"
        )
    }

    pub fn is_array(&self) -> bool {
        matches!(
            self,
            Type::Instance {
                class: TypeClass { name, .. },
                arguments
            } if name == "Array"
        )
    }

    pub fn is_indefinite(&self) -> bool {
        self.is_any() || self.is_uninferrable() || self.is_never() || self.is_nil()
    }
    pub fn from_class(class: TypeClass, arguments: Option<Vec<Self>>) -> Self {
        Type::Instance { class, arguments }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeClass {
    pub name: String,
    pub generics: Option<Vec<Type>>,
    pub interfaces: Vec<Interface>,
    pub properties: HashMap<String, Type>,
}

impl TypeClass {
    fn string_primitive() -> Self {
        TypeClass {
            name: String::from("String"),
            generics: None,
            interfaces: Vec::new(),
            properties: HashMap::new(),
        }
    }
    fn number_primitive() -> Self {
        TypeClass {
            name: String::from("Number"),
            generics: None,
            interfaces: Vec::new(),
            properties: HashMap::new(),
        }
    }
    fn character_primitive() -> Self {
        TypeClass {
            name: String::from("Character"),
            generics: None,
            interfaces: Vec::new(),
            properties: HashMap::new(),
        }
    }
    fn boolean_primitive() -> Self {
        TypeClass {
            name: String::from("Boolean"),
            generics: None,
            interfaces: Vec::new(),
            properties: HashMap::new(),
        }
    }
    fn array_primitive() -> Self {
        TypeClass {
            name: String::from("Array"),
            generics: Some(vec![Type::Generic {
                name: String::from("T"),
                implements: Vec::new(),
            }]),
            interfaces: Vec::new(),
            properties: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Interface {
    Sum,
    Computable,
}

pub enum BlockContext {
    IfStatement,
    ForLoop,
    Loop,
    WhileLoop,
    ElseStatement,
    TestBlock,
    AnonymousBlock,
    Module,
    Class,
    ClassConstructor,
    FunctionExpression,
    FunctionDeclaration,
}

pub struct TypeChecker<'a> {
    values: RefCell<Stage<&'a str, Atom<'a>>>,
    classes: RefCell<Stage<&'a str, TypeClass>>,
    errors: RefCell<TypeErrors>,
}

impl<'a> TypeChecker<'a> {
    /// Type checks a sequence of statements and returns a list of errors encountered.
    pub fn check(statements: Vec<Statement<'a>>) -> TypeErrors {
        let checker = TypeChecker {
            values: RefCell::new(Stage::new()),
            classes: RefCell::new(Stage::new()),
            errors: RefCell::new(vec![]),
        };

        checker.set_primitives();

        for statement in statements {
            checker.visit_statement(&statement);
        }
        checker.errors.take()
    }
}

impl<'a> TypeChecker<'a> {
    /// Creates all primitive types.
    fn set_primitives(&'a self) {
        let primitives = [
            ("String", TypeClass::string_primitive()),
            ("Number", TypeClass::number_primitive()),
            ("Boolean", TypeClass::boolean_primitive()),
            ("Character", TypeClass::character_primitive()),
            ("Array", TypeClass::array_primitive()),
        ];
        for primitive in primitives {
            self.classes.borrow_mut().set(primitive.0, primitive.1);
        }
    }
    /// Create a new instance type from a type label.
    fn create_instance(&'a self, label: &ast::Type<'a>) -> Type {
        match label {
            // For concrete types, e.g. a: String, b: Array<Number>
            ast::Type::Concrete(concrete) => {
                let name = concrete.name.value;
                let class_option = self.classes.borrow().lookup(name).cloned();
                // Unknown classes.
                if class_option.is_none() {
                    self.errors
                        .borrow_mut()
                        .push((SemanticError::Undeclared(String::from(name)), concrete.span));
                    return Type::Uninferrable;
                }

                let class = class_option.unwrap();

                // Deal with generics.
                let arg_length = concrete.arguments.len();
                // No generics.
                if class.generics.is_none() {
                    if arg_length > 0 {
                        self.errors.borrow_mut().push((
                            SemanticError::UnexpectedGenerics(String::from(name)),
                            concrete.span,
                        ));
                        return Type::Uninferrable;
                    }
                    return Type::from_class(class, None);
                }
                // Some generics.
                let generics = class.generics.as_ref().unwrap();
                let generic_length = generics.len();

                // Unequal generic arguments.
                if arg_length != generic_length {
                    self.errors.borrow_mut().push((
                        SemanticError::UnequalGenericArgs(
                            String::from(name),
                            generic_length,
                            arg_length,
                        ),
                        concrete.span,
                    ));
                    return Type::Uninferrable;
                }

                let mut arguments = Vec::new();
                for input in &concrete.arguments {
                    let argument = self.create_instance(input);
                    arguments.push(argument);
                }
                return Type::from_class(class, Some(arguments));
            }
            ast::Type::Function(_) => todo!(),
            ast::Type::Dot(_) => todo!(),
        }
    }
    /// Checks if two types can be coerced into each other, with the first type taking precedence over the other.
    fn resolve_types(&'a self, type1: &Type, type2: &Type) -> Type {
        match (type1, type2) {
            // If at least type is uninferable.
            (_, Type::Uninferrable) | (Type::Uninferrable, _) => Type::Uninferrable,
            // If the second type is never.
            (x, Type::Never) => x.clone(),
            // If either type is unknown.
            (x, Type::Unknown) | (Type::Unknown, x) => {
                if x.is_unknown() {
                    Type::Uninferrable
                } else {
                    x.clone()
                }
            }
            // If at least one type is nil.
            (_, Type::Nil) | (Type::Nil, _) => Type::Nil,
            // If at least one type is any.
            (_, Type::Any) | (Type::Any, _) => Type::Any,
            // If both types are equal.
            (x, y) if x == y => x.clone(),
            // For types with unknown arguments. Assumes that both sides are the same class and have equal number of arguments.
            (
                Type::Instance {
                    arguments: Some(x),
                    class: class1,
                },
                Type::Instance {
                    arguments: Some(y),
                    class: class2,
                },
            ) => {
                if (class1 != class2) {
                    return Type::Uninferrable;
                }
                let mut resolved_arguments = vec![];
                let mut i = 0;
                while i < x.len() {
                    resolved_arguments.push(self.resolve_types(&x[i], &y[i]));
                    i += 1;
                }
                Type::from_class(class1.clone(), Some(resolved_arguments))
            }
            // Otherwise
            _ => Type::Uninferrable,
        }
    }
}

impl<'a> Visitor<'a, Type> for TypeChecker<'a> {
    /// Typecheck an identifier.
    fn visit_ident(&'a self, ident: &Identifier<'a>) -> Type {
        if let Some(atom) = self.values.borrow().lookup(ident.value) {
            if !atom.is_initialized {
                self.errors.borrow_mut().push((
                    SemanticError::Uninitialized(String::from(ident.value)),
                    ident.span,
                ))
            }
            atom.given_type.clone()
        } else {
            self.errors.borrow_mut().push((
                SemanticError::Undeclared(String::from(ident.value)),
                ident.span,
            ));
            Type::Uninferrable
        }
    }

    fn visit_string(&'a self, string: &ast::TextString<'a>) -> Type {
        Type::string()
    }

    fn visit_number(&'a self, number: &ast::Number<'a>) -> Type {
        Type::number()
    }

    fn visit_character(&'a self, charac: &ast::Character<'a>) -> Type {
        Type::character()
    }

    fn visit_boolean(&'a self, boolean: &ast::Boolean<'a>) -> Type {
        Type::boolean()
    }

    fn visit_self_expression(&'a self, self_: &ast::SelfExpression) -> Type {
        todo!()
    }

    /// Typecheck a binary expression.
    fn visit_binary_expression(&'a self, bin_exp: &ast::BinaryExpression<'a>) -> Type {
        let type1 = self.visit_expression(&bin_exp.operands[0]);
        let type2 = self.visit_expression(&bin_exp.operands[1]);
        let operator = bin_exp.operator.clone();
        if type1.is_unknown()
            || type2.is_unknown()
            || type1.is_uninferrable()
            || type2.is_uninferrable()
        {
            Type::Uninferrable
        }
        // Block operation on nil values.
        else if type1.is_nil() || type2.is_nil() {
            self.errors
                .borrow_mut()
                .push((SemanticError::OperationOnNil, bin_exp.span));
            Type::Uninferrable
        } else if type1.is_any() || type2.is_any() {
            Type::Any
        } else if type1 != type2 {
            self.errors.borrow_mut().push((
                SemanticError::UnsupportedBinaryOperation(operator, type1, type2),
                bin_exp.span,
            ));
            Type::Uninferrable
        } else {
            match operator {
                ast::Operator::Add => {
                    // string + string = string
                    // number + number = number
                    if type1.is_string() || type1.is_number() {
                        type1
                    } else {
                        self.errors.borrow_mut().push((
                            SemanticError::UnsupportedBinaryOperation(operator, type1, type2),
                            bin_exp.span,
                        ));
                        Type::Uninferrable
                    }
                }
                // With operator x, number x number = number
                ast::Operator::Multiply
                | ast::Operator::Subtract
                | ast::Operator::Divide
                | ast::Operator::Remainder
                | ast::Operator::PowerOf
                | ast::Operator::BitwiseOr
                | ast::Operator::BitwiseAnd
                | ast::Operator::BitWiseNot
                | ast::Operator::BitwiseLeftShift
                | ast::Operator::BitwiseRightShift
                | ast::Operator::GreaterThan
                | ast::Operator::LessThan
                | ast::Operator::GreaterThanOrEquals
                | ast::Operator::LessThanOrEquals => {
                    if type1 == Type::number() {
                        type1
                    } else {
                        self.errors.borrow_mut().push((
                            SemanticError::UnsupportedBinaryOperation(operator, type1, type2),
                            bin_exp.span,
                        ));
                        Type::Uninferrable
                    }
                }
                // Already checked for equality.
                ast::Operator::Equals | ast::Operator::NotEquals => type1,
                _ => unreachable!(),
            }
        }
    }

    // Typecheck logical expression.
    fn visit_logical_expression(&'a self, log_exp: &ast::LogicalExpression<'a>) -> Type {
        let type1 = self.visit_expression(&log_exp.operands[0]);
        let type2 = self.visit_expression(&log_exp.operands[1]);
        let operator = log_exp.operator.clone();
        if type1.is_unknown()
            || type2.is_unknown()
            || type1.is_uninferrable()
            || type2.is_uninferrable()
        {
            Type::Uninferrable
        }
        // Block operation on nil values.
        else if type1.is_nil() || type2.is_nil() {
            self.errors
                .borrow_mut()
                .push((SemanticError::OperationOnNil, log_exp.span));
            Type::Uninferrable
        } else if type1.is_any() || type2.is_any() {
            Type::Any
        } else if type1 != type2 {
            self.errors.borrow_mut().push((
                SemanticError::UnsupportedBinaryOperation(operator, type1, type2),
                log_exp.span,
            ));
            Type::Uninferrable
        } else {
            match log_exp.operator {
                ast::Operator::LogicalAnd | ast::Operator::LogicalOr => {
                    // boolean || boolean = boolean
                    // boolean && boolean = boolean
                    if type1.is_boolean() {
                        type1
                    } else {
                        self.errors.borrow_mut().push((
                            SemanticError::UnsupportedLogicalOperation(operator, type1, type2),
                            log_exp.span,
                        ));
                        Type::Uninferrable
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    fn visit_dot_expression(&'a self, dot_exp: &ast::DotExpression<'a>) -> Type {
        todo!()
    }

    // Typecheck unary expression.
    fn visit_unary_expression(&'a self, unary_exp: &ast::UnaryExpression<'a>) -> Type {
        let operand_type = self.visit_expression(&unary_exp.operand);
        match unary_exp.operator {
            ast::Operator::LogicalNot => self.resolve_types(&operand_type, &Type::boolean()),
            _ => self.resolve_types(&operand_type, &Type::number()),
        }
    }

    fn visit_namespace_expression(&'a self, namespace_exp: &ast::NamespaceExpression<'a>) -> Type {
        todo!()
    }

    // Typecheck assignment expression.
    fn visit_assignment_expression(&'a self, assign_exp: &ast::AssignmentExpression<'a>) -> Type {
        if let ast::Expression::IdentifierExpression(identifier) = &assign_exp.operands[0] {
            let atom_option = self.values.borrow().lookup(identifier.value).cloned();
            // left hand side does not exist.
            if atom_option.is_none() {
                self.errors.borrow_mut().push((
                    SemanticError::Undeclared(String::from(identifier.value)),
                    identifier.span,
                ));
                return Type::Uninferrable;
            }

            let mut atom = atom_option.unwrap();

            // Left hand side is constant.
            if atom.is_constant {
                self.errors
                    .borrow_mut()
                    .push((SemanticError::AssignmentToConst, assign_exp.span));
                return atom.given_type;
            }

            let assignment_type = {
                let rhs = self.visit_expression(&assign_exp.operands[1]);
                let result_type = self.resolve_types(&atom.given_type, &rhs);
                match assign_exp.operator {
                    ast::Operator::Assign => result_type,
                    ast::Operator::AddAssign
                        if result_type.is_string()
                            || result_type.is_number()
                            || result_type.is_any() =>
                    {
                        result_type
                    }
                    ast::Operator::SubtractAssign
                    | ast::Operator::DivideAssign
                    | ast::Operator::MultiplyAssign
                        if result_type.is_number() || result_type.is_any() =>
                    {
                        result_type
                    }
                    ast::Operator::LogicalAndAssign | ast::Operator::LogicalOrAssign
                        if result_type.is_boolean() || result_type.is_any() =>
                    {
                        result_type
                    }
                    _ => {
                        self.errors.borrow_mut().push((
                            SemanticError::UnsupportedBinaryOperation(
                                assign_exp.operator.clone(),
                                atom.given_type.clone(),
                                rhs,
                            ),
                            assign_exp.span,
                        ));
                        atom.given_type.clone()
                    }
                }
            };

            // Initialize uninitialized variable. Only initialize if the operator is =.
            if !atom.is_initialized {
                if let ast::Operator::Assign = assign_exp.operator {
                    atom.is_initialized = true;
                } else {
                    self.errors.borrow_mut().push((
                        SemanticError::Uninitialized(String::from(identifier.value)),
                        identifier.span,
                    ));
                }
            };

            if !assignment_type.is_indefinite() {
                atom.given_type = assignment_type.clone()
            }
            self.values.borrow_mut().set(identifier.value, atom);

            assignment_type
        } else {
            Type::Uninferrable
        }
    }

    // Typecheck an index expression.
    fn visit_index_expression(&'a self, index_exp: &ast::IndexExpression<'a>) -> Type {
        let accessor_type = self.visit_expression(&index_exp.accessor_and_property[0]);
        let property_type = self.visit_expression(&index_exp.accessor_and_property[1]);

        if !accessor_type.is_array() {
            self.errors
                .borrow_mut()
                .push((SemanticError::InvalidIndex(accessor_type), index_exp.span));
            return Type::Uninferrable;
        }
        if !property_type.is_number() {
            self.errors
                .borrow_mut()
                .push((SemanticError::InvalidIndexer(property_type), index_exp.span));
        }
        // Retrieve the argument passed to the array.
        if let Type::Instance {
            arguments: Some(v), ..
        } = accessor_type
        {
            return v[0].clone();
        } else {
            unreachable!()
        }
    }

    fn visit_call_expression(&'a self, call_exp: &ast::CallExpression<'a>) -> Type {
        todo!()
    }

    // Typecheck array expression.
    fn visit_array_expression(&'a self, array_exp: &ast::ArrayExpression<'a>) -> Type {
        if array_exp.elements.len() == 0 {
            return Type::array(Type::Unknown);
        }
        // Compare all known element types to the first type.
        let first_type = self.visit_expression(&array_exp.elements[0]);
        for element in &array_exp.elements {
            let element_type = self.resolve_types(&first_type, &self.visit_expression(element));
            if element_type.is_any() {
                return Type::array(Type::Any);
            } else if element_type.is_nil() {
                self.errors
                    .borrow_mut()
                    .push((SemanticError::AssigningToNil, element.get_range()));
                return Type::array(Type::Uninferrable);
            } else if element_type.is_uninferrable() {
                if first_type != element_type {
                    self.errors.borrow_mut().push((
                        SemanticError::HeterogenousArray(first_type.clone(), element_type),
                        array_exp.span,
                    ))
                }
            }
        }
        Type::array(first_type)
    }

    // Typecheck ternary expression.
    fn visit_ternary_expression(&'a self, tern_exp: &ast::TernaryExpression<'a>) -> Type {
        let test_type = self.visit_expression(&tern_exp.test);
        let consequent_type = self.visit_expression(&tern_exp.consequent);
        let alternate_type = self.visit_expression(&tern_exp.alternate);

        // Confirm that the test expression is boolean.
        if !(test_type.is_boolean() || test_type.is_any()) {
            self.errors.borrow_mut().push((
                SemanticError::InvalidTernaryTest(test_type),
                tern_exp.test.get_range(),
            ))
        }
        // Confirm that the consequent and alternate expressions produce values of the same type.
        let ternary_type = self.resolve_types(&consequent_type, &alternate_type);
        if consequent_type != alternate_type && ternary_type.is_uninferrable() {
            self.errors.borrow_mut().push((
                SemanticError::InconsistentTernarySides(consequent_type, alternate_type),
                tern_exp.span,
            ));
        }
        ternary_type
    }

    // Typecheck range expression.
    fn visit_range_expression(&'a self, rang_exp: &ast::RangeExpression<'a>) -> Type {
        let lower_type = self.visit_expression(&rang_exp.boundaries[0]);
        let upper_type = self.visit_expression(&rang_exp.boundaries[1]);

        let range_type = self.resolve_types(&lower_type, &upper_type);
        if (lower_type != upper_type && range_type.is_uninferrable())
            || !(range_type.is_number() || range_type.is_character())
        {
            self.errors
                .borrow_mut()
                .push((SemanticError::InvalidRangeBoundaries, rang_exp.span))
        }
        range_type
    }

    fn visit_function_expression(&'a self, fn_exp: &ast::FnExpression<'a>) -> Type {
        todo!()
    }

    fn visit_if_statement(&'a self, if_stmnt: &ast::IfStatement<'a>) {
        todo!()
    }

    fn visit_prepend_statement(&'a self, prepend_stmnt: &ast::PrependStatement<'a>) {
        todo!()
    }
    /// Typecheck a variable declaration.
    fn visit_variable_declaration(&'a self, var_decl: &ast::VariableDeclaration<'a>) {
        let name = var_decl.name.value;
        let is_declared = true;
        let is_initialized = var_decl.initializer.is_some();
        let is_constant = var_decl.kind.is_const();

        // Block redeclaring in the same scope.
        if self.values.borrow().get(name).is_some() {
            self.errors.borrow_mut().push((
                SemanticError::AlreadyDeclared(String::from(name)),
                var_decl.span,
            ));
            return;
        }

        // Check type label, if it exists.
        let mut given_type;
        if let Some(label) = &var_decl.type_label {
            given_type = self.create_instance(label);
        } else {
            given_type = Type::Unknown;
        }

        let inferred_type;
        // Check initializer, if it exists.
        if let Some(expression) = &var_decl.initializer {
            inferred_type = self.visit_expression(expression);
        } else {
            inferred_type = Type::Unknown;
        }

        let final_type = self.resolve_types(&given_type, &inferred_type);

        if final_type.is_uninferrable() {
            if given_type != inferred_type {
                self.errors.borrow_mut().push((
                    SemanticError::InconsistentAssignment(given_type, inferred_type),
                    var_decl.span,
                ));
            } else {
                self.errors.borrow_mut().push((
                    SemanticError::UnknownAssignment(String::from(name)),
                    var_decl.span,
                ));
            }
        } else if final_type.is_nil() {
            self.errors
                .borrow_mut()
                .push((SemanticError::OperationOnNil, var_decl.span));
            given_type = Type::Uninferrable;
        }

        self.values.borrow_mut().set(
            name,
            Atom {
                source_node: var_decl.name.clone(),
                given_type: final_type,
                is_initialized,
                is_constant,
            },
        )
    }

    fn visit_break(&'a self, break_: &ast::Break) {
        todo!()
    }

    fn visit_continue(&'a self, continue_: &ast::Continue<'a>) {
        todo!()
    }

    fn visit_test_block(&'a self, test_block: &ast::TestBlock<'a>) {
        todo!()
    }

    fn visit_loop_statement(&'a self, loop_stmnt: &ast::Loop<'a>) {
        todo!()
    }

    fn visit_for_statement(&'a self, for_loop: &ast::ForLoop<'a>) {
        todo!()
    }

    fn visit_while_statement(&'a self, while_stmnt: &ast::WhileStatement<'a>) {
        todo!()
    }

    fn visit_public_statement(&'a self, public_mod: &ast::PublicModifier<'a>) {
        todo!()
    }

    fn visit_block(&'a self, block: &ast::Block<'a>) {
        todo!()
    }

    fn visit_use_import(&'a self, use_stmnt: &ast::UseImport<'a>) {
        todo!()
    }

    fn visit_return_statement(&'a self, return_stmnt: &ast::ReturnStatement<'a>) {
        todo!()
    }

    fn visit_crash(&'a self, crash: &ast::CrashStatement<'a>) {
        todo!()
    }

    fn visit_try_block(&'a self, try_block: &ast::TryBlock<'a>) {
        todo!()
    }

    fn visit_function(&'a self, function: &ast::Function<'a>) {
        todo!()
    }

    fn visit_enum_declaration(&'a self, enum_: &ast::Enum<'a>) {
        todo!()
    }

    fn visit_record_declaration(&'a self, record: &ast::Record<'a>) {
        todo!()
    }

    fn visit_class_declaration(&'a self, class: &ast::Class<'a>) {
        todo!()
    }

    fn visit_property(&'a self, property: &ast::Property<'a>) -> Type {
        todo!()
    }

    fn visit_method(&'a self, method: &ast::Method<'a>) -> Type {
        todo!()
    }

    fn visit_constructor(&'a self, method: &ast::Method<'a>) -> Type {
        todo!()
    }

    fn visit_attribute(&'a self, attrib: &ast::Attribute<'a>) -> Type {
        todo!()
    }

    fn visit_implement(&'a self, implement: &ast::Implement<'a>) -> Type {
        todo!()
    }

    fn visit_record_mapping(&'a self, map: &ast::Mapping<'a>) {
        todo!()
    }

    fn visit_variant(&'a self, variant: &ast::Variant<'a>) {
        todo!()
    }

    fn visit_parameter(&'a self, param: &ast::Parameter<'a>) -> Type {
        todo!()
    }

    fn visit_type_alias(&'a self, type_alias: &ast::TypeAlias<'a>) {
        todo!()
    }

    fn visit_type_label(&'a self, label: &ast::Type<'a>) -> Type {
        todo!()
    }

    fn visit_concrete_type(&'a self, concrete_type: &ast::ConcreteType<'a>) -> Type {
        todo!()
    }

    fn visit_functional_type(&'a self, functional_type: &ast::FunctionType<'a>) -> Type {
        todo!()
    }

    fn visit_interface(&'a self, interface: &ast::Interface<'a>) {
        todo!()
    }

    fn visit_generic_argument(&'a self, argument: &ast::GenericArgument) {
        todo!()
    }
}
