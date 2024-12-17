use crate::lexer::{LiteralTypes, Token};

#[derive(Clone, Debug, PartialEq)]
pub enum Ast {
    Literal(LiteralTypes),
    Unary(Unary),
    Binary(Binary),
    Comparison(Comparison),
    Grouping(Grouping), // ()
}

#[derive(Debug, Clone, PartialEq)]
pub struct Unary {
    pub op: Token,
    pub right: Box<Ast>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Binary {
    pub left: Box<Ast>,
    pub op: Token,
    pub right: Box<Ast>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Comparison {
    pub left: Box<Ast>,
    pub op: Token,
    pub right: Box<Ast>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Grouping {
    pub expr: Box<Ast>,
}

pub trait Visitor<T> {
    fn visit_binary(&mut self, expr: &Binary) -> T;
    fn visit_grouping(&mut self, expr: &Grouping) -> T;
    fn visit_unary(&mut self, expr: &Unary) -> T;
    fn visit_literal(&self, expr: &LiteralTypes) -> T;
    fn visit_comparison(&mut self, expr: &Comparison) -> T;
}

impl Ast {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Ast::Binary(b) => visitor.visit_binary(b),
            Ast::Literal(b) => visitor.visit_literal(b),
            Ast::Comparison(b) => visitor.visit_comparison(b),
            Ast::Unary(u) => visitor.visit_unary(u),
            Ast::Grouping(g) => visitor.visit_grouping(g),
        }
    }
}
