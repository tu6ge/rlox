use crate::lexer::{Token, TokenType::*};

use super::{
    ast::Expr,
    stmt::{self, Stmt},
    LiteralTypes, Visitor,
};

pub struct Interpreter {}

#[derive(Debug, Clone)]
enum RuntimeError {
    String(std::string::String),
}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }
    pub fn interpret(&mut self, stmt: &[Stmt]) {
        for i in stmt {
            self.execute(i);
        }
    }
    fn execute(&mut self, stmt: &Stmt) {
        stmt.accept(self)
    }

    fn evaluate(&mut self, ast: &Expr) -> Result<LiteralTypes, RuntimeError> {
        ast.accept(self)
    }
    fn error(&self, token: &Token, message: &str) -> RuntimeError {
        let line = token.line;

        let msg = format!("line:{line} {message}");

        RuntimeError::String(msg)
    }
}

impl Visitor<Result<LiteralTypes, RuntimeError>> for Interpreter {
    fn visit_binary(&mut self, expr: &super::ast::Binary) -> Result<LiteralTypes, RuntimeError> {
        let left = self.evaluate(&expr.left)?;

        let right = self.evaluate(&expr.right)?;

        match (expr.op.ttype.clone(), left, right) {
            (Plus, LiteralTypes::Number(left), LiteralTypes::Number(right)) => {
                Ok(LiteralTypes::Number(left + right))
            }
            (Plus, LiteralTypes::String(left), LiteralTypes::String(right)) => {
                Ok(LiteralTypes::String(format!("{}{}", left, right)))
            }
            (Plus, _, _) => Err(self.error(&expr.op, "Operand must be two numbers or two strings")),
            (Minus, LiteralTypes::Number(left), LiteralTypes::Number(right)) => {
                Ok(LiteralTypes::Number(left - right))
            }
            (Star, LiteralTypes::Number(left), LiteralTypes::Number(right)) => {
                Ok(LiteralTypes::Number(left * right))
            }
            (Slash, LiteralTypes::Number(left), LiteralTypes::Number(right)) => {
                if right == 0.0 {
                    return Err(self.error(&expr.op, "0 cannot be used as a divisor"));
                }
                Ok(LiteralTypes::Number(left / right))
            }
            _ => Err(self.error(&expr.op, "Operand must be two number")),
        }
    }

    fn visit_grouping(
        &mut self,
        expr: &super::ast::Grouping,
    ) -> Result<LiteralTypes, RuntimeError> {
        self.evaluate(&expr.expr)
    }

    fn visit_unary(&mut self, expr: &super::ast::Unary) -> Result<LiteralTypes, RuntimeError> {
        let value = self.evaluate(&expr.right)?;

        if let Minus = expr.op.ttype {
            if let LiteralTypes::Number(f) = value {
                return Ok(LiteralTypes::Number(-f));
            } else {
                return Err(self.error(&expr.op, "Minus must be a number"));
            }
        }
        if let Bang = expr.op.ttype {
            return Ok(match value {
                LiteralTypes::Bool(b) => LiteralTypes::Bool(!b),
                LiteralTypes::Nil => LiteralTypes::Bool(true),
                _ => LiteralTypes::Bool(false),
            });
        }

        Ok(LiteralTypes::Nil)
    }

    fn visit_literal(&self, expr: &LiteralTypes) -> Result<LiteralTypes, RuntimeError> {
        Ok(expr.clone())
    }

    fn visit_comparison(
        &mut self,
        expr: &super::ast::Comparison,
    ) -> Result<LiteralTypes, RuntimeError> {
        let left = self.evaluate(&expr.left)?;

        let right = self.evaluate(&expr.right)?;

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

        Ok(LiteralTypes::Bool(bool))
    }
}

impl stmt::Visitor<()> for Interpreter {
    fn visit_expression_stmt(&mut self, expr: &stmt::Expression) -> () {
        self.evaluate(&expr.expression);
    }

    fn visit_print_stmt(&mut self, expr: &stmt::Print) -> () {
        let res = self.evaluate(&expr.expression).unwrap();
        println!("{}", res.stringify());
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
        inter.evaluate(&ast).unwrap()
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

    #[test]
    fn run_stmt() {
        let stmt = Parser::new("print \"abc\";").parse().unwrap();

        let mut inter = Interpreter::new();
        inter.interpret(&stmt);
    }
}
