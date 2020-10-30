use std::ops::{Add, Div, Mul, Neg, Not, Sub};
use std::cmp::{Ordering, PartialOrd};
use std::fmt;
use crate::literal::*;
use crate::lox_error::*;


#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Boolean(bool),
    Number(i32),
    Str(String),
    Nil,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean(b) => write!(f, "{}", b),
            Self::Number(x) => write!(f, "{}", x),
            Self::Str(s) => write!(f, "{}", s),
            Self::Nil=> write!(f,"Nil"),
        }
    }
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Object) -> Option<Ordering> {
        match self {
            Object::Nil => match other {
                Object::Nil => Some(Ordering::Equal),
                _ => Some(Ordering::Greater),
            },
            Object::Number(value) => match other {
                Object::Number(other_value) => value.partial_cmp(other_value),
                _ => panic!("Can't compare a number with this value"),
            },
            _ => panic!("Can't compare these two types"),
        }
    }
}


impl Neg for Object {
    type Output = Result<Object, String>;
    fn neg(self) -> Result<Object, String> {
        match self {
            Object::Nil => Ok(Object::Boolean(true)),
            Object::Number(x) => Ok(Object::Number(-x)),
            Object::Boolean(_) => Err(String::from("Operation not supported")),
            Object::Str(_) => Err(String::from("Operation not supported"))
        }
    }
}

impl Not for Object {
    type Output = Object;

    fn not(self) -> Object {
        match self {
            Object::Boolean(b) => Object::Boolean(!b),
            Object::Nil => Object::Boolean(true),
            _ => Object::Boolean(false)
        }
    }
}

impl Div for Object {
    type Output = Object;

    fn div(self, rhs: Object) -> Object {
        match self {
            Object::Number(value) => match rhs {
                Object::Number(rhs_value) => Object::Number(value / rhs_value),
                _ => panic!("Can't divide these two values"),
            },
            _ => panic!("Can't divide these two values"),
        }
    }
}


impl Sub for Object{
    type Output = Object;

    fn sub(self, rhs: Object) -> Object {
        match self {
            Object::Number(value) => match rhs {
                Object::Number(rhs_value) => Object::Number(value - rhs_value),
                _ => panic!("Can't subtract these two values"),
            },
            _ => panic!("Can't subtract these two values"),
        }
    }
}

impl Mul for Object{
    type Output = Object;

    fn mul(self, rhs: Object) -> Object {
        match self {
            Object::Number(value) => match rhs {
                Object::Number(rhs_value) => Object::Number(value * rhs_value),
                _ => panic!("Can't multiply these two values"),
            },
            _ => panic!("Can't multiply these two values"),
        }
    }
}

impl Add for Object {
    type Output = Result<Object, LoxError>;

    fn add(self, rhs: Object) -> Result<Object, LoxError> {
        match self {
            Object::Number(value) => match rhs {
                Object::Number(rhs_value) => Ok(Object::Number(value + rhs_value)),
                _ => Err(LoxError::RuntimeError(
                    "right hand side must also be a number".to_string(),
                )),
            },
            Object::Str(value) => match rhs {
                Object::Str(rhs_value) => {
                    let mut new_str = value.clone();
                    new_str.push_str(&rhs_value);
                    Ok(Object::Str(new_str))
                }
                _ => panic!("TypeError: Can't add a string to a number."),
            },
            Object::Boolean(_value) => Err(LoxError::RuntimeError(
                "Cannot add value to boolean.".to_string(),
            )),
            Object::Nil => Err(LoxError::RuntimeError(
                "Cannot add value to nil.".to_string(),
            )),
        }
    }
}

impl Object {
    pub fn from_literal(literal: &Literal) -> Self {
        match literal {
            Literal::String(x) => Object::Str(x.clone()),
            Literal::Number(x) => Object::Number(*x),
            Literal::Boolean(x) => Object::Boolean(*x),
            Literal::None => Object::Nil,
        }
    }

    pub fn is_truthy(&mut self) -> bool {
        match self {
            Object::Nil => false,
            Object::Boolean(x) => *x,
            _ => true,
        }
    }
}
