//！```dnf
//！expression     → equality ;
//！equality       → comparison ( ( "!=" | "==" ) comparison )* ;
//！comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
//！term           → factor ( ( "-" | "+" ) factor )* ;
//！factor         → unary ( ( "/" | "*" ) unary )* ;
//！unary          → ( "!" | "-" ) unary
//！               | primary ;
//！primary        → NUMBER | STRING | "true" | "false" | "nil"
//！               | "(" expression ")" ;
//! ```

use ast::{Assign, Binary, Comparison, Expr, Variable};
use stmt::{Block, Expression, Print, Stmt};

use crate::lexer::{Lexer, LiteralTypes, Token, TokenType};
use ast::Visitor;
mod ast;
mod env;
mod inter;
mod stmt;

macro_rules! error_message {
    ($literal:literal) => {
        return Err(Error::String($literal.to_string()))
    };
    ($error:literal, $($y:expr),+ $(,)?) => {
        return Err(Error::String(format!($error, $($y),+)))
    };
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(source: &str) -> Self {
        let tokens = Lexer::new(source).scan_tokens();
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, Error> {
        if self.is_match(&[TokenType::Var]) {
            return self.var_declaration();
        }
        self.statement()
    }
    fn var_declaration(&mut self) -> Result<Stmt, Error> {
        self.consume(&TokenType::Identifier, "Expect variable name.")?;
        let name = self.previous();

        let mut initializer = Expr::Literal(LiteralTypes::Nil);
        if self.is_match(&[TokenType::Equal]) {
            initializer = self.expression()?;
        }

        self.consume(
            &TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Stmt::Var(stmt::Var { name, initializer }))
    }

    fn statement(&mut self) -> Result<Stmt, Error> {
        if self.is_match(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.is_match(&[TokenType::LeftBrace]) {
            return Ok(Stmt::Block(self.block()?));
        }

        self.expression_statement()
    }

    fn block(&mut self) -> Result<Block, Error> {
        let mut statements = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?)
        }

        self.consume(&TokenType::RightBrace, "Expect '}' after block.")?;

        Ok(Block { statements })
    }

    fn print_statement(&mut self) -> Result<Stmt, Error> {
        let expression = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expect ';' after value.")?;

        Ok(Stmt::Print(Print { expression }))
    }
    fn expression_statement(&mut self) -> Result<Stmt, Error> {
        let expression = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expect ';' after value.")?;

        Ok(Stmt::Expression(Expression { expression }))
    }

    pub fn expression(&mut self) -> Result<Expr, Error> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, Error> {
        let expr = self.equality()?;

        if self.is_match(&[TokenType::Equal]) {
            let equal = self.previous();
            let value = self.assignment()?;

            if let Expr::Variable(Variable { identifier, .. }) = expr {
                return Ok(Expr::Assign(Assign {
                    name: identifier,
                    value: Box::new(value),
                }));
            }

            return Err(self.token_error(&equal, "Invalid assignment target."));
        }

        Ok(expr)
    }

    /// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr = self.comparison()?;

        while self.is_match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous();
            let right = self.comparison()?;
            expr = Expr::Comparison(Comparison {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    /// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Result<Expr, Error> {
        let mut expr = self.term()?;
        while self.is_match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op = self.previous();
            let right = self.term()?;
            expr = Expr::Comparison(Comparison {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    /// term           → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Result<Expr, Error> {
        let mut expr = self.factor()?;
        while self.is_match(&[TokenType::Plus, TokenType::Minus]) {
            let op = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    /// factor         → unary ( ( "/" | "*" ) unary )*
    fn factor(&mut self) -> Result<Expr, Error> {
        let mut expr = self.unary()?;
        while self.is_match(&[TokenType::Slash, TokenType::Star]) {
            let op = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    /// unary → ( "!" | "-" ) unary | primary
    fn unary(&mut self) -> Result<Expr, Error> {
        use ast::Unary;
        if self.is_match(&[TokenType::Bang, TokenType::Minus]) {
            let op = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(Unary {
                op,
                right: Box::new(right),
            }));
        }

        self.primary()
    }
    fn primary(&mut self) -> Result<Expr, Error> {
        if self.is_match(&[
            TokenType::True,
            TokenType::False,
            TokenType::Nil,
            TokenType::Number,
            TokenType::String,
        ]) {
            return Ok(Expr::Literal(self.previous().literal));
        }

        if self.is_match(&[TokenType::Identifier]) {
            return Ok(Expr::Variable(Variable {
                identifier: self.previous(),
            }));
        }

        if self.is_match(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(&TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(ast::Grouping {
                expr: Box::new(expr),
            }));
        }

        Err(self.error("expect a expression"))
    }

    fn consume(&mut self, expect: &TokenType, message: &str) -> Result<(), Error> {
        if self.check(expect) {
            self.advance();
            return Ok(());
        }

        Err(self.error(message))
    }
    fn error(&self, message: &str) -> Error {
        self.token_error(self.peek(), message)
    }
    fn token_error(&self, token: &Token, message: &str) -> Error {
        let line = token.line;

        let msg = format!("line:{line} {message}");

        msg.into()
    }

    fn is_match(&mut self, tokens: &[TokenType]) -> bool {
        for t in tokens.iter() {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }
    fn check(&self, ttype: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().ttype == *ttype
    }
    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
    }
    fn is_at_end(&self) -> bool {
        self.peek().ttype == TokenType::Eof
    }
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}

#[derive(Debug, Clone)]
enum Error {
    String(String),
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}
impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normal_token(ttype: TokenType, lexeme: &str) -> Token {
        Token::new(ttype, lexeme.to_string(), LiteralTypes::Nil, 1)
    }

    #[test]
    fn test_normal() {
        let mut parser = Parser::new("1 + 2 * 3 - 4");
        let ast = parser.expression().unwrap();
        assert_eq!(
            ast,
            Expr::Binary(Binary {
                left: Box::new(Expr::Binary(Binary {
                    left: Box::new(Expr::Literal(LiteralTypes::Number(1.0))),
                    op: normal_token(TokenType::Plus, "+"),
                    right: Box::new(Expr::Binary(Binary {
                        left: Box::new(Expr::Literal(LiteralTypes::Number(2.0))),
                        op: normal_token(TokenType::Star, "*"),
                        right: Box::new(Expr::Literal(LiteralTypes::Number(3.0))),
                    }))
                })),
                op: normal_token(TokenType::Minus, "-"),
                right: Box::new(Expr::Literal(LiteralTypes::Number(4.0)))
            })
        )
    }
}
