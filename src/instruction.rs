use crate::instruction::OpCode::{HLT, IGL, LOAD, ADD, MUL, SUB, DIV, JMP, JMP_F, JMP_B, EQ, JEQ};

#[derive(Debug, PartialEq)]
pub enum OpCode {
    HLT,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    /* absolute jump */
    JMP_F,
    /* forward relative jump */
    JMP_B,
    /* backward relative jump */
    EQ,
    JEQ,
    IGL,
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
    use crate::instruction::OpCode::IGL;

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
