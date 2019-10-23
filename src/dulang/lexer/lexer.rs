/*
 * Copyright (c) 2019. NeroYang
 */

use std::iter::Peekable;
use std::str::Chars;
use core::fmt::Alignment::Left;
use std::i32;
use crate::dulang::lexer::token::Token;
use crate::dulang::lexer::token::Token::{TokenName, TokenLeftShift, TokenKeyword};
use crate::dulang::lexer::int::Int;
use crate::dulang::lexer::int::Int::{IntOct, IntHex, IntBin};
use crate::dulang::lexer::keyword::Keyword::{KeywordTypeDef, KeywordDefault, KeywordCase,
                                             KeywordSwitch, KeywordFor, KeywordDo, KeywordWhile,
                                             KeywordElse, KeywordIf, KeywordReturn, KeywordConst,
                                             KeywordContinue, KeywordBreak, KeywordTypeOf,
                                             KeywordSizeOf, KeywordGoto, KeywordImport, KeywordFunc,
                                             KeywordVar, KeywordStruct, KeywordEnum};
use crate::dulang::lexer::keyword::{Keyword, KEYWORD_TYPE_DEF, KEYWORD_ENUM, KEYWORD_STRUCT,
                                    KEYWORD_CONST, KEYWORD_VAR, KEYWORD_FUNC, KEYWORD_IMPORT,
                                    KEYWORD_GOTO, KEYWORD_SIZEOF, KEYWORD_TYPEOF, KEYWORD_BREAK,
                                    KEYWORD_CONTINUE, KEYWORD_RETURN, KEYWORD_IF, KEYWORD_ELSE,
                                    KEYWORD_WHILE, KEYWORD_DO, KEYWORD_FOR, KEYWORD_SWITCH,
                                    KEYWORD_CASE, KEYWORD_DEFAULT};


