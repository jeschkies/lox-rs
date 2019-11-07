use crate::syntax::{Expr, LiteralValue, Visitor};
use crate::token::Token;

/// A simple representation of an Lox object akin to a Java `Object`.
enum Object {
    Boolean(bool),
    Null,
    Number(f64),
    String(String),
}

pub struct Interpreter;

impl Visitor<Object> for Interpreter { // TODO: Book defines result type as Object

    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> Object {
        unimplemented!()
    }

    fn visit_grouping_expr(&self, expr: &Expr) -> Object {
        unimplemented!()
    }

    fn visit_literal_expr(&self, value: &LiteralValue) -> Object {
       match value  {
           LiteralValue::Boolean(b) => Object::Boolean(b.clone()),
           LiteralValue::Null => Object::Null,
           LiteralValue::Number(n) => Object::Number(n.clone()),
           LiteralValue::String(s) => Object::String(s.clone()),
       }
    }

    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> Object {
        unimplemented!()
    }
}