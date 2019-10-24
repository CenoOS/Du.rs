use crate::vm::instruction::OpCode::{
    ADD, ADDF64, ALOC, AND, CALL, DEC, DIV, DIVF64, EQ, EQF64, GT, GTE, GTEF64, GTF64, HLT, IGL,
    INC, JE, JG, JL, JMP, JMPB, JMPF, JNE, LOAD, LOADF64, LT, LTE, LTEF64, LTF64, MUL, MULF64,
    NEQF64, NOT, OR, POP, PRTS, PUSH, RET, SUB, SUBF64, XOR,
};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum OpCode {
    HLT = 0,
    LOAD = 1,
    ADD = 2,
    SUB = 3,
    MUL = 4,
    DIV = 5,
    JMP = 6,
    /* absolute jump */
    JMPF = 7,
    /* forward relative jump */
    JMPB = 8,
    /* backward relative jump */
    EQ = 9,
    JE = 10,
    ALOC = 11,
    INC = 12,
    DEC = 13,
    PRTS = 14,
    JNE = 15,
    JL = 16,
    JG = 17,
    LT = 18,
    LTE = 19,
    GT = 20,
    GTE = 21,
    LOADF64 = 22,
    ADDF64 = 23,
    SUBF64 = 24,
    MULF64 = 25,
    DIVF64 = 26,
    EQF64 = 27,
    NEQF64 = 28,
    GTF64 = 29,
    GTEF64 = 30,
    LTF64 = 31,
    LTEF64 = 32,

    AND = 33,
    OR = 34,
    XOR = 35,
    NOT = 36,

    PUSH = 37,
    POP = 38,

    CALL = 39,
    RET = 40,

    IGL = 41,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            _ => f.write_str(&format!("{}", *self as u8)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub(crate) opcode: OpCode,
}

impl From<u8> for OpCode {
    fn from(v: u8) -> Self {
        match v {
            0 => return HLT,
            1 => return LOAD,
            2 => return ADD,
            3 => return SUB,
            4 => return MUL,
            5 => return DIV,
            6 => return JMP,
            7 => return JMPF,
            8 => return JMPB,
            9 => return EQ,
            10 => return JE,
            11 => return ALOC,
            12 => return INC,
            13 => return DEC,
            14 => return PRTS,
            15 => return JNE,
            16 => return JL,
            17 => return JG,
            18 => return LT,
            19 => return LTE,
            20 => return GT,
            21 => return GTE,

            22 => return LOADF64,
            23 => return ADDF64,
            24 => return SUBF64,
            25 => return MULF64,
            26 => return DIVF64,
            27 => return EQF64,
            28 => return NEQF64,
            29 => return GTF64,
            30 => return GTEF64,
            31 => return LTF64,
            32 => return LTEF64,

            33 => return AND,
            34 => return OR,
            35 => return XOR,
            36 => return NOT,

            37 => return PUSH,
            38 => return POP,

            39 => return CALL,
            40 => return RET,

            _ => return IGL,
        }
    }
}

impl Instruction {
    pub fn new(opcode: OpCode) -> Instruction {
        Instruction { opcode }
    }
}
