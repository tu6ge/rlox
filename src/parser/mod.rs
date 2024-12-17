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

use ast::Ast;

use crate::lexer::{Lexer, LiteralTypes, Token, TokenType};

mod ast;

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

    pub fn expression(&mut self) -> Result<Ast, Error> {
        self.equality()
    }

    /// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Result<Ast, Error> {
        let mut expr = self.comparison()?;

        while self.is_match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous();
            let right = self.comparison()?;
            expr = Ast::Comparison {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Ast, Error> {
        let mut expr = self.term()?;
        while self.is_match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op = self.previous();
            let right = self.term()?;
            expr = Ast::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Ast, Error> {
        let mut expr = self.factor()?;
        while self.is_match(&[TokenType::Plus, TokenType::Minus]) {
            let op = self.previous();
            let right = self.factor()?;
            expr = Ast::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Ast, Error> {
        let mut expr = self.unary()?;
        while self.is_match(&[TokenType::Slash, TokenType::Star]) {
            let op = self.previous();
            let right = self.unary()?;
            expr = Ast::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }
    fn unary(&mut self) -> Result<Ast, Error> {
        if self.is_match(&[TokenType::Bang, TokenType::Minus]) {
            let op = self.previous();
            let right = self.unary()?;
            return Ok(Ast::Unary(op, Box::new(right)));
        }

        self.primary()
    }
    fn primary(&mut self) -> Result<Ast, Error> {
        if self.is_match(&[TokenType::True]) {
            return Ok(Ast::Bool(true));
        }
        if self.is_match(&[TokenType::False]) {
            return Ok(Ast::Bool(false));
        }
        if self.is_match(&[TokenType::Nil]) {
            return Ok(Ast::Nil);
        }
        if self.is_match(&[TokenType::Number]) {
            if let LiteralTypes::Number(f) = self.previous().literal {
                return Ok(Ast::Number(f));
            }
        }
        if self.is_match(&[TokenType::String]) {
            if let LiteralTypes::String(s) = self.previous().literal {
                return Ok(Ast::String(s));
            }
        }

        if self.is_match(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(&TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Ast::Group(Box::new(expr)));
        }

        Err(self.error("line: unexpect token type"))
    }

    fn consume(&mut self, expect: &TokenType, message: &str) -> Result<(), Error> {
        if self.check(expect) {
            self.advance();
            return Ok(());
        }

        Err(self.error(message))
    }
    fn error(&self, message: &str) -> Error {
        let line = self.peek().line;

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
            Ast::Binary {
                left: Box::new(Ast::Binary {
                    left: Box::new(Ast::Number(1.0)),
                    op: normal_token(TokenType::Plus, "+"),
                    right: Box::new(Ast::Binary {
                        left: Box::new(Ast::Number(2.0)),
                        op: normal_token(TokenType::Star, "*"),
                        right: Box::new(Ast::Number(3.0)),
                    })
                }),
                op: normal_token(TokenType::Minus, "-"),
                right: Box::new(Ast::Number(4.0))
            }
        )
    }
}
