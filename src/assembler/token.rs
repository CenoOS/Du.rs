use crate::instruction::OpCode;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { opcode: OpCode },
    Register { reg_num: u8 },
    IntegerOperand { value: i32 },
}
