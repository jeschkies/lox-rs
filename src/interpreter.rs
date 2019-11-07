use crate::syntax::{Expr, LiteralValue, Visitor};
use crate::token::{Token, TokenType};

/// A simple representation of an Lox object akin to a Java `Object`.
enum Object {
    Boolean(bool),
    Null,
    Number(f64),
    String(String),
}

pub struct Interpreter;

impl Interpreter {
    fn evaluate(&self, expression: &Expr) -> Object {
        expression.accept(self)
    }

    fn is_truthy(&self, object: &Object) -> bool {
        match object {
            Object::Null => false,
            Object::Boolean(b) => b.clone(),
            _ => true,
        }
    }
}

impl Visitor<Object> for Interpreter {

    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> Object {
        let l = self.evaluate(left);
        let r = self.evaluate(right);

        match (l, &operator.tpe, r) {
            (Object::Number(left_number), TokenType::Minus, Object::Number(right_number)) => {
                Object::Number(left_number - right_number)
            }
            (Object::Number(left_number), TokenType::Plus, Object::Number(right_number)) => {
                Object::Number(left_number + right_number)
            }
            (Object::String(left_string), TokenType::Plus, Object::String(right_string)) => {
                Object::String(left_string.clone() + &right_string)
            }
            (Object::Number(left_number), TokenType::Slash, Object::Number(right_number)) => {
                Object::Number(left_number / right_number)
            }
            (Object::Number(left_number), TokenType::Star, Object::Number(right_number)) => {
                Object::Number(left_number * right_number)
            }
            _ => unreachable!(), // TODO: handle other types
        }
    }

    fn visit_grouping_expr(&self, expr: &Expr) -> Object {
        self.evaluate(expr)
    }

    fn visit_literal_expr(&self, value: &LiteralValue) -> Object {
        match value {
            LiteralValue::Boolean(b) => Object::Boolean(b.clone()),
            LiteralValue::Null => Object::Null,
            LiteralValue::Number(n) => Object::Number(n.clone()),
            LiteralValue::String(s) => Object::String(s.clone()),
        }
    }

    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> Object {
        let right = self.evaluate(right);

        match (&operator.tpe, &right) {
            (TokenType::Minus, Object::Number(n)) => Object::Number(-n.clone()),
            (TokenType::Bang, _) => Object::Boolean(!self.is_truthy(&right)), // TODO: is_truthy could simply return an Object.
            _ => unreachable!(), // TODO: fail if right is not a number.
        }
    }
}
