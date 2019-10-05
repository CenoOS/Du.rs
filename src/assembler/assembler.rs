use std::iter::Peekable;
use std::str::{SplitWhitespace, Lines};
use crate::assembler::token::Token::{Op, Register, IntegerOperand, Directive, LabelDeclaration};
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
                    return Ok(AssemblerInstruction::new(Some(Op { opcode: op }),
                                                        None,
                                                        None,
                                                        Some(Register { reg_num: operand1 }),
                                                        None,
                                                        None));
                }
                Err(_e) => { return Err("An Unsigned Integer is expected(e.g. 1...255)"); }
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
                    return Ok(AssemblerInstruction::new(instruction.token,
                                                        None,
                                                        None,
                                                        Some(Register { reg_num: operand1 }),
                                                        instruction.operand1,
                                                        None));
                }
                Err(_e) => { return Err("An Unsigned Integer is expected(e.g. 1...255)"); }
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
                    return Ok(AssemblerInstruction::new(instruction.token,
                                                        None,
                                                        None,
                                                        Some(Register { reg_num: operand1 }),
                                                        instruction.operand1,
                                                        instruction.operand2));
                }
                Err(_e) => { return Err("An Unsigned Integer is expected(e.g. 1...255)"); }
            }
        } else {
            return Err("An Register is expected(e.g. $1)");
        }
    }

    pub fn parse_directive(&mut self) -> Result<AssemblerInstruction, &'static str> {
        // directive : http://web.mit.edu/gnu/doc/html/as_7.html
        let directive = &(*self.tokens.peek().unwrap().to_string())[1..];
        match directive {
            "asciz" => {
                return Ok(AssemblerInstruction::new(None, None, Option::from(Directive { name: directive.to_string() }), None, None, None));
            }
            _ => {
                return Err("Unsupported directive.");
            }
        }
    }

    pub fn parse_label_declaration(&mut self) -> Result<AssemblerInstruction, &'static str> {
        let label_declaration_with_tag = *self.tokens.peek().unwrap();
        let len = &label_declaration_with_tag.len() - 1;
        let label_declaration = &label_declaration_with_tag[0..len];
        // todo : need check is alpha.

        // hello:
        self.tokens.next();

        if self.tokens.peekable() {
            match self.tokens.peek().map_or(false,|word| word.starts_with('.')){
                let directive = self.parse_directive();
                return Ok()
            }
            // hello: .asciz "Hello, World!"
            // hello: JMP $0
        }
        return Ok(AssemblerInstruction::new(None, Option::from(LabelDeclaration { name: label_declaration.to_string() }), None, None, None, None));
    }

    pub fn parse_label_usage(&mut self) -> Result<AssemblerInstruction, &'static str> {
        let label_usage = &(*self.tokens.peek().unwrap().to_string())[1..];
        return Ok(AssemblerInstruction::new(None, Option::from(LabelDeclaration { name: label_usage.to_string() }), None, None, None, None));
    }

    pub fn parse_assembly_line(&mut self) -> Result<AssemblerInstruction, &'static str> {
        if self.tokens.peek().map_or(false, |word| word.starts_with(".")) {
            return self.parse_directive();
        }

        if self.tokens.peek().map_or(false, |word| word.ends_with(':')) {
            return self.parse_label_declaration();
        }

        // todo: not be here
        if self.tokens.peek().map_or(false, |word| word.starts_with('@')) {
            return self.parse_label_usage();
        }

        return self.parse_instruction();
    }
    pub fn parse_instruction(&mut self) -> Result<AssemblerInstruction, &'static str> {
        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "HLT".to_string()) {
            return Ok(AssemblerInstruction::new(Some(Op { opcode: HLT }),
                                                None,
                                                None,
                                                None,
                                                None,
                                                None));
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
                                    return Ok(AssemblerInstruction::new(Some(Op { opcode: LOAD }),
                                                                        None,
                                                                        None,
                                                                        Some(Register { reg_num: operand1 }),
                                                                        Some(IntegerOperand { value: operand2 }),
                                                                        None));
                                }
                                Err(_e) => { return Err("An Unsigned Integer is expected(e.g. 1...65536)"); }
                            }
                        } else {
                            return Err("An Immediate number is expected(e.g. #1)");
                        }
                    }
                    Err(_e) => { return Err("An Unsigned Integer is expected(e.g. 1...255)"); }
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

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "INC".to_string()) {
            self.tokens.next();
            return self.parse_one_register_instruction(INC);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "DEC".to_string()) {
            self.tokens.next();
            return self.parse_one_register_instruction(DEC);
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
        let mut token_parser = InstructionParser::new("hlt");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: HLT }),
            label: None,
            directive: None,
            operand1: None,
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_load_when_give_load() {
        let mut token_parser = InstructionParser::new("load $1 #300");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: LOAD }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(IntegerOperand { value: 300 }),
            operand3: None,
        });
    }

    #[test]
    fn should_return_add_when_give_add() {
        let mut token_parser = InstructionParser::new("add $0 $1 $2");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: ADD }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
    }

    #[test]
    fn should_return_sub_when_give_sub() {
        let mut token_parser = InstructionParser::new("sub $0 $1 $2");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: SUB }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
    }

    #[test]
    fn should_return_mul_when_give_mul() {
        let mut token_parser = InstructionParser::new("mul $0 $1     $2");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: MUL }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
    }

    #[test]
    fn should_return_div_when_give_div() {
        let mut token_parser = InstructionParser::new("div $0 $1 $2");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: DIV }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
    }

    #[test]
    fn should_return_jmp_when_give_jmp() {
        let mut token_parser = InstructionParser::new("jmp $1");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: JMP }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_jmp_f_when_give_jmp_f() {
        let mut token_parser = InstructionParser::new("jmp_f $1");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: JMP_F }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_jmp_b_when_give_jmp_b() {
        let mut token_parser = InstructionParser::new("jmp_b $1");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: JMP_B }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_eq_when_give_eq() {
        let mut token_parser = InstructionParser::new("eq $1 $2");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: EQ }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        });
    }

    #[test]
    fn should_return_jeq_when_give_jeq() {
        let mut token_parser = InstructionParser::new("jeq $1");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: JEQ }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_inc_when_give_inc() {
        let mut token_parser = InstructionParser::new("inc $1");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: INC }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_dec_when_give_dec() {
        let mut token_parser = InstructionParser::new("dec $1");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: DEC }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
    }


    #[test]
    fn should_return_error_when_give_unexpected_token() {
        let mut token_parser = InstructionParser::new("xxx $1");
        let token = token_parser.parse_instruction();
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
            token: Some(Op { opcode: HLT }),
            label: None,
            directive: None,
            operand1: None,
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[1], AssemblerInstruction {
            token: Some(Op { opcode: LOAD }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(IntegerOperand { value: 300 }),
            operand3: None,
        });
        assert_eq!(instructions[2], AssemblerInstruction {
            token: Some(Op { opcode: ADD }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
        assert_eq!(instructions[3], AssemblerInstruction {
            token: Some(Op { opcode: SUB }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
        assert_eq!(instructions[4], AssemblerInstruction {
            token: Some(Op { opcode: MUL }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
        assert_eq!(instructions[5], AssemblerInstruction {
            token: Some(Op { opcode: DIV }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
        assert_eq!(instructions[6], AssemblerInstruction {
            token: Some(Op { opcode: JMP }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[7], AssemblerInstruction {
            token: Some(Op { opcode: JMP_F }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[8], AssemblerInstruction {
            token: Some(Op { opcode: JMP_B }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[9], AssemblerInstruction {
            token: Some(Op { opcode: EQ }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        });
        assert_eq!(instructions[10], AssemblerInstruction {
            token: Some(Op { opcode: JEQ }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
    }
}
