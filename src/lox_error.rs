use crate::token::*;

#[derive(Debug)]
pub enum LoxError {
    RuntimeError(String),
    BindingError(String, String),
}

impl std::fmt::Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LoxError::RuntimeError(message) => write!(f, "RuntimeError: {}", message),
            LoxError::BindingError(token, message) => {
                write!(f, "BindingError for {}: {}", token, message)
            }
        }
    }
}

/// TODO
/// Improve error handling.
///
/// Modify the run function so it now can succesfully handle a parser as well as its
/// error handling function.
pub enum ParseError {
    SyntaxError(Token, String),
}
