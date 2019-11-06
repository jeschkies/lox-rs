use crate::syntax::{Expr, LiteralValue};
use crate::token::{Token, TokenType};

struct Parser {
    tokens: Vec<Token>,
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

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while matches!(self, TokenType::BangEqual, TokenType::EqualEqual) {
            let operator: Token = (*self.previous()).clone();
            let right: Expr = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        token_type == self.peek().tpe
    }

    fn advance(&mut self) -> &Token {
        if self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> &Token {
        self.tokens
            .get(self.current - 1)
            .expect("Previous was empty.")
    }

    fn peek(&self) -> &Token {
        self.tokens
            .get(self.current)
            .expect("Peek into end of token stream.")
    }

    fn is_at_end(&self) -> bool {
        self.peek().tpe == TokenType::EOF
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.addition();

        while matches!(
            self,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual
        ) {
            let operator:Token = self.previous().clone();
            let right = self.addition();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        expr
    }

    fn addition(&mut self) -> Expr {
        let mut expr = self.multiplication();

        while matches!(self, TokenType::Minus, TokenType::Plus) {
            let operator: Token = self.previous().clone();
            let right = self.multiplication();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        expr
    }

    fn multiplication(&mut self) -> Expr {
        let mut expr = self.unary();

        while matches!(self, TokenType::Slash, TokenType::Star) {
            let operator: Token = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if matches!(self, TokenType::Bang, TokenType::Minus) {
            let operator: Token = self.previous().clone();
            let right = self.unary();
            Expr::Unary {
                operator,
                right: Box::new(right),
            }
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expr {
        if !self.is_at_end() {
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
                    let expr = self.expression();
                    self.consume(TokenType::RightParen, "Expected ')' after expression.");
                    Expr::Grouping {
                        expression: Box::new(expr),
                    }
                }
                _ => panic!("Unexpected token"),
            };
            self.advance();
            expr
        } else {
            panic!("Unexpected end.")
        }
    }

    fn consume(&self, tpe: TokenType, msg: &str) {
        unimplemented!()
    }
}
