pub mod interpreter;
pub mod parser;
pub mod literal;
pub mod scanner;
pub mod token;
pub mod token_type;
pub mod lox_error;
pub mod expression;
pub mod statement;
pub mod object;
pub mod environment;


use literal::*;
use scanner::*;
use std::{
    fs,
    io::{self, Write},
};
use std::fmt;
use token::*;
use token_type::*;
use parser::*;
use crate::parser::*;

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
    use crate::object::*;
    use crate::interpreter::*;
    use crate::scanner;
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
    #[test]
    fn evaluation_test_2() {
        let input = vec![("6+6*2", Object::Number(18))];

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
    #[test]
    fn boolean_evaluation_test_() {
        let input = vec![("1==1", Object::Boolean(true))];

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
    #[test]
    fn boolean_evaluation_test_2() {
        let input = vec![("3==2", Object::Boolean(false))];

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
