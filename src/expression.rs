use crate::object::*;
use crate::token::*;
use crate::literal::*;
use crate::lox_error::*;

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    //Logical {
    //    left: Box<Expr>,
    //    operator: Token,
    //    right: Box<Expr>
    //},
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token
    },
    Grouping {
        expression: Box<Expr>,
    },

    Literal {
        literal: Literal,
    },

    Unary {
        operator: Token,
        right: Box<Expr>,
    },

    Assign {
        name: Token,
        value: Box<Expr>
    }
}
pub trait ExprVisitor {
    type Value;
    
    fn visit_assign_expression(
        &mut self,
        name: &Token,
        value: &Box<Expr>
    ) -> Result<Self::Value, LoxError>;

    fn visit_binary_expression(
        &mut self,
        left: &Box<Expr>,
        operator: &Token,
        right: &Box<Expr>,
    ) -> Result<Self::Value, LoxError>;

    fn visit_group_expression(
        &mut self,
        content: &Box<Expr>,
    ) -> Result<Self::Value, LoxError>;

    fn visit_literal_expression(
        &mut self,
        literal: &Literal,
    ) -> Result<Self::Value, LoxError>;

    fn visit_unary_expression(
        &mut self,
        operator: &Token,
        right: &Box<Expr>,
    ) -> Result<Self::Value, LoxError>;

    fn visit_variable_expression(
        &mut self,
        name: &Token
    ) -> Result<Self::Value, LoxError>;
}

pub trait Visitable {
    fn accept(&self, visitor: &mut dyn ExprVisitor<Value = Object>) -> Result<Object, LoxError>;
}


impl Visitable for Expr {
    fn accept(&self, expr: &mut dyn ExprVisitor<Value=Object>) -> Result<Object, LoxError> {
        match self {
            Expr::Binary {
                left,
               operator,
                right,
            } => expr.visit_binary_expression(&left, &operator, &right),
            Expr::Grouping { expression } => expr.visit_group_expression(&expression),
            Expr::Literal { literal } => expr.visit_literal_expression(&literal),
            //Expr::Logical {left, operator, right} => expr.visit_logical_expression(&self, &left, &operator, &right),
            Expr::Assign {name, value} => expr.visit_assign_expression(&name, &value),
            Expr::Unary {operator, right } => expr.visit_unary_expression( &operator, &right),
            Expr::Variable { name } => expr.visit_variable_expression(&name),
        }
    }
}

