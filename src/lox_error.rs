use crate::token::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoxError {
    #[error("Runtime error: {0}")]
    RuntimeError(String),
    #[error("Binding error for {0}: {1}")]
    BindingError(String, String),
}

//impl std::fmt::Display for LoxError {
//    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//        match self {
//            LoxError::RuntimeError(message) => write!(f, "RuntimeError: {}", message),
//            LoxError::BindingError(token, message) => {
//                write!(f, "BindingError for {}: {}", token, message)
//            }
//        }
//    }
//}

pub enum ParseError {
    SyntaxError(Token, String),
}
