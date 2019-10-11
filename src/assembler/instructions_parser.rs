use std::iter::{Peekable, Iterator};
use std::str::{SplitWhitespace, Lines};
use crate::assembler::token::Token::{Op, Register, IntegerOperand, Directive, LabelDeclaration, LabelUsage, IrString};
use crate::vm::instruction::OpCode::*;
use crate::assembler::assembler_instruction::AssemblerInstruction;
use crate::vm::instruction::OpCode;

const SYMBOL_REGISTER: &str = "$";
const SYMBOL_LABEL_USAGE: &str = "@";
const SYMBOL_LABEL_IMMEDIATE: &str = "#";
const SYMBOL_DIRECTIVE: &str = ".";
const SYMBOL_COLON: &str = ":";
const SYMBOL_STRING: &str = "\"";


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
        if self.tokens.peek().map_or(false, |word| word.starts_with(SYMBOL_REGISTER)) {
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
        } else if self.tokens.peek().map_or(false, |word| word.starts_with(SYMBOL_LABEL_USAGE)) {
            let label_usage = self.parse_label_usage();
            match label_usage {
                Ok(label) => {
                    return Ok(AssemblerInstruction::new(Some(Op { opcode: op }),
                                                        label.label,
                                                        None,
                                                        label.operand1,// todo : operand should find from symbol table
                                                        None,
                                                        None));
                }
                Err(e) => { return Err(e); }
            }
        } else {
            return Err("An Register is expected(e.g. $1)");
        }
    }

    fn parse_two_register_instruction(&mut self, op: OpCode) -> Result<AssemblerInstruction, &'static str> {
        if self.tokens.peek().map_or(false, |word| word.starts_with(SYMBOL_REGISTER)) {
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
        } else if self.tokens.peek().map_or(false, |word| word.starts_with(SYMBOL_LABEL_USAGE)) {
            let label_usage = self.parse_label_usage();
            match label_usage {
                Ok(label) => {
                    self.tokens.next();
                    let instruction = self.parse_one_register_instruction(op).unwrap();
                    return Ok(AssemblerInstruction::new(instruction.token,
                                                        label.label,
                                                        None,
                                                        label.operand1, // todo : operand should find from symbol table
                                                        instruction.operand1,
                                                        None));
                }
                Err(e) => { return Err(e); }
            }
        } else {
            return Err("An Register / Label is expected(e.g. $1 / @hello)");
        }
    }

    fn parse_three_register_instruction(&mut self, op: OpCode) -> Result<AssemblerInstruction, &'static str> {
        if self.tokens.peek().map_or(false, |word| word.starts_with(SYMBOL_REGISTER)) {
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
        } else if self.tokens.peek().map_or(false, |word| word.starts_with(SYMBOL_LABEL_USAGE)) {
            let label_usage = self.parse_label_usage();
            match label_usage {
                Ok(label) => {
                    self.tokens.next();
                    let instruction = self.parse_two_register_instruction(op).unwrap();
                    return Ok(AssemblerInstruction::new(instruction.token,
                                                        label.label,
                                                        None,
                                                        label.operand1, // todo : operand should find from symbol table
                                                        instruction.operand1,
                                                        instruction.operand2));
                }
                Err(e) => { return Err(e); }
            }
        } else {
            return Err("An Register / Label is expected(e.g. $1 / @hello)");
        }
    }

    pub fn parse_directive(&mut self) -> Result<AssemblerInstruction, &'static str> {
        // directive : http://web.mit.edu/gnu/doc/html/as_7.html
        let directive = &(*self.tokens.peek().unwrap().to_string())[1..];
        match directive {
            "asciiz" => {
                self.tokens.next();
                let mut str = String::from("");
                if self.tokens.peek().map_or(false, |w| w.starts_with(SYMBOL_STRING)) {
                    if self.tokens.peek().map_or(false, |w| w.ends_with(SYMBOL_STRING)) {
                        let str_part = &(*self.tokens.peek().unwrap().to_string());
                        let len = &str_part.len() - 1;
                        let str_part_all = &str_part[1..len];
                        str.push_str(str_part_all);
                        return Ok(AssemblerInstruction::new(None,
                                                            None,
                                                            Some(Directive { name: directive.to_string() }),
                                                            Some(IrString { name: str }),
                                                            None,
                                                            None));
                    } else {
                        let str_part = &(*self.tokens.peek().unwrap().to_string())[1..];
                        str.push_str(str_part);
                        self.tokens.next();
                        while self.tokens.peek().map_or(false, |w| !w.ends_with(SYMBOL_STRING)) {
                            let str_part_middle = &(*self.tokens.peek().unwrap().to_string());
                            str.push_str(" ");
                            str.push_str(str_part_middle);
                            self.tokens.next();
                        }
                        if self.tokens.peek().map_or(false, |w| w.ends_with(SYMBOL_STRING)) {
                            let str_part_last_all = &(*self.tokens.peek().unwrap().to_string());
                            let len = &str_part_last_all.len() - 1;
                            let str_part_last = &str_part_last_all[0..len];
                            str.push_str(" ");
                            str.push_str(str_part_last);
                        }

                        return Ok(AssemblerInstruction::new(None,
                                                            None,
                                                            Some(Directive { name: directive.to_string() }),
                                                            Some(IrString { name: str }),
                                                            None,
                                                            None));
                    }
                }
                return Err("Expect a string starts with \' and end with \'");
            }
            "code" => {
                return Ok(AssemblerInstruction::new(None,
                                                    None,
                                                    Some(Directive { name: directive.to_string() }),
                                                    None,
                                                    None,
                                                    None));
            }
            "data" => {
                return Ok(AssemblerInstruction::new(None,
                                                    None,
                                                    Some(Directive { name: directive.to_string() }),
                                                    None,
                                                    None,
                                                    None));
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

        if self.tokens.peek().is_some() {
            if self.tokens.peek().map_or(false, |word| word.starts_with(SYMBOL_DIRECTIVE)) { // hello: .asciz "Hello, World!"
                let directive = self.parse_directive();
                match directive {
                    Ok(ins) => {
                        return Ok(AssemblerInstruction::new(None,
                                                            Some(LabelDeclaration {
                                                                name: label_declaration.to_string()
                                                            }),
                                                            ins.directive,
                                                            ins.operand1,
                                                            ins.operand2,
                                                            ins.operand3));
                    }
                    Err(e) => { return Err(e); }
                }
            } else {// hello: JMP $0
                let instruction = self.parse_instruction();
                match instruction {
                    Ok(ins) => {
                        return Ok(AssemblerInstruction::new(ins.token,
                                                            Some(LabelDeclaration {
                                                                name: label_declaration.to_string()
                                                            }),
                                                            None,
                                                            ins.operand1,
                                                            ins.operand2,
                                                            ins.operand3));
                    }
                    Err(e) => { return Err(e); }
                }
            }
        }
        return Ok(AssemblerInstruction::new(None,
                                            Some(LabelDeclaration {
                                                name: label_declaration.to_string()
                                            }),
                                            None,
                                            None,
                                            None,
                                            None));
    }

    pub fn parse_label_usage(&mut self) -> Result<AssemblerInstruction, &'static str> {
        let label_usage = &(*self.tokens.peek().unwrap().to_string())[1..];
        return Ok(AssemblerInstruction::new(None,
                                            Some(LabelUsage {
                                                name: label_usage.to_string()
                                            }),
                                            None,
                                            None,
                                            None,
                                            None));
    }

    pub fn parse_assembly_line(&mut self) -> Result<AssemblerInstruction, &'static str> {
        if self.tokens.peek().map_or(false, |word| word.starts_with(SYMBOL_DIRECTIVE)) {
            return self.parse_directive();
        }

        if self.tokens.peek().map_or(false, |word| word.ends_with(SYMBOL_COLON)) {
            return self.parse_label_declaration();
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
            if self.tokens.peek().map_or(false, |word| word.starts_with(SYMBOL_REGISTER)) {
                let operand1_str = &(*self.tokens.peek().unwrap().to_string())[1..];
                let is_u8 = operand1_str.parse::<u8>();
                match is_u8 {
                    Ok(operand1) => {
                        self.tokens.next();
                        if self.tokens.peek().map_or(false, |word| word.starts_with(SYMBOL_LABEL_IMMEDIATE)) {
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
                        } else if self.tokens.peek().map_or(false, |word| word.starts_with(SYMBOL_LABEL_USAGE)) {
                            return self.parse_label_usage();
                        } else {
                            return Err("An Immediate number is expected(e.g. #1)");
                        }
                    }
                    Err(_e) => { return Err("An Unsigned Integer is expected(e.g. 1...255)"); }
                }
            } else if self.tokens.peek().map_or(false, |word| word.starts_with(SYMBOL_LABEL_USAGE)) {
                return self.parse_label_usage();
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
    fn should_return_label_declaration_when_parse_label_declaration() {
        let mut instruction_parser = InstructionParser::new("hello:");
        let label = instruction_parser.parse_label_declaration();
        assert_eq!(label.unwrap(), AssemblerInstruction {
            token: None,
            label: Some(LabelDeclaration { name: "hello".to_string() }),
            directive: None,
            operand1: None,
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_label_declaration_when_parse_label_declaration_with_directive() {
        let mut instruction_parser = InstructionParser::new("hello: .asciiz \"Hello, World!\"");
        let label = instruction_parser.parse_label_declaration();
        assert_eq!(label.unwrap(), AssemblerInstruction {
            token: None,
            label: Some(LabelDeclaration { name: "hello".to_string() }),
            directive: Some(Directive { name: "asciiz".to_string() }),
            operand1: Some(IrString { name: "Hello, World!".to_string() }),
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_label_declaration_when_parse_label_declaration_with_instruction() {
        let mut instruction_parser = InstructionParser::new("hello: JMP $0");
        let label = instruction_parser.parse_label_declaration();
        assert_eq!(label.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: OpCode::JMP }),
            label: Some(LabelDeclaration { name: "hello".to_string() }),
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: None,
            operand3: None,
        });
    }


    #[test]
    fn should_return_directive_when_parse_directive() {
        let mut instruction_parser = InstructionParser::new(".asciiz \"Hello, World!\"");
        let label = instruction_parser.parse_directive();
        assert_eq!(label.unwrap(), AssemblerInstruction {
            token: None,
            label: None,
            directive: Some(Directive { name: "asciiz".to_string() }),
            operand1: Some(IrString { name: "Hello, World!".to_string() }),
            operand2: None,
            operand3: None,
        });
    }


    #[test]
    fn should_return_instructions_when_parse_assembly_line() {
        let mut instruction_parser = InstructionParser::new("hello: .asciiz \"Hello, World!\"");
        let label = instruction_parser.parse_assembly_line();
        assert_eq!(label.unwrap(), AssemblerInstruction {
            token: None,
            label: Some(LabelDeclaration { name: "hello".to_string() }),
            directive: Some(Directive { name: "asciiz".to_string() }),
            operand1: Some(IrString { name: "Hello, World!".to_string() }),
            operand2: None,
            operand3: None,
        });
    }
}
