/*
 * Copyright (c) 2019. NeroYang
 */

use crate::dolang::lexer::int::Int;
use crate::dolang::lexer::keyword::Keyword;
use crate::dolang::lexer::token::Token::{
    TokenAdd, TokenAddAssign, TokenAnd, TokenAndAssign, TokenAssign, TokenBand, TokenBor,
    TokenChar, TokenColon, TokenColonAssign, TokenComma, TokenDec, TokenDiv, TokenDivAssign,
    TokenDot, TokenEof, TokenEqual, TokenFloat, TokenGreaterThan, TokenGreaterThanEqual,
    TokenHashTag, TokenInc, TokenInt, TokenKeyword, TokenLastChar, TokenLeftBrackets,
    TokenLeftCurlyBrackets, TokenLeftShift, TokenLeftShiftAssign, TokenLeftSquareBrackets,
    TokenLessThan, TokenLessThanEqual, TokenMod, TokenModAssign, TokenMul, TokenMulAssign,
    TokenName, TokenNot, TokenNotEqual, TokenOr, TokenOrAssign, TokenPound, TokenQuestionMark,
    TokenRightBrackets, TokenRightCurlyBrackets, TokenRightShift, TokenRightShiftAssign,
    TokenRightSquareBrackets, TokenSemiColon, TokenStr, TokenSub, TokenSubAssign, TokenXor,
    TokenXorAssign,
};
use std::fmt;
use std::fmt::{Display, Formatter};

