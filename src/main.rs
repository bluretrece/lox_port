pub mod environment;
pub mod expression;
pub mod interpreter;
pub mod literal;
pub mod lox_error;
pub mod object;
pub mod parser;
pub mod scanner;
pub mod statement;
pub mod token;
pub mod token_type;
use std::env;
use crate::parser::*;
use scanner::*;
use std::{
    fs,
    io::{self, Write},
};
use interpreter::*;
///
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

#[allow(dead_code)]
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
    let mut scanner = Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens.to_vec());
    dbg!(&parser);
    let expression = parser.parse_expression().unwrap();
    let mut interpreter = Interpreter::new();
    let result = interpreter.evaluate(&expression).unwrap();

    println!("{}", result);
    Ok(())
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    match args.len() {
        2 => todo!(), // FIXME Implement file handling.
        1 => {
            let _ = run_prompt();
        },
        _ => unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::*;
    use crate::object::*;
    use crate::token::*;
    use crate::token_type::*;
    use crate::literal::*;

    #[test]
    fn define_test() {
        let mut env = Environment::new();

        let definitions = vec![
            (
                Token::new(TokenType::IDENTIFIER, String::from("a"), None, 1),
                Object::Number(10),
            ),
            (
                Token::new(TokenType::IDENTIFIER, String::from("b"), None, 1),
                Object::Str(String::from("Hello world")),
            ),
            (
                Token::new(TokenType::IDENTIFIER, String::from("c"), None, 1),
                Object::Boolean(false),
            ),
        ];

        for (name, value) in definitions {
            env.define(&name.lexeme, &value);
            assert_eq!(env.get(name), value);
        }
    }

    #[test]
    fn scanner_test() {
        let input = "(+);".to_string();

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
        let input = "/".to_string();
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
        let input = "1+2".to_string();
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
    fn evaluation_test() {
        let input = vec![("1+2*3", Object::Number(7))];

        for (expression, expected_result) in input {
            let mut scanner = Scanner::new(expression.to_string());
            let tokens = scanner.scan_tokens();

            let mut parser = Parser::new(tokens.to_vec());

            let expr = parser.parse_expression().unwrap();

            let mut interpreter = Interpreter::new();
            let result = interpreter.evaluate(&expr).unwrap();

            assert_eq!(result, expected_result);
        }
    }

    #[test]
    fn expected_keywords_test() {
        let mut scanner = Scanner::new("and for if while".to_string());

        let tokens = scanner.scan_tokens();
        let token_types: Vec<TokenType> = tokens.into_iter().map(|t| t.of_type).collect();

        assert_eq!(
            token_types,
            vec![
                TokenType::AND,
                TokenType::FOR,
                TokenType::IF,
                TokenType::WHILE,
                TokenType::EOF
            ]
        );
    }
}
