use super::token::*;
use super::literal::*;
use super::token_type::*;


pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
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

    pub fn add_token(&mut self, of_type: TokenType, literal: Option<Literal>) {
        self.add_token_val(of_type, literal)
    }

    pub fn add_token_val(&mut self, of_type: TokenType, literal: Option<Literal>) {
        let text = &self.source[self.start..self.current];

        self.tokens
            .push(Token::new(of_type, text.to_string(), literal, self.line))
    }

    pub fn advance_if_then(&mut self, next: char) -> bool {
        if self.is_at_end() {
            return false;
        } else if self.source.chars().nth(self.current).unwrap() != next {
            return false;
        } else {
            self.current += 1;
            true
        }
    }
    pub fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN, None),
            ')' => self.add_token(TokenType::RIGHT_PAREN, None),
            '{' => self.add_token(TokenType::LEFT_BRACE, None),
            '}' => self.add_token(TokenType::RIGHT_BRACE, None),
            ',' => self.add_token(TokenType::COMMA, None),
            '.' => self.add_token(TokenType::DOT, None),
            '-' => self.add_token(TokenType::MINUS, None),
            '+' => self.add_token(TokenType::PLUS, None),
            ';' => self.add_token(TokenType::SEMICOLON, None),
            '*' => self.add_token(TokenType::STAR, None),
            'o' => {
                if self.advance_if_then('r') {
                    self.add_token(TokenType::OR, None)
                }
            }
            '!' => {
                if self.advance_if_then('=') {
                    self.add_token(TokenType::BANG_EQUAL, None)
                }
            }
            '=' => {
                if self.advance_if_then('=') {
                    self.add_token(TokenType::EQUAL_EQUAL, None)
                }
            }
            '<' => {
                if self.advance_if_then('=') {
                    self.add_token(TokenType::LESS_EQUAL, None)
                }
            }
            '>' => {
                if self.advance_if_then('=') {
                    self.add_token(TokenType::GREATER_EQUAL, None)
                }
            }
            '/' => {
                if self.advance_if_then('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH, None);
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => {
                self.string();
            }
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_apha(c) {
                    self.identifier()
                } else {
                    print!("Unexpected character.")
                }
            }
        }
    }

    pub fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = self.source[self.start..self.current].trim();
        
        if let Some(token_type) = self.match_identifier(text.to_string()) {
            // Keyword match.
            self.add_token(token_type, None)
        } else {
            // User defined identifier.
            self.add_token(TokenType::IDENTIFIER, None)
        }
    }

    // Returns Some(TokenType) if any of the identifiers matches.
    pub fn match_identifier(&mut self, c: String) -> Option<TokenType> {
        match c.as_str() {
            "else" => Some(TokenType::ELSE),
            "and" => Some(TokenType::AND),
            "class" => Some(TokenType::CLASS),
            "false" => Some(TokenType::FALSE),
            "for" => Some(TokenType::FOR),
            "fun" => Some(TokenType::FUN),
            "if" => Some(TokenType::IF),
            "nil" => Some(TokenType::NIL),
            "or" => Some(TokenType::OR),
            "print" => Some(TokenType::PRINT),
            "return" => Some(TokenType::RETURN),
            "super" => Some(TokenType::SUPER),
            "this" => Some(TokenType::THIS),
            "true" => Some(TokenType::TRUE),
            "var" => Some(TokenType::VAR),
            "while" => Some(TokenType::WHILE),
            _ => None,
        }
    }

    pub fn is_apha(&mut self, c: char) -> bool {
        c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
    }

    pub fn is_alphanumeric(&mut self, c: char) -> bool {
        self.is_apha(c) || self.is_digit(c)
    }

    pub fn is_digit(&mut self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    pub fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        let peek_next = self.peek_next();
        if self.peek() == '.' && self.is_digit(peek_next) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let lexeme = self.source[self.start..self.current]
            .trim()
            .chars()
            .collect::<String>();

        // Parses the lexeme to an i32 type.
        //
        // TODO: f32 as well as f64 support.

        let parsed_lexeme = lexeme.parse::<i32>().expect("Unexpected parsing behaviour");
        self.add_token(TokenType::NUMBER, Some(Literal::Number(parsed_lexeme)));
    }

    pub fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            let character = '\0';
            return character;
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    pub fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() != '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            print!("Error handling");
        }

        self.advance();

        let value = self.source[self.start + 1..self.current - 1]
            .chars()
            .collect::<String>();
        self.add_token(TokenType::STRING, Some(Literal::String(value)));
    }

    // advance()-like function, but doesn't consumes the character.
    // returns a reference to the next character.
    pub fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        } else {
            self.source.chars().nth(self.current).unwrap()
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
