use crate::error::Error;
use crate::object::Object;
use crate::token::Token;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>, // Parent
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn from(enclosing: &Rc<RefCell<Environment>>) -> Self {
        Environment {
            enclosing: Some(Rc::clone(enclosing)),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    fn ancestor(&self, distance: usize) -> Rc<RefCell<Environment>> {
        // Get first ancestor
        let parent = self
            .enclosing
            .clone()
            .expect(&format!("No enclosing environment at {}", 1));
        let mut environment = Rc::clone(&parent);

        // Get next ancestors
        for i in 1..distance {
            let parent = environment
                .borrow()
                .enclosing
                .clone()
                .expect(&format!("No enclosing environment at {}", i));
            environment = Rc::clone(&parent);
        }
        environment
    }

    pub fn get_at(&self, distance: usize, name: &Token) -> Result<Object, Error> {
        let key = &*name.lexeme;
        if distance > 0 {
            Ok(self
                .ancestor(distance)
                .borrow()
                .values
                .get(key)
                .expect(&format!("Undefined variable '{}'", key))
                .clone())
        } else {
            Ok(self
                .values
                .get(key)
                .expect(&format!("Undefined variable '{}'", key))
                .clone())
        }
    }

    pub fn assign_at(&mut self, distance: usize, name: &Token, value: Object) -> Result<(), Error> {
        if distance > 0 {
            self.ancestor(distance)
                .borrow_mut()
                .values
                .insert(name.lexeme.clone(), value);
        } else {
            self.values.insert(name.lexeme.clone(), value);
        }
        Ok(())
    }

    pub fn get(&self, name: &Token) -> Result<Object, Error> {
        let key = &*name.lexeme;
        if let Some(value) = self.values.get(key) {
            Ok((*value).clone())
        } else {
            if let Some(ref enclosing) = self.enclosing {
                enclosing.borrow().get(name)
            } else {
                Err(Error::Runtime {
                    token: name.clone(),
                    message: format!("Undefined variable '{}'.", key),
                })
            }
        }
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), Error> {
        let key = &*name.lexeme;
        if self.values.contains_key(key) {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        } else {
            if let Some(ref enclosing) = self.enclosing {
                enclosing.borrow_mut().assign(name, value)
            } else {
                Err(Error::Runtime {
                    token: name.clone(),
                    message: format!("Undefined variable '{}'", key),
                })
            }
        }
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "values: {:?}", self.values)
    }
}
