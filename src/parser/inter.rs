use std::{cell::RefCell, rc::Rc};

use crate::lexer::{Token, TokenType::*};

use super::{
    ast::Expr,
    env::Environment,
    stmt::{self, Stmt},
    LiteralTypes, Visitor,
};

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

#[derive(Debug, Clone)]
enum RuntimeError {
    String(std::string::String),
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }
    pub fn interpret(&mut self, stmt: &[Stmt]) -> Result<(), RuntimeError> {
        for i in stmt {
            self.execute(i)?;
        }

        Ok(())
    }
    fn execute_block(&mut self, stmt: &[Stmt], env: Environment) -> Result<(), RuntimeError> {
        let previous = self.environment.clone();
        self.environment = Rc::new(RefCell::new(env));
        for s in stmt {
            self.execute(&s)?;
        }

        self.environment = previous;

        Ok(())
    }
    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
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

    fn visit_variable(
        &mut self,
        expr: &super::ast::Variable,
    ) -> Result<LiteralTypes, RuntimeError> {
        self.environment
            .borrow()
            .get(&expr.identifier)
            .ok_or(self.error(
                &expr.identifier,
                &format!("Undefined variable '{}'.", &expr.identifier.lexeme),
            ))
    }

    fn visit_assign_expr(
        &mut self,
        expr: &super::ast::Assign,
    ) -> Result<LiteralTypes, RuntimeError> {
        let value = self.evaluate(&expr.value)?;
        self.environment
            .borrow_mut()
            .assign(&expr.name, value.clone())
            .map_err(|_| {
                self.error(
                    &expr.name,
                    &format!("Undefined variable '{}'.", expr.name.lexeme),
                )
            })?;

        Ok(value)
    }

    fn visit_logical_expr(
        &mut self,
        expr: &super::ast::Logical,
    ) -> Result<LiteralTypes, RuntimeError> {
        let left = self.evaluate(&expr.left)?;

        if expr.op.ttype == Or {
            if left.is_true() {
                return Ok(left);
            }
        } else {
            if !left.is_true() {
                return Ok(left);
            }
        }

        self.evaluate(&expr.right)
    }
}

impl stmt::Visitor<Result<(), RuntimeError>> for Interpreter {
    fn visit_expression_stmt(&mut self, expr: &stmt::Expression) -> Result<(), RuntimeError> {
        self.evaluate(&expr.expression)?;

        Ok(())
    }

    fn visit_print_stmt(&mut self, expr: &stmt::Print) -> Result<(), RuntimeError> {
        let res = self.evaluate(&expr.expression).unwrap();
        println!("{}", res.stringify());

        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &stmt::Var) -> Result<(), RuntimeError> {
        let mut value = LiteralTypes::Nil;
        if stmt.initializer != Expr::Literal(LiteralTypes::Nil) {
            value = self.evaluate(&stmt.initializer)?;
        }

        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), value);

        Ok(())
    }

    fn visit_block_stmt(&mut self, block: &stmt::Block) -> Result<(), RuntimeError> {
        self.execute_block(
            &block.statements,
            Environment::new_with_enclosing(self.environment.clone()),
        )?;

        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &stmt::If) -> Result<(), RuntimeError> {
        if self.evaluate(&stmt.condition)?.is_true() {
            self.execute(&stmt.then_branch)?;
        } else if let Some(else_branch) = &stmt.else_branch {
            self.execute(&else_branch)?;
        }

        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &stmt::While) -> Result<(), RuntimeError> {
        loop {
            let cond = self.evaluate(&stmt.condition)?;
            if cond.is_true() == false {
                break;
            }
            self.execute(&stmt.body)?;
        }

        Ok(())
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

    #[test]
    fn run_stmt_plus() {
        let code = r#"var a = 1;
        var b = 2;
        print a+b;
        "#;
        let stmt = Parser::new(code).parse().unwrap();

        let mut inter = Interpreter::new();
        inter.interpret(&stmt);
    }

    #[test]
    fn run_assign_stmt() {
        let code = r#"var b = 2;
        print b = 3;
        "#;
        let stmt = Parser::new(code).parse().unwrap();
        //println!("{:?}", stmt);

        let mut inter = Interpreter::new();
        inter.interpret(&stmt);
    }

    #[test]
    fn run_global_env() {
        let code = r#"var a = "global a";
var b = "global b";
var c = "global c";
{
  var a = "outer a";
  var b = "outer b";
  {
    var a = "inner a";
    print a;
    print b;
    print c;
  }
  print a;
  print b;
  print c;
}
print a;
print b;
print c;
        "#;
        let stmt = Parser::new(code).parse().unwrap();
        //println!("{:?}", stmt);

        let mut inter = Interpreter::new();
        inter.interpret(&stmt);
    }

    #[test]
    fn run_logical_stmt() {
        let code = r#"print nil or 123;"#;
        let stmt = Parser::new(code).parse().unwrap();
        //println!("{:?}", stmt);

        let mut inter = Interpreter::new();
        inter.interpret(&stmt);
    }

    #[test]
    fn run_for_stmt() {
        let code = r#"
var a = 0;
var temp;

for (var b = 1; a < 10000; b = temp + b) {
  print a;
  temp = a;
  a = b;
}"#;
        let stmt = Parser::new(code).parse().unwrap();
        //println!("{:?}", stmt);

        let mut inter = Interpreter::new();
        inter.interpret(&stmt);
    }
}
