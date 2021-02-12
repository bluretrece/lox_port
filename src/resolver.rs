use crate::expression::{Expr, ExprVisitor, Visitable as Visitable_Expr};
use crate::interpreter::{LoxResult, ReturnStatus};
use crate::literal::Literal;
use crate::lox_error::LoxError;
use crate::object::*;
use crate::statement::{Statement, StmtVisitor, Visitable};
use crate::token::Token;
use crate::Interpreter;
use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, LoxError>;

#[derive(Copy, Clone, Debug)]
pub enum FunctionType {
    None,
    Function,
}

pub struct Resolver<'a> {
    pub interpreter: &'a mut Interpreter,
    pub scopes: Vec<HashMap<String, bool>>,
    pub current_function: FunctionType,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: vec![],
            current_function: FunctionType::None,
        }
    }

    pub fn resolve(&mut self, statements: &Vec<Box<Statement>>) -> LoxResult<Object> {
        self.resolve_statements(statements)
    }

    pub fn resolve_function(
        &mut self,
        parameters: &Vec<Token>,
        body: &Vec<Box<Statement>>,
        function_type: FunctionType,
    ) -> LoxResult<Object> {
        let enclosing_function = self.current_function;
        self.current_function = function_type;
        self.begin_scope();

        for param in parameters {
            self.declare(param);
            self.define(param);
        }

        let r = self.resolve_statements(body);
        self.end_scope()?;
        self.current_function = enclosing_function;

        Ok(Object::Nil)
    }

    pub fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new())
    }

    pub fn resolve_statements(&mut self, statements: &Vec<Box<Statement>>) -> LoxResult<Object> {
        for stmt in statements {
            self.resolve_statement(stmt)?;
        }

        Ok(Object::Nil)
    }

    pub fn resolve_expression(&mut self, expr: &Expr) -> LoxResult<Object> {
        expr.accept(self)
    }

    pub fn end_scope(&mut self) -> LoxResult<Object> {
        self.scopes.pop();
        Ok(Object::Nil)
    }

    pub fn resolve_local(&mut self, variable: &Expr, name: &Token) -> LoxResult<Object> {
        for i in (0..self.scopes.len()).rev() {
            if self.scopes[i].contains_key(&name.lexeme()) {
                self.interpreter
                    .resolve_local(variable, self.scopes.len() - 1 - i);
                return Ok(Object::Nil);
            }
        }

        Ok(Object::Nil)
    }

    pub fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme(), true);
        }
    }

    pub fn resolve_statement(&mut self, stmt: &Box<Statement>) -> LoxResult<Object> {
        stmt.accept(self)
    }

    pub fn declare(&mut self, name: &Token) -> LoxResult<Object> {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name.lexeme()) {
                let error =
                    LoxError::RuntimeError(String::from("Variable already defined in scope."));
                return Err(ReturnStatus::Error(error));
            }
            scope.insert(name.lexeme(), false);
        }

        Ok(Object::Nil)
    }
}

impl<'a> ExprVisitor for Resolver<'a> {
    type Value = Object;
    fn visit_call_expression(
        &mut self,
        callee: &Box<Expr>,
        _paren: &Token,
        arguments: &Vec<Box<Expr>>,
    ) -> LoxResult<Self::Value> {
        self.resolve_expression(callee)?;

        for arg in arguments {
            self.resolve_expression(arg)?;
        }

        Ok(Object::Nil)
    }

    fn visit_assign_expression(
        &mut self,
        expr: &Expr,
        name: &Token,
        value: &Box<Expr>,
    ) -> LoxResult<Self::Value> {
        self.resolve_expression(value)?;
        self.resolve_local(expr, name)
    }

    fn visit_binary_expression(
        &mut self,
        left: &Box<Expr>,
        _operator: &Token,
        right: &Box<Expr>,
    ) -> LoxResult<Self::Value> {
        self.resolve_expression(left)?;
        self.resolve_expression(right)
    }

