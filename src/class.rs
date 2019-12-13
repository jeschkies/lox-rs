use crate::error::Error;
use crate::object::Object;
use crate::token::Token;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct LoxClass {
    pub name: String,
}

#[derive(Debug)]
pub struct LoxInstance {
    pub class: Rc<RefCell<LoxClass>>,
    fields: HashMap<String, Object>,
}

impl LoxInstance {
    /// Returns a new `LoxInstance` wrapped in an `Object::Instance`.
    pub fn new(class: &Rc<RefCell<LoxClass>>) -> Object {
        let instance = LoxInstance {
            class: Rc::clone(class),
            fields: HashMap::new(),
        };
        Object::Instance(Rc::new(RefCell::new(instance)))
    }

    pub fn get(&self, name: &Token) -> Result<Object, Error> {
        match self.fields.get(&name.lexeme) {
            Some(field) => Ok(field.clone()),
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
