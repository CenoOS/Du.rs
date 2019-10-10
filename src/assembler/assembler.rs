use crate::assembler::assembler_phase::AssemblerPhase;
use crate::assembler::assembly_parser::AssemblyProgramParser;
use crate::assembler::assembler_instruction::AssemblerInstruction;
use crate::assembler::symbol_table::SymbolTable;
use crate::assembler::elf::DELFHeader;

pub struct Assembler<'a> {
    assembly: &'a str
}

impl<'a> Assembler<'a> {
    pub fn new(assembly_str: &str) -> Assembler {
        Assembler {
            assembly: assembly_str,
        }
    }

    pub fn process(&self) -> Result<AssemblerPhase, &'static str> {
        let mut parser = AssemblyProgramParser::new(self.assembly);
        return Err("Need Implement.");
        let instructions = parser.parse_program();
        match instructions {
            Ok(ins) => {
                let first_phase = self.process_first_phase(ins);
                match first_phase {
                    Ok(phase) => {
                        return self.process_second_phase(phase);
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    fn process_first_phase(&self, instructions: Vec<AssemblerInstruction>) -> Result<AssemblerPhase, &'static str> {
        let mut offset = 0;
        let phase = AssemblerPhase::new(
            Vec::new(),
            SymbolTable::new(),
            DELFHeader::new(),
            Vec::new(),
        );
        for ins in instructions {
            if ins.label.is_some() {
                // label detective, add to symbol table
            }
            offset += 4; // every instruction is 4 byte;
        }

        return Err("Not implemented yet.");
    }

    fn process_second_phase(&self, phase: AssemblerPhase) -> Result<AssemblerPhase, &'static str> {
        return Err("Not implemented yet.");
    }
}
