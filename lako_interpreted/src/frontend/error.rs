use crate::frontend::token::{Token, TokenType};
use std::io;
use std::{convert, fmt};

pub fn error(line: i32, message: &str) {
    report(line, "", message);
}

pub fn report(line: i32, where_: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, where_, message);
}

pub fn parser_error(token: &Token, message: &str) {
    if token.t_type == TokenType::Eof {
        report(token.line, " at end", message);
    } else {
        report(token.line, &format!(" at '{}'", token.lexeme), message);
    }
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Parse,
    Runtime { token: Token, message: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(underlying) => write!(f, "IoError {}", underlying),
            Error::Parse => write!(f, "ParseError"),
            Error::Runtime { message, .. } => write!(f, "RuntimeError {}", message),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        "Lox Error"
    }
}

impl convert::From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}
