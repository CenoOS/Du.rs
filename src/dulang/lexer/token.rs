/*
 * Copyright (c) 2019. NeroYang
 */

use crate::dulang::lexer::int::Int;
use crate::dulang::lexer::keyword::Keyword;

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
    TokenInt { int: Int },
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
