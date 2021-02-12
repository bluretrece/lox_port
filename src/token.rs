use super::literal::*;
use super::token_type::*;
use std::fmt;

#[derive(Eq, Clone,PartialEq, PartialOrd, Hash)]
pub struct Token {
    pub of_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.of_type)
    }
}

impl Token {
    pub fn new(of_type: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Self {
        Self {
            of_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn lexeme(&self) -> String {
        return self.lexeme.clone()
    }
}
