use crate::assembler::token::Token;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub token: Token,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn new(token: Token, operand1: Option<Token>, operand2: Option<Token>, operand3: Option<Token>) -> AssemblerInstruction {
        AssemblerInstruction {
            token,
            operand1,
            operand2,
            operand3
        }
    }
}
