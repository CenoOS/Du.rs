use crate::assembler::token::Token;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub token: Option<Token>,
    pub label: Option<Token>,
    pub directive: Option<Token>,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,

}

impl AssemblerInstruction {
    pub fn new(token: Option<Token>, label: Option<Token>,
               directive: Option<Token>, operand1: Option<Token>,
               operand2: Option<Token>, operand3: Option<Token>) -> AssemblerInstruction {
        AssemblerInstruction {
            token,
            label,
            directive,
            operand1,
            operand2,
            operand3,
        }
    }

    pub fn is_label(&self) -> bool {
        return self.label.is_some();
    }

    pub fn is_directive(&self) -> bool {
        return self.directive.is_some();
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results: Vec<u8> = Vec::new();
        match self.token {
            Some(Token::Op { opcode }) => match opcode {
                _ => { results.push(opcode as u8) }
            },
            _ => {
                println!("None opCode found in opCode field.");
                std::process::exit(0);
            }
        };

        for operand in vec![&self.operand1, &self.operand2, &self.operand3] {
            match operand {
                Some(t) => {
                    match t {
                        Token::Register { reg_num } => {
                            results.push(*reg_num);
                        }
                        Token::IntegerOperand { value } => {
                            let converted = *value as u16;
                            let byte1 = converted;
                            let byte2 = converted >> 8;
                            results.push(byte2 as u8);
                            results.push(byte1 as u8);
                        }
                        _ => {
                            println!("Unexpected opcode in operand field.");
                            std::process::exit(0);
                        }
                    }
                }
                None => {}
            }
        }
        return results;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::token::Token::{Op, Register, IntegerOperand};
    use crate::vm::instruction::OpCode::LOAD;

    #[test]
    fn should_return_bytes_when_give_an_instruction() {
        let ins = AssemblerInstruction::new(Some(Op { opcode: LOAD }),
                                            None, None,
                                            Some(Register { reg_num: 1 }),
                                            Some(IntegerOperand { value: 500 }),
                                            None);
        let results = ins.to_bytes();
        assert_eq!(results[0], 1);
        assert_eq!(results[1], 1);
        assert_eq!(results[2], 1);
        assert_eq!(results[3], 244);
    }
}
