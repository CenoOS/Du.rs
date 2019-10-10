use crate::assembler::assembler_instruction::AssemblerInstruction;
use crate::assembler::symbol_table::SymbolTable;
use crate::assembler::elf::DELFHeader;


pub struct AssemblerPhase {
    instructions: Vec<AssemblerInstruction>,
    symbol_table: SymbolTable,
    elf_header: DELFHeader,

    ro_segment: Vec<u8>,
}
