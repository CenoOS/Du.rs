/*
 * Copyright (c) 2019. NeroYang
 */

use std::iter::Peekable;
use std::str::Chars;
use core::fmt::Alignment::Left;


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

    fn scan_char(&mut self) {}
    fn scan_str(&mut self) {}
    fn scan_float(&mut self) {}
    fn scan_int(&mut self) {}

    fn next_token(mut self) {
        match self.char_stream.peek() {
            Some(' ') | Some('\n') | Some('\r') | Some('\t') => {}
            Some('\'') => {
                self.scan_char();
            }
            Some('"') => {
                self.scan_str();
            }
            Some('.') => {
                self.scan_float();
            }
            Some('0') | Some('1') | Some('2') | Some('3') | Some('4') | Some('5') | Some('6') |
            Some('7') | Some('8') | Some('9') => {
                while Lexer::is_digit(self.char_stream.peek().unwrap()) {
                    self.char_stream.next();
                }
                if self.char_stream.peek().unwrap() == &'.' || self.char_stream.peek().unwrap().to_ascii_lowercase() == 'e' {
                    self.scan_float();
                } else {
                    self.scan_int();
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
                while Lexer::is_al_num(self.char_stream.peek().unwrap()) || self.char_stream.peek().unwrap() == &'_' {
                    self.char_stream.next();
                }
            }
            Some('<') => {}
            Some('>') => {}
            Some('(') => {}
            Some(')') => {}
            Some('{') => {}
            Some('}') => {}
            Some('[') => {}
            Some(']') => {}
            Some(',') => {}
            Some('#') => {}
            Some('?') => {}
            Some(';') => {}
            Some('!') => {}
            Some(':') => {}
            Some('=') => {}
            Some('^') => {}
            Some('*') => {}
            Some('%') => {}
            Some('+') => {}
            Some('-') => {}
            Some('&') => {}
            Some('|') => {}
            _ => {}
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
