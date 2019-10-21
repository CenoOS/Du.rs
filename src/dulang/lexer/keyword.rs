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

pub enum Keyword {
    KeywordTypeDef {},
    KeywordEnum {},
    KeywordStruct {},
    KeywordVar {},
    KeywordConst {},
    KeywordFunc {},
    KeywordSizeOf {},
    KeywordTypeOf {},
    KeywordBreak {},
    KeywordContinue {},
    KeywordReturn {},

    KeywordIf {},
    KeywordElse {},
    KeywordWhile {},
    KeywordDo {},
    KeywordFor {},
    KeywordSwitch {},
    KeywordCase {},
    KeywordDefault {},
    KeywordImport {},
    KeywordGoto {},
}
