use crate::interpreter::Interpreter;
use crate::object::Object;

#[derive(Debug, Clone)]
pub struct LoxCallable {
    pub arity: usize,
    //    fn call(interpreter: &mut Interpreter, arguments: &Vec<Object>) -> Object;
}
