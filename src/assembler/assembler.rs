use std::iter::Peekable;
use std::str::SplitWhitespace;
use crate::assembler::token::Token::{Op, Register, IntegerOperand};
use crate::instruction::OpCode::*;
use crate::assembler::assembler_instruction::AssemblerInstruction;
use crate::instruction::OpCode;


pub struct OpCodeParser<'a> {
    tokens: Peekable<SplitWhitespace<'a>>,
}

impl<'a> OpCodeParser<'a> {
    pub fn new(str: &str) -> OpCodeParser {
        OpCodeParser {
            tokens: str.split_whitespace().peekable()
        }
    }


    pub fn parse_one_register_instruction(&mut self, op: OpCode) -> Result<AssemblerInstruction, &'static str> {
        if self.tokens.peek().map_or(false, |word| word.starts_with("$")) {
            let operand1_str = &(*self.tokens.peek().unwrap().to_string())[1..];
            let is_u8 = operand1_str.parse::<u8>();
            match is_u8 {
                Ok(operand1) => {
                    return Ok(AssemblerInstruction {
                        token: Op { opcode: op },
                        operand1: Some(Register { reg_num: operand1 }),
                        operand2: None,
                        operand3: None,
                    });
                }
                Err(e) => { return Err("An Unsigned Integer is expected(e.g. 1...255)"); }
            }
        } else {
            return Err("An Register is expected(e.g. $1)");
        }
    }

    pub fn parse_two_register_instruction(&mut self, op: OpCode) -> Result<AssemblerInstruction, &'static str> {
        if self.tokens.peek().map_or(false, |word| word.starts_with("$")) {
            let operand1_str = &(*self.tokens.peek().unwrap().to_string())[1..];
            let is_u8 = operand1_str.parse::<u8>();
            match is_u8 {
                Ok(operand1) => {
                    self.tokens.next();
                    let instruction = self.parse_one_register_instruction(op).unwrap();
                    return Ok(AssemblerInstruction {
                        token: instruction.token,
                        operand1: Some(Register { reg_num: operand1 }),
                        operand2: instruction.operand1,
                        operand3: None,
                    });
                }
                Err(e) => { return Err("An Unsigned Integer is expected(e.g. 1...255)"); }
            }
        } else {
            return Err("An Register is expected(e.g. $1)");
        }
    }

    pub fn parse_three_register_instruction(&mut self, op: OpCode) -> Result<AssemblerInstruction, &'static str> {
        if self.tokens.peek().map_or(false, |word| word.starts_with("$")) {
            let operand1_str = &(*self.tokens.peek().unwrap().to_string())[1..];
            let is_u8 = operand1_str.parse::<u8>();
            match is_u8 {
                Ok(operand1) => {
                    self.tokens.next();
                    let instruction = self.parse_two_register_instruction(op).unwrap();
                    return Ok(AssemblerInstruction {
                        token: instruction.token,
                        operand1: Some(Register { reg_num: operand1 }),
                        operand2: instruction.operand1,
                        operand3: instruction.operand2,
                    });
                }
                Err(e) => { return Err("An Unsigned Integer is expected(e.g. 1...255)"); }
            }
        } else {
            return Err("An Register is expected(e.g. $1)");
        }
    }

    pub fn parse_instruction(&mut self) -> Result<AssemblerInstruction, &'static str> {
        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "HLT".to_string()) {
            return Ok(AssemblerInstruction {
                token: Op { opcode: HLT },
                operand1: None,
                operand2: None,
                operand3: None,
            });
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "LOAD".to_string()) {
            self.tokens.next();
            if self.tokens.peek().map_or(false, |word| word.starts_with("$")) {
                let operand1_str = &(*self.tokens.peek().unwrap().to_string())[1..];
                let is_u8 = operand1_str.parse::<u8>();
                match is_u8 {
                    Ok(operand1) => {
                        self.tokens.next();
                        if self.tokens.peek().map_or(false, |word| word.starts_with("#")) {
                            let operand2_str = &(*self.tokens.peek().unwrap().to_string())[1..];
                            let is_i32 = operand2_str.parse::<i32>();
                            match is_i32 {
                                Ok(operand2) => {
                                    return Ok(AssemblerInstruction {
                                        token: Op { opcode: LOAD },
                                        operand1: Some(Register { reg_num: operand1 }),
                                        operand2: Some(IntegerOperand { value: operand2 }),
                                        operand3: None,
                                    });
                                }
                                Err(e) => { return Err("An Unsigned Integer is expected(e.g. 1...65536)"); }
                            }
                        } else {
                            return Err("An # is expected(e.g. #1)");
                        }
                    }
                    Err(e) => { return Err("An Unsigned Integer is expected(e.g. 1...255)"); }
                }
            } else {
                return Err("An Register is expected(e.g. $1)");
            }
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "ADD".to_string()) {
            self.tokens.next();
            return self.parse_three_register_instruction(ADD);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "SUB".to_string()) {
            self.tokens.next();
            return self.parse_three_register_instruction(SUB);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "MUL".to_string()) {
            self.tokens.next();
            return self.parse_three_register_instruction(MUL);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "DIV".to_string()) {
            self.tokens.next();
            return self.parse_three_register_instruction(DIV);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "JMP".to_string()) {
            self.tokens.next();
            return self.parse_one_register_instruction(JMP);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "JMP_F".to_string()) {
            self.tokens.next();
            return self.parse_one_register_instruction(JMP_F);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "JMP_B".to_string()) {
            self.tokens.next();
            return self.parse_one_register_instruction(JMP_B);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "EQ".to_string()) {
            self.tokens.next();
            return self.parse_two_register_instruction(EQ);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "JEQ".to_string()) {
            self.tokens.next();
            return self.parse_one_register_instruction(JEQ);
        }

        Err("Unexpected Assembly Code.")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_hlt_when_give_hlt() {
        let mut tokenParser = OpCodeParser::new("hlt");
        let token = tokenParser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Op { opcode: HLT },
            operand1: None,
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_load_when_give_load() {
        let mut tokenParser = OpCodeParser::new("load $1 #300");
        let token = tokenParser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Op { opcode: LOAD },
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(IntegerOperand { value: 300 }),
            operand3: None,
        });
    }

    #[test]
    fn should_return_add_when_give_add() {
        let mut tokenParser = OpCodeParser::new("add $0 $1 $2");
        let token = tokenParser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Op { opcode: ADD },
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
    }

    #[test]
    fn should_return_sub_when_give_sub() {
        let mut tokenParser = OpCodeParser::new("sub $0 $1 $2");
        let token = tokenParser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Op { opcode: SUB },
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
    }

    #[test]
    fn should_return_mul_when_give_mul() {
        let mut tokenParser = OpCodeParser::new("mul $0 $1 $2");
        let token = tokenParser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Op { opcode: MUL },
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
    }

    #[test]
    fn should_return_div_when_give_div() {
        let mut tokenParser = OpCodeParser::new("div $0 $1 $2");
        let token = tokenParser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Op { opcode: DIV },
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
    }

    #[test]
    fn should_return_jmp_when_give_jmp() {
        let mut tokenParser = OpCodeParser::new("jmp $1");
        let token = tokenParser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Op { opcode: JMP },
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_jmp_f_when_give_jmp_f() {
        let mut tokenParser = OpCodeParser::new("jmp_f $1");
        let token = tokenParser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Op { opcode: JMP_F },
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_jmp_b_when_give_jmp_b() {
        let mut tokenParser = OpCodeParser::new("jmp_b $1");
        let token = tokenParser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Op { opcode: JMP_B },
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_eq_when_give_eq() {
        let mut tokenParser = OpCodeParser::new("eq $1 $2");
        let token = tokenParser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Op { opcode: EQ },
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        });
    }

    #[test]
    fn should_return_jeq_when_give_jeq() {
        let mut tokenParser = OpCodeParser::new("jeq $1");
        let token = tokenParser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Op { opcode: JEQ },
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
    }


    #[test]
    fn should_return_error_when_give_unexpected_token() {
        let mut tokenParser = OpCodeParser::new("xxx $1");
        let token = tokenParser.parse_instruction();
        assert_eq!(token.is_err(), true);
    }
}
