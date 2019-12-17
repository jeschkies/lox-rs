use crate::env::Environment;
use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::object::Object;
use crate::syntax::Stmt;
use crate::token::{Token, TokenType};

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
        is_initializer: bool,
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
            Function::User {
                params,
                body,
                closure,
                is_initializer,
                ..
            } => {
                let mut environment = Rc::new(RefCell::new(Environment::from(closure)));
                for (param, argument) in params.iter().zip(arguments.iter()) {
                    environment
                        .borrow_mut()
                        .define(param.lexeme.clone(), argument.clone());
                }
                match interpreter.execute_block(body, environment) {
                    Err(Error::Return { value }) => {
                        if *is_initializer {
                            Ok(closure
                                .borrow()
                                .get_at(0, &Token::new(TokenType::This, "this", 0))
                                .expect("Initializer should return 'this'."))
                        } else {
                            Ok(value)
                        }
                    }
                    Err(other) => Err(other),
                    // We don't have a return statement.
                    Ok(..) => {
                        if *is_initializer {
                            Ok(closure
                                .borrow()
                                .get_at(0, &Token::new(TokenType::This, "this", 0))
                                .expect("Initializer should return 'this'."))
                        } else {
                            Ok(Object::Null)
                        }
                    }
                }
            }
        }
    }

    pub fn bind(&self, instance: Object) -> Self {
        match self {
            Function::Native { body, .. } => unreachable!(),
            Function::User {
                name,
                params,
                body,
                closure,
                is_initializer,
            } => {
                let mut environment = Rc::new(RefCell::new(Environment::from(closure)));
                environment
                    .borrow_mut()
                    .define("this".to_string(), instance);
                Function::User {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                    closure: environment,
                    is_initializer: *is_initializer,
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
