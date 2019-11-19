/*
 * Copyright (c) 2019. NeroYang
 */
use std::io::{stdin, Read};

pub enum ArrowKey {
    Up,
    Down,
    Left,
    Right,
}

pub enum SpecialKey {
    Enter,
}

pub enum TerminalLine {
    StringLine { value: String },
    Interrupted,
    ArrowKey { arrow: ArrowKey },
    SpecialKey { key: SpecialKey },
}

pub struct Terminal {
    buffer: String,
    index: usize,
    length: usize,
    cursor: (usize, usize),
    history: Vec<String>,
}

impl Terminal {
    fn new(&self) -> Terminal {
        Terminal {
            buffer: "".to_string(),
            index: 0,
            length: 0,
            cursor: (0, 0),
            history: Vec::new(),
        }
    }

    fn read_line(&self) -> Result<TerminalLine, &'static str> {
        let mut buffer: Vec<u8> = Vec::with_capacity(30);
        let stdin = stdin();
        for c in stdin.bytes() {
            match c {
                Err(e) => {
                    return Err("");
                }
                _ => {}
            }
        }
        return Err("");
    }
}
