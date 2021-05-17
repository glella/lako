use crate::frontend::error::Error;
use crate::frontend::token::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Get {
        object: Box<Expr>,
        name: Token,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Set {
        object: Box<Expr>,
        name: Token,
        value: Box<Expr>,
    },
    Super {
        keyword: Token,
        method: Token,
    },
    This {
        keyword: Token,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
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

// *** New Stuff ==>

impl Expr {
    pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> Result<R, Error> {
        match self {
            Expr::Assign { name, value } => visitor.visit_assign_expr(name, value),
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary_expr(left, operator, right),
            Expr::Call {
                callee,
                paren,
                arguments,
            } => visitor.visit_call_expr(callee, paren, arguments),
            Expr::Get { object, name } => visitor.visit_get_expr(object, name),
            Expr::Grouping { expression } => visitor.visit_grouping_expr(expression),
            Expr::Literal { value } => visitor.visit_literal_expr(value),
            Expr::Logical {
                left,
                operator,
                right,
            } => visitor.visit_logical_expr(left, operator, right),
            Expr::Set {
                object,
                name,
                value,
            } => visitor.visit_set_expr(object, name, value),
            Expr::Super { keyword, method } => visitor.visit_super_expr(keyword, method),
            Expr::This { keyword } => visitor.visit_this_expr(keyword),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
            Expr::Variable { name } => visitor.visit_variable_expr(name),
        }
    }
}

// Implement Visitor Pattern
pub trait Visitor<R> {
    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<R, Error>;
    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<R, Error>;
    fn visit_call_expr(
        &mut self,
        callee: &Expr,
        paren: &Token,
        arguments: &[Expr],
    ) -> Result<R, Error>;
    fn visit_get_expr(&mut self, object: &Expr, name: &Token) -> Result<R, Error>;

    /// Visit a grouping expression.
    ///
    /// # Arguments
    ///
    /// * `expression` - This is the *inner* expression of the grouping.
    fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<R, Error>;
    fn visit_literal_expr(&self, value: &LiteralValue) -> Result<R, Error>;
    fn visit_logical_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<R, Error>;
    fn visit_set_expr(&mut self, object: &Expr, name: &Token, value: &Expr) -> Result<R, Error>;
    fn visit_super_expr(&mut self, keyword: &Token, method: &Token) -> Result<R, Error>;
    fn visit_this_expr(&mut self, keyword: &Token) -> Result<R, Error>;
    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<R, Error>;
    fn visit_variable_expr(&mut self, name: &Token) -> Result<R, Error>;
}

// pub trait Visitor<T> {
//     fn visit_expr(&mut self, e: &Expr) -> T;
//     // fn visit_stmt(&mut self, s: &Stmt) -> T; // to be implemented later
// }

// // Implement visitor patter in idiomatic Rust
// impl Expr {
//     pub fn walk_expr(visitor: &mut dyn Visitor<T>, e: &Expr) {
//         match *e {
//             Expr::Assign { name, value } => {}
//             Expr::Binary {
//                 left,
//                 operator,
//                 right,
//             } => {}
//             Expr::Call {
//                 callee,
//                 paren,
//                 arguments,
//             } => {}
//             Expr::Get { object, name } => {}
//             Expr::Grouping { expression } => {}
//             Expr::Literal { value } => {}
//             Expr::Logical {
//                 left,
//                 operator,
//                 right,
//             } => {}
//             Expr::Set {
//                 object,
//                 name,
//                 value,
//             } => {}
//             Expr::Super { keyword, method } => {}
//             Expr::This { keyword } => {}
//             Expr::Unary { operator, right } => {}
//             Expr::Variable { name } => {}
//         }
//     }
// }

// pub struct AstPrinter;

// impl AstPrinter {
//     pub fn print(&mut self, expr: Expr) -> Result<String, Error> {
//         expr.walk_expr(self)
//     }

//     fn parenthesize(&mut self, name: String, exprs: Vec<&Expr>) -> Result<String, Error> {
//         let mut r = String::new();
//         r.push_str("(");
//         r.push_str(&name);
//         for e in exprs {
//             r.push_str(" ");
//             r.push_str(&e.walk_epr(self)?);
//         }
//         r.push_str(")");
//         Ok(r)
//     }
// }

// impl Visitor<T> for AstPrinter {
//     fn visit_expr(&mut self, e: &Expr) -> T {
//         match *e {
//             Expr::IntLit(n) => n,
//             Expr::Add(ref lhs, ref rhs) => self.visit_expr(lhs) + self.visit_expr(rhs),
//             Expr::Sub(ref lhs, ref rhs) => self.visit_expr(lhs) - self.visit_expr(rhs),
//         }
//     }
// }

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
    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<String, Error> {
        self.parenthesize(operator.lexeme.clone(), vec![left, right])
    }

    fn visit_get_expr(&mut self, object: &Expr, name: &Token) -> Result<String, Error> {
        self.parenthesize(name.lexeme.clone(), vec![object])
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) -> Result<String, Error> {
        self.parenthesize("group".to_string(), vec![expr])
    }

    fn visit_literal_expr(&self, value: &LiteralValue) -> Result<String, Error> {
        Ok(value.to_string()) // check for null
    }

    fn visit_logical_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<String, Error> {
        self.parenthesize(operator.lexeme.clone(), vec![left, right])
    }

    fn visit_set_expr(
        &mut self,
        object: &Expr,
        name: &Token,
        value: &Expr,
    ) -> Result<String, Error> {
        self.parenthesize(name.lexeme.clone(), vec![object, value])
    }

    fn visit_super_expr(&mut self, _keyword: &Token, _method: &Token) -> Result<String, Error> {
        Ok("super".to_string())
    }

    fn visit_this_expr(&mut self, _keyword: &Token) -> Result<String, Error> {
        Ok("this".to_string())
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<String, Error> {
        self.parenthesize(operator.lexeme.clone(), vec![right])
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
        _arguments: &[Expr],
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
            left: Box::new(Expr::Unary {
                operator: Token::new(TokenType::Minus, "-", 1),
                right: Box::new(Expr::Literal {
                    value: LiteralValue::Number(123f64),
                }),
            }),
            operator: Token::new(TokenType::Star, "*", 1),
            right: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Literal {
                    value: LiteralValue::Number(45.67f64),
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
