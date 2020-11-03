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
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    pub environment: Rc<RefCell<Environment>>,
}

impl StmtVisitor for Interpreter {
    type Value = Object;
    fn visit_block_statement(
        &mut self,
        _stmt: &Statement,
        statements: &Vec<Box<Statement>>,
    ) -> Option<Object> {
        let env_ref = Rc::clone(&self.environment);

        self.execute_block(
            statements,
            Rc::new(RefCell::new(Environment::with_ref(env_ref))),
        );
        None
    }
    fn visit_expression_stmt(
        &mut self,
        _stmt: &Statement,
        expression: &Box<Expr>,
    ) -> Option<Object> {
        self.evaluate(expression).ok();
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
        self.environment.borrow_mut().define(&name.lexeme, &value);
        None
    }
}

impl ExprVisitor for Interpreter {
    type Value = Object;

    //fn visit_logical_expression(
    //    &mut self,
    //    _expr: &Expr,
    //    left: &Box<Expr>,
    //    operator: &Token,
    //    right: &Box<Expr>,
    //) -> Result<Self::Value, LoxError> {
    //    let left_expr = self.evaluate(left).unwrap();
    //    let is_truthy: Object = self.is_truthy(left_expr.clone());

    //    if operator.of_type == TokenType::OR {
    //        match is_truthy {
    //            Object::Boolean(true) => Ok(left_expr),
    //            _ => self.evaluate(right)
    //        }
    //    } else {
    //        match is_truthy {
    //            Object::Boolean(false) => Ok(left_expr),
    //            _ => self.evaluate(right),
    //        }
    //    }
    //}
    fn visit_assign_expression(
        &mut self,
        name: &Token,
        value: &Box<Expr>,
    ) -> Result<Self::Value, LoxError> {
        let value = self.evaluate(value)?;
        
        self.environment.borrow_mut().assign(name, value.clone());

        return Ok(value);
    }
    fn visit_variable_expression(&mut self, name: &Token) -> Result<Self::Value, LoxError> {
        Ok(self.environment.borrow_mut().get(name.clone()))
    }
    fn visit_binary_expression(
        &mut self,
        left: &Box<Expr>,
        operator: &Token,
        right: &Box<Expr>,
    ) -> Result<Self::Value, LoxError> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

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
        content: &Box<Expr>,
    ) -> Result<Self::Value, LoxError> {
        self.evaluate(content)
    }

    fn visit_literal_expression(
        &mut self,
        literal: &Literal,
    ) -> Result<Self::Value, LoxError> {
        return Ok(Object::from_literal(literal));
    }

    fn visit_unary_expression(
        &mut self,
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
    pub fn new() -> Self {
        Interpreter {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }
    pub fn evaluate(&mut self, expr: &Box<Expr>) -> Result<Object, LoxError> {
        expr.accept(self)
    }

    pub fn interpret(&mut self, stmt: &Vec<Box<Statement>>) -> Result<(), LoxError> {
        for statement in stmt {
            self.execute(statement);
        }

        Ok(())
    }
    pub fn is_truthy(&mut self, result: Object) -> Object {
        match result {
            Object::Nil => Object::Boolean(false),
            Object::Number(_) => Object::Boolean(true),
            Object::Str(_) => Object::Boolean(true),
            Object::Boolean(value) => Object::Boolean(value),
        }
    }
    pub fn execute(&mut self, stmt: &Box<Statement>) -> Option<Object> {
        stmt.accept(self)
    }
    pub fn execute_block(&mut self, stmt: &Vec<Box<Statement>>, env: Rc<RefCell<Environment>>) {
        let previous = Rc::clone(&self.environment);
        self.environment = env;

        for statement in stmt {
            self.execute(statement);
        }

        self.environment = previous;
    }
}
