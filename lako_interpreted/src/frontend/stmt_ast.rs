use crate::frontend::error::Error;
use crate::frontend::expr_ast::Expr;
use crate::frontend::token::Token;

#[derive(Clone)]
pub enum Stmt {
    Block {
        stmts: Vec<Stmt>,
    },
    Class {
        name: Token,
        sclass: Option<Expr>,
        methods: Vec<Stmt>,
    },
    Expression {
        expr: Expr,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    If {
        cond: Expr,
        then_: Box<Stmt>,
        else_: Box<Option<Stmt>>,
    },
    Print {
        expr: Expr,
    },
    Return {
        keywd: Token,
        val: Option<Expr>,
    },
    Var {
        name: Token,
        init: Option<Expr>,
    },
    While {
        cond: Expr,
        body: Box<Stmt>,
    },
    //Nil,
}

// Implement Visitor Pattern
// A visitor encapsulates an algorithm that operates over a heterogeneous collection of objects.
pub trait Visitor<T> {
    fn visit_block_stmt(&mut self, stmts: &[Stmt]) -> Result<T, Error>;
    fn visit_class_stmt(
        &mut self,
        name: &Token,
        sclass: &Option<Expr>,
        methods: &[Stmt],
    ) -> Result<T, Error>;
    fn visit_expression_stmt(&mut self, expr: &Expr) -> Result<T, Error>;
    fn visit_function_stmt(
        &mut self,
        name: &Token,
        params: &[Token],
        body: &[Stmt],
    ) -> Result<T, Error>;
    fn visit_if_stmt(
        &mut self,
        cond: &Expr,
        else_: &Option<Stmt>,
        then_: &Stmt,
    ) -> Result<T, Error>;
    fn visit_print_stmt(&mut self, expr: &Expr) -> Result<T, Error>;
    fn visit_return_stmt(&mut self, keywd: &Token, val: &Option<Expr>) -> Result<T, Error>;
    fn visit_var_stmt(&mut self, name: &Token, init: &Option<Expr>) -> Result<T, Error>;
    fn visit_while_stmt(&mut self, cond: &Expr, body: &Stmt) -> Result<T, Error>;
}

impl Stmt {
    pub fn accept<T>(&self, v: &mut dyn Visitor<T>) -> Result<T, Error> {
        match self {
            Stmt::Block { stmts } => v.visit_block_stmt(stmts),
            Stmt::Class {
                name,
                sclass,
                methods,
            } => v.visit_class_stmt(name, sclass, methods),
            Stmt::Expression { expr } => v.visit_expression_stmt(expr),
            Stmt::Function { name, params, body } => v.visit_function_stmt(name, params, body),
            Stmt::If { cond, else_, then_ } => v.visit_if_stmt(cond, else_, then_),
            Stmt::Print { expr } => v.visit_print_stmt(expr),
            Stmt::Return { keywd, val } => v.visit_return_stmt(keywd, val),
            Stmt::Var { name, init } => v.visit_var_stmt(name, init),
            Stmt::While { cond, body } => v.visit_while_stmt(cond, body),
            //Stmt::Nil => unimplemented!(),
        }
    }
}
