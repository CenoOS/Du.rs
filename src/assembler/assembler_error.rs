use crate::assembler::assembler_instruction::AssemblerInstruction;
use std::error::Error;
use std::fmt::{Debug, Formatter, Display};
use std::fmt;

#[derive(Debug, Clone)]
pub enum AssemblerError {
    ParseError { error: String },
    NoSectionDeclarationFound { instruction: u32 },
    NoLabelNameFound { instruction: u32 },
    SymbolAlreadyDeclared { instruction: u32 },
    NoDirectiveNameFound { instruction: u32 },
    UnknownDirectiveFound { directive: String },
    UnknownSectionFound { section_name: String },
    InsufficientSections,
    StringConstantNotFound,
    LabelNotFoundForStringConstant,
}

impl Error for AssemblerError {
    fn description(&self) -> &str {
        match self {
            _ => "Error:",
        }
    }
}

impl Display for AssemblerError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            _ => { f.write_str(&format!("Error:")) }
        }
    }
}
