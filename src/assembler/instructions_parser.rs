/*
 * Copyright (c) 2019. NeroYang
 */

use std::iter::{Peekable, Iterator};
use crate::assembler::token::Token::{Op, Register, IntegerOperand, Directive, LabelDeclaration, LabelUsage, IrString};
use crate::vm::instruction::OpCode::*;
use crate::assembler::assembler_instruction::AssemblerInstruction;
use crate::vm::instruction::OpCode;
use std::str::SplitWhitespace;

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
                                                        None,
                                                        None,
                                                        label.operand1,
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
                                                        None,
                                                        None,
                                                        label.operand1,
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
                                                        None,
                                                        None,
                                                        label.operand1,
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
                return Err("Expect a string starts with \" and end with \"");
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
                                            None,
                                            None,
                                            Some(LabelUsage {
                                                name: label_usage.to_string()
                                            }),
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
                            let label_usage = self.parse_label_usage();
                            match label_usage {
                                Ok(label) => {
                                    return Ok(AssemblerInstruction::new(Some(Op { opcode: LOAD }),
                                                                        None,
                                                                        None,
                                                                        Some(Register { reg_num: operand1 }),
                                                                        label.operand1,
                                                                        None));
                                }
                                Err(_e) => { return Err("An Label is expected(e.g. @foo)"); }
                            }
                        } else {
                            return Err("An Immediate number is expected(e.g. #1)");
                        }
                    }
                    Err(_e) => { return Err("An Unsigned Integer is expected(e.g. 1...255)"); }
                }
            } else if self.tokens.peek().map_or(false, |word| word.starts_with(SYMBOL_LABEL_USAGE)) {
                let label_usage = self.parse_label_usage();
                match label_usage {
                    Ok(label) => {
                        self.tokens.next();
                        if self.tokens.peek().map_or(false, |word| word.starts_with(SYMBOL_LABEL_IMMEDIATE)) {
                            let operand2_str = &(*self.tokens.peek().unwrap().to_string())[1..];
                            let is_i32 = operand2_str.parse::<i32>();
                            match is_i32 {
                                Ok(operand2) => {
                                    return Ok(AssemblerInstruction::new(Some(Op { opcode: LOAD }),
                                                                        None,
                                                                        None,
                                                                        label.operand1,
                                                                        Some(IntegerOperand { value: operand2 }),
                                                                        None));
                                }
                                Err(_e) => { return Err("An Unsigned Integer is expected(e.g. 1...65536)"); }
                            }
                        } else if self.tokens.peek().map_or(false, |word| word.starts_with(SYMBOL_LABEL_USAGE)) {
                            let label_usage2 = self.parse_label_usage();
                            match label_usage2 {
                                Ok(label2) => {
                                    return Ok(AssemblerInstruction::new(Some(Op { opcode: LOAD }),
                                                                        None,
                                                                        None,
                                                                        label.operand1,
                                                                        label2.operand1,
                                                                        None));
                                }
                                Err(_e) => { return Err("An Label is expected(e.g. @foo)"); }
                            }
                        } else {
                            return Err("An Immediate number is expected(e.g. #1)");
                        }
                    }
                    Err(e) => { return Err(e); }
                }
            } else {
                return Err("An Register / Label is expected(e.g. $1 / @hello)");
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

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "EQ".to_string()) {
            self.tokens.next();
            return self.parse_two_register_instruction(EQ);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "LT".to_string()) {
            self.tokens.next();
            return self.parse_two_register_instruction(LT);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "LTE".to_string()) {
            self.tokens.next();
            return self.parse_two_register_instruction(LTE);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "GT".to_string()) {
            self.tokens.next();
            return self.parse_two_register_instruction(GT);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "GTE".to_string()) {
            self.tokens.next();
            return self.parse_two_register_instruction(GTE);
        }

        // todo : loadf64

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "ADDF64".to_string()) {
            self.tokens.next();
            return self.parse_three_register_instruction(ADDF64);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "SUBF64".to_string()) {
            self.tokens.next();
            return self.parse_three_register_instruction(SUBF64);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "MULF64".to_string()) {
            self.tokens.next();
            return self.parse_three_register_instruction(MULF64);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "DIVF64".to_string()) {
            self.tokens.next();
            return self.parse_three_register_instruction(DIVF64);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "EQF64".to_string()) {
            self.tokens.next();
            return self.parse_two_register_instruction(EQF64);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "LTF64".to_string()) {
            self.tokens.next();
            return self.parse_two_register_instruction(LTF64);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "LTEF64".to_string()) {
            self.tokens.next();
            return self.parse_two_register_instruction(LTEF64);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "GTF64".to_string()) {
            self.tokens.next();
            return self.parse_two_register_instruction(GTF64);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "GTEF64".to_string()) {
            self.tokens.next();
            return self.parse_two_register_instruction(GTEF64);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "JE".to_string()) {
            self.tokens.next();
            return self.parse_one_register_instruction(JE);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "JNE".to_string()) {
            self.tokens.next();
            return self.parse_one_register_instruction(JNE);
        }


        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "JL".to_string()) {
            self.tokens.next();
            return self.parse_one_register_instruction(JL);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "JG".to_string()) {
            self.tokens.next();
            return self.parse_one_register_instruction(JG);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "INC".to_string()) {
            self.tokens.next();
            return self.parse_one_register_instruction(INC);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "DEC".to_string()) {
            self.tokens.next();
            return self.parse_one_register_instruction(DEC);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "JMP".to_string()) {
            self.tokens.next();
            return self.parse_one_register_instruction(JMP);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "JMPF".to_string()) {
            self.tokens.next();
            return self.parse_one_register_instruction(JMPF);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "JMPB".to_string()) {
            self.tokens.next();
            return self.parse_one_register_instruction(JMPB);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "AND".to_string()) {
            self.tokens.next();
            return self.parse_three_register_instruction(AND);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "OR".to_string()) {
            self.tokens.next();
            return self.parse_three_register_instruction(OR);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "XOR".to_string()) {
            self.tokens.next();
            return self.parse_three_register_instruction(XOR);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "NOT".to_string()) {
            self.tokens.next();
            return self.parse_two_register_instruction(NOT);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "PUSH".to_string()) {
            self.tokens.next();
            return self.parse_one_register_instruction(PUSH);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "POP".to_string()) {
            self.tokens.next();
            return self.parse_one_register_instruction(POP);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "CALL".to_string()) {
            self.tokens.next();
            return self.parse_one_register_instruction(CALL);
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "RET".to_string()) {
            return Ok(AssemblerInstruction::new(Some(Op { opcode: RET }),
                                                None,
                                                None,
                                                None,
                                                None,
                                                None));
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "PRTS".to_string()) {
            self.tokens.next();
            return self.parse_one_register_instruction(PRTS);
        }

        return Err("Unexpected Assembly Code.");
    }
}
