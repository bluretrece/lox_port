use crate::object::*;
use std::collections::HashMap;
use crate::token::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Environment {
    pub values: HashMap<String, Object>,
    pub enclosing: Option<Rc<RefCell<Environment>>>
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None
        }
    }
    
    pub fn with_ref(environment: Rc<RefCell<Environment>>) -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: Some(environment)
        }
    }

    pub fn define(&mut self, name: &str, value: &Object) {
        self.values.insert(name.to_string(), value.clone());
    }
    
    pub fn assign(&mut self, name: &Token, value: Object) {
        let variable = name.lexeme();
        
        match self.values.insert(variable, value.clone()) {
            Some(_) => {},
            None => {
                if let Some(enclosing) = &self.enclosing {
                    (*enclosing.borrow_mut()).assign(name, value.clone());
                } else {
                    panic!("Variable not found in scope.")
                }
            }
        }
    }
    
    pub fn get(&mut self, name: Token) -> Object {
        if let Some(value) = self.values.get(&name.lexeme()) {
            value.clone()
        } else if let Some(enclosing) = &self.enclosing {
            return (*enclosing.borrow_mut()).get(name.clone())
        }
        else { Object::Nil }
    }
}