pub const TOKEN_LEFT_SHIFT: &str = "<<";
pub const TOKEN_RIGHT_SHIFT: &str = ">>";
pub const TOKEN_EQ: &str = "==";
pub const TOKEN_NOT_EQ: &str = "!=";
pub const TOKEN_LESS_THAN_EQ: &str = "<=";
pub const TOKEN_GREATER_THAN_EQ: &str = ">=";
pub const TOKEN_AND: &str = "&&";
pub const TOKEN_OR: &str = "||";
pub const TOKEN_COLON_ASSIGN: &str = ":=";
pub const TOKEN_ADD_ASSIGN: &str = "+=";
pub const TOKEN_SUB_ASSIGN: &str = "-=";
pub const TOKEN_AND_ASSIGN: &str = "&=";
pub const TOKEN_OR_ASSIGN: &str = "|=";
pub const TOKEN_XOR_ASSIGN: &str = "^=";
pub const TOKEN_LEFT_SHIFT_ASSIGN: &str = "<<=";
pub const TOKEN_RIGHT_SHIFT_ASSIGN: &str = ">>=";
pub const TOKEN_MUL_ASSIGN: &str = "*=";
pub const TOKEN_DIV_ASSIGN: &str = "/=";
pub const TOKEN_MOD_ASSIGN: &str = "%=";
pub const TOKEN_INC: &str = "++";
pub const TOKEN_DEC: &str = "--";
pub const TOKEN_COMMA: &str = ",";
pub const TOKEN_DOT: &str = ".";
pub const TOKEN_COLON: &str = ":";
pub const TOKEN_SEMICOLON: &str = ";";
pub const TOKEN_ASSIGN: &str = "=";
pub const TOKEN_LEFT_BRACKETS: &str = "(";
pub const TOKEN_RIGHT_BRACKETS: &str = ")";
pub const TOKEN_LEFT_SQUARE_BRACKETS: &str = "[";
pub const TOKEN_RIGHT_SQUARE_BRACKETS: &str = "]";
pub const TOKEN_LEFT_CURLY_BRACKETS: &str = "{";
pub const TOKEN_RIGHT_CURLY_BRACKETS: &str = "}";
pub const TOKEN_NOT: &str = "!";
pub const TOKEN_QM: &str = "?";
pub const TOKEN_LT: &str = "<";
pub const TOKEN_GT: &str = ">";
pub const TOKEN_ADD: &str = "+";
pub const TOKEN_SUB: &str = "-";
pub const TOKEN_MUL: &str = "*";
pub const TOKEN_DIV: &str = "/";
pub const TOKEN_MOD: &str = "%";
pub const TOKEN_BAND: &str = "&";
pub const TOKEN_BOR: &str = "|";
pub const TOKEN_XOR: &str = "^";
pub const TOKEN_HASH_TAG: &str = "#";

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    TokenEof {},
    TokenLastChar {},

    TokenFloat { value: f64 },
    TokenInt { value: Int },
    TokenStr { value: String },
    TokenChar { value: char },
    TokenName { name: String },

    TokenLeftShift {},
    TokenRightShift {},

    TokenEqual {},
    TokenNotEqual {},
    TokenLessThanEqual {},
    TokenGreaterThanEqual {},

    TokenAnd {},
    TokenOr {},

    TokenColonAssign {},

    TokenAddAssign {},
    TokenSubAssign {},

    TokenAndAssign {},
    TokenOrAssign {},
    TokenXorAssign {},

    TokenLeftShiftAssign {},
    TokenRightShiftAssign {},

    TokenMulAssign {},
    TokenDivAssign {},
    TokenModAssign {},

    TokenInc {},
    TokenDec {},

    TokenPound {},
    TokenKeyword { keyword: Keyword },
    TokenComma {},
    TokenDot {},

    TokenNot {},

    TokenColon {},
    TokenSemiColon {},

    TokenAssign {},
    TokenLeftBrackets {},
    TokenRightBrackets {},

    TokenLeftSquareBrackets {},
    TokenRightSquareBrackets {},

    TokenLeftCurlyBrackets {},
    TokenRightCurlyBrackets {},

    TokenQuestionMark {},

    TokenLessThan {},
    TokenGreaterThan {},

    TokenAdd {},
    TokenSub {},
    TokenMul {},
    TokenDiv {},

    TokenMod {},
    TokenBand {},
    TokenBor {},
    TokenXor {},

    TokenHashTag {},
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            TokenEof {} => {
                return f.write_str(&format!("EOF"));
            }
            TokenLastChar {} => {
                return f.write_str(&format!("Char"));
            }
            TokenFloat { ref value } => {
                return f.write_str(&format!("Float"));
            }
            TokenInt { ref value } => {
                return f.write_str(&format!("Int"));
            }
            TokenStr { ref value } => {
                return f.write_str(&format!("String \"{}\"", value));
            }
            TokenChar { ref value } => {
                return f.write_str(&format!("Char '{}'", value));
            }
            TokenName { ref name } => {
                return f.write_str(&format!("Name {}", name));
            }
            TokenLeftShift {} => {
                return f.write_str(&format!("{}", TOKEN_LEFT_SHIFT));
            }
            TokenRightShift {} => {
                return f.write_str(&format!("{}", TOKEN_RIGHT_SHIFT));
            }
            TokenEqual {} => {
                return f.write_str(&format!("{}", TOKEN_EQ));
            }
            TokenNotEqual {} => {
                return f.write_str(&format!("{}", TOKEN_NOT_EQ));
            }
            TokenLessThanEqual {} => {
                return f.write_str(&format!("{}", TOKEN_LESS_THAN_EQ));
            }
            TokenGreaterThanEqual {} => {
                return f.write_str(&format!("{}", TOKEN_GREATER_THAN_EQ));
            }
            TokenAnd {} => {
                return f.write_str(&format!("{}", TOKEN_AND));
            }
            TokenOr {} => {
                return f.write_str(&format!("{}", TOKEN_OR));
            }
            TokenColonAssign {} => {
                return f.write_str(&format!("{}", TOKEN_COLON_ASSIGN));
            }
            TokenAddAssign {} => {
                return f.write_str(&format!("{}", TOKEN_ADD_ASSIGN));
            }
            TokenSubAssign {} => {
                return f.write_str(&format!("{}", TOKEN_SUB_ASSIGN));
            }
            TokenAndAssign {} => {
                return f.write_str(&format!("{}", TOKEN_AND_ASSIGN));
            }
            TokenOrAssign {} => {
                return f.write_str(&format!("{}", TOKEN_OR_ASSIGN));
            }
            TokenXorAssign {} => {
                return f.write_str(&format!("{}", TOKEN_XOR_ASSIGN));
            }
            TokenLeftShiftAssign {} => {
                return f.write_str(&format!("{}", TOKEN_LEFT_SHIFT_ASSIGN));
            }
            TokenRightShiftAssign {} => {
                return f.write_str(&format!("{}", TOKEN_RIGHT_SHIFT_ASSIGN));
            }
            TokenMulAssign {} => {
                return f.write_str(&format!("{}", TOKEN_MUL_ASSIGN));
            }
            TokenDivAssign {} => {
                return f.write_str(&format!("{}", TOKEN_DIV_ASSIGN));
            }
            TokenModAssign {} => {
                return f.write_str(&format!("{}", TOKEN_MOD_ASSIGN));
            }
            TokenInc {} => {
                return f.write_str(&format!("{}", TOKEN_INC));
            }
            TokenDec {} => {
                return f.write_str(&format!("{}", TOKEN_DEC));
            }
            TokenPound {} => {
                return f.write_str(&format!(""));
            }
            TokenKeyword { ref keyword } => {
                return f.write_str(&format!("{}", keyword));
            }
            TokenComma {} => {
                return f.write_str(&format!("{}", TOKEN_COMMA));
            }
            TokenDot {} => {
                return f.write_str(&format!("{}", TOKEN_DOT));
            }
            TokenNot {} => {
                return f.write_str(&format!("{}", TOKEN_NOT));
            }
            TokenColon {} => {
                return f.write_str(&format!("{}", TOKEN_COLON));
            }
            TokenSemiColon {} => {
                return f.write_str(&format!("{}", TOKEN_SEMICOLON));
            }
            TokenAssign {} => {
                return f.write_str(&format!("{}", TOKEN_ASSIGN));
            }
            TokenLeftBrackets {} => {
                return f.write_str(&format!("{}", TOKEN_LEFT_BRACKETS));
            }
            TokenRightBrackets {} => {
                return f.write_str(&format!("{}", TOKEN_RIGHT_BRACKETS));
            }
            TokenLeftSquareBrackets {} => {
                return f.write_str(&format!("{}", TOKEN_LEFT_SQUARE_BRACKETS));
            }
            TokenRightSquareBrackets {} => {
                return f.write_str(&format!("{}", TOKEN_RIGHT_SQUARE_BRACKETS));
            }
            TokenLeftCurlyBrackets {} => {
                return f.write_str(&format!("{}", TOKEN_LEFT_CURLY_BRACKETS));
            }
            TokenRightCurlyBrackets {} => {
                return f.write_str(&format!("{}", TOKEN_RIGHT_CURLY_BRACKETS));
            }
            TokenQuestionMark {} => {
                return f.write_str(&format!("{}", TOKEN_QM));
            }
            TokenLessThan {} => {
                return f.write_str(&format!("{}", TOKEN_LT));
            }
            TokenGreaterThan {} => {
                return f.write_str(&format!("{}", TOKEN_GT));
            }
            TokenAdd {} => {
                return f.write_str(&format!("{}", TOKEN_ADD));
            }
            TokenSub {} => {
                return f.write_str(&format!("{}", TOKEN_SUB));
            }
            TokenMul {} => {
                return f.write_str(&format!("{}", TOKEN_MUL));
            }
            TokenDiv {} => {
                return f.write_str(&format!("{}", TOKEN_DIV));
            }
            TokenMod {} => {
                return f.write_str(&format!("{}", TOKEN_MOD));
            }
            TokenBand {} => {
                return f.write_str(&format!("{}", TOKEN_BAND));
            }
            TokenBor {} => {
                return f.write_str(&format!("{}", TOKEN_BOR));
            }
            TokenXor {} => {
                return f.write_str(&format!("{}", TOKEN_XOR));
            }
            TokenHashTag {} => {
                return f.write_str(&format!("{}", TOKEN_HASH_TAG));
            }
            _ => f.write_str(&format!("Unknown")),
        }
    }
}
