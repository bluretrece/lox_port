use crate::environment::Environment;
use crate::expression::Visitable;
use crate::expression::*;
use crate::literal::*;
use crate::lox_error::*;
use crate::natives;
use crate::object::*;
use crate::statement::Visitable as VisitableStatement;
use crate::statement::*;
use crate::token::*;
use crate::token_type::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;



pub type Result<T> = std::result::Result<T, LoxError>;
pub type LoxResult<T> = std::result::Result<T, ReturnStatus>;

#[derive(Debug)]
pub enum ReturnStatus {
    Error(LoxError),
    Break,
    Return(Option<Object>),
}
pub struct Interpreter {
    pub environment: Rc<RefCell<Environment>>,
    pub globals: Rc<RefCell<Environment>>,
    pub locals: HashMap<Expr, usize>,
}

impl StmtVisitor for Interpreter {
    type Value = Object;

    fn visit_return_statement(
        &mut self,
        _keyword: &Token,
        value: &Option<Box<Expr>>,
    ) -> LoxResult<Object> {
        let mut return_val = None;

        if let Some(value) = value {
            return_val = Some(self.evaluate(value)?);
        }

        Err(ReturnStatus::Return(return_val)) // Should be return
    }

    fn visit_function_statement(
        &mut self,
        _stmt: &Statement,
        name: &Token,
        parameters: &Vec<Token>,
        body: &Vec<Box<Statement>>,
    ) -> LoxResult<Object> {
        let function = LoxFunction::new(name, parameters, body, self.environment.clone());
        let callabe = Object::Callable(Rc::new(RefCell::new(function)));
        self.environment.borrow_mut().define(&name.lexeme, &callabe);

        Ok(Object::Nil)
    }

    fn visit_block_statement(
        &mut self,
        _stmt: &Statement,
        statements: &Vec<Box<Statement>>,
    ) -> LoxResult<Object> {
        let env_ref = Rc::clone(&self.environment);

        self.execute_block(
            statements,
            Rc::new(RefCell::new(Environment::with_ref(env_ref))),
        );
        Ok(Object::Nil)
    }

    fn visit_while_statement(
        &mut self,
        _statement: &Statement,
        condition: &Box<Expr>,
        body: &Box<Statement>,
    ) -> LoxResult<Object> {
        let condition = self.evaluate(condition)?;
        let is_truthy: Object = self.is_truthy(condition);

        while is_truthy == Object::Boolean(true) {
            self.execute(body)?;
        }

        Ok(Object::Nil)
    }
    fn visit_expression_stmt(
        &mut self,
        _stmt: &Statement,
        expression: &Box<Expr>,
    ) -> LoxResult<Object> {
        self.evaluate(expression).ok();
        Ok(Object::Nil)
    }

    fn visit_if_statement(
        &mut self,
        _stmt: &Statement,
        condition: &Box<Expr>,
        then_branch: &Box<Statement>,
        else_brach: &Option<Box<Statement>>,
    ) -> LoxResult<Object> {
        let conditional = self.evaluate(condition).unwrap();
        let is_truthy: Object = self.is_truthy(conditional);

        match is_truthy {
            Object::Boolean(true) => self.execute(then_branch),
            _ => {
                if let Some(else_brach) = else_brach {
                    self.execute(else_brach)?;
                }
                Ok(Object::Nil)
            }
        }
    }

    fn visit_print_stmt(&mut self, _stmt: &Statement, expression: &Box<Expr>) -> LoxResult<Object> {
        let value = self.evaluate(expression);
        println!("{:?}", value);
        Ok(Object::Nil)
    }

    fn visit_var_stmt(
        &mut self,
        _stmt: &Statement,
        name: &Token,
        initializer: &Option<Box<Expr>>,
    ) -> LoxResult<Object> {
        let mut value = Object::Nil;

        if let Some(initializer) = initializer {
            if let Ok(initializer) = self.evaluate(initializer) {
                value = initializer;
            }
        }
        self.environment.borrow_mut().define(&name.lexeme, &value);
        Ok(Object::Nil)
    }
}

