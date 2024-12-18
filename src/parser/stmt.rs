use super::ast::Expr;

pub enum Stmt {
    Expression(Expression),
    Print(Print),
}
pub struct Expression {
    pub expression: Expr,
}
pub struct Print {
    pub expression: Expr,
}

pub trait Visitor<T> {
    fn visit_expression_stmt(&mut self, expr: &Expression) -> T;
    fn visit_print_stmt(&mut self, expr: &Print) -> T;
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Stmt::Expression(v) => visitor.visit_expression_stmt(v),
            Stmt::Print(p) => visitor.visit_print_stmt(p),
        }
    }
}
