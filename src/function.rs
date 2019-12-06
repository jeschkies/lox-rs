use crate::env::Environment;
use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::object::Object;
use crate::syntax::Stmt;
use crate::token::Token;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub enum Function {
    // An anonymous implementation of LoxCallable in the book.
    Native {
        arity: usize,
        body: Box<fn(&Vec<Object>) -> Object>,
    },

    // A LoxFunction in the book.
    User {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
        closure: Rc<RefCell<Environment>>,
    },
}

impl Function {
    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &Vec<Object>,
    ) -> Result<Object, Error> {
        match self {
            Function::Native { body, .. } => Ok(body(arguments)),
            Function::User { params, body, closure, .. } => {
                let mut environment =
                    Rc::new(RefCell::new(Environment::from(closure)));
                for (param, argument) in params.iter().zip(arguments.iter()) {
                    environment
                        .borrow_mut()
                        .define(param.lexeme.clone(), argument.clone());
                }
                match interpreter.execute_block(body, environment) {
                    Err(Error::Return { value }) => Ok(value),
                    Err(other) => Err(other),
                    Ok(..) => Ok(Object::Null), // We don't have a return statement.
                }
            }
        }
    }

    pub fn arity(&self) -> usize {
        match self {
            Function::Native { arity, .. } => *arity,
            Function::User { params, .. } => params.len(),
        }
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Function::Native { .. } => write!(f, "<native func>"),
            Function::User { name, .. } => write!(f, "<fn {}>", name.lexeme),
        }
    }
}

/// This implements the `to_string` aka `toString` from the book.
impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Function::Native { .. } => write!(f, "<native func>"),
            Function::User { name, .. } => write!(f, "<fn {}>", name.lexeme),
        }
    }
}
