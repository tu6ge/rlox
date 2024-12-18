use crate::lexer::TokenType::*;

use super::{ast::Expr, LiteralTypes, Visitor};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }
    fn evaluate(&mut self, ast: &Expr) -> LiteralTypes {
        ast.accept(self)
    }
}

impl Visitor<LiteralTypes> for Interpreter {
    fn visit_binary(&mut self, expr: &super::ast::Binary) -> LiteralTypes {
        let left = self.evaluate(&expr.left);

        let right = self.evaluate(&expr.right);

        match (expr.op.ttype.clone(), left, right) {
            (Plus, LiteralTypes::Number(left), LiteralTypes::Number(right)) => {
                LiteralTypes::Number(left + right)
            }
            (Plus, LiteralTypes::String(left), LiteralTypes::String(right)) => {
                LiteralTypes::String(format!("{}{}", left, right))
            }
            (Minus, LiteralTypes::Number(left), LiteralTypes::Number(right)) => {
                LiteralTypes::Number(left - right)
            }
            (Star, LiteralTypes::Number(left), LiteralTypes::Number(right)) => {
                LiteralTypes::Number(left * right)
            }
            (Slash, LiteralTypes::Number(left), LiteralTypes::Number(right)) => {
                if right == 0.0 {
                    todo!("0 cannot be used as a divisor")
                }
                LiteralTypes::Number(left / right)
            }
            _ => todo!("complier error"),
        }
    }

    fn visit_grouping(&mut self, expr: &super::ast::Grouping) -> LiteralTypes {
        self.evaluate(&expr.expr)
    }

    fn visit_unary(&mut self, expr: &super::ast::Unary) -> LiteralTypes {
        let value = self.evaluate(&expr.right);

        if let Minus = expr.op.ttype {
            if let LiteralTypes::Number(f) = value {
                return LiteralTypes::Number(-f);
            }
        }
        if let Bang = expr.op.ttype {
            return match value {
                LiteralTypes::Bool(b) => LiteralTypes::Bool(!b),
                LiteralTypes::Nil => LiteralTypes::Bool(true),
                _ => LiteralTypes::Bool(false),
            };
        }

        LiteralTypes::Nil
    }

    fn visit_literal(&self, expr: &LiteralTypes) -> LiteralTypes {
        expr.clone()
    }

    fn visit_comparison(&mut self, expr: &super::ast::Comparison) -> LiteralTypes {
        let left = self.evaluate(&expr.left);

        let right = self.evaluate(&expr.right);

        let bool = match (expr.op.ttype.clone(), left, right) {
            (Greater, LiteralTypes::Number(left), LiteralTypes::Number(right)) => left > right,
            (GreaterEqual, LiteralTypes::Number(left), LiteralTypes::Number(right)) => {
                left >= right
            }
            (Less, LiteralTypes::Number(left), LiteralTypes::Number(right)) => left < right,
            (LessEqual, LiteralTypes::Number(left), LiteralTypes::Number(right)) => left <= right,
            (BangEqual, left, right) => !left.equal(&right),
            (EqualEqual, left, right) => left.equal(&right),
            _ => false,
        };

        LiteralTypes::Bool(bool)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    use super::*;

    fn number(val: f64) -> LiteralTypes {
        LiteralTypes::Number(val)
    }
    fn get_value(source: &str) -> LiteralTypes {
        let ast = Parser::new(source).equality().unwrap();

        let mut inter = Interpreter::new();
        inter.evaluate(&ast)
    }

    #[test]
    fn noraml() {
        assert_eq!(get_value("-1"), number(-1.0));
        assert_eq!(get_value("1+ (-2)"), number(-1.0));

        assert_eq!(get_value("1 + 2 * (3+4) - 5"), number(10.0));
    }

    #[test]
    fn logic() {
        assert_eq!(get_value("true"), LiteralTypes::Bool(true));
        assert_eq!(get_value("nil"), LiteralTypes::Nil);
        assert_eq!(get_value("!!nil"), LiteralTypes::Bool(false));

        assert_eq!(get_value("true == false"), LiteralTypes::Bool(false));
        assert_eq!(get_value("false == true"), LiteralTypes::Bool(false));
        assert_eq!(get_value("nil == false"), LiteralTypes::Bool(true));

        assert_eq!(get_value("1 + 2 == 3"), LiteralTypes::Bool(true));
        assert_eq!(get_value("1 + 2 >  3"), LiteralTypes::Bool(false));
        assert_eq!(get_value("1 + 2 >= 3"), LiteralTypes::Bool(true));
        assert_eq!(get_value("1 + 2 <  4"), LiteralTypes::Bool(true));
        assert_eq!(get_value("1 + 2 <= 4"), LiteralTypes::Bool(true));
    }
}
