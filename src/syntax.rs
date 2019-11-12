use crate::error::Error;
use crate::token::Token;
use std::fmt;

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

pub enum LiteralValue {
    Boolean(bool),
    Null,
    Number(f64),
    String(String),
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralValue::Boolean(b) => write!(f, "{}", b),
            LiteralValue::Null => write!(f, "null"),
            LiteralValue::Number(n) => write!(f, "{}", n),
            LiteralValue::String(s) => write!(f, "{}", s),
        }
    }
}

impl Expr {
    pub fn accept<R>(&self, visitor: &expr::Visitor<R>) -> Result<R, Error> {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary_expr(left, operator, right),
            Expr::Grouping { expression } => visitor.visit_grouping_expr(expression),
            Expr::Literal { value } => visitor.visit_literal_expr(value),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
        }
    }
}

pub mod expr {
    use super::{Expr, LiteralValue};
    use crate::error::Error;
    use crate::token::Token;

    pub trait Visitor<R> {
        fn visit_binary_expr(
            &self,
            left: &Expr,
            operator: &Token,
            right: &Expr,
        ) -> Result<R, Error>;

        /// Visit a grouping expression.
        ///
        /// # Arguments
        ///
        /// * `expression` - This is the *inner* expression of the grouping.
        fn visit_grouping_expr(&self, expression: &Expr) -> Result<R, Error>;
        fn visit_literal_expr(&self, value: &LiteralValue) -> Result<R, Error>;
        fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> Result<R, Error>;
    }
}

pub enum Stmt {
    Block { statements: Vec<Stmt> },
    Expression { expression: Expr },
    Print { expression: Expr },
    Var { name: Token, initializer: Expr },
}

impl Stmt {
    pub fn accept<R>(&self, visitor: &stmt::Visitor<R>) -> Result<R, Error> {
        match self {
            Stmt::Block { statements } => visitor.visit_block_stmt(statements),
            Stmt::Expression { expression } => visitor.visit_expression_stmt(expression),
            Stmt::Print { expression } => visitor.visit_print_stmt(expression),
            Stmt::Var { name, initializer } => visitor.visit_var_stmt(name, initializer),
        }
    }
}

mod stmt {
    use super::{Expr, Stmt};
    use crate::error::Error;
    use crate::token::Token;

    pub trait Visitor<R> {
        fn visit_block_stmt(&self, statements: &Vec<Stmt>) -> Result<R, Error>;
        //        fn visit_class_stmt(&self, Class stmt); TODO: Classes chapter
        fn visit_expression_stmt(&self, expression: &Expr) -> Result<R, Error>;
        //        fn visit_function_stmt(&self, Function stmt); TODO: Functions chapter
        //        fn visit_if_stmt(&self, If stmt); TODO: Control Flows chapter
        fn visit_print_stmt(&self, expression: &Expr) -> Result<R, Error>;
        //        fn visit_return_stmt(&self, Return stmt); TODO: Functions chapter
        fn visit_var_stmt(&self, name: &Token, initializer: &Expr) -> Result<R, Error>;
        //        fn visit_while_stmt(&self, While stmt); TODO: Control Flows chapter
    }
}

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: Expr) -> Result<String, Error> {
        expr.accept(self)
    }

    fn parenthesize(&self, name: String, exprs: Vec<&Expr>) -> Result<String, Error> {
        let mut r = String::new();
        r.push_str("(");
        r.push_str(&name);
        for e in &exprs {
            r.push_str(" ");
            r.push_str(&e.accept(self)?);
        }
        r.push_str(")");
        Ok(r)
    }
}

impl expr::Visitor<String> for AstPrinter {
    fn visit_binary_expr(
        &self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<String, Error> {
        self.parenthesize(operator.lexeme.clone(), vec![left, right])
    }

    fn visit_grouping_expr(&self, expr: &Expr) -> Result<String, Error> {
        self.parenthesize("group".to_string(), vec![expr])
    }

    fn visit_literal_expr(&self, value: &LiteralValue) -> Result<String, Error> {
        Ok(value.to_string()) // check for null
    }

    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> Result<String, Error> {
        self.parenthesize(operator.lexeme.clone(), vec![right])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Token, TokenType};

    #[test]
    fn test_printer() {
        let expression = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: Token::new(TokenType::Minus, "-", 1),
                right: Box::new(Expr::Literal {
                    value: LiteralValue::Number(123f64),
                }),
            }),
            operator: Token::new(TokenType::Star, "*", 1),
            right: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Literal {
                    value: LiteralValue::Number(45.67f64),
                }),
            }),
        };
        let printer = AstPrinter;

        assert_eq!(
            printer.print(expression).unwrap(),
            "(* (- 123) (group 45.67))"
        );
    }
}
