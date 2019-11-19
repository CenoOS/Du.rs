/*
 * Copyright (c) 2019. NeroYang
 */
use crate::dolang::lexer::int::Int::{IntBin, IntHex, IntOct};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Int {
    IntHex { value: i32 },
    IntOct { value: i32 },
    IntBin { value: i32 },
}

impl Int {
    pub fn set_val(&self, value: i32) -> Int {
        match self {
            IntHex { value } => {
                return Int::IntHex { value: *value };
            }
            IntOct { value } => {
                return Int::IntOct { value: *value };
            }
            IntBin { value } => {
                return Int::IntBin { value: *value };
            }
        }
    }
}

impl Display for Int {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            IntHex { ref value } => {
                return f.write_str(&format!("IntHex(0x{})", value));
            }
            IntOct { ref value } => {
                return f.write_str(&format!("IntHex({})", value));
            }
            IntBin { ref value } => {
                return f.write_str(&format!("IntBin(0b{})", value));
            }
            _ => {
                return f.write_str(&format!("Unknown Int"));
            }
        }
    }
}
