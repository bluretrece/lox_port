use std::time::{SystemTime, UNIX_EPOCH};

use crate::interpreter::{Interpreter, LoxCallable, LoxResult};
use crate::object::*;
use crate::lox_error::*;

pub struct NativeClock;
impl NativeClock {
    pub fn new() -> Self {
        NativeClock {}
    }
}
impl LoxCallable for NativeClock {
    fn arity(&self) -> usize {
        0
    }
    fn call(&self, _: &mut Interpreter, _: &Vec<Object>) -> LoxResult<Option<Object>> {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        let seconds = since_the_epoch.as_secs_f64();
        let fractional = since_the_epoch.subsec_nanos() as f64 / 1e9;
        Ok(Some(Object::Number(seconds as i32 + fractional as i32)))
    }
}
