use std::cmp::{Ordering, PartialOrd};
use std::fmt;
pub mod literal;
pub mod scanner;
pub mod token;
pub mod token_type;
// Not necesary.
use std::ops::{Add, Div, Mul, Neg, Not, Sub};
use literal::*;
use scanner::*;
use std::{
    fs,
    io::{self, Write},
};
use token::*;
use token_type::*;

/// TODO
/// Improve error handling.
///
///
/// MORE TESTS
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

#[derive(Debug, PartialEq)]
pub enum Object {
    Boolean(bool),
    Number(i32),
    Str(String),
    Nil,
}

impl Object {
    pub fn from_literal(literal: &Literal) -> Self {
        match literal {
            Literal::String(x) => Object::Str(x.clone()),
            Literal::Number(x) => Object::Number(*x),
            Literal::Boolean(x) => Object::Boolean(*x),
            Literal::None => Object::Nil,
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

impl Visitor for Interpreter {
    type Value = Object;
    fn visit_binary_expression(
        &mut self,
        expr: &Expr,
        left: &Box<Expr>,
        operator: &Token,
        right: &Box<Expr>,
    ) -> Result<Self::Value, LoxError> {
        let mut left = self.evaluate(left)?;
        let mut right = self.evaluate(right)?;

        match operator.of_type {
            TokenType::MINUS => Ok(left - right),
            TokenType::SLASH => Ok(left / right),
            TokenType::STAR => Ok(left * right),
            TokenType::PLUS => left + right,
            TokenType::GREATER => Ok(Object::Boolean(left > right)),
            TokenType::GREATER_EQUAL => Ok(Object::Boolean(left >= right)),
            TokenType::LESS => Ok(Object::Boolean(left < right)),
            TokenType::LESS_EQUAL => Ok(Object::Boolean(left <= right)),
            TokenType::EQUAL_EQUAL => Ok(Object::Boolean(left == right)),
            TokenType::BANG_EQUAL => Ok(Object::Boolean(left != right)),
            _ => unreachable!()
        }
    }

    fn visit_group_expression(&mut self, expr: &Expr, content: &Box<Expr>) -> Result<Self::Value, LoxError> {
        self.evaluate(content)
    }

    fn visit_literal_expression(&mut self, expr: &Expr, literal: &Literal) -> Result<Self::Value, LoxError> {
        return Ok(Object::from_literal(literal));
    }

    fn visit_unary_expression(
        &mut self,
        _expr: &Expr,
        operator: &Token,
        right: &Box<Expr>,
    ) -> Result<Self::Value, LoxError> {
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

impl Add for Object {
    type Output = Result<Object, LoxError>;

    fn add(self, rhs: Object) -> Result<Object, LoxError> {
        match self {
            Object::Number(value) => match rhs {
                Object::Number(rhs_value) => Ok(Object::Number(value + rhs_value)),
                _ => Err(LoxError::RuntimeError(
                    "right hand side must also be a number".to_string(),
                )),
            },
            Object::Str(value) => match rhs {
                Object::Str(rhs_value) => {
                    let mut new_str = value.clone();
                    new_str.push_str(&rhs_value);
                    Ok(Object::Str(new_str))
                }
                _ => panic!("TypeError: Can't add a string to a number."),
            },
            Object::Boolean(_value) => Err(LoxError::RuntimeError(
                "Cannot add value to boolean.".to_string(),
            )),
            Object::Nil => Err(LoxError::RuntimeError(
                "Cannot add value to nil.".to_string(),
            )),
        }
    }
}

impl Mul for Object{
    type Output = Object;

    fn mul(self, rhs: Object) -> Object {
        match self {
            Object::Number(value) => match rhs {
                Object::Number(rhs_value) => Object::Number(value * rhs_value),
                _ => panic!("Can't multiply these two values"),
            },
            _ => panic!("Can't multiply these two values"),
        }
    }
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Object) -> Option<Ordering> {
        match self {
            Object::Nil => match other {
                Object::Nil => Some(Ordering::Equal),
                _ => Some(Ordering::Greater),
            },
            Object::Number(value) => match other {
                Object::Number(other_value) => value.partial_cmp(other_value),
                _ => panic!("Can't compare a number with this value"),
            },
            _ => panic!("Can't compare these two types"),
        }
    }
}

impl Div for Object {
    type Output = Object;

    fn div(self, rhs: Object) -> Object {
        match self {
            Object::Number(value) => match rhs {
                Object::Number(rhs_value) => Object::Number(value / rhs_value),
                _ => panic!("Can't divide these two values"),
            },
            _ => panic!("Can't divide these two values"),
        }
    }
}


impl Sub for Object{
    type Output = Object;

    fn sub(self, rhs: Object) -> Object {
        match self {
            Object::Number(value) => match rhs {
                Object::Number(rhs_value) => Object::Number(value - rhs_value),
                _ => panic!("Can't subtract these two values"),
            },
            _ => panic!("Can't subtract these two values"),
        }
    }
}
impl Interpreter {
    pub fn evaluate(&mut self, expr: &Box<Expr>) -> Result<Object, LoxError> {
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

    pub fn expression(&mut self) -> Result<Box<Expr>, LoxError> {
        self.equality()
    }

    pub fn parse(&mut self) -> Result<Box<Expr>, LoxError> {
        self.expression()
    }

    ///
    ///equality → comparison ( ( "!=" | "==" ) comparison )* ;
    ///
    pub fn equality(&mut self) -> Result<Box<Expr>, LoxError> {
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
            return false;
        }
        &self.peek().of_type == of_type
    }

    pub fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1
        }

        self.previous()
    }

    //Last consumed token.
    pub fn previous(&self) -> Token {
        self.tokens[self.current -1].clone()
    }
    pub fn comparison(&mut self) -> Result<Box<Expr>, LoxError> {
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
    pub fn unary(&mut self) -> Result<Box<Expr>, LoxError> {
        if self.if_match(&vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().clone();
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

    pub fn primary(&mut self) -> Result<Box<Expr>, LoxError> {
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
            Err(LoxError::RuntimeError(String::from("A"))) //Err(ParseError::SyntaxError(self.peek(), String::from("Expected expression")))
        }
    }

    pub fn consume(&mut self, token_type: TokenType, message: String) -> Result<Token, LoxError> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            Err(
                LoxError::RuntimeError(String::from("Error")), //ParseError::SyntaxError(self.peek().clone(), message)
            )
        }
    }

    pub fn multiplication(&mut self) -> Result<Box<Expr>, LoxError> {
        let mut expr = self.unary()?;

        while self.if_match(&vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right: right,
            });
        }

        Ok(expr)
    }

