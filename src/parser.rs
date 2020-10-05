use crate::expression::*;
use crate::literal::*;
use crate::lox_error::*;
use crate::object::*;
use crate::statement::*;
use crate::token::*;
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
    
    pub fn or(&mut self) -> Result<Box<Expr>, LoxError> {
        let mut expr = self.and()?;

        while self.if_match(&[TokenType::OR]) {
            let mut operator = self.previous();
            let mut right = self.and()?;
            let mut expr = Expr::Logical {
                left: expr.clone(),
                operator: operator,
                right: right,
            };
        }

        Ok(expr)
    }

    pub fn parse_expression(&mut self) -> Result<Box<Expr>, LoxError> {
        self.expression()
    }

    pub fn and(&mut self) -> Result<Box<Expr>, LoxError> {
        let mut expr = self.equality()?;

        while self.if_match(&[TokenType::AND]) {
            let mut operator = self.previous();
            let mut right = self.equality()?;
            let mut expr = Expr::Logical {
                left: expr.clone(),
                operator: operator,
                right: right,
            };
        }
        
        Ok(expr)
    }

    pub fn assignment(&mut self) -> Result<Box<Expr>, LoxError> {
        let mut expr = self.or()?;
        // let mut expr = self.equality()?;

        if self.if_match(&[TokenType::EQUAL]) {
            let mut equals = self.previous();
            let mut value = self.assignment()?;

            match *expr {
                Expr::Variable { name } => {
                    return Ok(Box::new(Expr::Assign {
                        name: name,
                        value: value,
                    }));
                }
                _ => {
                    return Err(LoxError::RuntimeError(String::from(
                        "Invalid assignment target",
                    )))
                }
            }
        }

        Ok(expr)
    }
    pub fn expression(&mut self) -> Result<Box<Expr>, LoxError> {
        self.assignment()
        //self.equality()
    }

    pub fn parse_statement(&mut self) -> Result<Vec<Box<Statement>>, LoxError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        return Ok(statements);
    }

    pub fn declaration(&mut self) -> Result<Box<Statement>, LoxError> {
        if self.if_match(&[TokenType::VAR]) {
            return self.var_declaration();
        } else {
            return self.statement();
        }
    }

    pub fn var_declaration(&mut self) -> Result<Box<Statement>, LoxError> {
        let name = self.consume(
            TokenType::IDENTIFIER,
            String::from(
                "
                Expect variable name.
                ",
            ),
        )?;

        let mut initializer = None;

        if self.if_match(&[TokenType::EQUAL]) {
            initializer = Some(self.expression()?);
        }

        self.consume(
            TokenType::SEMICOLON,
            String::from("Expect ';' after variable declaration"),
        );

        return Ok(Box::new(Statement::Variable {
            name: name,
            initializer: initializer,
        }));
    }

    pub fn print_statement(&mut self) -> Result<Box<Statement>, LoxError> {
        let mut value = self.expression()?;

        self.consume(
            TokenType::SEMICOLON,
            String::from("Expect ';' after value."),
        );

        return Ok(Box::new(Statement::Print { expression: value }));
    }

    pub fn while_statement(&mut self) -> Result<Box<Statement>, LoxError> {
        self.consume(TokenType::LEFT_PAREN, String::from("Expect ( after while statement"));

        let condition = self.expression()?;

        self.consume(TokenType::RIGHT_PAREN, String::from("Expect ) after while statement"));

        let body = self.statement()?;

        return Ok(Box::new(
                Statement::While {
                    condition: condition,
                    body: body
                }
        ))
    }
    pub fn for_statement(&mut self) -> Result<Box<Statement>, LoxError> {
        self.consume(TokenType::LEFT_PAREN, String::from("Expect ( for ."));

        let initializer = if self.if_match(&[TokenType::SEMICOLON]) {
            None
        } else if self.if_match(&[TokenType::VAR]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };
        
        let condition = if !self.if_match(&[TokenType::SEMICOLON]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::SEMICOLON, String::from("Expec ; after loop"));

        let increment = if self.if_match(&[TokenType::RIGHT_PAREN]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::RIGHT_PAREN, String::from("Expect ) after for clauses."));

        let mut body = self.statement()?;

        if let Some(increment) = increment {
            body = Box::new(Statement::Block {
                statements: vec![body, 
                Box::new(Statement::Expression 
                    {
                        expression: increment 
                    })
                ]
            })
        }

        if let Some(condition) = condition {
            body = Box::new(Statement::While {
                condition: condition,
                body: body,
            });
        } else {
            body = Box::new(Statement::While {
                condition: Box::new(Expr::Literal{
                    literal: Literal::Boolean(true),
                }),
                body: body,
            });
        }

        if let Some(initializer) = initializer {
            body = Box::new(
                Statement::Block {
                    statements: vec![initializer, body],
                }
            );
        }
        Ok(body)
    }
    pub fn statement(&mut self) -> Result<Box<Statement>, LoxError> {
        if self.if_match(&[TokenType::IF]) {
            return self.if_statement();
        }
        
        if self.if_match(&[TokenType::FOR]) {
            return self.for_statement();
        }

        if self.if_match(&[TokenType::WHILE]) {
            return self.while_statement();
        }
        if self.if_match(&[TokenType::PRINT]) {
            return self.print_statement();
        } else if self.if_match(&[TokenType::LEFT_BRACE]) {
            return Ok(Box::new(Statement::Block {
                statements: self.block(),
            }));
        } else {
            return self.expression_statement();
        }
    }
    pub fn if_statement(&mut self) -> Result<Box<Statement>, LoxError> {
        self.consume(
            TokenType::LEFT_PAREN,
            String::from("Expect '(' after 'if' keyword."),
        );

        let mut condition = self.expression()?;

        self.consume(
            TokenType::RIGHT_PAREN,
            String::from("Expect ')' after 'if' condition."),
        );

        let then_branch = self.statement()?;

        let mut else_branch = if self.if_match(&[TokenType::ELSE]) {
            Some(self.statement()?)
        } else { 
            None
        };

        return Ok(Box::new(Statement::If {
            condition,
            then_branch,
            else_branch,
        }));
    }
    pub fn block(&mut self) -> Vec<Box<Statement>> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.declaration().unwrap());
        }

        self.consume(
            TokenType::RIGHT_BRACE,
            String::from("Expect '}' after block."),
        );

        return statements;
    }

    pub fn expression_statement(&mut self) -> Result<Box<Statement>, LoxError> {
        let mut expr = self.expression()?;

        self.consume(
            TokenType::SEMICOLON,
            String::from("Expect ';' after expression"),
        );

        return Ok(Box::new(Statement::Expression { expression: expr }));
    }

    pub fn parse(&mut self) -> Result<Vec<Box<Statement>>, LoxError> {
        let mut statements: Vec<Box<Statement>> = vec![];

        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        return Ok(statements)
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
        self.tokens[self.current - 1].clone()
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
            return Ok(Box::new(Expr::Variable {
                name: self.previous().clone(),
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
