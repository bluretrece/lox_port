use crate::expression::*;
use crate::literal::*;
use crate::lox_error::*;
use crate::statement::*;
use crate::token::*;
use crate::token_type::*;

pub type Result<T> = std::result::Result<T, LoxError>;


#[derive(PartialEq, Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    depth: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            depth: 0,
        }
    }

    pub fn or(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.and()?;

        while self.if_match(&[TokenType::OR]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Box::new(Expr::Logical {
                left: expr.clone(),
                operator,
                right,
            });
        }

        Ok(expr)
    }

    pub fn parse_expression(&mut self) -> Result<Box<Expr>> {
        self.expression()
    }

    pub fn and(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.equality()?;

        while self.if_match(&[TokenType::AND]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Box::new(Expr::Logical {
                left: expr.clone(),
                operator,
                right,
            });
        }

        Ok(expr)
    }

    pub fn assignment(&mut self) -> Result<Box<Expr>> {
        let expr = self.or()?;

        if self.if_match(&[TokenType::EQUAL]) {
            let _equals = self.previous();
            let value = self.assignment()?;

            match *expr {
                Expr::Variable { name } => {
                    return Ok(Box::new(Expr::Assign { name, value }));
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
    pub fn expression(&mut self) -> Result<Box<Expr>> {
        self.assignment()
    }

    pub fn parse_statement(&mut self) -> Result<Vec<Box<Statement>>> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        return Ok(statements);
    }

    pub fn _declaration(&mut self) -> Result<Box<Statement>> {
        if self.if_match(&[TokenType::VAR]) {
            self.var_declaration()
        } else if self.if_match(&[TokenType::FUN]) {
            self.function("function")
        } else {
            self.statement()
        }
    }

    pub fn function(&mut self, _kind: &str) -> Result<Box<Statement>> {
        let name = self.consume(
            TokenType::IDENTIFIER,
            String::from(
                "
                Expect function name.
                ",
            ),
        )?;

        self.consume(
            TokenType::LEFT_PAREN,
            String::from(
                "
            Expect '( after function name
                ",
            ),
        )?;

        let mut parameters = Vec::new();
        if !self.check(&TokenType::RIGHT_PAREN) {
            loop {
                if parameters.len() >= 255 {
                    return Err(LoxError::RuntimeError(String::from(
                        "Parameter size overpassed.",
                    )));
                }

                parameters.push(
                    self.consume(TokenType::IDENTIFIER, String::from("Expect parameter name"))?,
                );

                if !self.if_match(&[TokenType::COMMA]) {
                    break;
                }
            }
        }
        self.consume(
            TokenType::RIGHT_PAREN,
            String::from("Expect ')' after parameters"),
        )?;
        self.consume(
            TokenType::LEFT_BRACE,
            String::from("Expect '{' before function body"),
        )?;
        let body = self.block()?;

        return Ok(Box::new(Statement::Function {
            name,
            params: parameters,
            body,
        }));
    }

    fn declaration(&mut self) -> Result<Box<Statement>> {
        match self._declaration() {
            Ok(r) => Ok(r),
            Err(e) => {
                self.synchronize();
                Err(e)
            }
        }
    }

    pub fn var_declaration(&mut self) -> Result<Box<Statement>> {
        let name = self
            .consume(
                TokenType::IDENTIFIER,
                String::from(
                    "
                Expect variable name.
                ",
                ),
            )?
            .clone();

        let mut initializer: Option<Box<Expr>> = None;

        if self.if_match(&[TokenType::EQUAL]) {
            initializer = Some(self.expression()?);
        }

        self.consume(
            TokenType::SEMICOLON,
            String::from("Expect ';' after variable declaration"),
        )?;

        Ok(Box::new(Statement::Variable { name, initializer }))
    }

    pub fn print_statement(&mut self) -> Result<Box<Statement>> {
        let value = self.expression()?;

        self.consume(
            TokenType::SEMICOLON,
            String::from("Expect ';' after value."),
        )?;

        return Ok(Box::new(Statement::Print { expression: value }));
    }

    pub fn while_statement(&mut self) -> Result<Box<Statement>> {
        self.consume(
            TokenType::LEFT_PAREN,
            String::from("Expect ( after while statement"),
        )?;

        let condition = self.expression()?;

        self.consume(
            TokenType::RIGHT_PAREN,
            String::from("Expect ) after condition"),
        )?;

        let body = self.statement()?;

        return Ok(Box::new(Statement::While { condition, body }));
    }
    pub fn for_statement(&mut self) -> Result<Box<Statement>> {
        self.consume(TokenType::LEFT_PAREN, String::from("Expect ( for ."))?;

        let mut initializer = None;

        if self.if_match(&[TokenType::SEMICOLON]) {
        } else if self.if_match(&[TokenType::VAR]) {
            initializer = Some(self.var_declaration()?); // CHECK
        } else {
            initializer = Some(self.expression_statement()?); // CHECK
        }

        let mut condition = None;
        if !self.check(&TokenType::SEMICOLON) {
            condition = Some(self.expression()?);
        }

        self.consume(
            TokenType::SEMICOLON,
            String::from("Expect ; after loop condition"),
        )?;

        let mut increment = None;
        if !self.check(&TokenType::RIGHT_PAREN) {
            increment = Some(self.expression()?)
        }

        self.consume(
            TokenType::RIGHT_PAREN,
            String::from("Expect ) after for clauses."),
        )?;

        let mut body = self.statement()?;

        if let Some(increment) = increment {
            body = Box::new(Statement::Block {
                statements: vec![
                    body,
                    Box::new(Statement::Expression {
                        expression: increment,
                    }),
                ],
            })
        }

        if let Some(condition) = condition {
            body = Box::new(Statement::While { condition, body });
        } else {
            body = Box::new(Statement::While {
                condition: Box::new(Expr::Literal {
                    literal: Literal::Boolean(true),
                }),
                body,
            });
        }

        if let Some(initializer) = initializer {
            body = Box::new(Statement::Block {
                statements: vec![initializer, body],
            });
        }
        Ok(body)
    }

    pub fn return_statement(&mut self) -> Result<Box<Statement>> {
        let keyword = self.previous();
        let mut value = None;

        if !self.check(&TokenType::SEMICOLON) {
            value = Some(self.expression()?);
        }

        self.consume(
            TokenType::SEMICOLON,
            String::from("Expect ';' after return value."),
        )?;

        Ok(Box::new(Statement::Return { keyword, value }))
    }

    pub fn statement(&mut self) -> Result<Box<Statement>> {
        if self.if_match(&[TokenType::IF]) {
            return self.if_statement();
        } else if self.if_match(&[TokenType::RETURN]) {
            return self.return_statement();
        } else if self.if_match(&[TokenType::FOR]) {
            self.depth += 1;
            let res = self.for_statement();
            self.depth -= 1;
            res
        } else if self.if_match(&[TokenType::WHILE]) {
            self.depth += 1;
            let res = self.while_statement();
            self.depth -= 1;
            res
        } else if self.if_match(&[TokenType::PRINT]) {
            return self.print_statement();
        } else if self.if_match(&[TokenType::LEFT_BRACE]) {
            return Ok(Box::new(Statement::Block {
                statements: self.block()?,
            }));
        } else {
            self.expression_statement()
        }
    }

    pub fn if_statement(&mut self) -> Result<Box<Statement>> {
        self.consume(
            TokenType::LEFT_PAREN,
            String::from("Expect '(' after 'if'."),
        )?;

        let condition = self.expression()?;

        self.consume(
            TokenType::RIGHT_PAREN,
            String::from("Expect ')' after 'if' condition."),
        )?;

        let then_branch = self.statement()?;

        let mut else_branch = None;
        if self.if_match(&[TokenType::ELSE]) {
            else_branch = Some(self.statement()?)
        }

        return Ok(Box::new(Statement::If {
            condition,
            then_branch,
            else_branch,
        }));
    }

    pub fn block(&mut self) -> Result<Vec<Box<Statement>>> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(
            TokenType::RIGHT_BRACE,
            String::from("Expect '}' after block."),
        )?;

        Ok(statements)
    }

    pub fn expression_statement(&mut self) -> Result<Box<Statement>> {
        let expr = self.expression()?;

        self.consume(
            TokenType::SEMICOLON,
            String::from("Expect ';' after expression"),
        )?;

        return Ok(Box::new(Statement::Expression { expression: expr }));
    }

    pub fn parse(&mut self) -> Result<Vec<Box<Statement>>> {
        let mut statements: Vec<Box<Statement>> = vec![];

        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        return Ok(statements);
    }

    pub fn equality(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.comparison()?;

        while self.if_match(&vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;

            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            });
        }
        Ok(expr)
    }

    // Returns true if the token stream passed are correct.
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

    // Checks if the current token is the same as the next one, by taking a reference of it.
    pub fn check(&self, of_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        &self.peek().of_type == of_type
    }

    // While is not EOF, advances the cursor.
    pub fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    // Returns the last consumed token.
    pub fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    // Handles a comparison expression.
    pub fn comparison(&mut self) -> Result<Box<Expr>> {
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

    /// If the current token is rather
    /// a band or minus sign, then we are in front
    /// of an unary expresion.
    ///
    /// the operator is consumed in the first call to unary()
    /// then we grab the token and recursively call unary() to
    /// parse the operand. Finally, wrap that all up in an unary
    /// expression syntax tree.
    ///
    /// Otherwise we parse a function.
    pub fn unary(&mut self) -> Result<Box<Expr>> {
        if self.if_match(&vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            return Ok(Box::new(Expr::Unary { operator, right }));
        }
        self.call()
    }

    // Parses a function expression.
    pub fn call(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.primary();

        loop {
            if self.if_match(&vec![TokenType::LEFT_PAREN]) {
                expr = self.finish_call(expr?);
            } else {
                break;
            }
        }

        expr
    }

    // One can see the function's argument as a list of tokens.
    // This function wraps each of the arguments passed to the function.
    pub fn finish_call(&mut self, callee: Box<Expr>) -> Result<Box<Expr>> {
        let mut arguments = vec![];
        if !self.check(&TokenType::RIGHT_PAREN) {
            loop {
                if arguments.len() >= 255 {
                    // just report error, don't bail.
                    unimplemented!()
                }
                arguments.push(self.expression()?);
                if !self.if_match(&[TokenType::COMMA]) {
                    break;
                }
            }
        }
        let paren = self
            .consume(TokenType::RIGHT_PAREN, "Expect \")\" after arguments.".to_string())?
            .clone();
        Ok(Box::new(Expr::Call {
            callee,
            paren,
            arguments,
        }))
    }

    // Parses a primary expression
    pub fn primary(&mut self) -> Result<Box<Expr>> {
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
            if let Some(l) = self.previous().clone().literal {
                return Ok(Box::new(Expr::Literal { literal: l }));
            }
        }

        if self.if_match(&vec![TokenType::IDENTIFIER]) {
            return Ok(Box::new(Expr::Variable {
                name: self.previous().clone(),
            }));
        }

        if self.if_match(&vec![TokenType::LEFT_PAREN]) {
            let expr = self.expression()?;

            self.consume(TokenType::RIGHT_PAREN, String::from("Expected ')'"))?;
            return Ok(Box::new(Expr::Grouping { expression: expr }));
        }
        Err(LoxError::RuntimeError(String::from("Expected expression.")))
    }

    // Consumes the current token and advances the cursor's pointer to the next one.
    pub fn consume(&mut self, token_type: TokenType, message: String) -> Result<Token> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            Err(LoxError::RuntimeError(message))
        }
    }

    // Handles an multiplication expression.
    pub fn multiplication(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.unary()?;

        while self.if_match(&vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
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

            match self.peek().of_type {
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => {
                    return;
                }
                _ => (),
            }
            self.advance();
        }
    }

    // Handles an addition expression.
    pub fn addition(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.multiplication()?;

        while self.if_match(&vec![TokenType::PLUS, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = self.multiplication()?;

            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            });
        }
        Ok(expr)
    }
}
