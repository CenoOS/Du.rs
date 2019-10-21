/*
 * Copyright (c) 2019. NeroYang
 */

use std::iter::Peekable;
use std::str::Chars;
use core::fmt::Alignment::Left;
use crate::dulang::lexer::token::Token;
use crate::dulang::lexer::token::Token::{TokenName, TokenLeftShift};


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

    fn scan_char(&mut self) -> Token {
        return Token::TokenChar {};
    }
    fn scan_str(&mut self) -> Token {
        return Token::TokenStr {};
    }
    fn scan_float(&mut self) -> Token {
        return Token::TokenFloat {};
    }
    fn scan_int(&mut self) -> Token {
        return Token::TokenInt {};
    }

    fn next_token(mut self) -> Result<Token, &'static str> {
        match self.char_stream.peek() {
            Some(' ') | Some('\n') | Some('\r') | Some('\t') => {
                while Lexer::is_space(self.char_stream.peek().unwrap()) {
                    self.char_stream.next();
                }
                return self.next_token();
            }
            Some('\'') => {
                return Ok(self.scan_char());
            }
            Some('"') => {
                return Ok(self.scan_str());
            }
            Some('.') => {
                return Ok(self.scan_float());
            }
            Some('0') | Some('1') | Some('2') | Some('3') | Some('4') | Some('5') | Some('6') |
            Some('7') | Some('8') | Some('9') => {
                while Lexer::is_digit(self.char_stream.peek().unwrap()) {
                    self.char_stream.next();
                }
                if self.char_stream.peek().unwrap().to_ascii_lowercase() == '.' || self.char_stream.peek().unwrap().to_ascii_lowercase() == 'e' {
                    return Ok(self.scan_float());
                } else {
                    return Ok(self.scan_int());
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
                while Lexer::is_al_num(self.char_stream.peek().unwrap()) || self.char_stream.peek().unwrap().to_ascii_lowercase() == '_' {
                    self.char_stream.next();
                }
                return Ok(TokenName {});
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
                if self.char_stream.peek().unwrap().to_ascii_lowercase() == '<' {
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
                }
                return Ok(Token::TokenAdd {});
            }
            Some('-') => {
                self.char_stream.next();
                if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                    return Ok(Token::TokenSubAssign {});
                }
                return Ok(Token::TokenSub {});
            }
            Some('&') => {
                self.char_stream.next();
                if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                    return Ok(Token::TokenAndAssign {});
                }
                return Ok(Token::TokenAnd {});
            }
            Some('|') => {
                self.char_stream.next();
                if self.char_stream.peek().unwrap().to_ascii_lowercase() == '=' {
                    return Ok(Token::TokenOrAssign {});
                }
                return Ok(Token::TokenOr {});
            }
            _ => { Err("") }
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
}
