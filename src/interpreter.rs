use crate::environment::*;
use crate::expression::Visitable;
use crate::expression::*;
use crate::literal::*;
use crate::lox_error::*;
use crate::object::*;
use crate::statement::Visitable as VisitableStatement;
use crate::statement::*;
use crate::token::*;
use crate::token_type::*;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Interpreter {
    pub environment: Rc<RefCell<Environment>>,
}

impl StmtVisitor for Interpreter {
    type Value = Object;
    fn visit_block_statement(
        &mut self,
        _stmt: &Statement,
        statements: &Vec<Box<Statement>>
    ) -> Option<Object> {
        let env_ref = Rc::clone(&self.environment);

        self.execute_block(
            statements,
            Rc::new(RefCell::new(Environment::with_ref(env_ref))));
        None
    }
    fn visit_expression_stmt(
        &mut self,
        _stmt: &Statement,
        expression: &Box<Expr>,
    ) -> Option<Object> {
        self.evaluate(expression);
        None
    }

    fn visit_print_stmt(&mut self, _stmt: &Statement, expression: &Box<Expr>) -> Option<Object> {
        let value = self.evaluate(expression);
        println!("{:?}", value);
        None
    }

    fn visit_var_stmt(
        &mut self,
        _stmt: &Statement,
        name: &Token,
        initializer: &Option<Box<Expr>>,
    ) -> Option<Object> {
        let mut value = Object::Nil;

        if let Some(initializer) = initializer {
            if let Ok(initializer) = self.evaluate(initializer) {
                value = initializer;
            }
        }
        self.environment.borrow_mut().define(&name.lexeme, value);
        None
    }
}

impl ExprVisitor for Interpreter {
    type Value = Object;

    fn visit_assign_expression(&mut self,
        expr: &Box<Expr>, name: &Token) -> Result<Self::Value, LoxError> {
        let mut value = self.evaluate(expr)?;

        self.environment.borrow_mut().assign(name, value.clone());

        return Ok(value)
    }
    fn visit_variable_expression(&mut self, name: &Token) -> Result<Self::Value, LoxError> {
        Ok(self.environment.borrow_mut().get(name.clone()))
    }
    fn visit_binary_expression(
        &mut self,
        expr: &Expr,
        left: &Box<Expr>,
        operator: &Token,
        right: &Box<Expr>,
    ) -> Result<Self::Value, LoxError> {
        let mut left = self.evaluate(left)?;
        let mut right = self.evaluate(right)?;

        match operator.of_type {
            TokenType::MINUS => Ok(left - right),
            TokenType::SLASH => Ok(left / right),
            TokenType::STAR => Ok(left * right),
            TokenType::PLUS => left + right,
            TokenType::GREATER => Ok(Object::Boolean(left > right)),
            TokenType::GREATER_EQUAL => Ok(Object::Boolean(left >= right)),
            TokenType::LESS => Ok(Object::Boolean(left < right)),
            TokenType::LESS_EQUAL => Ok(Object::Boolean(left <= right)),
            TokenType::EQUAL_EQUAL => Ok(Object::Boolean(left == right)),
            TokenType::BANG_EQUAL => Ok(Object::Boolean(left != right)),
            _ => unreachable!(),
        }
    }

    fn visit_group_expression(
        &mut self,
        expr: &Expr,
        content: &Box<Expr>,
    ) -> Result<Self::Value, LoxError> {
        self.evaluate(content)
    }

    fn visit_literal_expression(
        &mut self,
        expr: &Expr,
        literal: &Literal,
    ) -> Result<Self::Value, LoxError> {
        return Ok(Object::from_literal(literal));
    }

    fn visit_unary_expression(
        &mut self,
        _expr: &Expr,
        operator: &Token,
        right: &Box<Expr>,
    ) -> Result<Self::Value, LoxError> {
        let mut right = self.evaluate(right)?;

        match operator.of_type {
            TokenType::MINUS => match right {
                Object::Number(n) => Ok(Object::Number(-n)),
                _ => unimplemented!(),
            },
            TokenType::BANG => Ok(Object::Boolean(!right.is_truthy())),
            _ => unreachable!(),
        }
    }
}

impl Interpreter {
    pub fn evaluate(&mut self, expr: &Box<Expr>) -> Result<Object, LoxError> {
        expr.accept(self)
    }

    pub fn interpret(&mut self, stmt: &Vec<Box<Statement>>) {
        for statement in stmt {
            self.execute(statement)
        }
    }

    pub fn execute(&mut self, stmt: &Box<Statement>) {
        stmt.accept(self);
    }
    pub fn execute_block(&mut self, stmt: &Vec<Box<Statement>>, env: Rc<RefCell<Environment>>) {
        let mut previous = Rc::clone(&self.environment);
        self.environment = env;

        for statement in stmt {
            self.execute(statement);
        }

        self.environment = previous;

    }
}
