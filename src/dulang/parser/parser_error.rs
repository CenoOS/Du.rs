/*
 * Copyright (c) 2019. NeroYang
 */

use crate::dulang::lexer::token::Token;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter, Write};

#[derive(Debug, Clone)]
pub enum ParserError {
    UnexpectedTokenError { token: Token, line: usize },
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
