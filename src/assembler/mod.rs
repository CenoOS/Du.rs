use crate::instruction::OpCode;

pub mod opcode_parsers;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { opcode: OpCode },
}



