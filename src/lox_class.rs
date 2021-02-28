use crate::interpreter::{Interpreter, LoxCallable, LoxResult};
use crate::object::Object;
use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Instance {
    class: LoxClass,
    fields: HashMap<String, Object>
}

impl Instance {
    pub fn new(class: LoxClass)-> Self {
        Instance {
            class: LoxClass::new(),
            fields: HashMap::new()
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoxClass {
    pub name: String,
}

impl LoxClass {
    pub fn new(name: String) -> Self {
        LoxClass { name }
    }
}

impl std::fmt::Display for LoxClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl LoxCallable for LoxClass {
    fn call(&self, interpreter: &mut Interpreter, arguments: &Vec<Object>) -> LoxResult<Option<Object>> {
        let instance = Object::Instance(Instance::new(self));

        Ok(Some(instance))
    }

    fn arity(&self) -> usize {
        0
    }
}
