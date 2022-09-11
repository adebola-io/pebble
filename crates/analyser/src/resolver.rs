use std::collections::HashMap;

use ast::TextSpan;

pub struct Resolver<'a> {
    _scopes: Vec<HashMap<&'a str, Item>>,
}

pub struct Item {
    pub _type: ItemType,
    pub declared_at: TextSpan,
}

pub enum ItemType {
    Class,
    Function { parameters: () },
    Constant,
    Variable,
    Type,
}
