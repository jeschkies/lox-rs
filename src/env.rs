use crate::error::Error;
use crate::object::Object;
use crate::token::Token;

use std::collections::HashMap;
use std::fmt;

pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Object, Error> {
        let key = &*name.lexeme;
        if let Some(value) = self.values.get(key) {
            Ok((*value).clone())
        } else {
            Err(Error::Runtime {
                token: name.clone(),
                message: format!("Undefined variable '{}'.", key),
            })
        }
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), Error> {
        let key = &*name.lexeme;
        if self.values.contains_key(key) {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        } else {
            Err(Error::Runtime {
                token: name.clone(),
                message: format!("Undefined variable '{}'", key),
            })
        }
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "values: {:?}", self.values)
    }
}
