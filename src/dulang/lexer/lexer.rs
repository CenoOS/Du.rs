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

    fn scan_int(&mut self, str: &mut String) -> Result<Token, &'static str> {
        return Ok(Token::TokenInt {});
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
                return Ok(TokenName { name });
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
}
