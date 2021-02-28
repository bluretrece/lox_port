use crate::object::*;
use crate::token::*;
use crate::literal::*;
use crate::interpreter::LoxResult;

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum Expr {
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
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

    Get {
        object: Box<Expr>,
        name: Token,
    },

    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Box<Expr>>
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
    
    fn visit_call_expression(
        &mut self,
        callee: &Box<Expr>,
        paren: &Token,
        arguments: &Vec<Box<Expr>>,
    ) -> LoxResult<Self::Value>;

    fn visit_assign_expression(
        &mut self,
        expr: &Expr,
        name: &Token,
        value: &Box<Expr>
    ) -> LoxResult<Self::Value>;

    fn visit_binary_expression(
        &mut self,
        left: &Box<Expr>,
        operator: &Token,
        right: &Box<Expr>,
    ) -> LoxResult<Self::Value>;

    fn visit_group_expression(
        &mut self,
        content: &Box<Expr>,
    ) -> LoxResult<Self::Value>;

    fn visit_literal_expression(
        &mut self,
        literal: &Literal,
    ) -> LoxResult<Self::Value>;

    fn visit_get_expression(&mut self,
        expr: &Expr,
        object: &Box<Expr>,
        name: &Token) -> LoxResult<Self::Value>;

    fn visit_unary_expression(
        &mut self,
        operator: &Token,
        right: &Box<Expr>,
    ) -> LoxResult<Self::Value>;
    
    fn visit_logical_expression(&mut self, expr: &Expr, left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> LoxResult<Self::Value>;

    fn visit_variable_expression(
        &mut self,
        expr: &Expr,
        name: &Token
    ) -> LoxResult<Self::Value>;
}

pub trait Visitable {
    fn accept(&self, visitor: &mut dyn ExprVisitor<Value = Object>) -> LoxResult<Object>;
}


impl Visitable for Expr {
    fn accept(&self, expr: &mut dyn ExprVisitor<Value=Object>) -> LoxResult<Object> {
        match self {
            Expr::Get {object, name} => expr.visit_get_expression(&self, object, name),
            Expr::Binary {
                left,
               operator,
                right,
            } => expr.visit_binary_expression(&left, &operator, &right),
            Expr::Grouping {expression} => expr.visit_group_expression(&expression),
            Expr::Literal { literal } => expr.visit_literal_expression(&literal),
            Expr::Logical {left, operator, right} => expr.visit_logical_expression(&self, &left, &operator, &right),
            Expr::Assign {name, value} => expr.visit_assign_expression(&self, &name, &value),
            Expr::Unary {operator, right } => expr.visit_unary_expression( &operator, &right),
            Expr::Variable { name } => expr.visit_variable_expression(&self, &name),
            Expr::Call { callee, paren, arguments } => expr.visit_call_expression(callee, paren, arguments),
        }
    }
}

