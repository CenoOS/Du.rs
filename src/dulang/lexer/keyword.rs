/*
 * Copyright (c) 2019. NeroYang
 */

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
