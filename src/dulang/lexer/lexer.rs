/*
 * Copyright (c) 2019. NeroYang
 */

use std::iter::Peekable;
use std::str::{Lines, Chars};


struct Lexer<'a> {
    char_stream: Peekable<Chars<'a>>,
    current_line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(&self, source: &'a str) -> Lexer {
        Lexer {
            char_stream: source.chars().peekable(),
            current_line: 0,
        }
    }

    fn scan_char(&mut self) {}
    fn scan_str(&mut self) {}
    fn scan_float(&mut self) {}

    fn next_token(&mut self) {
        match self.char_stream.peek() {
            Some(' ') => {}
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
            Some('7') | Some('8') | Some('9') => {}
            Some('a') | Some('b') | Some('c') | Some('d') | Some('e') | Some('f') | Some('g') |
            Some('h') | Some('i') | Some('j') | Some('k') | Some('l') | Some('m') | Some('n') |
            Some('o') | Some('p') | Some('q') | Some('r') | Some('s') | Some('t') | Some('u') |
            Some('v') | Some('w') | Some('x') | Some('y') | Some('z') |
            Some('A') | Some('B') | Some('C') | Some('D') | Some('E') | Some('F') | Some('G') |
            Some('H') | Some('I') | Some('J') | Some('K') | Some('L') | Some('M') | Some('N') |
            Some('O') | Some('P') | Some('Q') | Some('R') | Some('S') | Some('T') | Some('U') |
            Some('V') | Some('W') | Some('X') | Some('Y') | Some('Z') | Some('_') => {}
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
}
