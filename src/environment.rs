use crate::lox_error::LoxError;
use crate::lox_class::LoxClass;
use crate::object::*;
use crate::token::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::interpreter::{ReturnStatus, LoxResult};


#[derive(Clone, Debug)]
pub struct Environment {
    pub values: HashMap<String, Object>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn get_at(&self, distance: usize, name: &str) -> LoxResult<Object> {
        if distance == 0 {
            if let Some(v) = self.values.get(name) {
                return Ok(v.clone());
            }
        }
        if let Some(ancestor) = &self.enclosing {
            return ancestor.borrow().get_at(distance - 1, name);
        }

        let error = LoxError::RuntimeError(String::from(
            "Cannot read local variable in its own initializer.",
        ));
        return Err(ReturnStatus::Error(error));
    }

    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn with_ref(environment: Rc<RefCell<Environment>>) -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: Some(environment),
        }
    }

    pub fn child_of(parent: Rc<RefCell<Environment>>) -> Self {
        Self {
            enclosing: Some(parent),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: &Object) {
        self.values.insert(name.to_string(), value.clone());
    }

    pub fn assign(&mut self, name: &Token, value: Object) {
        let variable = name.lexeme();

        match self.values.insert(variable, value.clone()) {
            Some(_) => {}
            None => {
                if let Some(enclosing) = &self.enclosing {
                    (*enclosing.borrow_mut()).assign(name, value.clone());
                } else {
                    panic!("Variable not found in scope.")
                }
            }
        }
    }

    pub fn get(&self, name: &Token) -> Object {
        if let Some(value) = self.values.get(&name.lexeme()) {
            value.clone()
        } else if let Some(enclosing) = &self.enclosing {
            return (*enclosing.borrow_mut()).get(name);
        } else {
            Object::Nil
        }
    }
}