struct Lexer<'a> {
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
            if Lexer::escape_to_char(self.char_stream.peek().unwrap()) == 0 as char || self.char_stream.peek().unwrap().to_ascii_lowercase() == '0' {
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
                if val == 0 as char || self.char_stream.peek().unwrap().to_ascii_lowercase() == '0' {
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
            if self.char_stream.peek().unwrap().to_ascii_lowercase() == '+' || self.char_stream.peek().unwrap().to_ascii_lowercase() == '-' {
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
        let doubleVal = value.parse::<f64>().unwrap();

        return Ok(Token::TokenFloat { value: doubleVal });
    }

    fn scan_int(&mut self, value: &mut String) -> Result<Token, &'static str> {
        let mut integer: Int = IntOct { value: 0 };
        let mut intVal = 0;
        println!("{}#", value.trim());
        if value == "0x" {
            self.char_stream.next();
            while Lexer::is_digit(self.char_stream.peek().unwrap()) || Lexer::is_hex_char(self.char_stream.peek().unwrap()) {
                value.push(self.char_stream.peek().unwrap().to_ascii_lowercase());
                self.char_stream.next();
            }
            intVal = i32::from_str_radix(&value[2..], 16).unwrap();
            integer = IntHex { value: intVal };
        } else if value == "0b" {
            self.char_stream.next();
            while Lexer::is_digit(self.char_stream.peek().unwrap()) {
                value.push(self.char_stream.peek().unwrap().to_ascii_lowercase());
                self.char_stream.next();
            }
            intVal = i32::from_str_radix(&value[2..], 2).unwrap();
            integer = IntBin { value: intVal };
        } else {
            intVal = value.trim().parse::<i32>().unwrap();
            integer = IntOct { value: intVal };
        }
        return Ok(Token::TokenInt { int: integer });
    }

    fn next_token(&mut self) -> Result<Token, &'static str> {
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
            Some('0') | Some('1') | Some('2') | Some('3') | Some('4') | Some('5') | Some('6') |
            Some('7') | Some('8') | Some('9') => {
                let mut value = String::from("");
                while Lexer::is_digit(self.char_stream.peek().unwrap()) {
                    value.push(self.char_stream.peek().unwrap().to_ascii_lowercase());
                    self.char_stream.next();
                }
                if self.char_stream.peek().unwrap().to_ascii_lowercase() == '.' || self.char_stream.peek().unwrap().to_ascii_lowercase() == 'e' {
                    value.push(self.char_stream.peek().unwrap().to_ascii_lowercase());
                    return self.scan_float(&mut value);
                } else {
                    value.push(self.char_stream.peek().unwrap().to_ascii_lowercase());
                    return self.scan_int(&mut value);
                }
            }
            Some('a') | Some('b') | Some('c') | Some('d') | Some('e') | Some('f') | Some('g') |
            Some('h') | Some('i') | Some('j') | Some('k') | Some('l') | Some('m') | Some('n') |
            Some('o') | Some('p') | Some('q') | Some('r') | Some('s') | Some('t') | Some('u') |
            Some('v') | Some('w') | Some('x') | Some('y') | Some('z') |
            Some('A') | Some('B') | Some('C') | Some('D') | Some('E') | Some('F') | Some('G') |
            Some('H') | Some('I') | Some('J') | Some('K') | Some('L') | Some('M') | Some('N') |
            Some('O') | Some('P') | Some('Q') | Some('R') | Some('S') | Some('T') | Some('U') |
            Some('V') | Some('W') | Some('X') | Some('Y') | Some('Z') | Some('_') => {
                let mut name = String::from("");
                while Lexer::is_al_num(self.char_stream.peek().unwrap()) || self.char_stream.peek().unwrap().to_ascii_lowercase() == '_' {
                    name.push(*self.char_stream.peek().unwrap());
                    self.char_stream.next();
                }
                let keyword = Lexer::to_keyword(&name);
                match keyword {
                    Some(k) => { return Ok(TokenKeyword { keyword: k }); }
                    None => { return Ok(TokenName { name }); }
                }
            }
            Some('<') => {
                self.char_stream.next();
                if self.char_stream.peek().unwrap().to_ascii_lowercase() == '<' {
                    self.char_stream.next();
                    if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                        return Ok(Token::TokenLeftShiftAssign {});
                    }
                    return Ok(Token::TokenLeftShift {});
                } else if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                    return Ok(Token::TokenLessThanEqual {});
                }
                return Ok(Token::TokenLessThan {});
            }
            Some('>') => {
                self.char_stream.next();
                if self.char_stream.peek().unwrap().to_ascii_lowercase() == '>' {
                    self.char_stream.next();
                    if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                        return Ok(Token::TokenRightShiftAssign {});
                    }
                    return Ok(Token::TokenRightShift {});
                } else if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                    return Ok(Token::TokenGreaterThanEqual {});
                }
                return Ok(Token::TokenGreaterThan {});
            }
            Some('(') => {
                return Ok(Token::TokenLeftBrackets {});
            }
            Some(')') => {
                return Ok(Token::TokenRightBrackets {});
            }
            Some('{') => {
                return Ok(Token::TokenLeftCurlyBrackets {});
            }
            Some('}') => {
                return Ok(Token::TokenRightCurlyBrackets {});
            }
            Some('[') => {
                return Ok(Token::TokenLeftSquareBrackets {});
            }
            Some(']') => {
                return Ok(Token::TokenRightSquareBrackets {});
            }
            Some(',') => {
                return Ok(Token::TokenComma {});
            }
            Some('#') => {
                return Ok(Token::TokenHashTag {});
            }
            Some('?') => {
                return Ok(Token::TokenQuestionMark {});
            }
            Some(';') => {
                return Ok(Token::TokenSemiColon {});
            }
            Some('!') => {
                self.char_stream.next();
                if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                    return Ok(Token::TokenNotEqual {});
                }
                return Ok(Token::TokenNot {});
            }
            Some(':') => {
                self.char_stream.next();
                if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                    return Ok(Token::TokenColonAssign {});
                }
                return Ok(Token::TokenColon {});
            }
            Some('=') => {
                self.char_stream.next();
                if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                    return Ok(Token::TokenEqual {});
                }
                return Ok(Token::TokenAssign {});
            }
            Some('^') => {
                self.char_stream.next();
                if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                    return Ok(Token::TokenXorAssign {});
                }
                return Ok(Token::TokenXor {});
            }
            Some('*') => {
                self.char_stream.next();
                if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                    return Ok(Token::TokenMulAssign {});
                }
                return Ok(Token::TokenMul {});
            }
            Some('%') => {
                self.char_stream.next();
                if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                    return Ok(Token::TokenModAssign {});
                }
                return Ok(Token::TokenMod {});
            }
            Some('+') => {
                self.char_stream.next();
                if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                    return Ok(Token::TokenAddAssign {});
                } else if self.char_stream.peek().unwrap().to_ascii_lowercase() == '+' {
                    return Ok(Token::TokenInc {});
                }
                return Ok(Token::TokenAdd {});
            }
            Some('-') => {
                self.char_stream.next();
                if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                    return Ok(Token::TokenSubAssign {});
                } else if self.char_stream.peek().unwrap().to_ascii_lowercase() == '-' {
                    return Ok(Token::TokenDec {});
                }
                return Ok(Token::TokenSub {});
            }
            Some('&') => {
                self.char_stream.next();
                if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                    return Ok(Token::TokenAndAssign {});
                } else if self.char_stream.peek().unwrap().to_ascii_lowercase() == '&' {
                    return Ok(Token::TokenAnd {});
                }
                return Ok(Token::TokenBand {});
            }
            Some('|') => {
                self.char_stream.next();
                if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                    return Ok(Token::TokenOrAssign {});
                } else if self.char_stream.peek().unwrap().to_ascii_lowercase() == '|' {
                    return Ok(Token::TokenOr {});
                }
                return Ok(Token::TokenBor {});
            }
            _ => { Err("") }
        }
    }

    fn to_keyword(name: &String) -> Option<Keyword> {
        match name.as_str() {
            KEYWORD_TYPE_DEF => {
                return Some(KeywordTypeDef { name: KEYWORD_TYPE_DEF.to_string() });
            }
            KEYWORD_ENUM => {
                return Some(KeywordEnum { name: KEYWORD_ENUM.to_string() });
            }
            KEYWORD_STRUCT => {
                return Some(KeywordStruct { name: KEYWORD_STRUCT.to_string() });
            }
            KEYWORD_CONST => {
                return Some(KeywordConst { name: KEYWORD_CONST.to_string() });
            }
            KEYWORD_VAR => {
                return Some(KeywordVar { name: KEYWORD_VAR.to_string() });
            }
            KEYWORD_FUNC => {
                return Some(KeywordFunc { name: KEYWORD_FUNC.to_string() });
            }
            KEYWORD_IMPORT => {
                return Some(KeywordImport { name: KEYWORD_IMPORT.to_string() });
            }
            KEYWORD_GOTO => {
                return Some(KeywordGoto { name: KEYWORD_GOTO.to_string() });
            }
            KEYWORD_SIZEOF => {
                return Some(KeywordSizeOf { name: KEYWORD_SIZEOF.to_string() });
            }
            KEYWORD_TYPEOF => {
                return Some(KeywordTypeOf { name: KEYWORD_TYPEOF.to_string() });
            }
            KEYWORD_BREAK => {
                return Some(KeywordBreak { name: KEYWORD_BREAK.to_string() });
            }
            KEYWORD_CONTINUE => {
                return Some(KeywordContinue { name: KEYWORD_CONTINUE.to_string() });
            }
            KEYWORD_RETURN => {
                return Some(KeywordReturn { name: KEYWORD_RETURN.to_string() });
            }
            KEYWORD_IF => {
                return Some(KeywordIf { name: KEYWORD_IF.to_string() });
            }
            KEYWORD_ELSE => {
                return Some(KeywordElse { name: KEYWORD_ELSE.to_string() });
            }
            KEYWORD_WHILE => {
                return Some(KeywordWhile { name: KEYWORD_WHILE.to_string() });
            }
            KEYWORD_DO => {
                return Some(KeywordDo { name: KEYWORD_DO.to_string() });
            }
            KEYWORD_FOR => {
                return Some(KeywordFor { name: KEYWORD_FOR.to_string() });
            }
            KEYWORD_SWITCH => {
                return Some(KeywordSwitch { name: KEYWORD_SWITCH.to_string() });
            }
            KEYWORD_CASE => {
                return Some(KeywordCase { name: KEYWORD_CASE.to_string() });
            }
            KEYWORD_DEFAULT => {
                return Some(KeywordDefault { name: KEYWORD_DEFAULT.to_string() });
            }
            _ => {
                return None;
            }
        }
    }

    fn is_digit(c: &char) -> bool {
        return *c >= '0' && *c <= '9';
    }

    fn is_alpha(c: &char) -> bool {
        return (*c >= 'a' && *c <= 'z') || (*c >= 'A' && *c <= 'Z');
    }
    fn is_al_num(c: &char) -> bool {
        return Lexer::is_digit(c) || Lexer::is_alpha(c);
    }
    fn is_space(c: &char) -> bool {
        return *c == ' ' || *c == '\n' || *c == '\r' || *c == '\t';
    }

    fn escape_to_char(c: &char) -> char {
        match *c {
            'n' => { return '\n'; }
            'r' => { return '\r'; }
            't' => { return '\t'; }
            _ => { return *c; }
        }
    }

    fn is_hex_char(c: &char) -> bool {
        return (*c >= 'a' && *c <= 'f') || (*c >= 'A' && *c <= 'F');
    }

    fn hex_char_to_digit(c: &char) -> u8 {
        match *c {
            '0' => { return 0; }
            '1' => { return 1; }
            '2' => { return 2; }
            '3' => { return 3; }
            '4' => { return 4; }
            '5' => { return 5; }
            '6' => { return 6; }
            '7' => { return 7; }
            '8' => { return 8; }
            '9' => { return 9; }
            'A' | 'a' => { return 10; }
            'B' | 'b' => { return 11; }
            'C' | 'c' => { return 12; }
            'D' | 'd' => { return 13; }
            'E' | 'e' => { return 14; }
            'F' | 'f' => { return 15; }
            _ => { return *c as u8; }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_true_when_give_a_alpha() {
        for c in vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
                      'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
                      'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
                      'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'] {
            assert_eq!(Lexer::is_alpha(&c), true);
        }
    }

    #[test]
    fn should_return_true_when_give_a_number() {
        for c in vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] {
            assert_eq!(Lexer::is_digit(&c), true);
        }
    }

    #[test]
    fn should_return_true_when_give_a_number_or_alpha() {
        for c in vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
                      'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
                      'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
                      'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
                      '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] {
            assert_eq!(Lexer::is_al_num(&c), true);
        }
    }

    #[test]
    fn should_return_token_char() {
        let mut lexer = Lexer::new(" 'a' 'b' 'E' '1' '0'");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenChar {
            value: 'a'
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenChar {
            value: 'b'
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenChar {
            value: 'E'
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenChar {
            value: '1'
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenChar {
            value: '0'
        });
    }

    #[test]
    #[should_panic(expected = "SyntaxError: Char literal cannot contain newline")]
    fn should_throw_when_token_char_contains_new_line() {
        let mut lexer = Lexer::new(" '\n'");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenChar {
            value: '\n'
        });
    }

    #[test]
    fn should_return_token_str() {
        let mut lexer = Lexer::new("\"xxxx\" \"aaa\" \"111\" \"000000\" \"z\"");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenStr {
            value: "xxxx".to_string()
        });
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenStr {
            value: "aaa".to_string()
        });
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenStr {
            value: "111".to_string()
        });
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenStr {
            value: "000000".to_string()
        });
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenStr {
            value: "z".to_string()
        });
    }

    #[test]
    fn should_return_token_float() {
        let mut lexer = Lexer::new("1.324 .23 0.34 1.23e-1 1.22e+12 0.0 ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenFloat {
            value: 1.324
        });
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenFloat {
            value: 0.23
        });
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenFloat {
            value: 0.34
        });
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenFloat {
            value: 1.23e-1
        });
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenFloat {
            value: 1.22e+12
        });
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenFloat {
            value: 0.0
        });
    }

    #[test]
    fn should_return_u8_when_give_a_hex_char() {
        let mut result = vec![];
        for c in vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
                      'a', 'A', 'b', 'B', 'c', 'C', 'd', 'D', 'e', 'E', 'f', 'F'] {
            result.push(Lexer::hex_char_to_digit(&c));
        }
        assert_eq!(result, vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15
        ]);
    }

    #[test]
    fn should_return_token_int() {
        let mut lexer = Lexer::new("0xa 0b110 12345 0 321 ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenInt {
            int: IntHex { value: 10 },
        });
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenInt {
            int: IntBin { value: 6 },
        });
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenInt {
            int: IntOct { value: 12345 },
        });
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenInt {
            int: IntOct { value: 0 },
        });
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenInt {
            int: IntOct { value: 321 },
        });
    }

    #[test]
    fn should_return_token_keyword() {
        let mut lexer = Lexer::new("typedef enum struct const var func import goto \
        sizeof typeof \
        break continue return \
        if else while do for \
        switch case default ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenKeyword {
            keyword: KeywordTypeDef { name: "typedef".to_string() },
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenKeyword {
            keyword: KeywordEnum { name: "enum".to_string() },
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenKeyword {
            keyword: KeywordStruct { name: "struct".to_string() },
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenKeyword {
            keyword: KeywordConst { name: "const".to_string() },
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenKeyword {
            keyword: KeywordVar { name: "var".to_string() },
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenKeyword {
            keyword: KeywordFunc { name: "func".to_string() },
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenKeyword {
            keyword: KeywordImport { name: "import".to_string() },
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenKeyword {
            keyword: KeywordGoto { name: "goto".to_string() },
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenKeyword {
            keyword: KeywordSizeOf { name: "sizeof".to_string() },
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenKeyword {
            keyword: KeywordTypeOf { name: "typeof".to_string() },
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenKeyword {
            keyword: KeywordBreak { name: "break".to_string() },
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenKeyword {
            keyword: KeywordContinue { name: "continue".to_string() },
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenKeyword {
            keyword: KeywordReturn { name: "return".to_string() },
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenKeyword {
            keyword: KeywordIf { name: "if".to_string() },
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenKeyword {
            keyword: KeywordElse { name: "else".to_string() },
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenKeyword {
            keyword: KeywordWhile { name: "while".to_string() },
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenKeyword {
            keyword: KeywordDo { name: "do".to_string() },
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenKeyword {
            keyword: KeywordFor { name: "for".to_string() },
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenKeyword {
            keyword: KeywordSwitch { name: "switch".to_string() },
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenKeyword {
            keyword: KeywordCase { name: "case".to_string() },
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenKeyword {
            keyword: KeywordDefault { name: "default".to_string() },
        });
    }

    #[test]
    fn should_return_token_name() {
        let mut lexer = Lexer::new("name age _year address_detail phone_ email1 email2 ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenName {
            name: "name".to_string()
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenName {
            name: "age".to_string()
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenName {
            name: "_year".to_string()
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenName {
            name: "address_detail".to_string()
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenName {
            name: "phone_".to_string()
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenName {
            name: "email1".to_string()
        });

        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenName {
            name: "email2".to_string()
        });
    }

    #[test]
    fn should_return_token_less_than() {
        let mut lexer = Lexer::new("< ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenLessThan {});
    }

    #[test]
    fn should_return_token_less_than_eq() {
        let mut lexer = Lexer::new("<= ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenLessThanEqual {});
    }

    #[test]
    fn should_return_token_left_shift() {
        let mut lexer = Lexer::new("<< ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenLeftShift {});
    }

    #[test]
    fn should_return_token_left_shift_assign() {
        let mut lexer = Lexer::new("<<= ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenLeftShiftAssign {});
    }

    #[test]
    fn should_return_token_greater_than() {
        let mut lexer = Lexer::new("> ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenGreaterThan {});
    }

    #[test]
    fn should_return_token_greater_than_eq() {
        let mut lexer = Lexer::new(">= ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenGreaterThanEqual {});
    }

    #[test]
    fn should_return_token_right_shift() {
        let mut lexer = Lexer::new(">> ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenRightShift {});
    }

    #[test]
    fn should_return_token_right_shift_assign() {
        let mut lexer = Lexer::new(">>= ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenRightShiftAssign {});
    }

    #[test]
    fn should_return_token_not() {
        let mut lexer = Lexer::new("! ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenNot {});
    }

    #[test]
    fn should_return_token_not_equal() {
        let mut lexer = Lexer::new("!= ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenNotEqual {});
    }

    #[test]
    fn should_return_token_colon() {
        let mut lexer = Lexer::new(": ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenColon {});
    }

    #[test]
    fn should_return_token_colon_assign() {
        let mut lexer = Lexer::new(":= ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenColonAssign {});
    }

    #[test]
    fn should_return_token_xor() {
        let mut lexer = Lexer::new("^ ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenXor {});
    }

    #[test]
    fn should_return_token_xor_assign() {
        let mut lexer = Lexer::new("^= ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenXorAssign {});
    }

    #[test]
    fn should_return_token_mul() {
        let mut lexer = Lexer::new("* ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenMul {});
    }

    #[test]
    fn should_return_token_mul_assign() {
        let mut lexer = Lexer::new("*= ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenMulAssign {});
    }

    #[test]
    fn should_return_token_mod() {
        let mut lexer = Lexer::new("% ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenMod {});
    }

    #[test]
    fn should_return_token_mod_assign() {
        let mut lexer = Lexer::new("%= ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenModAssign {});
    }

    #[test]
    fn should_return_token_add() {
        let mut lexer = Lexer::new("+ ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenAdd {});
    }

    #[test]
    fn should_return_token_add_assign() {
        let mut lexer = Lexer::new("+= ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenAddAssign {});
    }

    #[test]
    fn should_return_token_inc_assign() {
        let mut lexer = Lexer::new("++ ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenInc {});
    }

    #[test]
    fn should_return_token_sub() {
        let mut lexer = Lexer::new("- ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenSub {});
    }

    #[test]
    fn should_return_token_sub_assign() {
        let mut lexer = Lexer::new("-= ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenSubAssign {});
    }

    #[test]
    fn should_return_token_dec_assign() {
        let mut lexer = Lexer::new("-- ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenDec {});
    }

    #[test]
    fn should_return_token_bor() {
        let mut lexer = Lexer::new("| ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenBor {});
    }

    #[test]
    fn should_return_token_bor_assign() {
        let mut lexer = Lexer::new("|= ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenOrAssign {});
    }

    #[test]
    fn should_return_token_or() {
        let mut lexer = Lexer::new("|| ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenOr {});
    }

    #[test]
    fn should_return_token_band() {
        let mut lexer = Lexer::new("& ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenBand {});
    }

    #[test]
    fn should_return_token_and_assign() {
        let mut lexer = Lexer::new("&= ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenAndAssign {});
    }

    #[test]
    fn should_return_token_and() {
        let mut lexer = Lexer::new("&& ");
        let tokenResult = lexer.next_token();
        assert_eq!(tokenResult.unwrap(), Token::TokenAnd {});
    }
}
