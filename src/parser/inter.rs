use crate::lexer::{Token, TokenType};

use super::{ast::Ast, LiteralTypes, Visitor};

pub struct Interpreter {
    ast: Ast,
}

impl Interpreter {
    fn evaluate(&mut self, ast: &Ast) -> LiteralTypes {
        ast.accept(self)
    }
}

impl Visitor<LiteralTypes> for Interpreter {
    fn visit_binary(&mut self, expr: &super::ast::Binary) -> LiteralTypes {
        let mut left_string = String::new();
        let mut right_string = String::new();
        let mut is_num = 0_u8;
        let mut is_str = 0_u8;
        let left = if let LiteralTypes::Number(val) = self.evaluate(&expr.left) {
            is_num += 1;
            val
        } else {
            if let LiteralTypes::String(str) = self.evaluate(&expr.left) {
                left_string = str;
                is_str += 1;
                0.0
            } else {
                return LiteralTypes::Nil;
            }
        };
        let right = if let LiteralTypes::Number(val) = self.evaluate(&expr.right) {
            is_num += 1;
            val
        } else {
            if let LiteralTypes::String(str) = self.evaluate(&expr.right) {
                right_string = str;
                is_str += 1;
                0.0
            } else {
                return LiteralTypes::Nil;
            }
        };

        match expr.op.ttype {
            TokenType::Plus => {
                if is_num == 2 {
                    LiteralTypes::Number(left + right)
                } else if is_str == 2 {
                    left_string.push_str(&right_string);
                    LiteralTypes::String(left_string)
                } else if is_num == 1 && is_str == 1 {
                    todo!("number can not plus to string");
                } else {
                    todo!("the type can not plus");
                }
            }
            TokenType::Minus => LiteralTypes::Number(left - right),
            TokenType::Star => LiteralTypes::Number(left * right),
            TokenType::Slash => {
                if right == 0.0 {
                    todo!("runtime error")
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

        if let TokenType::Minus = expr.op.ttype {
            if let LiteralTypes::Number(f) = value {
                return LiteralTypes::Number(-f);
            }
        }
        if let TokenType::Bang = expr.op.ttype {
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
        let left = if let LiteralTypes::Number(val) = self.evaluate(&expr.left) {
            val
        } else {
            return LiteralTypes::Bool(false);
        };
        let right = if let LiteralTypes::Number(val) = self.evaluate(&expr.right) {
            val
        } else {
            return LiteralTypes::Bool(false);
        };
        let bool = match expr.op.ttype {
            TokenType::Greater => left > right,
            TokenType::GreaterEqual => left >= right,
            TokenType::Less => left < right,
            TokenType::LessEqual => left <= right,
            TokenType::BangEqual => left != right, //TODO
            TokenType::EqualEqual => left == right,
            _ => false,
        };

        LiteralTypes::Bool(bool)
    }
}
