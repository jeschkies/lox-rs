use crate::function::Function;

/// A simple representation of an Lox object akin to a Java `Object`.
#[derive(Debug, Clone)]
pub enum Object {
    Boolean(bool),
    Callable(Function),
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
