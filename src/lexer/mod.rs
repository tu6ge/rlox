pub use token::{LiteralTypes, Token, TokenType};

mod token;

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    //For each entity, it calls scan token function and return final vector of tokens
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            LiteralTypes::Nil,
            self.line,
        ));

        self.tokens.clone()
    }

    //Contains all the tokens we need to recognize
    fn scan_token(&mut self) {
        let c: u8 = self.advance();
        match c {
            b'(' => self.add_token(TokenType::LeftParen, LiteralTypes::Nil),
            b')' => self.add_token(TokenType::RightParen, LiteralTypes::Nil),
            b'{' => self.add_token(TokenType::LeftBrace, LiteralTypes::Nil),
            b'}' => self.add_token(TokenType::RightBrace, LiteralTypes::Nil),
            b',' => self.add_token(TokenType::Comma, LiteralTypes::Nil),
            b'.' => self.add_token(TokenType::Dot, LiteralTypes::Nil),
            b'-' => self.add_token(TokenType::Minus, LiteralTypes::Nil),
            b'+' => self.add_token(TokenType::Plus, LiteralTypes::Nil),
            b';' => self.add_token(TokenType::Semicolon, LiteralTypes::Nil),
            b'*' => self.add_token(TokenType::Star, LiteralTypes::Nil),

            b'!' => {
                let is_equal = self.is_next_expected(b'=');
                self.add_token(
                    if is_equal {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    },
                    LiteralTypes::Nil,
                );
            }
            b'=' => {
                let is_equal = self.is_next_expected(b'=');
                self.add_token(
                    if is_equal {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    },
                    LiteralTypes::Nil,
                );
            }
            b'<' => {
                let is_equal = self.is_next_expected(b'=');
                self.add_token(
                    if is_equal {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    },
                    LiteralTypes::Nil,
                );
            }
            b'>' => {
                let is_equal = self.is_next_expected(b'=');
                self.add_token(
                    if is_equal {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    },
                    LiteralTypes::Nil,
                );
            }
            b'/' => {
                let slash = self.is_next_expected(b'/');
                if slash {
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, LiteralTypes::Nil)
                }
            }

            b'\r' | b' ' | b'\t' => {}
            b'\n' => self.line += 1,
            b'"' => self.string(),

            _ => {
                if c.is_ascii_digit() {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    self.add_token(TokenType::Error, LiteralTypes::Nil);
                }
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> u8 {
        let c = self.source.as_bytes()[self.current];
        self.current += 1;
        c
    }

    fn add_token(&mut self, ttype: TokenType, literal: LiteralTypes) {
        let lexeme = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(ttype, lexeme, literal, self.line))
    }

    fn is_next_expected(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        };

        if self.source.as_bytes()[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            return b'\0';
        }
        return self.source.as_bytes()[self.current];
    }

    fn peek_next(&self) -> u8 {
        if self.is_at_end() {
            return b'\0';
        }
        return self.source.as_bytes()[self.current + 1];
    }

    fn string(&mut self) {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.current += 1;
        }

        if self.is_next_expected(b'"') {
            let value: String = self.source[self.start + 1..self.current - 1].to_string();
            self.add_token(TokenType::String, LiteralTypes::String(value));
        } else {
            self.add_token(TokenType::Error, LiteralTypes::Nil);
        }
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.current += 1;
        }

        if self.peek() == b'.' && self.peek_next().is_ascii_digit() {
            self.current += 1;

            while self.peek().is_ascii_digit() {
                self.current += 1;
            }
        }

        let value: f64 = self.source[self.start..self.current].parse().unwrap();
        self.add_token(TokenType::Number, LiteralTypes::Number(value))
    }

    // Check for either identifier or keywords
    fn identifier(&mut self) {
        while self.is_alpha(self.peek()) || self.peek().is_ascii_digit() {
            self.current += 1;
        }

        let text = &self.source[self.start..self.current];
        let ttype = self.get_keyword(text);

        match ttype {
            Some(t) => match &t {
                TokenType::True => self.add_token(t, LiteralTypes::Bool(true)),
                TokenType::False => self.add_token(t, LiteralTypes::Bool(false)),
                _ => self.add_token(t, LiteralTypes::Nil),
            },
            None => self.add_token(
                TokenType::Identifier,
                LiteralTypes::String(text.to_string()),
            ),
        }
    }

    fn is_alpha(&self, c: u8) -> bool {
        c.is_ascii_alphabetic() || c == b'_'
    }

    fn get_keyword(&self, word: &str) -> Option<TokenType> {
        match word {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normal_token(ttype: TokenType, lexeme: &str) -> Token {
        Token::new(ttype, lexeme.to_string(), LiteralTypes::Nil, 1)
    }
    fn number_token(lexeme: &str, number: f64) -> Token {
        Token::new(
            TokenType::Number,
            lexeme.to_string(),
            LiteralTypes::Number(number),
            1,
        )
    }
    fn string_token(lexeme: &str, value: &str) -> Token {
        Token::new(
            TokenType::String,
            lexeme.to_string(),
            LiteralTypes::String(value.to_string()),
            1,
        )
    }
    fn iden_token(lexeme: &str) -> Token {
        Token::new(
            TokenType::Identifier,
            lexeme.to_string(),
            LiteralTypes::String(lexeme.to_string()),
            1,
        )
    }

    #[test]
    fn test_expression() {
        let tokens = Lexer::new("1 + a - 2.5").scan_tokens();
        assert_eq!(
            tokens,
            vec![
                number_token("1", 1.0),
                normal_token(TokenType::Plus, "+"),
                iden_token("a"),
                normal_token(TokenType::Minus, "-"),
                number_token("2.5", 2.5),
                normal_token(TokenType::Eof, "")
            ]
        )
    }

    #[test]
    fn test_assign() {
        let tokens = Lexer::new("var foo = \"bar\"").scan_tokens();
        assert_eq!(
            tokens,
            vec![
                normal_token(TokenType::Var, "var"),
                iden_token("foo"),
                normal_token(TokenType::Equal, "="),
                string_token("\"bar\"", "bar"),
                normal_token(TokenType::Eof, "")
            ]
        )
    }

    #[test]
    fn test_error() {
        let tokens = Lexer::new("@#").scan_tokens();
        assert_eq!(
            tokens,
            vec![
                normal_token(TokenType::Error, "@"),
                normal_token(TokenType::Error, "#"),
                normal_token(TokenType::Eof, "")
            ]
        )
    }

    #[test]
    fn test_string_error() {
        let tokens = Lexer::new("\"foo").scan_tokens();

        assert_eq!(
            tokens,
            vec![
                normal_token(TokenType::Error, "\"foo"),
                normal_token(TokenType::Eof, "")
            ]
        )
    }
}