    pub fn synchronize(&mut self) {
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

    pub fn addition(&mut self) -> Result<Box<Expr>, LoxError> {
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

pub trait Visitor {
    type Value;

    fn visit_binary_expression(
        &mut self,
        expr: &Expr,
        left: &Box<Expr>,
        operator: &Token,
        right: &Box<Expr>,
    ) -> Result<Self::Value, LoxError>;

    fn visit_group_expression(
        &mut self,
        expr: &Expr,
        content: &Box<Expr>,
    ) -> Result<Self::Value, LoxError>;

    fn visit_literal_expression(
        &mut self,
        expr: &Expr,
        literal: &Literal,
    ) -> Result<Self::Value, LoxError>;

    fn visit_unary_expression(
        &mut self,
        expr: &Expr,
        operator: &Token,
        right: &Box<Expr>,
    ) -> Result<Self::Value, LoxError>;
}

pub trait Visitable {
    fn accept(&self, visitor: &mut Visitor<Value = Object>) -> Result<Object, LoxError>;
}


impl Visitable for Expr {
    fn accept(&self, expr: &mut Visitor<Value=Object>) -> Result<Object, LoxError> {
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
        let input = vec![("3-1", Object::Number(2))];

        for (expression, expected_result) in input {
            let mut scanner = Scanner::new(expression.to_string());
            let tokens = scanner.scan_tokens();

            let mut parser = Parser::new(tokens.to_vec());

            let expr = parser.parse().unwrap();

            let mut interpreter = Interpreter {};
            let result = interpreter.evaluate(&expr).unwrap();

            assert_eq!(result, expected_result);
        }
    }
}
