use crate::assembler::assembler_instruction::AssemblerInstruction;

pub enum AssemblerError {
    NoSectionDeclarationFound { instruction: u32 },
    NoLabelNameFound { instruction: u32 },
    SymbolAlreadyDeclared { instruction: u32 },
}
