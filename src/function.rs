use crate::env::Environment;
use crate::interpreter::Interpreter;
use crate::object::Object;
use crate::syntax::Stmt;
use crate::token::Token;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub enum Function {
    Native {
        arity: usize,
        body: Box<fn(&Vec<Object>) -> Object>,
    },
    User {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
}

impl Function {
    pub fn call(&self, interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Object {
        match self {
            Function::Native { body, .. } => body(arguments),
            Function::User { params, body, .. } => {
                let mut environment =
                    Rc::new(RefCell::new(Environment::from(&interpreter.globals)));
                for (param, argument) in params.iter().zip(arguments.iter()) {
                    environment
                        .borrow_mut()
                        .define(param.lexeme.clone(), argument.clone());
                }
                interpreter.execute_block(body, environment);
                Object::Null
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
