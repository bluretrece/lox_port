use crate::expression::*;
use crate::object::*;
use crate::token::*;
use crate::lox_error::*;
use crate::interpreter::LoxResult;

#[derive(Clone)]
pub enum Statement {
    If {
        condition: Box<Expr>,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>
    },

    Return {
        keyword: Token,
        value: Option<Box<Expr>>,
    },

    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Box<Statement>>
    },

    Print {
        expression: Box<Expr>,
    },

    Expression {
        expression: Box<Expr>,
    },
    Variable {
        name: Token,
        initializer: Option<Box<Expr>>,
    },
    Class {
        name: Token,
        methods: Vec<Box<Statement>>,
    },

    Block {
        statements: Vec<Box<Statement>>,
    },
    
    While {
        condition: Box<Expr>,
        body: Box<Statement>
    }
}

pub trait StmtVisitor {
    type Value;
    fn visit_class_stmt(&mut self, name: &Token) -> LoxResult<Self::Value>;
    fn visit_return_statement(
        &mut self,
        keyword: &Token,
        value: &Option<Box<Expr>>
    ) -> LoxResult<Self::Value>;
    fn visit_function_statement(
        &mut self,
        stmt: &Statement,
        name: &Token,
        paremeters: &Vec<Token>,
        body: &Vec<Box<Statement>>
    ) -> LoxResult<Self::Value>;
    fn visit_var_stmt(
        &mut self,
        stmt: &Statement,
        name: &Token,
        initializer: &Option<Box<Expr>>,
    ) -> LoxResult<Self::Value>;
    fn visit_expression_stmt(&mut self, stmt: &Statement, expr: &Box<Expr>) -> LoxResult<Self::Value>;
    fn visit_print_stmt(&mut self, stmt: &Statement, expr: &Box<Expr>) -> LoxResult<Self::Value>;
    fn visit_block_statement(
        &mut self,
        _stmt: &Statement,
        statements: &Vec<Box<Statement>>,
    ) -> LoxResult<Self::Value>;
    fn visit_if_statement(
        &mut self,
        stmt: &Statement,
        condition: &Box<Expr>, 
        then_branch: &Box<Statement>, 
        else_brach: &Option<Box<Statement>>) -> LoxResult<Self::Value>;
   fn visit_while_statement(&mut self, statement: &Statement, condition: &Box<Expr>, body: &Box<Statement>) -> LoxResult<Self::Value>;
}

pub trait Visitable {
    fn accept(&self, expr: &mut dyn StmtVisitor<Value = Object>) -> LoxResult<Object>;
}

impl Visitable for Statement {
    fn accept(&self, visitor: &mut dyn StmtVisitor<Value = Object>) -> LoxResult<Object> {
        match self {
            Self::Class { name, methods} => visitor.visit_class_stmt(name),
            Self::Return { keyword, value } => visitor.visit_return_statement(keyword, value),
            Self::Function { name, params, body } => visitor.visit_function_statement(&self, name, params, body),
            Self::Expression { expression } => visitor.visit_expression_stmt(&self, &expression),
            Self::Print { expression } => visitor.visit_print_stmt(&self, &expression),
            Self::Variable { name, initializer } => {
                visitor.visit_var_stmt(&self, &name, &initializer)
            }
            Self::Block { statements } => visitor.visit_block_statement(&self, statements),
            Self::While { condition, body } => visitor.visit_while_statement(&self, condition, body),
            Self::If {
                 condition,
                 then_branch,
                 else_branch,
             } => visitor.visit_if_statement(&self, condition, then_branch, else_branch),
        }
    }
}
