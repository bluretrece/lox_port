use crate::expression::*;
use crate::object::*;
use crate::token::*;

#[derive(Clone)]
pub enum Statement {
    While {
        condition: Box<Expr>,
        body: Box<Statement>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
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

    Block {
        statements: Vec<Box<Statement>>,
    },
}

pub trait StmtVisitor {
    type Value;
    fn visit_var_stmt(
        &mut self,
        stmt: &Statement,
        name: &Token,
        initializer: &Option<Box<Expr>>,
    ) -> Option<Self::Value>;
    fn visit_expression_stmt(&mut self, stmt: &Statement, expr: &Box<Expr>) -> Option<Self::Value>;
    fn visit_print_stmt(&mut self, stmt: &Statement, expr: &Box<Expr>) -> Option<Self::Value>;
    fn visit_block_statement(
        &mut self,
        _stmt: &Statement,
        statements: &Vec<Box<Statement>>,
    ) -> Option<Self::Value>;

    fn visit_if_statement(
        &mut self,
        stmt: &Statement,
        condition: &Box<Expr>,
        then_branch: &Box<Statement>,
        else_branch: &Option<Box<Statement>>,
    ) -> Option<Self::Value>;
    fn visit_while_statement(
        &mut self,
        _stmt: &Statement,
        condition: &Box<Expr>,
        body: &Box<Statement> 
    ) -> Option<Self::Value>;
}

pub trait Visitable {
    fn accept(&self, expr: &mut dyn StmtVisitor<Value = Object>) -> Option<Object>;
}

impl Visitable for Statement {
    fn accept(&self, visitor: &mut dyn StmtVisitor<Value = Object>) -> Option<Object> {
        match self {
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
