use crate::frontend::error::Error;
use crate::frontend::token::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Expr {
    Assign {
        name: Token,
        val: Box<Expr>,
    },
    Binary {
        lhs: Box<Expr>,
        op: Token,
        rhs: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arg: Vec<Expr>,
    },
    Get {
        obj: Box<Expr>,
        name: Token,
    },
    Grouping {
        expr: Box<Expr>,
    },
    Literal {
        val: LiteralValue,
    },
    Logical {
        lhs: Box<Expr>,
        op: Token,
        rhs: Box<Expr>,
    },
    Set {
        obj: Box<Expr>,
        name: Token,
        val: Box<Expr>,
    },
    Super {
        keywd: Token,
        method: Token,
    },
    This {
        keywd: Token,
    },
    Unary {
        op: Token,
        rhs: Box<Expr>,
    },
    Variable {
        name: Token,
    },
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

// Implement Display for Ast printer
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralValue::Number(n) => write!(f, "{}", n),
            LiteralValue::String(s) => write!(f, "{}", s),
            LiteralValue::Boolean(b) => write!(f, "{}", b),
            LiteralValue::Nil => write!(f, "nil"),
        }
    }
}

// Implement Visitor Pattern:
// A visitor encapsulates an algorithm that operates over a heterogeneous collection of objects.
pub trait Visitor<T> {
    fn visit_assign_expr(&mut self, name: &Token, val: &Expr) -> Result<T, Error>;
    fn visit_binary_expr(&mut self, lhs: &Expr, op: &Token, rhs: &Expr) -> Result<T, Error>;
    fn visit_call_expr(&mut self, callee: &Expr, paren: &Token, arg: &[Expr]) -> Result<T, Error>;
    fn visit_get_expr(&mut self, obj: &Expr, name: &Token) -> Result<T, Error>;
    fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<T, Error>;
    fn visit_literal_expr(&self, val: &LiteralValue) -> Result<T, Error>;
    fn visit_logical_expr(&mut self, lhs: &Expr, op: &Token, rhs: &Expr) -> Result<T, Error>;
    fn visit_set_expr(&mut self, obj: &Expr, name: &Token, val: &Expr) -> Result<T, Error>;
    fn visit_super_expr(&mut self, keywd: &Token, method: &Token) -> Result<T, Error>;
    fn visit_this_expr(&mut self, keywd: &Token) -> Result<T, Error>;
    fn visit_unary_expr(&mut self, op: &Token, rhs: &Expr) -> Result<T, Error>;
    fn visit_variable_expr(&mut self, name: &Token) -> Result<T, Error>;
}

impl Expr {
    pub fn accept<T>(&self, v: &mut dyn Visitor<T>) -> Result<T, Error> {
        match self {
            Expr::Assign { name, val } => v.visit_assign_expr(name, val),
            Expr::Binary { lhs, op, rhs } => v.visit_binary_expr(lhs, op, rhs),
            Expr::Call { callee, paren, arg } => v.visit_call_expr(callee, paren, arg),
            Expr::Get { obj, name } => v.visit_get_expr(obj, name),
            Expr::Grouping { expr } => v.visit_grouping_expr(expr),
            Expr::Literal { val } => v.visit_literal_expr(val),
            Expr::Logical { lhs, op, rhs } => v.visit_logical_expr(lhs, op, rhs),
            Expr::Set { obj, name, val } => v.visit_set_expr(obj, name, val),
            Expr::Super { keywd, method } => v.visit_super_expr(keywd, method),
            Expr::This { keywd } => v.visit_this_expr(keywd),
            Expr::Unary { op, rhs } => v.visit_unary_expr(op, rhs),
            Expr::Variable { name } => v.visit_variable_expr(name),
        }
    }
}

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&mut self, expr: Expr) -> Result<String, Error> {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: String, exprs: Vec<&Expr>) -> Result<String, Error> {
        let mut r = String::new();
        r.push('(');
        r.push_str(&name);
        for e in exprs {
            r.push(' ');
            r.push_str(&e.accept(self)?);
        }
        r.push(')');
        Ok(r)
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, lhs: &Expr, op: &Token, rhs: &Expr) -> Result<String, Error> {
        self.parenthesize(op.lexeme.clone(), vec![lhs, rhs])
    }

    fn visit_get_expr(&mut self, obj: &Expr, name: &Token) -> Result<String, Error> {
        self.parenthesize(name.lexeme.clone(), vec![obj])
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<String, Error> {
        self.parenthesize("group".to_string(), vec![expr])
    }

    fn visit_literal_expr(&self, val: &LiteralValue) -> Result<String, Error> {
        Ok(val.to_string())
    }

    fn visit_logical_expr(&mut self, lhs: &Expr, op: &Token, rhs: &Expr) -> Result<String, Error> {
        self.parenthesize(op.lexeme.clone(), vec![lhs, rhs])
    }

    fn visit_set_expr(&mut self, obj: &Expr, name: &Token, val: &Expr) -> Result<String, Error> {
        self.parenthesize(name.lexeme.clone(), vec![obj, val])
    }

    fn visit_super_expr(&mut self, _keywd: &Token, _method: &Token) -> Result<String, Error> {
        Ok("super".to_string())
    }

    fn visit_this_expr(&mut self, _keywd: &Token) -> Result<String, Error> {
        Ok("this".to_string())
    }

    fn visit_unary_expr(&mut self, op: &Token, rhs: &Expr) -> Result<String, Error> {
        self.parenthesize(op.lexeme.clone(), vec![rhs])
    }

    fn visit_variable_expr(&mut self, name: &Token) -> Result<String, Error> {
        Ok(name.lexeme.clone())
    }

    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<String, Error> {
        self.parenthesize(name.lexeme.clone(), vec![value])
    }

    fn visit_call_expr(
        &mut self,
        _callee: &Expr,
        _paren: &Token,
        _arg: &[Expr],
    ) -> Result<String, Error> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::token::{Token, TokenType};

    #[test]
    fn test_printer() {
        let expression = Expr::Binary {
            lhs: Box::new(Expr::Unary {
                op: Token::new(TokenType::Minus, "-", 1),
                rhs: Box::new(Expr::Literal {
                    val: LiteralValue::Number(123f64),
                }),
            }),
            op: Token::new(TokenType::Star, "*", 1),
            rhs: Box::new(Expr::Grouping {
                expr: Box::new(Expr::Literal {
                    val: LiteralValue::Number(45.67f64),
                }),
            }),
        };
        let mut printer = AstPrinter;

        assert_eq!(
            printer.print(expression).unwrap(),
            "(* (- 123) (group 45.67))"
        );
    }
}
