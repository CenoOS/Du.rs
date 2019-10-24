/*
 * Copyright (c) 2019. NeroYang
 */
#[derive(Debug, Clone, PartialEq)]
pub enum Int {
    IntHex { value: i32 },
    IntOct { value: i32 },
    IntBin { value: i32 },
}

impl Int {
    pub fn set_val(&self, value: i32) -> Int {
        match self {
            IntHex => {
                return Int::IntHex { value };
            }
            IntOct => {
                return Int::IntOct { value };
            }
            IntBin => {
                return Int::IntBin { value };
            }
        }
    }
}