    fn visit_group_expression(&mut self, content: &Box<Expr>) -> LoxResult<Self::Value> {
        self.resolve_expression(content)
    }

    fn visit_literal_expression(&mut self, _literal: &Literal) -> LoxResult<Self::Value> {
        Ok(Object::Nil)
    }

    fn visit_unary_expression(
        &mut self,
        _operator: &Token,
        right: &Box<Expr>,
    ) -> LoxResult<Self::Value> {
        self.resolve_expression(right)
    }

    fn visit_logical_expression(
        &mut self,
        _expr: &Expr,
        left: &Box<Expr>,
        _operator: &Token,
        right: &Box<Expr>,
    ) -> LoxResult<Self::Value> {
        self.resolve_expression(left)?;
        self.resolve_expression(right)
    }
    fn visit_variable_expression(&mut self, expr: &Expr, name: &Token) -> LoxResult<Self::Value> {
        if let Some(scope) = self.scopes.last() {
            if let Some(defined) = scope.get(&name.lexeme()) {
                if !defined {
                    let error = LoxError::RuntimeError(String::from(
                        "Cannot read local variable in its own initializer.",
                    ));
                    return Err(ReturnStatus::Error(error));
                }
            }
        }

        self.resolve_local(&expr, name)
    }
}

impl<'a> StmtVisitor for Resolver<'a> {
    type Value = Object;

    fn visit_return_statement(
        &mut self,
        _keyword: &Token,
        value: &Option<Box<Expr>>,
    ) -> LoxResult<Object> {
        match self.current_function {
            FunctionType::None => {
                let error =
                    LoxError::RuntimeError(String::from("Can't return from top-level code"));
                return Err(ReturnStatus::Error(error));
            }
            _ => {
                if let Some(value) = value {
                    self.resolve_expression(value)?;
                }

                Ok(Object::Nil)
            }
        }
    }

    fn visit_while_statement(
        &mut self,
        _statement: &Statement,
        condition: &Box<Expr>,
        body: &Box<Statement>,
    ) -> LoxResult<Object> {
        self.resolve_expression(condition)?;
        self.resolve_statement(body)?;
        Ok(Object::Nil)
    }

    fn visit_print_stmt(&mut self, _stmt: &Statement, expr: &Box<Expr>) -> LoxResult<Self::Value> {
        self.resolve_expression(expr)?;
        Ok(Object::Nil)
    }

    fn visit_expression_stmt(
        &mut self,
        _stmt: &Statement,
        expression: &Box<Expr>,
    ) -> LoxResult<Object> {
        self.resolve_expression(expression)
    }
    fn visit_if_statement(
        &mut self,
        _stmt: &Statement,
        condition: &Box<Expr>,
        then_branch: &Box<Statement>,
        else_brach: &Option<Box<Statement>>,
    ) -> LoxResult<Object> {
        self.resolve_expression(condition)?;
        self.resolve_statement(then_branch)?;

        if let Some(else_branch) = else_brach {
            self.resolve_statement(else_branch)
        } else {
            Ok(Object::Nil)
        }
    }
    fn visit_function_statement(
        &mut self,
        _stmt: &Statement,
        name: &Token,
        paremeters: &Vec<Token>,
        body: &Vec<Box<Statement>>,
    ) -> LoxResult<Object> {
        self.declare(name)?;
        self.define(name);

        self.resolve_function(paremeters, body, FunctionType::Function)
    }
    fn visit_block_statement(
        &mut self,
        _stmt: &Statement,
        statements: &Vec<Box<Statement>>,
    ) -> LoxResult<Object> {
        self.begin_scope();
        self.resolve_statements(statements)?;
        self.end_scope()?;
        Ok(Object::Nil)
    }

    fn visit_var_stmt(
        &mut self,
        _stmt: &Statement,
        name: &Token,
        initializer: &Option<Box<Expr>>,
    ) -> LoxResult<Object> {
        self.declare(name);

        if let Some(initializer) = initializer {
            self.resolve_expression(initializer)?;
        }

        self.define(name);
        Ok(Object::Nil)
    }
}
