/*
 * Copyright (c) 2019. NeroYang
 */

use std::fmt::{Write, Display, Formatter};
use std::error::Error;
use std::fmt;
use crate::dulang::lexer::token::Token;

#[derive(Debug, Clone)]
pub enum ParserError {
    UnexpectedTokenError { token: Token, line: usize }
}

impl Error for ParserError {
    fn description(&self) -> &str {
        match self {
            _ => "Syntax Error:",
        }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            _ => f.write_str(&format!("Syntax Error:")),
        }
    }
}
