use crate::frontend::error::{parser_error, Error};
//use crate::frontend::stmt_ast::Stmt;
use super::expr_ast::{Expr, LiteralValue};
use crate::frontend::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    // pub fn parse(&mut self) -> Result<Vec<Stmt>, Error> {
    //     let mut statements: Vec<Stmt> = Vec::new();
    //     while !self.is_at_end() {
    //         statements.push(self.declaration()?);
    //     }
    //     Ok(statements)
    // }

    // main public method
    pub fn parse(&mut self) -> Result<Expr, Error> {
        self.expression()
    }

    // token stream helper methods
    // checks if we reached the end of the token stream
    fn is_at_end(&self) -> bool {
        self.peek().t_type == TokenType::Eof
    }

    // peeks at current token - returns the current token without consuming it
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    // returns current token and advances to the next
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    // returns previous token
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    // checks if current Token TokenType is == argument
    fn check(&self, t_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        t_type == self.peek().t_type
    }

    // consumes tokens until finding ")". If does not find it returns error message
    fn consume(&mut self, t_type: TokenType, message: &str) -> Result<Token, Error> {
        if self.check(t_type) {
            Ok(self.advance().clone())
        } else {
            Err(self.error(self.peek(), message))
        }
    }

    // returns parse error
    fn error(&self, token: &Token, message: &str) -> Error {
        parser_error(token, message);
        Error::Parse
    }

    // unused - and don't remember why I coded this
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().t_type == TokenType::Semicolon {
                return;
            }

            match self.peek().t_type {
                TokenType::Class
                | TokenType::Fn
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {} //_ => self.advance(),
            };
            self.advance();
        }
    }

    // compares current token to array of tokens
    fn t_match(&mut self, token_types: &[TokenType]) -> bool {
        for tt in token_types {
            if self.check(tt.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    // GRAMMAR:
    // expression     → equality ;
    // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    // term           → factor ( ( "-" | "+" ) factor )* ;
    // factor         → unary ( ( "/" | "*" ) unary )* ;
    // unary          → ( "!" | "-" ) unary
    //                | primary ;
    // primary        → NUMBER | STRING | "true" | "false" | "nil"
    //                | "(" expression ")" ;

    // *** Grammar rules - Each grammar rule is a method ***
    // expression     → equality ;
    fn expression(&mut self) -> Result<Expr, Error> {
        self.equality()
    }

    // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr = self.comparison()?;

        while self.t_match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous().clone();
            let rhs = self.comparison()?;
            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            };
        }
        Ok(expr)
    }

    // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Result<Expr, Error> {
        let mut expr = self.term()?;

        while self.t_match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op = self.previous().clone();
            let rhs = self.term()?;
            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            };
        }
        Ok(expr)
    }

    // term           → factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Result<Expr, Error> {
        let mut expr = self.factor()?;

        while self.t_match(&[TokenType::Minus, TokenType::Plus]) {
            let op = self.previous().clone();
            let rhs = self.factor()?;
            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            };
        }
        Ok(expr)
    }

    // factor         → unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Result<Expr, Error> {
        let mut expr = self.unary()?;

        while self.t_match(&[TokenType::Slash, TokenType::Star]) {
            let op = self.previous().clone();
            let rhs = self.unary()?;
            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            };
        }
        Ok(expr)
    }

    // unary          → ( "!" | "-" ) unary
    //                | primary ;
    fn unary(&mut self) -> Result<Expr, Error> {
        if self.t_match(&[TokenType::Bang, TokenType::Minus]) {
            let op = self.previous().clone();
            let rhs = self.unary()?;
            Ok(Expr::Unary {
                op,
                rhs: Box::new(rhs),
            })
        } else {
            self.primary()
        }
    }

    //                | primary ;
    // we match on primary type and extract the literals
    fn primary(&mut self) -> Result<Expr, Error> {
        let expr = match &self.peek().t_type {
            TokenType::False => Expr::Literal {
                val: LiteralValue::Boolean(false),
            },
            TokenType::True => Expr::Literal {
                val: LiteralValue::Boolean(true),
            },
            TokenType::Nil => Expr::Literal {
                val: LiteralValue::Nil,
            },
            TokenType::String { literal } => Expr::Literal {
                val: LiteralValue::String(literal.clone()),
            },
            TokenType::Number { literal } => Expr::Literal {
                val: LiteralValue::Number(*literal),
            },
            // TokenType::Super => {
            //     let keyword = self.advance().clone();
            //     self.consume(TokenType::Dot, "Expect '.' after 'super'.")?;
            //     let method = self.consume(
            //         TokenType::Identifier {
            //             literal: "".to_string(),
            //         },
            //         "Expect superclass method name.",
            //     )?;

            //     // We already advance so we cut it short here.
            //     return Ok(Expr::Super {
            //         keywd: keyword,
            //         method,
            //     });
            // }
            // TokenType::This => Expr::This {
            //     keywd: self.peek().clone(),
            // },
            // TokenType::Identifier { literal } => Expr::Variable {
            //     name: self.peek().clone(),
            // },
            TokenType::LeftParen => {
                self.advance(); // if not we enter a recursive loop with '(' and we overflow the stack
                let expression = self.expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
                return Ok(Expr::Grouping {
                    expr: Box::new(expression),
                });
            }
            _ => return Err(self.error(self.peek(), "Expect expression.")),
        };

        self.advance();

        Ok(expr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::expr_ast::AstPrinter;
    use crate::frontend::scanner::Scanner;

    #[test]
    fn test_parser_equality() {
        // "!=" | "=="
        // 1 + 3 == 4  ->  (== (+ 1 3) 4)
        let mut scanner = Scanner::new("1 + 3 == 4".to_string());
        let mut tokens = scanner.scan_tokens().clone();
        let mut parser = Parser::new(tokens);
        let mut statements = parser.parse().expect("Could not parse sample code.");
        let mut printer = AstPrinter;
        assert_eq!(printer.print(statements).unwrap(), "(== (+ 1 3) 4)");
        // 1 + 3 != 2  ->  (!= (+ 1 3) 2)
        scanner = Scanner::new("1 + 3 != 2".to_string());
        tokens = scanner.scan_tokens().clone();
        parser = Parser::new(tokens);
        statements = parser.parse().expect("Could not parse sample code.");
        assert_eq!(printer.print(statements).unwrap(), "(!= (+ 1 3) 2)");
    }

    #[test]
    fn test_parser_comparison() {
        // ">" | ">=" | "<" | "<="
        // 4 > 2  ->  (> 4 2)
        let mut scanner = Scanner::new("4 > 2".to_string());
        let mut tokens = scanner.scan_tokens().clone();
        let mut parser = Parser::new(tokens);
        let mut statements = parser.parse().expect("Could not parse sample code.");
        let mut printer = AstPrinter;
        assert_eq!(printer.print(statements).unwrap(), "(> 4 2)");
        // 3 >= 3  ->  (>= 3 3)
        scanner = Scanner::new("3 >= 3".to_string());
        tokens = scanner.scan_tokens().clone();
        parser = Parser::new(tokens);
        statements = parser.parse().expect("Could not parse sample code.");
        assert_eq!(printer.print(statements).unwrap(), "(>= 3 3)");
        // 6 < 7  ->  (< 6 7)
        scanner = Scanner::new("6 < 7".to_string());
        tokens = scanner.scan_tokens().clone();
        parser = Parser::new(tokens);
        statements = parser.parse().expect("Could not parse sample code.");
        assert_eq!(printer.print(statements).unwrap(), "(< 6 7)");
        // 8 <= 8  ->  (<= 8 8)
        scanner = Scanner::new("8 <= 8".to_string());
        tokens = scanner.scan_tokens().clone();
        parser = Parser::new(tokens);
        statements = parser.parse().expect("Could not parse sample code.");
        assert_eq!(printer.print(statements).unwrap(), "(<= 8 8)");
    }

    #[test]
    fn test_parser_term() {
        //  "-" | "+"
        // 7 - 2 + 3  ->  (+ (- 7 2) 3)
        let mut scanner = Scanner::new("7 - 2 + 3".to_string());
        let tokens = scanner.scan_tokens().clone();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().expect("Could not parse sample code.");
        let mut printer = AstPrinter;
        assert_eq!(printer.print(statements).unwrap(), "(+ (- 7 2) 3)");
    }

    #[test]
    fn test_parser_factor() {
        // "/" | "*"
        // 8 * 2 / 4  ->  (/ (* 8 2) 4)
        let mut scanner = Scanner::new("8 * 2 / 4".to_string());
        let tokens = scanner.scan_tokens().clone();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().expect("Could not parse sample code.");
        let mut printer = AstPrinter;
        assert_eq!(printer.print(statements).unwrap(), "(/ (* 8 2) 4)");
    }

    #[test]
    fn test_parser_unary() {
        // "!" | "-"
        // -4 + 5 ->  (+ (- 4) 5)
        let mut scanner = Scanner::new("-4 + 5".to_string());
        let mut tokens = scanner.scan_tokens().clone();
        let mut parser = Parser::new(tokens);
        let mut statements = parser.parse().expect("Could not parse sample code.");
        let mut printer = AstPrinter;
        assert_eq!(printer.print(statements).unwrap(), "(+ (- 4) 5)");
        // !3  ->  (! 3)
        scanner = Scanner::new("!3".to_string());
        tokens = scanner.scan_tokens().clone();
        parser = Parser::new(tokens);
        statements = parser.parse().expect("Could not parse sample code.");
        assert_eq!(printer.print(statements).unwrap(), "(! 3)");
    }

    #[test]
    fn test_parser_primary() {
        // false
        let mut scanner = Scanner::new("false".to_string());
        let mut tokens = scanner.scan_tokens().clone();
        let mut parser = Parser::new(tokens);
        let mut statements = parser.parse().expect("Could not parse sample code.");
        let mut printer = AstPrinter;
        assert_eq!(printer.print(statements).unwrap(), "false");
        // true
        scanner = Scanner::new("true".to_string());
        tokens = scanner.scan_tokens().clone();
        parser = Parser::new(tokens);
        statements = parser.parse().expect("Could not parse sample code.");
        assert_eq!(printer.print(statements).unwrap(), "true");
        // nil
        scanner = Scanner::new("nil".to_string());
        tokens = scanner.scan_tokens().clone();
        parser = Parser::new(tokens);
        statements = parser.parse().expect("Could not parse sample code.");
        assert_eq!(printer.print(statements).unwrap(), "nil");
        // string
        scanner = Scanner::new("\"hello\"".to_string());
        tokens = scanner.scan_tokens().clone();
        parser = Parser::new(tokens);
        statements = parser.parse().expect("Could not parse sample code.");
        assert_eq!(printer.print(statements).unwrap(), "hello");
        // number
        scanner = Scanner::new("3.141519".to_string());
        tokens = scanner.scan_tokens().clone();
        parser = Parser::new(tokens);
        statements = parser.parse().expect("Could not parse sample code.");
        assert_eq!(printer.print(statements).unwrap(), "3.141519");
    }

    #[test]
    fn test_parser_grouping() {
        // (..)
        let mut scanner = Scanner::new("(2 + 3) * 5".to_string());
        let tokens = scanner.scan_tokens().clone();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().expect("Could not parse sample code.");
        let mut printer = AstPrinter;
        assert_eq!(printer.print(statements).unwrap(), "(* (group (+ 2 3)) 5)");
    }

    #[test]
    fn test_parser_sample_code() {
        let mut scanner = Scanner::new("-123 * 45.67".to_string());
        let tokens = scanner.scan_tokens().clone();
        let mut parser = Parser::new(tokens);
        let statements = parser.parse().expect("Could not parse sample code.");
        let mut printer = AstPrinter;
        assert_eq!(printer.print(statements).unwrap(), "(* (- 123) 45.67)");
    }
}
