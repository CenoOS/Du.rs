/*
 * Copyright (c) 2019. NeroYang
 */
use std::io;
use std::io::Error;
use std::io::{stdin, stdout, Read};

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
    size: (usize, usize),
    index: usize,
    length: usize,
    cursor: (usize, usize),
    history: Vec<String>,
}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {
            buffer: "".to_string(),
            size: (0, 0),
            index: 0,
            length: 0,
            cursor: (0, 0),
            history: Vec::new(),
        }
    }

    fn char_to_key_press(&self, c: &Result<u8, Error>) -> Option<Key> {
        match c {
            Ok(c) => {
                if !(*c as char).is_control() {
                    return Some(Key::Char(*c as char));
                }
                match *c {
                    0x00 => Some(Key::Ctrl(' ')),
                    0x01 => Some(Key::Ctrl('A')),
                    0x02 => Some(Key::Ctrl('B')),
                    0x03 => Some(Key::Ctrl('C')),
                    0x04 => Some(Key::Ctrl('D')),
                    0x05 => Some(Key::Ctrl('E')),
                    0x06 => Some(Key::Ctrl('F')),
                    0x07 => Some(Key::Ctrl('G')),
                    0x08 => Some(Key::Backspace), // '\b'
                    0x09 => Some(Key::Tab),       // '\t'
                    0x0a => Some(Key::Ctrl('J')), // '\n'
                    0x0b => Some(Key::Ctrl('K')),
                    0x0c => Some(Key::Ctrl('L')),
                    0x0d => Some(Key::Enter), // '\r'
                    0x0e => Some(Key::Ctrl('N')),
                    0x0f => Some(Key::Ctrl('O')),
                    0x10 => Some(Key::Ctrl('P')),
                    0x12 => Some(Key::Ctrl('R')),
                    0x13 => Some(Key::Ctrl('S')),
                    0x14 => Some(Key::Ctrl('T')),
                    0x15 => Some(Key::Ctrl('U')),
                    0x16 => Some(Key::Ctrl('V')),
                    0x17 => Some(Key::Ctrl('W')),
                    0x18 => Some(Key::Ctrl('X')),
                    0x19 => Some(Key::Ctrl('Y')),
                    0x1a => Some(Key::Ctrl('Z')),
                    0x1b => Some(Key::Esc), // Ctrl-[
                    0x1c => Some(Key::Ctrl('\\')),
                    0x1d => Some(Key::Ctrl(']')),
                    0x1e => Some(Key::Ctrl('^')),
                    0x1f => Some(Key::Ctrl('_')),
                    0x7f => Some(Key::Backspace),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub fn read_line(&self) -> Result<String, &'static str> {
        let mut buffer: Vec<u8> = Vec::with_capacity(30);
        let _stdout = stdout();
        loop {
            let stdin = stdin();
            stdin.lock();

            let mut peekable = stdin.bytes().peekable();
            let c = peekable.peek();
            match c {
                Some(c) => {
                    let key = self.char_to_key_press(c);
                    match key {
                        Some(ref k) => match k {
                            Key::Ctrl(ref _control_char) => {}
                            Key::Enter => {
                                break;
                            }
                            Key::Esc => {}
                            Key::Backspace => {
                                buffer.pop();
                            }
                            Key::Tab => {}
                            Key::Char(ref ch) => {
                                buffer.push(*ch as u8);
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
                _ => {
                    break;
                }
            }
            peekable.next();
        }
        let string =
            String::from_utf8(buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e));
        return Ok(string.unwrap());
    }
}
