use crate::error::Error;
use crate::function::Function;
use crate::object::Object;
use crate::token::Token;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct LoxClass {
    pub name: String,
    pub superclass: Option<Rc<RefCell<LoxClass>>>,
    pub methods: HashMap<String, Function>,
}

impl LoxClass {
    pub fn find_method(&self, name: &str) -> Option<Function> {
        if self.methods.contains_key(name) {
            self.methods.get(name).map(|f| f.clone())
        } else {
            if let Some(ref superclass) = self.superclass {
                superclass.borrow().find_method(name)
            } else {
                None
            }
        }
    }
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

    /// Returns a member field of this instance.
    ///
    /// # Args
    /// * name - The name of the member.
    /// * instance - A reference to this instance as an object.
    pub fn get(&self, name: &Token, instance: &Object) -> Result<Object, Error> {
        if let Some(field) = self.fields.get(&name.lexeme) {
            Ok(field.clone())
        } else if let Some(method) = self.class.borrow().find_method(&name.lexeme) {
            Ok(Object::Callable(method.bind(instance.clone())))
        } else {
            Err(Error::Runtime {
                token: name.clone(),
                message: format!("Undefined property '{}'.", name.lexeme),
            })
        }
    }

    pub fn set(&mut self, name: &Token, value: Object) {
        self.fields.insert(name.lexeme.clone(), value);
    }
}
