use crate::assembler::assembler_phase::AssemblerPhase;
use crate::assembler::assembly_parser::AssemblyProgramParser;
use crate::assembler::assembler_instruction::AssemblerInstruction;
use crate::assembler::symbol_table::{SymbolTable, Symbol};
use crate::assembler::elf::{DELFHeader, ELF_HEADER_PREFIX, ELF_HEADER_LENGTH};
use crate::assembler::token::Token::LabelDeclaration;
use crate::assembler::assembler_phase::AssemblerPhase::{FIRST, SECOND};
use crate::assembler::assembler_section::AssemblerSection;
use crate::assembler::assembler_error::AssemblerError;
use crate::assembler::assembler_error::AssemblerError::NoSectionDeclarationFound;

pub struct Assembler {
    assemble_phase: AssemblerPhase,
    pub symbol_table: SymbolTable,
    ro_section: Vec<u8>,
    byte_code: Vec<u8>,
    ro_offset: u32,
    sections: Vec<AssemblerSection>,
    current_section: Option<AssemblerSection>,
    current_instruction: u32,
    errors: Vec<AssemblerError>,
}

impl Assembler {
    pub fn new(assembly_str: &str) -> Assembler {
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

    fn write_delf_header(&self) -> Vec<u8> {
        let mut header: Vec<u8> = Vec::<u8>::new();
        for byte in ELF_HEADER_PREFIX.into_iter() {
            header.push(byte.clone());
        }
        while header.len() < ELF_HEADER_LENGTH {
            header.push(0xff as u8);
        }

        return header;
    }

    pub fn process(&mut self, assembly: &str) -> Result<Vec<u8>, &'static str> {
        let mut parser = AssemblyProgramParser::new(assembly);
        let instructions = parser.parse_program();
        match instructions {
            Ok(ins) => {
                let mut assembled_program: Vec<u8> = self.write_delf_header();
                self.process_first_phase(&ins);
                let mut body: Vec<u8> = self.process_second_phase(&ins);

                assembled_program.append(&mut body);
                return Ok(assembled_program);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    // scan symbol declaration to symbol table
    fn process_first_phase(&mut self, instructions: &Vec<AssemblerInstruction>) {
        for ins in instructions {
            if ins.is_label() {
                if self.current_section.is_some() {
//                    self.process_label(&ins);
                } else {
                    self.errors.push(NoSectionDeclarationFound { instruction: self.current_instruction })
                }
            }
            if ins.is_directive() {
//                self.process_directive(ins);
            }

            self.current_instruction += 1;
        }
        self.assemble_phase = SECOND;
    }

    // translate symbol usage to memory offset
    fn process_second_phase(&self, instructions: &Vec<AssemblerInstruction>) -> Vec<u8> {
        return Vec::new();
    }
}
