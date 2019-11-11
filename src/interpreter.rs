use crate::error::Error;
use crate::syntax::expr::Visitor;
use crate::syntax::{Expr, LiteralValue};
use crate::token::{Token, TokenType};

/// A simple representation of an Lox object akin to a Java `Object`.
enum Object {
    Boolean(bool),
    Null,
    Number(f64),
    String(String),
}

impl Object {
    fn equals(&self, other: &Object) -> bool {
        match (self, other) {
            (Object::Null, Object::Null) => true,
            (_, Object::Null) => false,
            (Object::Null, _) => false,
            (Object::Boolean(left), Object::Boolean(right)) => left == right,
            (Object::Number(left), Object::Number(right)) => left == right,
            (Object::String(left), Object::String(right)) => left.eq(right),
            _ => false,
        }
    }
}

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(&self, expression: &Expr) -> Result<String, Error> {
        self.evaluate(expression).map(|value| self.stringify(value))
    }

    fn evaluate(&self, expression: &Expr) -> Result<Object, Error> {
        expression.accept(self)
    }

    fn is_truthy(&self, object: &Object) -> bool {
        match object {
            Object::Null => false,
            Object::Boolean(b) => b.clone(),
            _ => true,
        }
    }

    fn is_equal(&self, left: &Object, right: &Object) -> bool {
        left.equals(right)
    }

    fn stringify(&self, object: Object) -> String {
        match object {
            Object::Null => "nil".to_string(),
            Object::Number(n) => n.to_string(),
            Object::Boolean(b) => b.to_string(),
            Object::String(s) => s,
        }
    }

    /// Equivalent to checkNumberOperands
    fn number_operand_error<R>(&self, operator: &Token) -> Result<R, Error> {
        Err(Error::Runtime {
            token: operator.clone(),
            message: "Operand must be a number.".to_string(),
        })
    }
}

impl Visitor<Object> for Interpreter {
    fn visit_binary_expr(
        &self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Object, Error> {
        let l = self.evaluate(left)?;
        let r = self.evaluate(right)?;

        match &operator.tpe {
            TokenType::Greater => match (l, r) {
                (Object::Number(left_number), Object::Number(right_number)) => {
                    Ok(Object::Boolean(left_number > right_number))
                }
                _ => self.number_operand_error(operator),
            },
            TokenType::GreaterEqual => match (l, r) {
                (Object::Number(left_number), Object::Number(right_number)) => {
                    Ok(Object::Boolean(left_number >= right_number))
                }
                _ => self.number_operand_error(operator),
            },
            TokenType::Less => match (l, r) {
                (Object::Number(left_number), Object::Number(right_number)) => {
                    Ok(Object::Boolean(left_number < right_number))
                }
                _ => self.number_operand_error(operator),
            },
            TokenType::LessEqual => match (l, r) {
                (Object::Number(left_number), Object::Number(right_number)) => {
                    Ok(Object::Boolean(left_number <= right_number))
                }
                _ => self.number_operand_error(operator),
            },
            TokenType::Minus => match (l, r) {
                (Object::Number(left_number), Object::Number(right_number)) => {
                    Ok(Object::Number(left_number - right_number))
                }
                _ => self.number_operand_error(operator),
            },
            TokenType::Plus => match (l, r) {
                (Object::Number(left_number), Object::Number(right_number)) => {
                    Ok(Object::Number(left_number + right_number))
                }
                (Object::String(left_string), Object::String(right_string)) => {
                    Ok(Object::String(left_string.clone() + &right_string))
                }
                _ => Err(Error::Runtime {
                    token: operator.clone(),
                    message: "Operands must be two numbers or two strings.".to_string(),
                }),
            },
            TokenType::Slash => match (l, r) {
                (Object::Number(left_number), Object::Number(right_number)) => {
                    Ok(Object::Number(left_number / right_number))
                }
                _ => self.number_operand_error(operator),
            },
            TokenType::Star => match (l, r) {
                (Object::Number(left_number), Object::Number(right_number)) => {
                    Ok(Object::Number(left_number * right_number))
                }
                _ => self.number_operand_error(operator),
            },
            TokenType::BangEqual => Ok(Object::Boolean(!self.is_equal(&l, &r))),
            TokenType::EqualEqual => Ok(Object::Boolean(self.is_equal(&l, &r))),
            _ => unreachable!(),
        }
    }

    fn visit_grouping_expr(&self, expr: &Expr) -> Result<Object, Error> {
        self.evaluate(expr)
    }

    fn visit_literal_expr(&self, value: &LiteralValue) -> Result<Object, Error> {
        match value {
            LiteralValue::Boolean(b) => Ok(Object::Boolean(b.clone())),
            LiteralValue::Null => Ok(Object::Null),
            LiteralValue::Number(n) => Ok(Object::Number(n.clone())),
            LiteralValue::String(s) => Ok(Object::String(s.clone())),
        }
    }

    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> Result<Object, Error> {
        let right = self.evaluate(right)?;

        match &operator.tpe {
            TokenType::Minus => match right {
                Object::Number(n) => Ok(Object::Number(-n.clone())),
                _ => self.number_operand_error(operator),
            },
            TokenType::Bang => Ok(Object::Boolean(!self.is_truthy(&right))), // TODO: is_truthy could simply return an Object.
            _ => unreachable!(), // TODO: fail if right is not a number.
        }
    }
}
