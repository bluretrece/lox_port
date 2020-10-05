use crate::token::*;
use crate::literal::*;
use crate::lox_error::*;
use crate::expression::*;
use crate::statement::*;
use crate::token_type::*;


#[derive(PartialEq, Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            current: 0,
        }
    }
    pub fn assignment(&mut self) -> Result<Box<Expr>, LoxError> {
        let mut expr = self.equality()?;

        if self.if_match(&[TokenType::EQUAL]) {
            let mut equals = self.previous();
            let mut value = self.assignment()?;

            match *expr {
                Expr::Variable { name } => {
                    return Ok(
                        Box::new(Expr::Assign {
                            name: name,
                            value: value,
                        })
                    );
                },
                _ => return Err(LoxError::RuntimeError(String::from("Invalid assignment target")))
            }
        }

        Ok(expr)

    }
    pub fn expression(&mut self) -> Result<Box<Expr>, LoxError> {
        self.assignment()
        //self.equality()
    }

    pub fn parse_statement(&mut self) -> Result<Vec<Box<Statement>>, LoxError>{
        let mut statements= Vec::new();

        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        return Ok(statements)
    }

    pub fn declaration(&mut self) -> Result<Box<Statement>, LoxError> {
        if self.if_match(&[TokenType::VAR]) {
            return self.var_declaration()
        } else { return self.statement() }
    }

    pub fn var_declaration(&mut self) -> Result<Box<Statement>, LoxError> {
        let name = self.consume(TokenType::IDENTIFIER, String::from("
                Expect variable name.
                "))?;

        let mut initializer = None;

        if self.if_match(&[TokenType::EQUAL]) {
            initializer = Some(self.expression()?);
        }

        self.consume(TokenType::SEMICOLON, String::from("Expect ';' after variable declaration"));

        return Ok(Box::new(Statement::Variable {
            name: name,
            initializer: initializer
        }))
    }

    pub fn print_statement(&mut self) -> Result<Box<Statement>, LoxError> {
        let mut value = self.expression()?;

        self.consume(TokenType::SEMICOLON, String::from("Expect ';' after value."));

        return Ok(Box::new(Statement::Print{expression:value}))
    }

    pub fn statement(&mut self) -> Result<Box<Statement>, LoxError> {
        if self.if_match(&[TokenType::PRINT]) {
            return self.print_statement()

        } else if self.if_match(&[TokenType::LEFT_BRACE]){
            return Ok(Box::new(
                    Statement::Block{
                        statements: self.block()
                    }
            ))
        }
            else {
            return self.expression_statement()
        }
    }

    pub fn block(&mut self) -> Vec<Box<Statement>> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RIGHT_BRACE) && !self.is_at_end(){
            statements.push(self.declaration().unwrap());
        }
        
        self.consume(TokenType::RIGHT_BRACE, String::from("Expect '}' after block."));

        return statements
    } 

    pub fn expression_statement(&mut self) -> Result<Box<Statement>, LoxError> {
       let mut expr = self.expression()?;

       self.consume(TokenType::SEMICOLON, String::from("Expect ';' after expression"));

       return Ok(Box::new(Statement::Expression{expression:expr}))
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
        Ok(expr) } pub fn if_match(&mut self, token_types: &[TokenType]) -> bool {
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
        
        if self.if_match(&vec![TokenType::IDENTIFIER]) {
            return Ok(
                Box::new(
                    Expr::Variable {
                        name: self.previous().clone()
                    }
                )
            )
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

