use crate::lexer::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum Ast {
    Number(f64),
    Bool(bool),
    Nil,
    String(String),
    Unary(Token, Box<Ast>),
    Binary {
        left: Box<Ast>,
        op: Token,
        right: Box<Ast>,
    },
    Comparison {
        left: Box<Ast>,
        op: Token,
        right: Box<Ast>,
    },
    Group(Box<Ast>), // ()
}
