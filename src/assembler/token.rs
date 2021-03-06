use crate::vm::instruction::OpCode;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Op { opcode: OpCode },
    Register { reg_num: u8 },
    IntegerOperand { value: i32 },
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String },
    IrString { name: String },
}
