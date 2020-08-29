pub mod token_type;
use std::fmt;
pub mod literal;
use literal::Literal;
use std::{
    fs,
    io::{self, Write},
};
use token_type::TokenType;

#[derive(Eq, PartialEq, PartialOrd)]
pub struct Token {
    of_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref literal) = self.literal {
            write!(f, "{:?} {:?} {:?}", self.of_type, self.lexeme, literal)
        } else {
            write!(f, "{:?} {:?}", self.of_type, self.lexeme)
        }
    }
}

impl Token {
    fn new(of_type: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Self {
        Self {
            of_type,
            lexeme,
            literal: literal,
            line,
        }
    }
}

fn run_file(file: &String) {
    let bytes = fs::read_to_string(file).expect("Error reading external file.");

    run(&bytes);
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: String) -> Self {
        Self {
            source: source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    pub fn add_token(&mut self, of_type: TokenType) {
        self.add_token_val(of_type, None )
    }

    pub fn add_token_val(&mut self, of_type: TokenType, literal: Option<Literal>) {
        let text = &self.source[self.start..self.current];

        self.tokens.push(Token::new(
            of_type,
            text.to_string(),
            literal,
            self.line,
        ))
    }
    
    pub fn advance_if_then(&mut self, next: char) -> bool {
        if self.is_at_end() { return false }
        else if self.source.chars().nth(self.current).unwrap() != next { return false }
        else {
            self.current += 1;
            true
        }
    }
    pub fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                if self.advance_if_then('='){
                    self.add_token(TokenType::BANG_EQUAL)
                }
            }
            _ => print!("Unexpected character."),
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), None, self.line));

        &self.tokens
    }

    // Test purposes. Returns the current token.
    pub fn tokens_helper(self) -> Vec<Token> {
        self.tokens
    }
}

fn run_prompt() {
    let buffer = io::stdin();
    let mut stdout = io::stdout();
    let mut source = String::new();
    loop {
        print!("> ");
        stdout.flush();
        source.clear();
        buffer.read_line(&mut source).expect("Error handling input");

        run(&source);
    }
}

fn run(source: &String) -> Result<(), String> {
    let mut input = "(+);".to_string();
    let mut scanner = Scanner::new(input);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scanner_test() {
        let mut input = "(+);".to_string();

        let mut scanner = Scanner::new(input);

        scanner.scan_tokens();

        assert_eq!(
            scanner.tokens_helper(),
            vec!(
                Token {
                    of_type: TokenType::LEFT_PAREN,
                    lexeme: String::from("("),
                    literal: None,
                    line: 1,
                },
                Token {
                    of_type: TokenType::PLUS,
                    lexeme: String::from("+"),
                    literal: None,
                    line: 1,
                },
                Token {
                    of_type: TokenType::RIGHT_PAREN,
                    lexeme: String::from(")"),
                    literal: None,
                    line: 1,
                },
                Token {
                    of_type: TokenType::SEMICOLON,
                    lexeme: String::from(";"),
                    literal: None,
                    line: 1,
                },
                Token {
                    of_type: TokenType::EOF,
                    lexeme: String::from(""),
                    literal: None,
                    line: 1,
                },
            )
        );
    }
}
