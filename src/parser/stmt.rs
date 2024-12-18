use crate::lexer::Token;

use super::ast::Expr;

pub enum Stmt {
    Expression(Expression),
    Print(Print),
    Var(Var),
}
pub struct Expression {
    pub expression: Expr,
}
pub struct Print {
    pub expression: Expr,
}
pub struct Var {
    pub name: Token,
    pub initializer: Expr,
}

pub trait Visitor<T> {
    fn visit_expression_stmt(&mut self, expr: &Expression) -> T;
    fn visit_print_stmt(&mut self, expr: &Print) -> T;
    fn visit_var_stmt(&mut self, stmt: &Var) -> T;
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Stmt::Expression(v) => visitor.visit_expression_stmt(v),
            Stmt::Print(p) => visitor.visit_print_stmt(p),
            Stmt::Var(v) => visitor.visit_var_stmt(v),
        }
    }
}
