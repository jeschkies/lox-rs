use crate::error::{parser_error, Error};
use crate::syntax::{Expr, LiteralValue, Stmt};
use crate::token::{Token, TokenType};

pub struct Parser<'t> {
    tokens: &'t Vec<Token>,
    current: usize,
}

/// AKA match in Chapter 6.
macro_rules! matches {
    ( $sel:ident, $( $x:expr ),* ) => {
        {
            if $( $sel.check($x) )||* {
                $sel.advance();
                true
            } else {
                false
            }
        }
    };
}

impl<'t> Parser<'t> {
    pub fn new(tokens: &'t Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut statments: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            statments.push(self.statement()?);
        }
        Ok(statments)
    }

    fn expression(&mut self) -> Result<Expr, Error> {
        self.equality()
    }

    fn statement(&mut self) -> Result<Stmt, Error> {
        if matches!(self, TokenType::Print) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, Error> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print { expression: value })
    }

    fn expression_statement(&mut self) -> Result<Stmt, Error> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression { expression: expr })
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr = self.comparison()?;

        while matches!(self, TokenType::BangEqual, TokenType::EqualEqual) {
            let operator: Token = (*self.previous()).clone();
            let right: Expr = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        token_type == self.peek().tpe
    }

    fn consume(&mut self, tpe: TokenType, message: &str) -> Result<Token, Error> {
        if self.check(tpe) {
            Ok(self.advance().clone())
        } else {
            Err(self.error(self.peek(), message))
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> &Token {
        self.tokens
            .get(self.current - 1)
            .expect("Previous was empty.")
    }

    fn error(&self, token: &Token, message: &str) -> Error {
        parser_error(token, message);
        Error::Parse
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().tpe == TokenType::Semicolon {
                return;
            }

            match self.peek().tpe {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => self.advance(),
            };
        }
    }

    fn peek(&self) -> &Token {
        self.tokens
            .get(self.current)
            .expect("Peek into end of token stream.")
    }

    fn is_at_end(&self) -> bool {
        self.peek().tpe == TokenType::EOF
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
        let mut expr = self.addition()?;

        while matches!(
            self,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual
        ) {
            let operator: Token = self.previous().clone();
            let right = self.addition()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn addition(&mut self) -> Result<Expr, Error> {
        let mut expr = self.multiplication()?;

        while matches!(self, TokenType::Minus, TokenType::Plus) {
            let operator: Token = self.previous().clone();
            let right = self.multiplication()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn multiplication(&mut self) -> Result<Expr, Error> {
        let mut expr = self.unary()?;

        while matches!(self, TokenType::Slash, TokenType::Star) {
            let operator: Token = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if matches!(self, TokenType::Bang, TokenType::Minus) {
            let operator: Token = self.previous().clone();
            let right = self.unary()?;
            Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        // We don't use matches!() here since we want to extract the literals.
        let expr = match &self.peek().tpe {
            TokenType::False => Expr::Literal {
                value: LiteralValue::Boolean(false),
            },
            TokenType::True => Expr::Literal {
                value: LiteralValue::Boolean(true),
            },
            TokenType::Nil => Expr::Literal {
                value: LiteralValue::Null,
            },
            TokenType::String { literal } => Expr::Literal {
                value: LiteralValue::String(literal.clone()),
            },
            TokenType::Number { literal } => Expr::Literal {
                value: LiteralValue::Number(literal.clone()),
            },
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expected ')' after expression.")?;
                Expr::Grouping {
                    expression: Box::new(expr),
                }
            }
            _ => return Err(self.error(self.peek(), "Expect expression.")),
        };

        self.advance();

        Ok(expr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::Scanner;
    use crate::syntax::AstPrinter;

    #[test]
    fn test_parser() {
        let mut scanner = Scanner::new("-123 * 45.67".to_string());
        let tokens = scanner.scan_tokens();

        let mut parser = Parser::new(tokens);
        let expression = parser.parse().expect("Could not parse sample code.");
        let printer = AstPrinter;

        assert_eq!(printer.print(expression).unwrap(), "(* (- 123) 45.67)");
    }
}
