use crate::expression::*;
use crate::lox_error::*;
use crate::object::*;
use crate::token::*;

pub enum Statement {
    Print { expression: Box<Expr> },
    Expression { expression: Box<Expr> },
    Variable {
        name: Token,
        initializer: Option<Box<Expr>> 
    },

    Block {
        statements: Vec<Box<Statement>>
    }
}

pub trait StmtVisitor {
    type Value;
    fn visit_var_stmt(&mut self, stmt: &Statement, name: &Token, initializer: &Option<Box<Expr>>) -> Option<Self::Value>;
    fn visit_expression_stmt(&mut self, stmt: &Statement, expr: &Box<Expr>) -> Option<Self::Value>;
    fn visit_print_stmt(&mut self, stmt: &Statement, expr: &Box<Expr>) -> Option<Self::Value>;
    fn visit_block_statement(
        &mut self,
        _stmt: &Statement,
        statements: &Vec<Box<Statement>>
    ) -> Option<Self::Value>;
}

pub trait Visitable {
    fn accept(&self, expr: &mut StmtVisitor<Value = Object>) -> Option<Object>;
}

impl Visitable for Statement {
    fn accept(&self, visitor: &mut StmtVisitor<Value = Object>) -> Option<Object> {
        match self {
            Self::Expression { expression } => visitor.visit_expression_stmt(&self, &expression),
            Self::Print { expression } => visitor.visit_print_stmt(&self, &expression),
            Self::Variable { name, initializer} => unimplemented!()
        }
    }
}
