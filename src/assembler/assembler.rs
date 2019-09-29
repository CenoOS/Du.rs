use std::iter::Peekable;
use std::str::{SplitWhitespace, Lines};
use crate::assembler::token::Token::{Op, Register, IntegerOperand};
use crate::vm::instruction::OpCode::*;
use crate::assembler::assembler_instruction::AssemblerInstruction;
use crate::vm::instruction::OpCode;


pub struct InstructionParser<'a> {
    tokens: Peekable<SplitWhitespace<'a>>,
}

impl<'a> InstructionParser<'a> {
    pub fn new(str: &str) -> InstructionParser {
        InstructionParser {
            tokens: str.split_whitespace().peekable()
        }
    }


    fn parse_one_register_instruction(&mut self, op: OpCode) -> Result<AssemblerInstruction, &'static str> {
        if self.tokens.peek().map_or(false, |word| word.starts_with("$")) {
            let operand1_str = &(*self.tokens.peek().unwrap().to_string())[1..];
            let is_u8 = operand1_str.parse::<u8>();
            match is_u8 {
                Ok(operand1) => {
                    return Ok(AssemblerInstruction::new(Op { opcode: op }, Some(Register { reg_num: operand1 }), None, None));
                }
                Err(e) => { return Err("An Unsigned Integer is expected(e.g. 1...255)"); }
            }
        } else {
            return Err("An Register is expected(e.g. $1)");
        }
    }

    fn parse_two_register_instruction(&mut self, op: OpCode) -> Result<AssemblerInstruction, &'static str> {
        if self.tokens.peek().map_or(false, |word| word.starts_with("$")) {
            let operand1_str = &(*self.tokens.peek().unwrap().to_string())[1..];
            let is_u8 = operand1_str.parse::<u8>();
            match is_u8 {
                Ok(operand1) => {
                    self.tokens.next();
                    let instruction = self.parse_one_register_instruction(op).unwrap();
                    return Ok(AssemblerInstruction::new(instruction.token, Some(Register { reg_num: operand1 }), instruction.operand1, None));
                }
                Err(e) => { return Err("An Unsigned Integer is expected(e.g. 1...255)"); }
            }
        } else {
            return Err("An Register is expected(e.g. $1)");
        }
    }

    fn parse_three_register_instruction(&mut self, op: OpCode) -> Result<AssemblerInstruction, &'static str> {
        if self.tokens.peek().map_or(false, |word| word.starts_with("$")) {
            let operand1_str = &(*self.tokens.peek().unwrap().to_string())[1..];
            let is_u8 = operand1_str.parse::<u8>();
            match is_u8 {
                Ok(operand1) => {
                    self.tokens.next();
                    let instruction = self.parse_two_register_instruction(op).unwrap();
                    return Ok(AssemblerInstruction::new(instruction.token, Some(Register { reg_num: operand1 }), instruction.operand1, instruction.operand2));
                }
                Err(e) => { return Err("An Unsigned Integer is expected(e.g. 1...255)"); }
            }
        } else {
            return Err("An Register is expected(e.g. $1)");
        }
    }

    pub fn parse_instruction(&mut self) -> Result<AssemblerInstruction, &'static str> {
        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "HLT".to_string()) {
            return Ok(AssemblerInstruction::new(Op { opcode: HLT }, None, None, None));
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
                                    return Ok(AssemblerInstruction::new(Op { opcode: LOAD }, Some(Register { reg_num: operand1 }), Some(IntegerOperand { value: operand2 }), None));
                                }
                                Err(e) => { return Err("An Unsigned Integer is expected(e.g. 1...65536)"); }
                            }
                        } else {
                            return Err("An Immediate number is expected(e.g. #1)");
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

pub struct AssemblyProgramParser<'a> {
    instructions: Peekable<Lines<'a>>,
}

impl<'a> AssemblyProgramParser<'a> {
    pub fn new(str: &str) -> AssemblyProgramParser {
        AssemblyProgramParser {
            instructions: str.lines().peekable()
        }
    }

    fn parse_program(&mut self) -> Result<Vec<AssemblerInstruction>, &'static str> {
        let mut assembler_instructions: Vec<AssemblerInstruction> = Vec::new();
        while self.instructions.peek().is_some() {
            match self.instructions.peek() {
                Some(instruction_str) => {
                    let mut instruction_parser = InstructionParser::new(instruction_str);
                    let instruction = instruction_parser.parse_instruction();
                    match instruction {
                        Ok(ins) => {
                            assembler_instructions.push(ins);
                            self.instructions.next();
                        }
                        Err(e) => { return Err(e); }
                    }
                }
                _ => { break; }
            }
        }
        return Ok(assembler_instructions);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_hlt_when_give_hlt() {
        let mut tokenParser = InstructionParser::new("hlt");
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
        let mut tokenParser = InstructionParser::new("load $1 #300");
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
        let mut tokenParser = InstructionParser::new("add $0 $1 $2");
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
        let mut tokenParser = InstructionParser::new("sub $0 $1 $2");
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
        let mut tokenParser = InstructionParser::new("mul $0 $1     $2");
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
        let mut tokenParser = InstructionParser::new("div $0 $1 $2");
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
        let mut tokenParser = InstructionParser::new("jmp $1");
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
        let mut tokenParser = InstructionParser::new("jmp_f $1");
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
        let mut tokenParser = InstructionParser::new("jmp_b $1");
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
        let mut tokenParser = InstructionParser::new("eq $1 $2");
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
        let mut tokenParser = InstructionParser::new("jeq $1");
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
        let mut tokenParser = InstructionParser::new("xxx $1");
        let token = tokenParser.parse_instruction();
        assert_eq!(token.is_err(), true);
    }


    #[test]
    fn should_return_instruction_list_when_give_assembly_code() {
        let mut assembler = AssemblyProgramParser::new(
            "hlt\n\
                load $1 #300\n\
                add $0 $1 $2\n\
                sub $0 $1 $2\n\
                mul $0 $1     $2\n\
                div $0 $1 $2\n\
                jmp $1\n\
                jmp_f $1\n\
                jmp_b $1\n\
                eq $1 $2\n\
                jeq $1");
        let instructions = assembler.parse_program().unwrap();
        assert_eq!(instructions[0], AssemblerInstruction {
            token: Op { opcode: HLT },
            operand1: None,
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[1], AssemblerInstruction {
            token: Op { opcode: LOAD },
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(IntegerOperand { value: 300 }),
            operand3: None,
        });
        assert_eq!(instructions[2], AssemblerInstruction {
            token: Op { opcode: ADD },
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
        assert_eq!(instructions[3], AssemblerInstruction {
            token: Op { opcode: SUB },
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
        assert_eq!(instructions[4], AssemblerInstruction {
            token: Op { opcode: MUL },
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
        assert_eq!(instructions[5], AssemblerInstruction {
            token: Op { opcode: DIV },
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
        assert_eq!(instructions[6], AssemblerInstruction {
            token: Op { opcode: JMP },
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[7], AssemblerInstruction {
            token: Op { opcode: JMP_F },
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[8], AssemblerInstruction {
            token: Op { opcode: JMP_B },
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[9], AssemblerInstruction {
            token: Op { opcode: EQ },
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        });
        assert_eq!(instructions[10], AssemblerInstruction {
            token: Op { opcode: JEQ },
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
    }
}
