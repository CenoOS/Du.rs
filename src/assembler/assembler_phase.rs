use crate::assembler::assembler_instruction::AssemblerInstruction;
use crate::assembler::symbol_table::SymbolTable;
use crate::assembler::elf::DELFHeader;


pub struct AssemblerPhase {
    instructions: Vec<AssemblerInstruction>,
    pub symbol_table: SymbolTable,
    elf_header: DELFHeader,

    ro_segment: Vec<u8>,
}

impl AssemblerPhase {
    pub fn new(instructions: Vec<AssemblerInstruction>,
               symbol_table: SymbolTable,
               elf_header: DELFHeader,
               ro_segment: Vec<u8>) -> AssemblerPhase {
        AssemblerPhase {
            instructions,
            symbol_table,
            elf_header,
            ro_segment,
        }
    }
}
