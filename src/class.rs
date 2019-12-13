use crate::error::Error;
use crate::object::Object;
use crate::token::Token;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct LoxInstance {
    pub name: String, // TODO: This should be a ref to the LoxClass.
    fields: HashMap<String, Object>,
}

impl LoxInstance {
    /// Returns a new `LoxInstance` wrapped in an `Object::Instance`.
    pub fn new(name: String) -> Object {
        let instance = LoxInstance {
            name,
            fields: HashMap::new(),
        };
        Object::Instance(Rc::new(RefCell::new(instance)))
    }

    pub fn get(&self, name: &Token) -> Result<Object, Error> {
        match self.fields.get(&name.lexeme) {
            Some(field) => {
                Ok(field.clone())
            }
            None => Err(Error::Runtime {
                token: name.clone(),
                message: format!("Undefined property '{}'.", name.lexeme),
            }),
        }
    }

    pub fn set(&mut self, name: &Token, value: Object) {
        self.fields.insert(name.lexeme.clone(), value);
    }
}
