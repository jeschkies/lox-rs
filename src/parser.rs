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
            statments.push(self.declaration()?);
        }
        Ok(statments)
    }

    fn expression(&mut self) -> Result<Expr, Error> {
        self.assignment()
    }

    fn declaration(&mut self) -> Result<Stmt, Error> {
        let statement = if matches!(self, TokenType::Var) {
            self.var_declaration()
        } else if matches!(self, TokenType::Class) {
            self.class_declaration()
        } else if matches!(self, TokenType::Fun) {
            self.function("function")
        } else {
            self.statement()
        };

        match statement {
            Err(Error::Parse) => {
                self.synchronize();
                Ok(Stmt::Null)
            }
            other => other,
        }
    }

    fn class_declaration(&mut self) -> Result<Stmt, Error> {
        let name = self.consume(TokenType::Identifier, "Expect class name.")?;
        self.consume(TokenType::LeftBrace, "Expect '{' before class body,")?;

        let mut methods: Vec<Stmt> = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            methods.push(self.function("method")?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after class body.")?;
        Ok(Stmt::Class { name, methods })
    }

    fn statement(&mut self) -> Result<Stmt, Error> {
        if matches!(self, TokenType::For) {
            self.for_statement()
        } else if matches!(self, TokenType::If) {
            self.if_statement()
        } else if matches!(self, TokenType::Print) {
            self.print_statement()
        } else if matches!(self, TokenType::Return) {
            self.return_statement()
        } else if matches!(self, TokenType::While) {
            self.while_statement()
        } else if matches!(self, TokenType::LeftBrace) {
            Ok(Stmt::Block {
                statements: self.block()?,
            })
        } else {
            self.expression_statement()
        }
    }

    fn for_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let initializer = if matches!(self, TokenType::Semicolon) {
            None
        } else if matches!(self, TokenType::Var) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if !self.check(TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let increment = if !self.check(TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let Some(inc) = increment {
            let inc_stmt = Stmt::Expression { expression: inc };
            body = Stmt::Block {
                statements: vec![body, inc_stmt],
            }
        }

        body = Stmt::While {
            condition: condition.unwrap_or(Expr::Literal {
                value: LiteralValue::Boolean(true),
            }),
            body: Box::new(body),
        };

        if let Some(init_stmt) = initializer {
            body = Stmt::Block {
                statements: vec![init_stmt, body],
            }
        }

        Ok(body)
    }

    fn if_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);
        let else_branch = if matches!(self, TokenType::Else) {
            Box::new(Some(self.statement()?))
        } else {
            Box::new(None)
        };

        Ok(Stmt::If {
            condition,
            else_branch,
            then_branch,
        })
    }

    fn print_statement(&mut self) -> Result<Stmt, Error> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print { expression: value })
    }

    fn return_statement(&mut self) -> Result<Stmt, Error> {
        let keyword: Token = self.previous().clone();
        let value = if !self.check(TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "Expect ';' after return values.")?;
        Ok(Stmt::Return { keyword, value })
    }

    fn var_declaration(&mut self) -> Result<Stmt, Error> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;

        let initializer = if matches!(self, TokenType::Equal) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Stmt::Var { name, initializer })
    }

    fn while_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
        let body = Box::new(self.statement()?);
        Ok(Stmt::While { condition, body })
    }

    fn expression_statement(&mut self) -> Result<Stmt, Error> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression { expression: expr })
    }

    fn function(&mut self, kind: &str) -> Result<Stmt, Error> {
        let name = self.consume(
            TokenType::Identifier,
            format!("Expect {} name.", kind).as_str(),
        )?;
        self.consume(
            TokenType::LeftParen,
            format!("Expect '(' after {} name.", kind).as_str(),
        )?;
        let mut params: Vec<Token> = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if params.len() >= 255 {
                    // We are not returning an error here.
                    self.error(self.peek(), "Cannot have more than 255 parameters.");
                }
                params.push(self.consume(TokenType::Identifier, "Expect parameter name.")?);

                if !matches!(self, TokenType::Comma) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;

        self.consume(
            TokenType::LeftBrace,
            format!("Expect '{{' before {} body.", kind).as_str(),
        )?;
        let body = self.block()?;
        Ok(Stmt::Function { name, params, body })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn assignment(&mut self) -> Result<Expr, Error> {
        let expr = self.or_()?;

        if matches!(self, TokenType::Equal) {
            let value = Box::new(self.assignment()?);

            if let Expr::Variable { name } = expr {
                return Ok(Expr::Assign { name, value });
            } else if let Expr::Get { object, name } = expr {
                return Ok(Expr::Set {
                    object,
                    name,
                    value,
                });
            }

            // We are just reporting the error but not return them.
            // See note in http://craftinginterpreters.com/statements-and-state.html#assignment-syntax.
            let equals = self.previous();
            self.error(equals, "Invalid assignment target.");
        }

        Ok(expr)
    }

    fn or_(&mut self) -> Result<Expr, Error> {
        let mut expr = self.and_()?;

        while matches!(self, TokenType::Or) {
            let operator: Token = (*self.previous()).clone();
            let right: Expr = self.and_()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn and_(&mut self) -> Result<Expr, Error> {
        let mut expr = self.equality()?;

        while matches!(self, TokenType::And) {
            let operator: Token = (*self.previous()).clone();
            let right: Expr = self.equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
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
            self.call()
        }
    }

    fn call(&mut self) -> Result<Expr, Error> {
        let mut expr = self.primary()?;

        loop {
            if matches!(self, TokenType::LeftParen) {
                expr = self.finish_call(expr)?;
            } else if matches!(self, TokenType::Dot) {
                let name = self.consume(TokenType::Identifier, "Expect property after '.'.")?;
                expr = Expr::Get {
                    object: Box::new(expr),
                    name: name,
                }
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, Error> {
        let mut arguments: Vec<Expr> = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    // We are just reporting the error but not return them.
                    self.error(self.peek(), "Cannot have more than 255 arguments.");
                }
                arguments.push(self.expression()?);
                if !matches!(self, TokenType::Comma) {
                    break;
                }
            }
        }

        let parent = self.consume(TokenType::RightParen, "Exprec ')' after arguments.")?;
        Ok(Expr::Call {
            callee: Box::new(callee),
            paren: parent,
            arguments,
        })
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
            TokenType::This => Expr::This {
                keyword: self.peek().clone(),
            },
            TokenType::Identifier => Expr::Variable {
                name: self.peek().clone(),
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
        let statements = parser.parse().expect("Could not parse sample code.");
        let printer = AstPrinter;

        //        assert_eq!(printer.print(statements).unwrap(), "(* (- 123) 45.67)");
    }
}
