/*
 * Copyright (c) 2019. NeroYang
 */

use crate::dulang::lexer::keyword::Keyword::{
    KeywordBreak, KeywordCase, KeywordConst, KeywordContinue, KeywordDefault, KeywordDo,
    KeywordElse, KeywordEnum, KeywordFor, KeywordFunc, KeywordGoto, KeywordIf, KeywordImport,
    KeywordReturn, KeywordSizeOf, KeywordStruct, KeywordSwitch, KeywordTypeDef, KeywordTypeOf,
    KeywordVar, KeywordWhile,
};
use crate::dulang::lexer::token::Token::TokenEof;
use std::fmt;
use std::fmt::{Display, Formatter};

pub const KEYWORD_TYPE_DEF: &str = "typedef";
pub const KEYWORD_ENUM: &str = "enum";
pub const KEYWORD_STRUCT: &str = "struct";
pub const KEYWORD_CONST: &str = "const";
pub const KEYWORD_VAR: &str = "var";
pub const KEYWORD_FUNC: &str = "func";
pub const KEYWORD_IMPORT: &str = "import";
pub const KEYWORD_GOTO: &str = "goto";
pub const KEYWORD_SIZEOF: &str = "sizeof";
pub const KEYWORD_TYPEOF: &str = "typeof";
pub const KEYWORD_BREAK: &str = "break";
pub const KEYWORD_CONTINUE: &str = "continue";
pub const KEYWORD_RETURN: &str = "return";
pub const KEYWORD_IF: &str = "if";
pub const KEYWORD_ELSE: &str = "else";
pub const KEYWORD_WHILE: &str = "while";
pub const KEYWORD_DO: &str = "do";
pub const KEYWORD_FOR: &str = "for";
pub const KEYWORD_SWITCH: &str = "switch";
pub const KEYWORD_CASE: &str = "case";
pub const KEYWORD_DEFAULT: &str = "default";

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    KeywordTypeDef { name: String },
    KeywordEnum { name: String },
    KeywordStruct { name: String },
    KeywordVar { name: String },
    KeywordConst { name: String },
    KeywordFunc { name: String },
    KeywordSizeOf { name: String },
    KeywordTypeOf { name: String },
    KeywordBreak { name: String },
    KeywordContinue { name: String },
    KeywordReturn { name: String },

    KeywordIf { name: String },
    KeywordElse { name: String },
    KeywordWhile { name: String },
    KeywordDo { name: String },
    KeywordFor { name: String },
    KeywordSwitch { name: String },
    KeywordCase { name: String },
    KeywordDefault { name: String },
    KeywordImport { name: String },
    KeywordGoto { name: String },
}

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            KeywordTypeDef { ref name } => {
                return f.write_str(&format!("{}", KEYWORD_TYPE_DEF));
            }
            KeywordEnum { ref name } => {
                return f.write_str(&format!("{}", KEYWORD_ENUM));
            }
            KeywordStruct { ref name } => {
                return f.write_str(&format!("{}", KEYWORD_STRUCT));
            }
            KeywordVar { ref name } => {
                return f.write_str(&format!("{}", KEYWORD_VAR));
            }
            KeywordConst { ref name } => {
                return f.write_str(&format!("{}", KEYWORD_CONST));
            }
            KeywordFunc { ref name } => {
                return f.write_str(&format!("{}", KEYWORD_FUNC));
            }
            KeywordSizeOf { ref name } => {
                return f.write_str(&format!("{}", KEYWORD_SIZEOF));
            }
            KeywordTypeOf { ref name } => {
                return f.write_str(&format!("{}", KEYWORD_TYPEOF));
            }
            KeywordBreak { ref name } => {
                return f.write_str(&format!("{}", KEYWORD_BREAK));
            }
            KeywordContinue { ref name } => {
                return f.write_str(&format!("{}", KEYWORD_CONTINUE));
            }
            KeywordReturn { ref name } => {
                return f.write_str(&format!("{}", KEYWORD_RETURN));
            }
            KeywordIf { ref name } => {
                return f.write_str(&format!("{}", KEYWORD_IF));
            }
            KeywordElse { ref name } => {
                return f.write_str(&format!("{}", KEYWORD_ELSE));
            }
            KeywordWhile { ref name } => {
                return f.write_str(&format!("{}", KEYWORD_WHILE));
            }
            KeywordDo { ref name } => {
                return f.write_str(&format!("{}", KEYWORD_DO));
            }
            KeywordFor { ref name } => {
                return f.write_str(&format!("{}", KEYWORD_FOR));
            }
            KeywordSwitch { ref name } => {
                return f.write_str(&format!("{}", KEYWORD_SWITCH));
            }
            KeywordCase { ref name } => {
                return f.write_str(&format!("{}", KEYWORD_CASE));
            }
            KeywordDefault { ref name } => {
                return f.write_str(&format!("{}", KEYWORD_DEFAULT));
            }
            KeywordImport { ref name } => {
                return f.write_str(&format!("{}", KEYWORD_IMPORT));
            }
            KeywordGoto { ref name } => {
                return f.write_str(&format!("{}", KEYWORD_GOTO));
            }
            _ => {
                return f.write_str(&format!("Unknown"));
            }
        }
    }
}
