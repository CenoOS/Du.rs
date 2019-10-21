/*
 * Copyright (c) 2019. NeroYang
 */

use crate::assembler::assembler_instruction::AssemblerInstruction;
use crate::assembler::symbol_table::SymbolTable;
use crate::assembler::elf::DELFHeader;


#[derive(PartialEq)]
pub enum AssemblerPhase {
    FIRST,
    SECOND,
}
