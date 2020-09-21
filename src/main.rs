use std::fmt;
pub mod literal;
pub mod token;
pub mod token_type;

use literal::*;
use token::*;
use token_type::*;
pub type Result<T> = std::result::Result<T, String>;
use std::{
    fs,
    io::{self, Write},
};

/// TODO
/// Improve error handling.
///
/// Modify the run function so it now can succesfully handle a parser as well as its
/// error handling function.

pub enum ParseError {
    SyntaxError(Token, String),
}

#[derive(PartialEq, Debug)]
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

struct Interpreter;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Object {
    Boolean(bool),
    Number(i32),
    Str(String),
    Undefined,
    Nil,
}

impl Object {
    pub fn from_literal(literal: &Literal) -> Self {
        match literal {
            Literal::String(x) => Object::Str(x.clone()),
            Literal::Number(x) => Object::Number(*x),
            Literal::Boolean(x) => Object::Boolean(*x),
            _ => Object::Undefined,
        }
    }

    pub fn is_truthy(&mut self) -> bool {
        match self {
            Object::Nil => false,
            Object::Boolean(x) => *x,
            _ => true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub token: Option<Token>,
    pub message: String,
}

impl RuntimeError {
    pub fn new(token: &Token, message: &str) -> Self {
        Self {
            token: Some(token.to_owned()),
            message: message.to_owned(),
        }
    }
    pub fn with_message(message: &str) -> Self {
        Self {
            token: None,
            message: message.to_owned(),
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(token) = &self.token {
            write!(f, "token: {:?} error:\"{:?}\"", token, self.message)
        } else {
            write!(f, "error:\"{}\"", self.message)
        }
    }
}

pub enum InterpretResultStatus {
    // Returned when a runtime error occurs
    Error(RuntimeError),

    // Returned when control is flowing up the stack from a brack statement to the innermost loop.
    Break,

    // Return statement in a function, carrying optional return value payload.
    Return(Option<Object>),
}

impl Visitor<Result<Object>> for Interpreter {
    fn visit_binary_expression(
        &mut self,
        expr: &Expr,
        left: &Box<Expr>,
        operator: &Token,
        right: &Box<Expr>,
    ) -> Result<Object> {
        let mut left = self.evaluate(left)?;
        let mut right = self.evaluate(right)?;

        match operator.of_type {
            TokenType::MINUS => {
                if let Object::Number(left) = left {
                    if let Object::Number(right) = right {
                        Ok(Object::Number(left - right))
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }
            }
            TokenType::STAR => {
                if let Object::Number(left) = left {
                    if let Object::Number(right) = right {
                        Ok(Object::Number(left * right))
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }
            }

            TokenType::PLUS => {
                if let Object::Number(left) = left {
                    if let Object::Number(right) = right {
                        Ok(Object::Number(left + right))
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }
            }

            TokenType::SLASH => {
                if let Object::Number(left) = left {
                    if let Object::Number(right) = right {
                        Ok(Object::Number(left / right))
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }
            }
            TokenType::GREATER => {
                if let Object::Number(left) = left {
                    if let Object::Number(right) = right {
                        Ok(Object::Boolean(left > right))
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }
            }

            TokenType::LESS => {
                if let Object::Number(left) = left {
                    if let Object::Number(right) = right {
                        Ok(Object::Boolean(left < right))
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }
            }
            TokenType::LESS_EQUAL => {
                if let Object::Number(left) = left {
                    if let Object::Number(right) = right {
                        Ok(Object::Boolean(left <= right))
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }
            }

            TokenType::GREATER_EQUAL => {
                if let Object::Number(left) = left {
                    if let Object::Number(right) = right {
                        Ok(Object::Boolean(left >= right))
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }
            }

            TokenType::EQUAL_EQUAL => {
                if let Object::Number(left) = left {
                    if let Object::Number(right) = right {
                        Ok(Object::Boolean(left == right))
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }
            }

            TokenType::BANG_EQUAL => {
                if let Object::Number(left) = left {
                    if let Object::Number(right) = right {
                        Ok(Object::Boolean(left != right))
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }
            }

            _ => unreachable!(),
        }
    }

    fn visit_group_expression(
        &mut self,
        expr: &Expr,
        content: &Box<Expr>,
    ) -> Result<Object> {
        self.evaluate(content)
    }

    fn visit_literal_expression(
        &mut self,
        expr: &Expr,
        literal: &Literal,
    ) -> Result<Object> {
        return Ok(Object::from_literal(literal));
    }

    fn visit_unary_expression(
        &mut self,
        _expr: &Expr,
        operator: &Token,
        right: &Box<Expr>,
    ) -> Result<Object> {
        let mut right = self.evaluate(right)?;

        match operator.of_type {
            TokenType::MINUS => match right {
                Object::Number(n) => Ok(Object::Number(-n)),
                _ => unimplemented!(),
            },
            TokenType::BANG => Ok(Object::Boolean(!right.is_truthy())),
            _ => unreachable!(),
        }
    }
}

impl Interpreter {
    pub fn evaluate(&mut self, expr: &Box<Expr>) -> Result<Object> {
        match self._evaluate(expr) {
            Ok(result) => Ok(result),
            Err(e) => unimplemented!(),
        }
    }

    fn _evaluate(&mut self, expr: &Box<Expr>) -> Result<Object> {
        expr.accept(self)
    }
}

///
///expression     → equality ;
///equality       → comparison ( ( "!=" | "==" ) comparison )* ;
///comparison     → addition ( ( ">" | ">=" | "<" | "<=" ) addition )* ;
///addition       → multiplication ( ( "-" | "+" ) multiplication )* ;
///multiplication → unary ( ( "/" | "*" ) unary )* ;
///unary          → ( "!" | "-" ) unary
///               | primary ;
///primary        → NUMBER | STRING | "false" | "true" | "nil"
///               | "(" expression ")" ;
///

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            current: 0,
        }
    }

    pub fn expression(&mut self) -> Result<Box<Expr>>{
        self.equality()
    }

    pub fn parse(&mut self) -> Result<Box<Expr>>{
        self.expression()
    }

    ///
    ///equality → comparison ( ( "!=" | "==" ) comparison )* ;
    ///
    pub fn equality(&mut self) -> Result<Box<Expr>>{
        let mut expr = self.comparison()?;

        while self.if_match(&vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;

            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right: right,
            });
        }
        Ok(expr)
    }

    pub fn if_match(&mut self, token_types: &[TokenType]) -> bool {
        for token in token_types {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    // Returns the current token we have yet to consume.
    pub fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    // Checks if we run out of tokens to parse.
    pub fn is_at_end(&self) -> bool {
        self.peek().of_type == TokenType::EOF
    }
    pub fn check(&self, of_type: &TokenType) -> bool {
        if self.is_at_end() {
            return true;
        }
        &self.peek().of_type == of_type
    }

    pub fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1
        }

        self.previous()
    }

    //Last consumed token.
    pub fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).expect("AAAAAAAAAAAA")
    }
    pub fn comparison(&mut self) -> Result<Box<Expr>>{
        let mut expr = self.addition()?;

        while self.if_match(&[
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous().clone();
            let right = self.addition()?;

            expr = Box::new(Expr::Binary {
                right: expr,
                operator,
                left: right,
            });
        }
        Ok(expr)
    }

    /// unary -> ("!" | "-") unary
    ///          | primary ;
    /// If the current token is rather
    /// a band or minux sign, then we are in the precense
    /// of an unary expresion.
    ///
    /// the operator is consumed in the first call to unary()
    /// then we grab the token and recursively call unary() to
    /// parse the operand. Finally, wrap that all up in an unary
    /// expression syntax tree.
    pub fn unary(&mut self) -> Result<Box<Expr>> {
        if self.if_match(&vec![TokenType::BANG, TokenType::MINUS]) {
            let operator: Token = self.previous().clone();
            let right = self.unary()?;

            return Ok(Box::new(Expr::Unary {
                operator,
                right: right,
            }));
        }
        /// a primary expression is reached.
        /// where
        /// primary → NUMBER | STRING | "false" | "true" | "nil"
        ///         | "(" expression ")" ;
        self.primary()
    }

    pub fn primary(&mut self) -> Result<Box<Expr>>{
        if self.if_match(&vec![TokenType::FALSE]) {
            return Ok(Box::new(Expr::Literal {
                literal: Literal::Boolean(false),
            }));
        }
        if self.if_match(&vec![TokenType::TRUE]) {
            return Ok(Box::new(Expr::Literal {
                literal: Literal::Boolean(true),
            }));
        }
        if self.if_match(&vec![TokenType::NIL]) {
            return Ok(Box::new(Expr::Literal {
                literal: Literal::None,
            }));
        }
        if self.if_match(&vec![TokenType::NUMBER, TokenType::STRING]) {
            return Ok(Box::new(Expr::Literal {
                literal: self.previous().literal.clone().unwrap(),
            }));
        }
        if self.if_match(&vec![TokenType::LEFT_PAREN]) {
            let expr = self.expression();

            self.consume(TokenType::RIGHT_PAREN, String::from("Expected ')'"));
            return Ok(Box::new(Expr::Grouping { expression: expr? }));
        } else {
            unimplemented!() //Err(ParseError::SyntaxError(self.peek(), String::from("Expected expression")))
        }
    }

    pub fn consume(&mut self, token_type: TokenType, message: String) -> Result<&Token> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            Err(
                String::from("Error"), //ParseError::SyntaxError(self.peek().clone(), message)
            )
        }
    }

    pub fn multiplication(&mut self) -> Result<Box<Expr>>{
        let mut expr = self.unary()?;

        while self.if_match(&vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            expr = Box::new(Expr::Binary {
                left:expr, 
                operator,
                right: right,
            });
        }

        Ok(expr)
    }

    pub fn synchronize(&mut self) -> () {
        self.advance();

        while !self.is_at_end() {
            if self.previous().of_type == TokenType::SEMICOLON {
                return;
            }
        }

        match self.peek().of_type {
            TokenType::CLASS => {}
            TokenType::FUN => {}
            TokenType::VAR => {}
            TokenType::FOR => {}
            TokenType::IF => {}
            TokenType::WHILE => {}
            TokenType::PRINT => {}
            TokenType::RETURN => {}
            _ => (),
        }

        self.advance();
    }

    pub fn addition(&mut self) -> Result<Box<Expr>>{
        let mut expr = self.multiplication()?;

        while self.if_match(&vec![TokenType::PLUS, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = self.multiplication()?;

            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right: right,
            });
        }
        Ok(expr)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },

    Grouping {
        expression: Box<Expr>,
    },

    Literal {
        literal: Literal,
    },

    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

pub trait Visitor<Q> {
        fn visit_binary_expression(
        &mut self,
        expr: &Expr,
        left: &Box<Expr>,
        operator: &Token,
        right: &Box<Expr>,
    ) -> Q;

    fn visit_group_expression(&mut self, expr: &Expr, content: &Box<Expr>) -> Q;

    fn visit_literal_expression(&mut self, expr: &Expr, literal: &Literal) -> Q;

    fn visit_unary_expression(&mut self, expr: &Expr, operator: &Token, right: &Box<Expr>) -> Q;
}

struct AstPrinter;

impl AstPrinter {
    fn new() -> Self {
        AstPrinter {}
    }
    fn print(&mut self, expr: &Vec<Box<Expr>>) -> String {
        let mut builder = String::new();

        for expression in expr {
            builder.push_str(expression.accept(self).as_str());
        }

        builder
    }

    fn parenthesize(&mut self, name: &str, expr: &Vec<&Box<Expr>>) -> String {
        let mut builder = String::from("(");

        builder.push_str(name);

        for expression in expr {
            builder.push_str(" ");
            builder.push_str(expression.accept(self).as_str());
        }

        builder.push_str(")");

        builder
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expression(
        &mut self,
        expr: &Expr,
        left: &Box<Expr>,
        operator: &Token,
        right: &Box<Expr>,
    ) -> String {
        self.parenthesize(&operator.lexeme, &vec![left, right])
    }

    fn visit_group_expression(&mut self, expr: &Expr, content: &Box<Expr>) -> String {
        self.parenthesize("Group", &vec![content])
    }

    fn visit_literal_expression(&mut self, expr: &Expr, literal: &Literal) -> String {
        literal.to_string()
    }

    fn visit_unary_expression(
        &mut self,
        _expr: &Expr,
        operator: &Token,
        right: &Box<Expr>,
    ) -> String {
        self.parenthesize(&operator.lexeme, &vec![right])
    }
}

impl Expr {
    fn accept<T, R>(&self, expr: &mut T) -> R
    where
        T: Visitor<R>,
    {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => expr.visit_binary_expression(&self, &left, &operator, &right),

            Expr::Grouping { expression } => expr.visit_group_expression(&self, &expression),
            Expr::Literal { literal } => expr.visit_literal_expression(&self, &literal),
            _ => unreachable!(),
        }
    }
}

fn run_file(file: &String) {
    let bytes = fs::read_to_string(file).expect("Error reading external file.");

    run(&bytes);
}

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
        let peek_character = self.peek();
        while self.is_alphanumeric(peek_character) {
            self.advance();
        }

        let mut text = self.source[self.start..self.current].trim();

        if let Some(token_Type) = self.match_identifier(text.to_string()) {
            // Keyword match.
            self.add_token(token_Type, None)
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
        let peek = self.peek();
        while self.is_digit(peek) {
            self.advance();
        }

        let peek_next = self.peek_next();
        if self.peek() == '.' && self.is_digit(peek_next) {
            self.advance();

            while self.is_digit(peek) {
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

fn run(source: &String) -> Result<()> {
    let mut input = "(+);".to_string();
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens.to_vec());

    let mut expression = parser.expression();

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

    #[test]
    fn free_form_code_test() {
        let mut input = "/".to_string();
        let mut scanner = Scanner::new(input);

        scanner.scan_tokens();

        assert_eq!(
            scanner.tokens_helper(),
            vec!(
                Token {
                    of_type: TokenType::SLASH,
                    lexeme: String::from("/"),
                    literal: None,
                    line: 1,
                },
                Token {
                    of_type: TokenType::EOF,
                    lexeme: String::from(""),
                    literal: None,
                    line: 1
                },
            )
        );
    }

    #[test]
    fn number_parsing_test() {
        let mut input = "1+2".to_string();
        let mut scanner = Scanner::new(input);

        scanner.scan_tokens();

        assert_eq!(
            scanner.tokens_helper(),
            vec!(
                Token {
                    of_type: TokenType::NUMBER,
                    lexeme: String::from("1"),
                    literal: Some(Literal::Number(1)),
                    line: 1
                },
                Token {
                    of_type: TokenType::PLUS,
                    lexeme: String::from("+"),
                    literal: None,
                    line: 1
                },
                Token {
                    of_type: TokenType::NUMBER,
                    lexeme: String::from("2"),
                    literal: Some(Literal::Number(2)),
                    line: 1
                },
                Token {
                    of_type: TokenType::EOF,
                    lexeme: String::from(""),
                    literal: None,
                    line: 1
                },
            )
        );
    }

    #[test]
    fn basic_number_test() {
        let input = "1".to_string();
        let mut scanner = Scanner::new(input);

        scanner.scan_tokens();

        assert_eq!(
            scanner.tokens_helper(),
            vec!(
                Token {
                    of_type: TokenType::NUMBER,
                    literal: Some(Literal::Number(1)),
                    lexeme: String::from("1"),
                    line: 1
                },
                Token {
                    of_type: TokenType::EOF,
                    literal: None,
                    lexeme: String::from(""),
                    line: 1,
                }
            )
        )
    }

    #[test]
    fn evaluation_test() {
        let input = vec![
            ("1+2+3", Object::Number(7)),
        ];

        for (expression, expected_result) in input {
            let mut scanner = Scanner::new(expression.to_string());
            let tokens = scanner.scan_tokens();

            let mut parser = Parser::new(tokens.to_vec());

            let expr = parser.parse().unwrap();

            let mut interpreter = Interpreter{};
            let result = interpreter.evaluate(&expr).unwrap();

            assert_eq!(result, expected_result);
        }
    }
}
