use crate::assembler::assembler_instruction::AssemblerInstruction;
use crate::assembler::symbol_table::SymbolTable;
use crate::assembler::elf::DELFHeader;


pub enum AssemblerPhase {
    FIRST,
    SECOND
}
