use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::syntax::{Expr, LiteralValue, Stmt};
use crate::syntax::{expr, stmt};
use crate::token::Token;

use std::collections::HashMap;

struct Resolver {
    interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    fn new(interpreter: Interpreter) -> Self {
        Resolver {
            interpreter: interpreter,
            scopes: Vec::new(),
        }
    }

    fn resolve_stmt(&mut self, statement: &Stmt) {
        statement.accept(self);
    }

    fn resolve_stmts(&mut self, statements: &Vec<Stmt>) {
        for statement in statements {
            self.resolve_stmt(statement);
        }
    }

    fn resolve_expr(&mut self, expression: &Expr) {
        expression.accept(self);
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        unimplemented!()
    }

    fn define(&mut self, name: &Token) {
        unimplemented!()
    }
}

impl expr::Visitor<()> for Resolver {

    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<(), Error> {
        unimplemented!()
    }

    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<(), Error> {
        unimplemented!()
    }

    fn visit_call_expr(
        &mut self,
        callee: &Expr,
        paren: &Token,
        arguments: &Vec<Expr>,
    ) -> Result<(), Error> {
        unimplemented!()
    }

    fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<(), Error> {
        unimplemented!()
    }

    fn visit_literal_expr(&self, value: &LiteralValue) -> Result<(), Error> {
        unimplemented!()
    }

    fn visit_logical_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<(), Error> {
        unimplemented!()
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<(), Error> {
       unimplemented!()
    }

    fn visit_variable_expr(&mut self, name: &Token) -> Result<(), Error> {
        unimplemented!()
    }
}

impl stmt::Visitor<()> for Resolver {

    fn visit_block_stmt(&mut self, statements: &Vec<Stmt>) -> Result<(), Error> {
        self.begin_scope();
        self.resolve_stmts(statements);
        self.end_scope();
        Ok(())
    }

    fn visit_expression_stmt(&mut self, expression: &Expr) -> Result<(), Error> {
        unimplemented!()
    }

    fn visit_function_stmt(
        &mut self,
        name: &Token,
        params: &Vec<Token>,
        body: &Vec<Stmt>,
    ) -> Result<(), Error> {
        unimplemented!()
    }

    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        else_branch: &Option<Stmt>,
        then_branch: &Stmt,
    ) -> Result<(), Error> {
        unimplemented!()
    }

    fn visit_print_stmt(&mut self, expression: &Expr) -> Result<(), Error> {
        unimplemented!()
    }

    fn visit_return_stmt(&mut self, keyword: &Token, value: &Option<Expr>) -> Result<(), Error> {
        unimplemented!()
    }

    fn visit_var_stmt(&mut self, name: &Token, initializer: &Option<Expr>) -> Result<(), Error> {
        self.declare(name);
        if let Some(init) = initializer {
            self.resolve_expr(init);
        }
        self.define(name);
        Ok(())
    }

    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Result<(), Error> {
        unimplemented!()
    }
}