#[allow(dead_code)]
pub struct LoxFunction {
    name: Token,
    params: Vec<Token>,
    body: Vec<Box<Statement>>,
    closure: Rc<RefCell<Environment>>,
}

impl LoxFunction {
    fn new(
        name: &Token,
        params: &Vec<Token>,
        body: &Vec<Box<Statement>>,
        closure: Rc<RefCell<Environment>>,
    ) -> Self {
        Self {
            name: name.clone(),
            params: params.clone(),
            body: body.to_vec(),
            closure,
        }
    }
}

impl LoxCallable for LoxFunction {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &Vec<Object>,
    ) -> LoxResult<Option<Object>> {
        let environment = Rc::new(RefCell::new(Environment::child_of(self.closure.clone())));

        for i in 0..self.params.len() {
            environment
                .borrow_mut()
                .define(&self.params[i].lexeme, &arguments[i]);
        }

        let return_value = interpreter.execute_block(&self.body, environment);

        match return_value {
            // If there's no return value, return Nil.
            Ok(_) => Ok(Some(Object::Nil)),
            Err(e) => match e {
                ReturnStatus::Return(x) => match x {
                    Some(x) => Ok(Some(x)),
                    None => Ok(None),
                },
                _ => Err(e),
            },
        }
    }

    fn arity(&self) -> usize {
        self.params.len()
    }
}

pub trait LoxCallable {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &Vec<Object>,
    ) -> LoxResult<Option<Object>>;
    fn arity(&self) -> usize; // Number of arguments permitted.
}
impl std::fmt::Debug for dyn LoxCallable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<callable arity {}>", self.arity())
    }
}

impl std::fmt::Display for dyn LoxCallable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<callable arity {}>", self.arity())
    }
}

impl PartialEq<dyn LoxCallable> for dyn LoxCallable {
    fn eq(&self, other: &Self) -> bool {
        &self == &other
    }
}

impl ExprVisitor for Interpreter {
    type Value = Object;
    fn visit_call_expression(
        &mut self,
        callee: &Box<Expr>,
        _paren: &Token,
        arguments: &Vec<Box<Expr>>,
    ) -> LoxResult<Self::Value> {
        let callee: Object = self.evaluate(callee)?;
        let mut args = vec![];
        for arg in arguments {
            args.push(self.evaluate(arg)?);
        }

        // Research for Rust version of instanceof
        if let Object::Callable(callable) = callee {
            // Checks if wrong number of arguments is passed.
            if args.len() != callable.borrow().arity() {
                return Err(ReturnStatus::Error(LoxError::RuntimeError(String::from(
                    "Wrong number of arguments.",
                ))));
            }

            if let Some(function) = callable.borrow().call(self, &args)? {
                return Ok(function);
            } else {
                return Ok(Object::Nil);
            }
        }
        return Err(ReturnStatus::Error(LoxError::RuntimeError(String::from(
            "Can only call functions and classes.",
        ))));
    }

    fn visit_logical_expression(
        &mut self,
        _expr: &Expr,
        left: &Box<Expr>,
        operator: &Token,
        right: &Box<Expr>,
    ) -> LoxResult<Self::Value> {
        let left_expr = self.evaluate(left).unwrap();
        let is_truthy: Object = self.is_truthy(left_expr.clone());

        if operator.of_type == TokenType::OR {
            match is_truthy {
                Object::Boolean(true) => Ok(left_expr),
                _ => self.evaluate(right),
            }
        } else {
            match is_truthy {
                Object::Boolean(false) => Ok(left_expr),
                _ => self.evaluate(right),
            }
        }
    }

