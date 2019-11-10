/*
 * Copyright (c) 2019. NeroYang
 */

use crate::dulang::lexer::int::Int;
use crate::dulang::lexer::int::Int::{IntBin, IntHex, IntOct};
use crate::dulang::lexer::keyword::Keyword::{
    KeywordBreak, KeywordCase, KeywordConst, KeywordContinue, KeywordDefault, KeywordDo,
    KeywordElse, KeywordEnum, KeywordFor, KeywordFunc, KeywordGoto, KeywordIf, KeywordImport,
    KeywordReturn, KeywordSizeOf, KeywordStruct, KeywordSwitch, KeywordTypeDef, KeywordTypeOf,
    KeywordVar, KeywordWhile,
};
use crate::dulang::lexer::keyword::{
    Keyword, KEYWORD_BREAK, KEYWORD_CASE, KEYWORD_CONST, KEYWORD_CONTINUE, KEYWORD_DEFAULT,
    KEYWORD_DO, KEYWORD_ELSE, KEYWORD_ENUM, KEYWORD_FOR, KEYWORD_FUNC, KEYWORD_GOTO, KEYWORD_IF,
    KEYWORD_IMPORT, KEYWORD_RETURN, KEYWORD_SIZEOF, KEYWORD_STRUCT, KEYWORD_SWITCH, KEYWORD_TYPEOF,
    KEYWORD_TYPE_DEF, KEYWORD_VAR, KEYWORD_WHILE,
};
use crate::dulang::lexer::token::Token;
use crate::dulang::lexer::token::Token::{TokenKeyword, TokenLeftShift, TokenName};
use core::fmt::Alignment::Left;
use std::i32;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    char_stream: Peekable<Chars<'a>>,
    current_line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer {
        Lexer {
            char_stream: source.chars().peekable(),
            current_line: 0,
        }
    }

    fn scan_char(&mut self) -> Result<Token, &'static str> {
        let value: char;
        self.char_stream.next();
        if self.char_stream.peek().unwrap().to_ascii_lowercase() == '\'' {
            return Err("SyntaxError: Char literal cannot be empty");
        } else if self.char_stream.peek().unwrap().to_ascii_lowercase() == '\n' {
            return Err("SyntaxError: Char literal cannot contain newline");
        } else if self.char_stream.peek().unwrap().to_ascii_lowercase() == '\\' {
            self.char_stream.next();
            if Lexer::escape_to_char(self.char_stream.peek().unwrap()) == 0 as char
                || self.char_stream.peek().unwrap().to_ascii_lowercase() == '0'
            {
                return Err("SyntaxError: Invalid char literal escape");
            }
        }
        value = *self.char_stream.peek().unwrap();
        self.char_stream.next();
        if self.char_stream.peek().unwrap().to_ascii_lowercase() != '\'' {
            return Err("SyntaxError: Expected closing char quote");
        } else {
            self.char_stream.next();
        }
        return Ok(Token::TokenChar { value });
    }

    fn scan_str(&mut self) -> Result<Token, &'static str> {
        let mut value = String::from("");
        self.char_stream.next();
        while self.char_stream.peek().unwrap().to_ascii_lowercase() != '\"' {
            if self.char_stream.peek().unwrap().to_ascii_lowercase() == '\n' {
                return Err("SyntaxError: String literal cannot contain newline");
            } else if self.char_stream.peek().unwrap().to_ascii_lowercase() == '\\' {
                self.char_stream.next();
                let val = Lexer::escape_to_char(self.char_stream.peek().unwrap());
                if val == 0 as char || self.char_stream.peek().unwrap().to_ascii_lowercase() == '0'
                {
                    return Err("SyntaxError: Invalid string literal escape");
                }
            }
            value.push(*self.char_stream.peek().unwrap());
            self.char_stream.next();
        }
        self.char_stream.next();
        return Ok(Token::TokenStr { value });
    }

    fn scan_float(&mut self, value: &mut String) -> Result<Token, &'static str> {
        self.char_stream.next();
        while Lexer::is_digit(self.char_stream.peek().unwrap()) {
            value.push(self.char_stream.peek().unwrap().to_ascii_lowercase());
            self.char_stream.next();
        }
        if self.char_stream.peek().unwrap().to_ascii_lowercase() == '.' {
            value.push(self.char_stream.peek().unwrap().to_ascii_lowercase());
            self.char_stream.next();
        }
        while Lexer::is_digit(self.char_stream.peek().unwrap()) {
            value.push(self.char_stream.peek().unwrap().to_ascii_lowercase());
            self.char_stream.next();
        }
        if self.char_stream.peek().unwrap().to_ascii_lowercase() == 'e' {
            value.push(self.char_stream.peek().unwrap().to_ascii_lowercase());
            self.char_stream.next();
            if self.char_stream.peek().unwrap().to_ascii_lowercase() == '+'
                || self.char_stream.peek().unwrap().to_ascii_lowercase() == '-'
            {
                value.push(self.char_stream.peek().unwrap().to_ascii_lowercase());
                self.char_stream.next();
            }
            if !Lexer::is_digit(self.char_stream.peek().unwrap()) {
                return Err("SyntaxError: Expected digit after float literal exponent");
            }
            while Lexer::is_digit(self.char_stream.peek().unwrap()) {
                value.push(self.char_stream.peek().unwrap().to_ascii_lowercase());
                self.char_stream.next();
            }
        }
        let double_val = value.parse::<f64>().unwrap();

        return Ok(Token::TokenFloat { value: double_val });
    }

    fn scan_int(&mut self, value: &mut String, mode: bool) -> Result<Token, &'static str> {
        let mut integer: Int = IntOct { value: 0 };
        let mut int_val = 0;
        if value == "0x" {
            self.char_stream.next();
            while self.char_stream.peek().is_some()
                && (Lexer::is_digit(self.char_stream.peek().unwrap())
                    || Lexer::is_hex_char(self.char_stream.peek().unwrap()))
            {
                value.push(self.char_stream.peek().unwrap().to_ascii_lowercase());
                self.char_stream.next();
            }
            int_val = i32::from_str_radix(&value[2..], 16).unwrap();
            integer = IntHex { value: int_val };
        } else if value == "0b" {
            self.char_stream.next();
            while self.char_stream.peek().is_some()
                && Lexer::is_digit(self.char_stream.peek().unwrap())
            {
                value.push(self.char_stream.peek().unwrap().to_ascii_lowercase());
                self.char_stream.next();
            }
            int_val = i32::from_str_radix(&value[2..], 2).unwrap();
            integer = IntBin { value: int_val };
        } else {
            if mode {
                value.pop();
            }
            int_val = value.parse::<i32>().unwrap();
            integer = IntOct { value: int_val };
        }
        return Ok(Token::TokenInt { value: integer });
    }

    pub fn next_token(&mut self) -> Result<Token, &'static str> {
        if self.char_stream.peek().is_some() {
            match self.char_stream.peek() {
                Some(' ') | Some('\n') | Some('\r') | Some('\t') => {
                    while Lexer::is_space(self.char_stream.peek().unwrap()) {
                        self.char_stream.next();
                    }
                    return self.next_token();
                }
                Some('\'') => {
                    return self.scan_char();
                }
                Some('"') => {
                    return self.scan_str();
                }
                Some('.') => {
                    return self.scan_float(&mut "0.".to_string());
                }
                Some('0') | Some('1') | Some('2') | Some('3') | Some('4') | Some('5')
                | Some('6') | Some('7') | Some('8') | Some('9') => {
                    let mut value = String::from("");
                    while self.char_stream.peek().is_some()
                        && Lexer::is_digit(self.char_stream.peek().unwrap())
                    {
                        value.push(self.char_stream.peek().unwrap().to_ascii_lowercase());
                        self.char_stream.next();
                    }
                    if self.char_stream.peek().is_some() {
                        if self.char_stream.peek().unwrap().to_ascii_lowercase() == '.'
                            || self.char_stream.peek().unwrap().to_ascii_lowercase() == 'e'
                        {
                            value.push(self.char_stream.peek().unwrap().to_ascii_lowercase());
                            return self.scan_float(&mut value);
                        } else {
                            value.push(self.char_stream.peek().unwrap().to_ascii_lowercase());
                            return self.scan_int(&mut value, true);
                        }
                    } else {
                        return self.scan_int(&mut value, false);
                    }
                }
                Some('a') | Some('b') | Some('c') | Some('d') | Some('e') | Some('f')
                | Some('g') | Some('h') | Some('i') | Some('j') | Some('k') | Some('l')
                | Some('m') | Some('n') | Some('o') | Some('p') | Some('q') | Some('r')
                | Some('s') | Some('t') | Some('u') | Some('v') | Some('w') | Some('x')
                | Some('y') | Some('z') | Some('A') | Some('B') | Some('C') | Some('D')
                | Some('E') | Some('F') | Some('G') | Some('H') | Some('I') | Some('J')
                | Some('K') | Some('L') | Some('M') | Some('N') | Some('O') | Some('P')
                | Some('Q') | Some('R') | Some('S') | Some('T') | Some('U') | Some('V')
                | Some('W') | Some('X') | Some('Y') | Some('Z') | Some('_') => {
                    let mut name = String::from("");
                    while self.char_stream.peek().is_some()
                        && (Lexer::is_al_num(self.char_stream.peek().unwrap())
                            || self.char_stream.peek().unwrap().to_ascii_lowercase() == '_')
                    {
                        name.push(*self.char_stream.peek().unwrap());
                        self.char_stream.next();
                    }
                    let keyword = Lexer::to_keyword(&name);
                    match keyword {
                        Some(k) => {
                            return Ok(TokenKeyword { keyword: k });
                        }
                        None => {
                            return Ok(TokenName { name });
                        }
                    }
                }
                Some('<') => {
                    self.char_stream.next();
                    if self.char_stream.peek().unwrap().to_ascii_lowercase() == '<' {
                        self.char_stream.next();
                        if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                            self.char_stream.next();
                            return Ok(Token::TokenLeftShiftAssign {});
                        }
                        return Ok(Token::TokenLeftShift {});
                    } else if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                        self.char_stream.next();
                        return Ok(Token::TokenLessThanEqual {});
                    }
                    return Ok(Token::TokenLessThan {});
                }
                Some('>') => {
                    self.char_stream.next();
                    if self.char_stream.peek().unwrap().to_ascii_lowercase() == '>' {
                        self.char_stream.next();
                        if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                            self.char_stream.next();
                            return Ok(Token::TokenRightShiftAssign {});
                        }
                        return Ok(Token::TokenRightShift {});
                    } else if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                        self.char_stream.next();
                        return Ok(Token::TokenGreaterThanEqual {});
                    }
                    return Ok(Token::TokenGreaterThan {});
                }
                Some('(') => {
                    self.char_stream.next();
                    return Ok(Token::TokenLeftBrackets {});
                }
                Some(')') => {
                    self.char_stream.next();
                    return Ok(Token::TokenRightBrackets {});
                }
                Some('{') => {
                    self.char_stream.next();
                    return Ok(Token::TokenLeftCurlyBrackets {});
                }
                Some('}') => {
                    self.char_stream.next();
                    return Ok(Token::TokenRightCurlyBrackets {});
                }
                Some('[') => {
                    self.char_stream.next();
                    return Ok(Token::TokenLeftSquareBrackets {});
                }
                Some(']') => {
                    self.char_stream.next();
                    return Ok(Token::TokenRightSquareBrackets {});
                }
                Some(',') => {
                    self.char_stream.next();
                    return Ok(Token::TokenComma {});
                }
                Some('#') => {
                    self.char_stream.next();
                    return Ok(Token::TokenHashTag {});
                }
                Some('?') => {
                    self.char_stream.next();
                    return Ok(Token::TokenQuestionMark {});
                }
                Some(';') => {
                    self.char_stream.next();
                    return Ok(Token::TokenSemiColon {});
                }
                Some('!') => {
                    self.char_stream.next();
                    if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                        self.char_stream.next();
                        return Ok(Token::TokenNotEqual {});
                    }
                    return Ok(Token::TokenNot {});
                }
                Some(':') => {
                    self.char_stream.next();
                    if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                        self.char_stream.next();
                        return Ok(Token::TokenColonAssign {});
                    }
                    return Ok(Token::TokenColon {});
                }
                Some('=') => {
                    self.char_stream.next();
                    if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                        self.char_stream.next();
                        return Ok(Token::TokenEqual {});
                    }
                    return Ok(Token::TokenAssign {});
                }
                Some('^') => {
                    self.char_stream.next();
                    if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                        self.char_stream.next();
                        return Ok(Token::TokenXorAssign {});
                    }
                    return Ok(Token::TokenXor {});
                }
                Some('*') => {
                    self.char_stream.next();
                    if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                        self.char_stream.next();
                        return Ok(Token::TokenMulAssign {});
                    }
                    return Ok(Token::TokenMul {});
                }
                Some('%') => {
                    self.char_stream.next();
                    if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                        self.char_stream.next();
                        return Ok(Token::TokenModAssign {});
                    }
                    return Ok(Token::TokenMod {});
                }
                Some('+') => {
                    self.char_stream.next();
                    if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                        self.char_stream.next();
                        return Ok(Token::TokenAddAssign {});
                    } else if self.char_stream.peek().unwrap().to_ascii_lowercase() == '+' {
                        self.char_stream.next();
                        return Ok(Token::TokenInc {});
                    }
                    return Ok(Token::TokenAdd {});
                }
                Some('-') => {
                    self.char_stream.next();
                    if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                        self.char_stream.next();
                        return Ok(Token::TokenSubAssign {});
                    } else if self.char_stream.peek().unwrap().to_ascii_lowercase() == '-' {
                        self.char_stream.next();
                        return Ok(Token::TokenDec {});
                    }
                    return Ok(Token::TokenSub {});
                }
                Some('&') => {
                    self.char_stream.next();
                    if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                        self.char_stream.next();
                        return Ok(Token::TokenAndAssign {});
                    } else if self.char_stream.peek().unwrap().to_ascii_lowercase() == '&' {
                        self.char_stream.next();
                        return Ok(Token::TokenAnd {});
                    }
                    return Ok(Token::TokenBand {});
                }
                Some('|') => {
                    self.char_stream.next();
                    if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                        self.char_stream.next();
                        return Ok(Token::TokenOrAssign {});
                    } else if self.char_stream.peek().unwrap().to_ascii_lowercase() == '|' {
                        self.char_stream.next();
                        return Ok(Token::TokenOr {});
                    }
                    return Ok(Token::TokenBor {});
                }
                _ => Err(""),
            }
        } else {
            return Ok(Token::TokenEof {});
        }
    }

    fn to_keyword(name: &String) -> Option<Keyword> {
        match name.as_str() {
            KEYWORD_TYPE_DEF => {
                return Some(KeywordTypeDef {
                    name: KEYWORD_TYPE_DEF.to_string(),
                });
            }
            KEYWORD_ENUM => {
                return Some(KeywordEnum {
                    name: KEYWORD_ENUM.to_string(),
                });
            }
            KEYWORD_STRUCT => {
                return Some(KeywordStruct {
                    name: KEYWORD_STRUCT.to_string(),
                });
            }
            KEYWORD_CONST => {
                return Some(KeywordConst {
                    name: KEYWORD_CONST.to_string(),
                });
            }
            KEYWORD_VAR => {
                return Some(KeywordVar {
                    name: KEYWORD_VAR.to_string(),
                });
            }
            KEYWORD_FUNC => {
                return Some(KeywordFunc {
                    name: KEYWORD_FUNC.to_string(),
                });
            }
            KEYWORD_IMPORT => {
                return Some(KeywordImport {
                    name: KEYWORD_IMPORT.to_string(),
                });
            }
            KEYWORD_GOTO => {
                return Some(KeywordGoto {
                    name: KEYWORD_GOTO.to_string(),
                });
            }
            KEYWORD_SIZEOF => {
                return Some(KeywordSizeOf {
                    name: KEYWORD_SIZEOF.to_string(),
                });
            }
            KEYWORD_TYPEOF => {
                return Some(KeywordTypeOf {
                    name: KEYWORD_TYPEOF.to_string(),
                });
            }
            KEYWORD_BREAK => {
                return Some(KeywordBreak {
                    name: KEYWORD_BREAK.to_string(),
                });
            }
            KEYWORD_CONTINUE => {
                return Some(KeywordContinue {
                    name: KEYWORD_CONTINUE.to_string(),
                });
            }
            KEYWORD_RETURN => {
                return Some(KeywordReturn {
                    name: KEYWORD_RETURN.to_string(),
                });
            }
            KEYWORD_IF => {
                return Some(KeywordIf {
                    name: KEYWORD_IF.to_string(),
                });
            }
            KEYWORD_ELSE => {
                return Some(KeywordElse {
                    name: KEYWORD_ELSE.to_string(),
                });
            }
            KEYWORD_WHILE => {
                return Some(KeywordWhile {
                    name: KEYWORD_WHILE.to_string(),
                });
            }
            KEYWORD_DO => {
                return Some(KeywordDo {
                    name: KEYWORD_DO.to_string(),
                });
            }
            KEYWORD_FOR => {
                return Some(KeywordFor {
                    name: KEYWORD_FOR.to_string(),
                });
            }
            KEYWORD_SWITCH => {
                return Some(KeywordSwitch {
                    name: KEYWORD_SWITCH.to_string(),
                });
            }
            KEYWORD_CASE => {
                return Some(KeywordCase {
                    name: KEYWORD_CASE.to_string(),
                });
            }
            KEYWORD_DEFAULT => {
                return Some(KeywordDefault {
                    name: KEYWORD_DEFAULT.to_string(),
                });
            }
            _ => {
                return None;
            }
        }
    }

    pub(crate) fn is_digit(c: &char) -> bool {
        return *c >= '0' && *c <= '9';
    }

    pub(crate) fn is_alpha(c: &char) -> bool {
        return (*c >= 'a' && *c <= 'z') || (*c >= 'A' && *c <= 'Z');
    }
    pub(crate) fn is_al_num(c: &char) -> bool {
        return Lexer::is_digit(c) || Lexer::is_alpha(c);
    }
    fn is_space(c: &char) -> bool {
        return *c == ' ' || *c == '\n' || *c == '\r' || *c == '\t';
    }

    fn escape_to_char(c: &char) -> char {
        match *c {
            'n' => {
                return '\n';
            }
            'r' => {
                return '\r';
            }
            't' => {
                return '\t';
            }
            _ => {
                return *c;
            }
        }
    }

    fn is_hex_char(c: &char) -> bool {
        return (*c >= 'a' && *c <= 'f') || (*c >= 'A' && *c <= 'F');
    }

    pub(crate) fn hex_char_to_digit(c: &char) -> u8 {
        match *c {
            '0' => {
                return 0;
            }
            '1' => {
                return 1;
            }
            '2' => {
                return 2;
            }
            '3' => {
                return 3;
            }
            '4' => {
                return 4;
            }
            '5' => {
                return 5;
            }
            '6' => {
                return 6;
            }
            '7' => {
                return 7;
            }
            '8' => {
                return 8;
            }
            '9' => {
                return 9;
            }
            'A' | 'a' => {
                return 10;
            }
            'B' | 'b' => {
                return 11;
            }
            'C' | 'c' => {
                return 12;
            }
            'D' | 'd' => {
                return 13;
            }
            'E' | 'e' => {
                return 14;
            }
            'F' | 'f' => {
                return 15;
            }
            _ => {
                return *c as u8;
            }
        }
    }
}
