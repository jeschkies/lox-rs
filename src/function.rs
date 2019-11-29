use crate::interpreter::Interpreter;
use crate::object::Object;

use std::fmt;

#[derive(Clone)]
pub struct LoxCallable {
    pub arity: usize,
    pub body: Box<fn(&Vec<Object>) -> Object>,
}

impl LoxCallable {
    pub fn call(&self, interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Object {
        (self.body)(arguments)
    }
}

impl fmt::Debug for LoxCallable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native func>{}", self.arity)
    }
}

impl fmt::Display for LoxCallable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native func>{}", self.arity)
    }
}
