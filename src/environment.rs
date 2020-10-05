use crate::object::*;
use std::collections::HashMap;
use crate::token::*;
use crate::lox_error::*;
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

    pub fn define(&mut self, name: &str, value: Object) {
        self.values.insert(name.to_string(), value);
    }
    
    pub fn assign(&mut self, name: &Token, value: Object) {
        match self.values.get(&name.lexeme) {
            Some(_) => {
                self.values.insert(name.lexeme.clone(), value);
                
            },
            None => unimplemented!()
        }
    }
    
    pub fn get(&mut self, name: Token) -> Object {
        if let Some(value) = self.values.get(&name.lexeme()) {
            return value.clone()
        }
        
        if let Some(enclosing) = &self.enclosing{
            return (*enclosing.borrow_mut()).get(name.clone())
        }
        else { todo!() }
    }
}
