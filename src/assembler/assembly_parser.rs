/*
 * Copyright (c) 2019. NeroYang
 */

use std::iter::Peekable;
use crate::assembler::assembler_instruction::AssemblerInstruction;
use crate::assembler::instructions_parser::InstructionParser;
use std::str::Lines;

pub struct AssemblyProgramParser<'a> {
    instructions: Peekable<Lines<'a>>,
}

impl<'a> AssemblyProgramParser<'a> {
    pub fn new(str: &str) -> AssemblyProgramParser {
        AssemblyProgramParser {
            instructions: str.lines().peekable()
        }
    }

    pub fn parse_program(&mut self) -> Result<Vec<AssemblerInstruction>, &'static str> {
        let mut assembler_instructions: Vec<AssemblerInstruction> = Vec::new();
        while self.instructions.peek().is_some() {
            match self.instructions.peek() {
                Some(instruction_str) => {
                    let mut instruction_parser = InstructionParser::new(instruction_str);
                    let instruction = instruction_parser.parse_assembly_line();
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
