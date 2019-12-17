use crate::error::{report, Error};
use crate::interpreter::Interpreter;
use crate::syntax::{expr, stmt};
use crate::syntax::{Expr, LiteralValue, Stmt};
use crate::token::{Token, TokenType};

use std::collections::HashMap;
use std::mem;

#[derive(Debug, Clone)]
enum FunctionType {
    None,
    Function,
    Initializer,
    Method,
}

#[derive(Debug, Clone)]
enum ClassType {
    None,
    Class,
    Subclass,
}

pub struct Resolver<'i> {
    interpreter: &'i mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
    current_class: ClassType,
    pub had_error: bool,
}

impl<'i> Resolver<'i> {
    pub fn new(interpreter: &'i mut Interpreter) -> Self {
        Resolver {
            interpreter: interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
            current_class: ClassType::None,
            had_error: false,
        }
    }

    fn resolve_stmt(&mut self, statement: &Stmt) {
        statement.accept(self);
    }

    pub fn resolve_stmts(&mut self, statements: &Vec<Stmt>) {
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
        let mut already_defined: bool = false;
        match self.scopes.last_mut() {
            Some(ref mut scope) => {
                already_defined = scope.contains_key(&name.lexeme);
                scope.insert(name.lexeme.clone(), false);
            }
            None => (),
        };

        // Report an error if the variable was already defined.
        if already_defined {
            self.error(
                name,
                "Variable with this name already declared in this scope.",
            );
        }
    }

    fn define(&mut self, name: &Token) {
        match self.scopes.last_mut() {
            Some(ref mut scope) => {
                scope.insert(name.lexeme.clone(), true);
            }
            None => (),
        };
    }

    fn resolve_function(&mut self, params: &Vec<Token>, body: &Vec<Stmt>, tpe: FunctionType) {
        let enclosing_function = mem::replace(&mut self.current_function, tpe);

        self.begin_scope();
        for param in params {
            self.declare(param);
            self.define(param);
        }
        self.resolve_stmts(body);
        self.end_scope();
        self.current_function = enclosing_function;
    }

    fn resolve_local(&mut self, name: &Token) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(name, i);
            }
        }
    }

    fn error(&mut self, token: &Token, message: &str) {
        if token.tpe == TokenType::EOF {
            report(token.line, " at end", message);
        } else {
            report(token.line, &format!(" at '{}'", token.lexeme), message);
        }
        self.had_error = true;
    }
}

impl<'i> expr::Visitor<()> for Resolver<'i> {
    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<(), Error> {
        self.resolve_expr(value);
        self.resolve_local(name);
        Ok(())
    }

    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        _operator: &Token,
        right: &Expr,
    ) -> Result<(), Error> {
        self.resolve_expr(left);
        self.resolve_expr(right);
        Ok(())
    }

    fn visit_call_expr(
        &mut self,
        callee: &Expr,
        _paren: &Token,
        arguments: &Vec<Expr>,
    ) -> Result<(), Error> {
        self.resolve_expr(callee);
        for argument in arguments {
            self.resolve_expr(argument);
        }
        Ok(())
    }

    fn visit_get_expr(&mut self, object: &Expr, _name: &Token) -> Result<(), Error> {
        self.resolve_expr(object);
        Ok(())
    }

    fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<(), Error> {
        self.resolve_expr(expression);
        Ok(())
    }

    fn visit_literal_expr(&self, _value: &LiteralValue) -> Result<(), Error> {
        Ok(())
    }

    fn visit_logical_expr(
        &mut self,
        left: &Expr,
        _operator: &Token,
        right: &Expr,
    ) -> Result<(), Error> {
        self.resolve_expr(left);
        self.resolve_expr(right);
        Ok(())
    }

    fn visit_set_expr(&mut self, object: &Expr, _name: &Token, value: &Expr) -> Result<(), Error> {
        self.resolve_expr(value);
        self.resolve_expr(object);
        Ok(())
    }

    fn visit_super_expr(&mut self, keyword: &Token, _method: &Token) -> Result<(), Error> {
        match self.current_class {
            ClassType::None => self.error(keyword, "Cannot use 'super' outside of a class."),
            ClassType::Class => {
                self.error(keyword, "Cannot use 'super' in a class with no superclass.")
            }
            _ => self.resolve_local(keyword),
        }
        Ok(())
    }

    fn visit_this_expr(&mut self, keyword: &Token) -> Result<(), Error> {
        if let ClassType::None = self.current_class {
            self.error(keyword, "Cannot use 'this' outside of a class.");
        } else {
            self.resolve_local(keyword);
        }
        Ok(())
    }

    fn visit_unary_expr(&mut self, _operator: &Token, right: &Expr) -> Result<(), Error> {
        self.resolve_expr(right);
        Ok(())
    }

    fn visit_variable_expr(&mut self, name: &Token) -> Result<(), Error> {
        if let Some(scope) = self.scopes.last() {
            if let Some(flag) = scope.get(&name.lexeme) {
                if *flag == false {
                    self.error(name, "Cannot read local variable in its own initializer.");
                }
            }
        };
        self.resolve_local(name);
        Ok(())
    }
}

