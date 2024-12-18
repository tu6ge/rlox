use crate::lexer::Token;

use super::ast::Expr;

#[derive(Debug)]
pub enum Stmt {
    Expression(Expression),
    Print(Print),
    Var(Var),
    Block(Block),
    If(If),
}
#[derive(Debug)]
pub struct Expression {
    pub expression: Expr,
}
#[derive(Debug)]
pub struct Print {
    pub expression: Expr,
}
#[derive(Debug)]
pub struct Var {
    pub name: Token,
    pub initializer: Expr,
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

#[derive(Debug)]
pub struct If {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

pub trait Visitor<T> {
    fn visit_expression_stmt(&mut self, expr: &Expression) -> T;
    fn visit_print_stmt(&mut self, expr: &Print) -> T;
    fn visit_var_stmt(&mut self, stmt: &Var) -> T;
    fn visit_block_stmt(&mut self, block: &Block) -> T;
    fn visit_if_stmt(&mut self, i: &If) -> T;
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Stmt::Expression(v) => visitor.visit_expression_stmt(v),
            Stmt::Print(p) => visitor.visit_print_stmt(p),
            Stmt::Var(v) => visitor.visit_var_stmt(v),
            Stmt::Block(b) => visitor.visit_block_stmt(b),
            Stmt::If(i) => visitor.visit_if_stmt(i),
        }
    }
}
