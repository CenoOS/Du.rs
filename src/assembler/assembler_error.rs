use crate::assembler::assembler_instruction::AssemblerInstruction;

pub enum AssemblerError {
    NoSectionDeclarationFound { instruction: u32 },
    NoLabelNameFound { instruction: u32 },
    SymbolAlreadyDeclared { instruction: u32 },
    NoDirectiveNameFound { instruction: u32 },
    UnknownDirectiveFound { directive: String },
    UnknownSectionFound { section_name: String },
}