impl<'i> stmt::Visitor<()> for Resolver<'i> {
    fn visit_block_stmt(&mut self, statements: &Vec<Stmt>) -> Result<(), Error> {
        self.begin_scope();
        self.resolve_stmts(statements);
        self.end_scope();
        Ok(())
    }

    fn visit_class_stmt(
        &mut self,
        name: &Token,
        superclass: &Option<Expr>,
        methods: &Vec<Stmt>,
    ) -> Result<(), Error> {
        let enclosing_class = mem::replace(&mut self.current_class, ClassType::Class);

        self.declare(name);
        self.define(name);

        if let Some(Expr::Variable {
            name: superclass_name,
        }) = superclass
        {
            if name.lexeme == superclass_name.lexeme {
                self.error(superclass_name, "A class cannot inherit from itself.");
            }

            self.current_class = ClassType::Subclass;
            self.resolve_local(superclass_name);

            self.begin_scope();
            self.scopes
                .last_mut()
                .expect("Scopes is empty.")
                .insert("super".to_owned(), true);
        }

        self.begin_scope();
        self.scopes
            .last_mut()
            .expect("Scopes is empty.")
            .insert("this".to_owned(), true);

        for method in methods {
            if let Stmt::Function { name, params, body } = method {
                let declaration = if name.lexeme == "init" {
                    FunctionType::Initializer
                } else {
                    FunctionType::Method
                };
                self.resolve_function(params, body, declaration);
            } else {
                unreachable!()
            }
        }

        if superclass.is_some() {
            self.end_scope()
        }

        self.end_scope();

        self.current_class = enclosing_class;

        Ok(())
    }

    fn visit_expression_stmt(&mut self, expression: &Expr) -> Result<(), Error> {
        self.resolve_expr(expression);
        Ok(())
    }

    fn visit_function_stmt(
        &mut self,
        name: &Token,
        params: &Vec<Token>,
        body: &Vec<Stmt>,
    ) -> Result<(), Error> {
        self.declare(name);
        self.define(name);

        self.resolve_function(params, body, FunctionType::Function);
        Ok(())
    }

    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        else_branch: &Option<Stmt>,
        then_branch: &Stmt,
    ) -> Result<(), Error> {
        self.resolve_expr(condition);
        self.resolve_stmt(then_branch);
        if let Some(else_stmt) = else_branch {
            self.resolve_stmt(else_stmt);
        }
        Ok(())
    }

    fn visit_print_stmt(&mut self, expression: &Expr) -> Result<(), Error> {
        self.resolve_expr(expression);
        Ok(())
    }

    fn visit_return_stmt(&mut self, keyword: &Token, value: &Option<Expr>) -> Result<(), Error> {
        if let FunctionType::None = self.current_function {
            self.error(keyword, "Cannot return from top-level code.");
        }

        if let Some(return_value) = value {
            if let FunctionType::Initializer = self.current_function {
                self.error(keyword, "Cannot return value from initializer.");
            }
            self.resolve_expr(return_value);
        }
        Ok(())
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
        self.resolve_expr(condition);
        self.resolve_stmt(body);
        Ok(())
    }
}
