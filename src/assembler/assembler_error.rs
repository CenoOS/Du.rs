use crate::assembler::assembler_instruction::AssemblerInstruction;

#[derive(Clone)]
pub enum AssemblerError {
    ParseError { error: String },
    NoSectionDeclarationFound { instruction: u32 },
    NoLabelNameFound { instruction: u32 },
    SymbolAlreadyDeclared { instruction: u32 },
    NoDirectiveNameFound { instruction: u32 },
    UnknownDirectiveFound { directive: String },
    UnknownSectionFound { section_name: String },
    InsufficientSections,
}
