use crate::lexer::{LiteralTypes, Token};

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Literal(LiteralTypes),
    Unary(Unary),
    Binary(Binary),
    Comparison(Comparison),
    Grouping(Grouping), // ()
    Variable(Variable),
    Assign(Assign),
    Logical(Logical),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Unary {
    pub op: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Binary {
    pub left: Box<Expr>,
    pub op: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Comparison {
    pub left: Box<Expr>,
    pub op: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Grouping {
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub identifier: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assign {
    pub name: Token,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Logical {
    pub left: Box<Expr>,
    pub op: Token,
    pub right: Box<Expr>,
}

pub trait Visitor<T> {
    fn visit_binary(&mut self, expr: &Binary) -> T;
    fn visit_grouping(&mut self, expr: &Grouping) -> T;
    fn visit_unary(&mut self, expr: &Unary) -> T;
    fn visit_literal(&self, expr: &LiteralTypes) -> T;
    fn visit_comparison(&mut self, expr: &Comparison) -> T;
    fn visit_variable(&mut self, expr: &Variable) -> T;
    fn visit_assign_expr(&mut self, expr: &Assign) -> T;
    fn visit_logical_expr(&mut self, expr: &Logical) -> T;
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Expr::Binary(b) => visitor.visit_binary(b),
            Expr::Literal(b) => visitor.visit_literal(b),
            Expr::Comparison(b) => visitor.visit_comparison(b),
            Expr::Unary(u) => visitor.visit_unary(u),
            Expr::Grouping(g) => visitor.visit_grouping(g),
            Expr::Variable(var) => visitor.visit_variable(var),
            Expr::Assign(a) => visitor.visit_assign_expr(a),
            Expr::Logical(l) => visitor.visit_logical_expr(l),
        }
    }
}
