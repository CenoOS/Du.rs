/*
 * Copyright (c) 2019. NeroYang
 */
use std::io::{stdin, Read};
use std::io;

pub enum Key {
    UnknownEscSeq,
    Backspace,
    // Ctrl('H')
    BackTab,
    BracketedPasteStart,
    BracketedPasteEnd,
    Char(char),
    ControlDown,
    ControlLeft,
    ControlRight,
    ControlUp,
    Ctrl(char),
    Delete,
    Down,
    End,
    Enter,
    // Ctrl('M')
    Esc,
    // Ctrl('[')
    F(u8),
    Home,
    Insert,
    Left,
    Meta(char),
    Null,
    PageDown,
    PageUp,
    Right,
    ShiftDown,
    ShiftLeft,
    ShiftRight,
    ShiftUp,
    Tab,
    // Ctrl('I')
    Up,
}

pub struct Terminal {
    buffer: String,
    index: usize,
    length: usize,
    cursor: (usize, usize),
    history: Vec<String>,
}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {
            buffer: "".to_string(),
            index: 0,
            length: 0,
            cursor: (0, 0),
            history: Vec::new(),
        }
    }

    fn char_to_key_press(&self, c: char) -> Option<Key> {
        if !c.is_control() {
            return Some(Key::Char(c));
        }
        match c {
            '\x00' => Some(Key::Ctrl(' ')),
            '\x01' => Some(Key::Ctrl('A')),
            '\x02' => Some(Key::Ctrl('B')),
            '\x03' => Some(Key::Ctrl('C')),
            '\x04' => Some(Key::Ctrl('D')),
            '\x05' => Some(Key::Ctrl('E')),
            '\x06' => Some(Key::Ctrl('F')),
            '\x07' => Some(Key::Ctrl('G')),
            '\x08' => Some(Key::Backspace), // '\b'
            '\x09' => Some(Key::Tab),       // '\t'
            '\x0a' => Some(Key::Ctrl('J')), // '\n'
            '\x0b' => Some(Key::Ctrl('K')),
            '\x0c' => Some(Key::Ctrl('L')),
            '\x0d' => Some(Key::Enter), // '\r'
            '\x0e' => Some(Key::Ctrl('N')),
            '\x0f' => Some(Key::Ctrl('O')),
            '\x10' => Some(Key::Ctrl('P')),
            '\x12' => Some(Key::Ctrl('R')),
            '\x13' => Some(Key::Ctrl('S')),
            '\x14' => Some(Key::Ctrl('T')),
            '\x15' => Some(Key::Ctrl('U')),
            '\x16' => Some(Key::Ctrl('V')),
            '\x17' => Some(Key::Ctrl('W')),
            '\x18' => Some(Key::Ctrl('X')),
            '\x19' => Some(Key::Ctrl('Y')),
            '\x1a' => Some(Key::Ctrl('Z')),
            '\x1b' => Some(Key::Esc), // Ctrl-[
            '\x1c' => Some(Key::Ctrl('\\')),
            '\x1d' => Some(Key::Ctrl(']')),
            '\x1e' => Some(Key::Ctrl('^')),
            '\x1f' => Some(Key::Ctrl('_')),
            '\x7f' => Some(Key::Backspace),
            _ => None,
        }
    }

    pub fn read_line(&self) -> Result<String, &'static str> {
        let mut buffer: Vec<u8> = Vec::with_capacity(30);
        loop {
            let stdin = stdin();
            stdin.lock();
            let mut peekable = stdin.bytes().peekable();
            let c = peekable.peek();
            match c {
                Some(c) => {
                    match c {
                        Ok(0x7f) => {
                            buffer.pop();
                        }
                        Ok(27) => {
                            peekable.next();
                            let cc = peekable.peek().unwrap();
                            match cc {
                                Ok(91) => {
                                    peekable.next();
                                    let ccc = peekable.peek().unwrap();
                                    match ccc {
                                        Ok(65) => { print!("UP") }
                                        Ok(66) => { print!("DOWN") }
                                        Ok(67) => { print!("LEFT") }
                                        Ok(68) => { print!("RIGHT") }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        Ok(0xD) | Ok(0xA) => {
                            print!("xxxx");
                            break;
                        }
                        Ok(c) => {
                            print!("{}", c);
                            buffer.push(*c);
                        }
                        Err(e) => {
                            return Err("");
                        }
                    }
                }
                _ => { break; }
            }
            peekable.next();
        }
        let string = String::from_utf8(buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e));
        return Ok(string.unwrap());
    }
}
