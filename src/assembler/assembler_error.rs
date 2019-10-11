use crate::assembler::assembler_instruction::AssemblerInstruction;

pub enum AssemblerError {
    NoSectionDeclarationFound { instruction: u32 }
}
