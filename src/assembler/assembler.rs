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

    pub fn process(&mut self, assembly: &str) -> Result<Vec<u8>, Vec<AssemblerError>> {
        let mut parser = AssemblyProgramParser::new(assembly);
        let instructions = parser.parse_program();
        match instructions {
            Ok(ins) => {
                let mut assembled_program: Vec<u8> = self.write_delf_header();
                self.process_first_phase(&ins);

                if !self.errors.is_empty() {
                    return Err(self.errors.clone());
                }

                if self.sections.len() < 2 { // at lease code and data section are need.
                    self.errors.push(AssemblerError::InsufficientSections);
                    return Err(self.errors.clone());
                }

                let mut body: Vec<u8> = self.process_second_phase(&ins);

                assembled_program.append(&mut body);
                return Ok(assembled_program);
            }
            Err(e) => {
                return Err(vec![AssemblerError::ParseError { error: e.to_string() }]);
            }
        }
    }

    fn process_label_declaration(&mut self, instruction: &AssemblerInstruction) {
        match instruction.get_label_name() {
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

    fn handle_asciiz(&mut self, instruction: &AssemblerInstruction) {
        if self.assemble_phase != AssemblerPhase::FIRST { return; }
        match instruction.get_string_constant() {
            Some(s) => {
                match instruction.get_label_name() {
                    Some(name) => {
                        self.symbol_table.set_symbol_offset(&name, self.ro_offset);
                    }
                    None => { self.errors.push(AssemblerError::LabelNotFoundForStringConstant); }
                }
                for byte in s.as_bytes() {
                    self.ro_section.push(*byte);
                    self.ro_offset += 1;
                }
                self.ro_section.push(0x0); // end of zero
                self.ro_offset += 1;
            }
            None => {
                self.errors.push(AssemblerError::StringConstantNotFound);
            }
        }
    }

    fn process_section_header(&mut self, header_name: &str) {
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

    // scan symbol declaration to symbol table,and sections
    fn process_first_phase(&mut self, instructions: &Vec<AssemblerInstruction>) {
        for instruction in instructions {
            if instruction.is_label() {
                if self.current_section.is_some() {
                    self.process_label_declaration(instruction);
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

        for instruction in instructions {
            if instruction.is_opcode() {
                let mut bytes = instruction.to_bytes();
                program.append(&mut bytes);
            }

            if instruction.is_directive() {
                self.process_directive(&instruction);
            }
            self.current_instruction += 1;
        }
        return program;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::assembler_section::AssemblerSection::{Code, Data};

    #[test]
    fn should_write_elf_header() {
        let mut assembler = Assembler::new();
        let header = assembler.write_delf_header();
        assert_eq!(header, vec![0x64, 0x65, 0x6c, 0x66, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, ])
    }

    #[test]
    fn should_process_program() {
        let mut assembler = Assembler::new();
        let result = assembler.process(
            ".code\n\
                    main:   load $1 #300\n\
                            add $0 $1 $2\n\
                            sub $0 $1 $2\n\
                            mul $0 $1     $2\n\
                            div $0 $1 $2\n\
                    hello:  jmp $0\n\
                            jmp_f $1\n\
                            jmp_b $1\n\
                            eq $1 $2\n\
                            jeq $0\n\
                            hlt\n\
                 .data\n\
                    hw:     .asciiz \"hello,World\"\n\
                    about:  .asciiz \"hello, I am Nero Yang\"");

        assert_eq!(assembler.current_section, Some(Data { instruction_starting: None }));
        assert_eq!(assembler.symbol_table.get_symbol("main"), Some(&Symbol::new("main".to_string(), 0, SymbolType::Label)));
        assert_eq!(assembler.symbol_table.get_symbol("hello"), Some(&Symbol::new("hello".to_string(), 0, SymbolType::Label)));
        assert_eq!(assembler.symbol_table.get_symbol("hw"), Some(&Symbol::new("hw".to_string(), 0, SymbolType::Label)));
        assert_eq!(assembler.symbol_table.get_symbol("about"), Some(&Symbol::new("about".to_string(), 12, SymbolType::Label)));


        assert_eq!(assembler.ro_section, vec![
            0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x00,
            0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x49, 0x20, 0x61, 0x6d, 0x20, 0x4e, 0x65, 0x72, 0x6f, 0x20, 0x59, 0x61, 0x6e, 0x67, 0x00
        ]);
    }

    #[test]
    fn should_process_section_header() {
        let mut assembler = Assembler::new();
        assembler.process_section_header("code");
        assert_eq!(assembler.current_section, Some(Code { instruction_starting: None }));

        assembler.process_section_header("data");
        assert_eq!(assembler.current_section, Some(Data { instruction_starting: None }));

        assembler.process_section_header("bss");
        assert_eq!(assembler.current_section, Some(Data { instruction_starting: None }));
    }
}
