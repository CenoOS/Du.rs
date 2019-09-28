use crate::assembler::token::Token;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub token: Token,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}