    fn visit_assign_expression(
        &mut self,
        _expr: &Expr,
        name: &Token,
        value: &Box<Expr>,
    ) -> LoxResult<Self::Value> {
        let value = self.evaluate(value)?;
        self.environment.borrow_mut().assign(name, value.clone());

        return Ok(value);
    }
    fn visit_variable_expression(&mut self, expr: &Expr, name: &Token) -> LoxResult<Self::Value> {
        self.look_up_variable(name, expr)
        // Ok(self.environment.borrow_mut().get(name.clone()))
    }
    fn visit_binary_expression(
        &mut self,
        left: &Box<Expr>,
        operator: &Token,
        right: &Box<Expr>,
    ) -> LoxResult<Self::Value> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match operator.of_type {
            TokenType::MINUS => {
                if let Object::Number(left) = left {
                    if let Object::Number(right) = right {
                        Ok(Object::Number(left - right))
                    } else {
                        Err(ReturnStatus::Error(LoxError::RuntimeError(String::from(
                            "Make sure right operand is a number",
                        ))))
                    }
                } else {
                    Err(ReturnStatus::Error(LoxError::RuntimeError(String::from(
                        "Make sure the left operand is a number.",
                    ))))
                }
            }
            TokenType::SLASH => {
                if let Object::Number(left) = left {
                    if let Object::Number(right) = right {
                        Ok(Object::Number(left / right))
                    } else {
                        Err(ReturnStatus::Error(LoxError::RuntimeError(String::from(
                            "Make sure the left operand is a number.",
                        ))))
                    }
                } else {
                    Err(ReturnStatus::Error(LoxError::RuntimeError(String::from(
                        "Make sure the left operand is a number.",
                    ))))
                }
            }
            TokenType::STAR => {
                if let Object::Number(left) = left {
                    if let Object::Number(right) = right {
                        Ok(Object::Number(left * right))
                    } else {
                        Err(ReturnStatus::Error(LoxError::RuntimeError(String::from(
                            "Make sure the left operand is a number.",
                        ))))
                    }
                } else {
                    Err(ReturnStatus::Error(LoxError::RuntimeError(String::from(
                        "Make sure the left operand is a number.",
                    ))))
                }
            }
            TokenType::PLUS => {
                if let Object::Number(left) = left {
                    if let Object::Number(right) = right {
                        Ok(Object::Number(left + right))
                    } else {
                        Err(ReturnStatus::Error(LoxError::RuntimeError(String::from(
                            "Make sure the left operand is a number.",
                        ))))
                    }
                } else {
                    Err(ReturnStatus::Error(LoxError::RuntimeError(String::from(
                        "Make sure the left operand is a number.",
                    ))))
                }
            }
            TokenType::GREATER => {
                if let Object::Number(left) = left {
                    if let Object::Number(right) = right {
                        Ok(Object::Boolean(left > right))
                    } else {
                        Err(ReturnStatus::Error(LoxError::RuntimeError(String::from(
                            "Make sure the left operand is a number.",
                        ))))
                    }
                } else {
                    Err(ReturnStatus::Error(LoxError::RuntimeError(String::from(
                        "Make sure the left operand is a number.",
                    ))))
                }
            }
            TokenType::GREATER_EQUAL => {
                if let Object::Number(left) = left {
                    if let Object::Number(right) = right {
                        Ok(Object::Boolean(left >= right))
                    } else {
                        Err(ReturnStatus::Error(LoxError::RuntimeError(String::from(
                            "Make sure the left operand is a number.",
                        ))))
                    }
                } else {
                    Err(ReturnStatus::Error(LoxError::RuntimeError(String::from(
                        "Make sure the left operand is a number.",
                    ))))
                }
            }
            TokenType::LESS => {
                if let Object::Number(left) = left {
                    if let Object::Number(right) = right {
                        Ok(Object::Boolean(left < right))
                    } else {
                        Err(ReturnStatus::Error(LoxError::RuntimeError(String::from(
                            "Make sure the left operand is a number.",
                        ))))
                    }
                } else {
                    Err(ReturnStatus::Error(LoxError::RuntimeError(String::from(
                        "Make sure the left operand is a number.",
                    ))))
                }
            }
            TokenType::LESS_EQUAL => {
                if let Object::Number(left) = left {
                    if let Object::Number(right) = right {
                        Ok(Object::Boolean(left <= right))
                    } else {
                        Err(ReturnStatus::Error(LoxError::RuntimeError(String::from(
                            "Make sure the left operand is a number.",
                        ))))
                    }
                } else {
                    Err(ReturnStatus::Error(LoxError::RuntimeError(String::from(
                        "Make sure the left operand is a number.",
                    ))))
                }
            }
            TokenType::EQUAL_EQUAL => {
                if let Object::Number(left) = left {
                    if let Object::Number(right) = right {
                        Ok(Object::Boolean(left == right))
                    } else {
                        Err(ReturnStatus::Error(LoxError::RuntimeError(String::from(
                            "Make sure the left operand is a number.",
                        ))))
                    }
                } else {
                    Err(ReturnStatus::Error(LoxError::RuntimeError(String::from(
                        "Make sure the left operand is a number.",
                    ))))
                }
            }
            TokenType::BANG_EQUAL => {
                if let Object::Number(left) = left {
                    if let Object::Number(right) = right {
                        Ok(Object::Boolean(left != right))
                    } else {
                        Err(ReturnStatus::Error(LoxError::RuntimeError(String::from(
                            "Make sure the left operand is a number.",
                        ))))
                    }
                } else {
                    Err(ReturnStatus::Error(LoxError::RuntimeError(String::from(
                        "Make sure the left operand is a number.",
                    ))))
                }
            }
            _ => unreachable!(),
        }
    }

    fn visit_group_expression(&mut self, content: &Box<Expr>) -> LoxResult<Self::Value> {
        self.evaluate(content)
    }

    fn visit_literal_expression(&mut self, literal: &Literal) -> LoxResult<Self::Value> {
        return Ok(Object::from_literal(literal));
    }

    fn visit_unary_expression(
        &mut self,
        operator: &Token,
        right: &Box<Expr>,
    ) -> LoxResult<Self::Value> {
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
    pub fn look_up_variable(&self, name: &Token, expr: &Expr) -> LoxResult<Object> {
        if let Some(distance) = self.locals.get(expr) {
            let v = self.environment.borrow().get_at(*distance, &name.lexeme)?;
            Ok(v)
        } else {
            let v = self.globals.borrow().get(name);
            Ok(v)
        }
    }

    pub fn environment(&self) -> Rc<RefCell<Environment>> {
        self.environment.clone()
    }

    pub fn resolve_local(&mut self, variable: &Expr, depth: usize) {
        self.locals.insert(variable.clone(), depth);
    }
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));
        globals.borrow_mut().define(
            "Clock",
            &Object::Callable(Rc::new(RefCell::new(natives::NativeClock))),
        );

        Interpreter {
            environment: Rc::new(RefCell::new(Environment::new())),
            globals: globals.clone(),
            locals: HashMap::new()
        }
    }
    pub fn evaluate(&mut self, expr: &Box<Expr>) -> LoxResult<Object> {
        expr.accept(self)
    }

    pub fn interpret(&mut self, stmt: &Vec<Box<Statement>>) -> LoxResult<()> {
        for statement in stmt {
            self.execute(statement)?;
        }

        Ok(())
    }
    pub fn is_truthy(&mut self, result: Object) -> Object {
        match result {
            Object::Nil => Object::Boolean(false),
            Object::Number(_) => Object::Boolean(true),
            Object::Str(_) => Object::Boolean(true),
            Object::Boolean(value) => Object::Boolean(value),
            Object::Callable(_) => Object::Boolean(true),
        }
    }
    pub fn execute(&mut self, stmt: &Box<Statement>) -> LoxResult<Object> {
        stmt.accept(self)
    }
    pub fn execute_block(
        &mut self,
        stmt: &Vec<Box<Statement>>,
        env: Rc<RefCell<Environment>>,
    ) -> LoxResult<Object> {
        let previous = Rc::clone(&self.environment);
        self.environment = env;

        for statement in stmt {
            self.execute(statement)?;
        }

        self.environment = previous;

        Ok(Object::Nil)
    }
}
