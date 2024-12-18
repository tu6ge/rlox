#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: LiteralTypes,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralTypes {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
    //Callable(Callable),
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: LiteralTypes, line: usize) -> Self {
        Token {
            ttype,
            lexeme,
            literal,
            line,
        }
    }

    pub fn show(&self) -> String {
        format!(
            "line:{} ttype:{:?} lexeme:{} literal:{:?}",
            self.line, self.ttype, self.lexeme, self.literal
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,  // (
    RightParen, // )
    LeftBrace,  // {
    RightBrace, // }
    Comma,      // ,
    Dot,        // .
    Minus,      // -
    Plus,       // +
    Semicolon,  // ;
    Slash,      // /
    Star,       // *

    // One or two character tokens.
    Bang,         // !
    BangEqual,    // !=
    Equal,        // =
    EqualEqual,   // ==
    Greater,      // >
    GreaterEqual, // >=
    Less,         // <
    LessEqual,    // <=

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    Eof,
}

impl LiteralTypes {
    pub fn equal(&self, rhs: &LiteralTypes) -> bool {
        match (self, rhs) {
            (LiteralTypes::Number(left), LiteralTypes::Number(right)) => left == right,
            (LiteralTypes::String(left), LiteralTypes::String(right)) => left == right,
            (LiteralTypes::Bool(left), LiteralTypes::Bool(right)) => left == right,
            (LiteralTypes::Nil, LiteralTypes::Nil) => true,
            (LiteralTypes::Nil, LiteralTypes::Bool(b)) if *b == false => true,
            (LiteralTypes::Bool(b), LiteralTypes::Nil) if *b == false => true,
            _ => false,
        }
    }
}
