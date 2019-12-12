use crate::function::Function;

use std::collections::HashMap;

/// A simple representation of an Lox object akin to a Java `Object`.
#[derive(Debug, Clone)]
pub enum Object {
    Boolean(bool),

    // Called LoxClass in book.
    Class {
        name: String,
    },

    Callable(Function),

    // Called LoxInstance in book.
    Instance {
        name: String,
        fields: HashMap<String, Object>,
    },

    Null,
    Number(f64),
    String(String),
}

impl Object {
    pub fn equals(&self, other: &Object) -> bool {
        match (self, other) {
            (Object::Null, Object::Null) => true,
            (_, Object::Null) => false,
            (Object::Null, _) => false,
            (Object::Boolean(left), Object::Boolean(right)) => left == right,
            (Object::Number(left), Object::Number(right)) => left == right,
            (Object::String(left), Object::String(right)) => left.eq(right),
            _ => false,
        }
    }
}
