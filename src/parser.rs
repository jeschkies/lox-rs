use crate::syntax::Expr;
use crate::token::{Token, TokenType};

struct Parser {
    tokens: Vec<Token>,
    current: i64,
}

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
            let operator: Token = self.previous();
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

    fn advance(&self) {
        unimplemented!()
    }

    fn previous(&self) -> Token {
        unimplemented!()
    }

    fn peek(&self) -> Token {
        unimplemented!()
    }

    fn is_at_end(&self) -> bool {
        unimplemented!()
    }

    fn comparison(&self) -> Expr {
        unimplemented!()
    }
}
