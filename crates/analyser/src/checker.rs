#![allow(unused)]

use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    collections::HashMap,
    fmt::Display,
    hash::Hash,
    string,
};

use ast::{Expression, Identifier, Statement, TextSpan, Visitor};
use errors::SemanticError;
use utils::Stage;

pub struct Atom<'a> {
    source_node: Identifier<'a>,
    given_type: Type,
    is_initialized: bool,
    is_constant: bool,
}

type TypeErrors = Vec<(SemanticError<Type>, TextSpan)>;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Unknown,
    Uninferable,
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

    /// Returns `true` if the type is [`Uninferable`].
    ///
    /// [`Uninferable`]: Type::Uninferable
    pub fn is_uninferable(&self) -> bool {
        matches!(self, Self::Uninferable)
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
    }

    /// Returns `true` if the type is [`Nil`].
    ///
    /// [`Nil`]: Type::Nil
    pub fn is_nil(&self) -> bool {
        matches!(self, Self::Nil)
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

pub struct Checker<'a> {
    values: RefCell<Stage<&'a str, Atom<'a>>>,
    classes: RefCell<Stage<&'a str, TypeClass>>,
    errors: RefCell<TypeErrors>,
}

impl<'a> Checker<'a> {
    /// Type checks a sequence of statements and returns a list of errors encountered.
    pub fn check(statements: Vec<Statement<'a>>) -> TypeErrors {
        let checker = Checker {
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

impl<'a> Checker<'a> {
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
                    return Type::Uninferable;
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
                        return Type::Uninferable;
                    }
                    return Type::from_class(class, None);
                }
                // Some generics.
                let generics = class.generics.unwrap();
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
                    return Type::Uninferable;
                }
                todo!()
            }
            ast::Type::Function(_) => todo!(),
            ast::Type::Dot(_) => todo!(),
        }
    }

    /// Checks if two types can be coerced into each other, with the first type taking precedence over the other.
    fn resolve_types(&'a self, type1: &Type, type2: &Type) -> Type {
        match (type1, type2) {
            // If at least type is uninferable.
            (_, Type::Uninferable) | (Type::Uninferable, _) => Type::Uninferable,
            // If the second type is never.
            (x, Type::Never) => x.clone(),
            // If either type is unknown.
            (x, Type::Unknown) | (Type::Unknown, x) => {
                if x.is_unknown() {
                    Type::Uninferable
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
            // Otherwise
            _ => Type::Uninferable,
        }
    }
}

impl<'a> Visitor<'a, Type> for Checker<'a> {
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
            Type::Uninferable
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
            || type1.is_uninferable()
            || type2.is_uninferable()
        {
            Type::Uninferable
        }
        // Block operation on nil values.
        else if type1.is_nil() || type2.is_nil() {
            self.errors
                .borrow_mut()
                .push((SemanticError::OperationOnNil, bin_exp.span));
            Type::Uninferable
        } else if type1.is_any() || type2.is_any() {
            Type::Any
        } else if type1 != type2 {
            self.errors.borrow_mut().push((
                SemanticError::UnsupportedBinaryOperation(operator, type1, type2),
                bin_exp.span,
            ));
            Type::Uninferable
        } else {
            match operator {
                ast::Operator::Add => {
                    // string + string = string
                    // number + number = number
                    if type1 == Type::string() || type1 == Type::number() {
                        type1
                    } else {
                        self.errors.borrow_mut().push((
                            SemanticError::UnsupportedBinaryOperation(operator, type1, type2),
                            bin_exp.span,
                        ));
                        Type::Uninferable
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
                        Type::Uninferable
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
            || type1.is_uninferable()
            || type2.is_uninferable()
        {
            Type::Uninferable
        }
        // Block operation on nil values.
        else if type1.is_nil() || type2.is_nil() {
            self.errors
                .borrow_mut()
                .push((SemanticError::OperationOnNil, log_exp.span));
            Type::Uninferable
        } else if type1.is_any() || type2.is_any() {
            Type::Any
        } else if type1 != type2 {
            self.errors.borrow_mut().push((
                SemanticError::UnsupportedBinaryOperation(operator, type1, type2),
                log_exp.span,
            ));
            Type::Uninferable
        } else {
            match log_exp.operator {
                ast::Operator::LogicalAnd | ast::Operator::LogicalOr => {
                    // boolean || boolean = boolean
                    // boolean && boolean = boolean
                    if type1 == Type::boolean() {
                        type1
                    } else {
                        self.errors.borrow_mut().push((
                            SemanticError::UnsupportedLogicalOperation(operator, type1, type2),
                            log_exp.span,
                        ));
                        Type::Uninferable
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    fn visit_dot_expression(&'a self, dot_exp: &ast::DotExpression<'a>) -> Type {
        todo!()
    }

    fn visit_unary_expression(&'a self, unary_exp: &ast::UnaryExpression<'a>) -> Type {
        todo!()
    }

    fn visit_namespace_exp(&'a self, namespace_exp: &ast::NamespaceExpression<'a>) -> Type {
        todo!()
    }

    fn visit_assign_expression(&'a self, assign_exp: &ast::AssignmentExpression<'a>) -> Type {
        todo!()
    }

    fn visit_index_expression(&'a self, index_exp: &ast::IndexExpression<'a>) -> Type {
        todo!()
    }

    fn visit_call_expression(&'a self, call_exp: &ast::CallExpression<'a>) -> Type {
        todo!()
    }

    fn visit_array_expression(&'a self, array_exp: &ast::ArrayExpression<'a>) -> Type {
        todo!()
    }

    fn visit_ternary_expression(&'a self, tern_exp: &ast::TernaryExpression<'a>) -> Type {
        todo!()
    }

    fn visit_range_expression(&'a self, rang_exp: &ast::RangeExpression<'a>) -> Type {
        todo!()
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

        if final_type.is_uninferable() {
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
            given_type = Type::Uninferable;
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
