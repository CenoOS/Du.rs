/*
 * Copyright (c) 2019. NeroYang
 */

use crate::assembler::assembler_phase::AssemblerPhase;
use crate::assembler::assembly_parser::AssemblyProgramParser;
use crate::assembler::assembler_instruction::AssemblerInstruction;
use crate::assembler::symbol_table::{SymbolTable, Symbol, SymbolType};
use crate::assembler::elf::{ELF_HEADER_PREFIX, ELF_HEADER_LENGTH};
use crate::assembler::token::Token;
use crate::assembler::assembler_phase::AssemblerPhase::FIRST;
use crate::assembler::assembler_section::AssemblerSection;
use crate::assembler::assembler_error::AssemblerError;
use crate::assembler::assembler_error::AssemblerError::{
    NoSectionDeclarationFound,
    NoLabelNameFound,
    SymbolAlreadyDeclared,
    UnknownDirectiveFound,
    UnknownSectionFound,
};
use crate::assembler::assembler_section::AssemblerSection::UnKnown;
use crate::assembler::token::Token::{IntegerOperand, Op, Register};
use crate::vm::instruction::OpCode;
use crate::vm::vm::TMP_REGISTER;

pub struct Assembler {
    assemble_phase: AssemblerPhase,
    pub symbol_table: SymbolTable,
    pub ro_section: Vec<u8>,
    byte_code: Vec<u8>,
    ro_offset: u32,
    sections: Vec<AssemblerSection>,
    pub(crate) current_section: Option<AssemblerSection>,
    current_instruction: u32,
    errors: Vec<AssemblerError>,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            assemble_phase: FIRST,
            symbol_table: SymbolTable::new(),
            ro_section: Vec::new(),
            byte_code: Vec::new(),
            ro_offset: 0,
            sections: Vec::new(),
            current_section: None,
            current_instruction: 0,
            errors: Vec::new(),
        }
    }

    pub(crate) fn write_delf_header(&self) -> Vec<u8> {
        let mut header: Vec<u8> = Vec::<u8>::new();
        for byte in ELF_HEADER_PREFIX.into_iter() {
            header.push(byte.clone());
        }
        while header.len() < ELF_HEADER_LENGTH {
            header.push(0xFF as u8);
        }

        return header;
    }

    pub fn process(&mut self, assembly: &str) -> Result<Vec<u8>, Vec<AssemblerError>> {
        let mut parser = AssemblyProgramParser::new(assembly);
        let mut instructions = parser.parse_program();
        match instructions {
            Ok(ins) => {
                return self.process_instructions(&ins);
            }
            Err(e) => {
                return Err(vec![AssemblerError::ParseError { error: e.to_string() }]);
            }
        }
    }

    pub fn process_instructions(&mut self, instructions: &Vec<AssemblerInstruction>) -> Result<Vec<u8>, Vec<AssemblerError>> {
        let mut assembled_program: Vec<u8> = self.write_delf_header();
        self.process_first_phase(&instructions);

        if !self.errors.is_empty() {
            return Err(self.errors.clone());
        }

        if self.sections.len() < 2 { // at lease code and data section are need.
            self.errors.push(AssemblerError::InsufficientSections);
            return Err(self.errors.clone());
        }

        let mut body: Vec<u8> = self.process_second_phase(&instructions);

        assembled_program.append(&mut body);
        return Ok(assembled_program);
    }

    fn process_label_declaration(&mut self, instruction: &AssemblerInstruction) {
        match instruction.get_label_declaration_name() {
            Some(name) => {
                if self.symbol_table.get_symbol(&name).is_none() {
                    let symbol = Symbol::default(name.to_string(), SymbolType::Label);
                    self.symbol_table.add_symbol(symbol);
                } else {
                    self.errors.push(SymbolAlreadyDeclared { instruction: self.current_instruction })
                }
            }
            None => {
                self.errors.push(NoLabelNameFound { instruction: self.current_instruction })
            }
        }
    }

    fn process_label_declaration_second_phase(&mut self, instruction: &AssemblerInstruction, offset: usize) {
        if instruction.token.is_some() {
            match instruction.get_label_declaration_name() {
                Some(name) => {
                    if self.symbol_table.get_symbol(&name).is_some() {
                        self.symbol_table.set_symbol_offset(&name, offset as u32);
                    }
                }
                None => {
                    self.errors.push(NoLabelNameFound { instruction: self.current_instruction })
                }
            }
        }
    }

    fn handle_asciiz(&mut self, instruction: &AssemblerInstruction) {
        if self.assemble_phase != AssemblerPhase::FIRST { return; }
        match instruction.get_string_constant() {
            Some(s) => {
                match instruction.get_label_declaration_name() {
                    Some(name) => {
                        self.symbol_table.set_symbol_offset(&name, self.ro_offset);
                    }
                    None => { self.errors.push(AssemblerError::LabelNotFoundForStringConstant); }
                }
                for byte in s.as_bytes() {
                    if *byte == 92 {
                        let byte_addr: *const u8 = unsafe { byte as *const u8 };
                        let byte_addr = unsafe { byte_addr.add(1) };
                        if unsafe { *byte_addr } == 110 {
                            self.ro_section.push(0xA);
                            self.ro_offset += 1;
                        } else {
                            self.ro_section.push(*byte);
                            self.ro_offset += 1;
                        }
                    } else {
                        if *byte == 110 {
                            let byte_addr: *const u8 = unsafe { byte as *const u8 };
                            let byte_addr = unsafe { byte_addr.sub(1) };
                            if unsafe { *byte_addr } != 92 {
                                self.ro_section.push(*byte);
                                self.ro_offset += 1;
                            }
                        } else {
                            self.ro_section.push(*byte);
                            self.ro_offset += 1;
                        }
                    }
                }
                self.ro_section.push(0x0); // end of zero
                self.ro_offset += 1;
            }
            None => {
                self.errors.push(AssemblerError::StringConstantNotFound);
            }
        }
    }

    pub(crate) fn process_section_header(&mut self, header_name: &str) {
        let new_section: AssemblerSection = header_name.into();
        if new_section == AssemblerSection::UnKnown {
            self.errors.push(UnknownSectionFound { section_name: header_name.to_string() });
            return;
        }

        self.sections.push(new_section);
        self.current_section = Some(new_section);
    }

    fn process_directive(&mut self, instruction: &AssemblerInstruction) {
        if instruction.has_operands() {
            match instruction.get_directive_name().unwrap().as_ref() {
                "asciiz" => {
                    self.handle_asciiz(instruction);
                }
                _ => {
                    self.errors.push(UnknownDirectiveFound {
                        directive: instruction.get_directive_name().unwrap().clone()
                    });
                }
            }
        } else {
            self.process_section_header(instruction.get_directive_name().unwrap().as_ref());
        }
    }

    fn process_label_usage(&mut self, instruction: &AssemblerInstruction) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();

        match &instruction.operand1 {
            Some(Token::LabelUsage { name }) => { self.process_label_offset(&instruction, &mut bytes, &name) }
            _ => {}
        }

        match &instruction.operand2 {
            Some(Token::LabelUsage { name }) => { self.process_label_offset(&instruction, &mut bytes, &name) }
            _ => {}
        }

        match &instruction.operand3 {
            Some(Token::LabelUsage { name }) => { self.process_label_offset(&instruction, &mut bytes, &name) }
            _ => {}
        }

        return bytes;
    }

    fn process_label_offset(&mut self, instruction: &AssemblerInstruction, bytes: &mut Vec<u8>, name: &&String) -> () {
        let offset = self.symbol_table.get_symbol_offset(&name).unwrap();
        let save_offset_instruction = AssemblerInstruction {
            token: Some(Op { opcode: OpCode::LOAD }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: TMP_REGISTER }),
            operand2: Some(IntegerOperand { value: offset as i32 }),
            operand3: None,
        };
        bytes.append(&mut save_offset_instruction.to_bytes());
        match &instruction.token {
            Some(Token::Op { opcode }) => match opcode {
                _ => { bytes.push(*opcode as u8) }
            },
            _ => {
                println!("None opCode found in opCode field.");
                std::process::exit(0);
            }
        }
        bytes.push(TMP_REGISTER);
    }

    // scan symbol declaration to symbol table,and sections
    fn process_first_phase(&mut self, instructions: &Vec<AssemblerInstruction>) {
        for instruction in instructions {
            if instruction.is_label_declaration() {
                if self.current_section.is_some() {
                    self.process_label_declaration(&instruction);
                } else {
                    self.errors.push(NoSectionDeclarationFound { instruction: self.current_instruction })
                }
            }
            if instruction.is_directive() {
                self.process_directive(&instruction);
            }

            self.current_instruction += 1;
        }
        self.assemble_phase = AssemblerPhase::SECOND;
    }

    // translate symbol usage to memory offset
    fn process_second_phase(&mut self, instructions: &Vec<AssemblerInstruction>) -> Vec<u8> {
        self.current_instruction = 0;
        let mut program = Vec::<u8>::new();

        for mut instruction in instructions {
            if instruction.is_label_declaration() {
                self.process_label_declaration_second_phase(&instruction, program.len());
            }
            if instruction.is_opcode() {
                if instruction.is_label_usage() {
                    let mut bytes = self.process_label_usage(&instruction);
                    program.append(&mut bytes);
                } else {
                    let mut bytes = instruction.to_bytes();
                    program.append(&mut bytes);
                }
            }

            if instruction.is_directive() {
                self.process_directive(&instruction);
            }

            self.current_instruction += 1;
        }
        return program;
    }
}
