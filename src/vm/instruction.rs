use crate::vm::instruction::OpCode::{HLT, LOAD, ADD, MUL, SUB, DIV, JMP, JMP_F, JMP_B, EQ, JEQ, IGL, ALOC, INC, DEC};

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
    JMP_F = 7,
    /* forward relative jump */
    JMP_B = 8,
    /* backward relative jump */
    EQ = 9,
    JEQ = 10,

    ALOC = 11,
    INC = 12,
    DEC = 13,
    IGL = 14,
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: OpCode,
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
            7 => return JMP_F,
            8 => return JMP_B,
            9 => return EQ,
            10 => return JEQ,
            11 => return ALOC,
            12 => return INC,
            13 => return DEC,
            _ => return IGL,
        }
    }
}

impl Instruction {
    pub fn new(opcode: OpCode) -> Instruction {
        Instruction {
            opcode
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::instruction::OpCode::IGL;

    #[test]
    fn should_create_opcode() {
        let opcode = HLT;
        assert_eq!(opcode, HLT);
    }

    #[test]
    fn should_create_instruction() {
        let instruction = Instruction::new(HLT);
        assert_eq!(instruction.opcode, HLT);
    }
}
