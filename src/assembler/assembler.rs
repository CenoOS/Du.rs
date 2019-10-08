use crate::assembler::assembler_instruction::AssemblerInstruction;

pub struct Assembler {
    instructions: Vec<AssemblerInstruction>
}

impl Assembler {
    pub fn new(instructions: Vec<AssemblerInstruction>) -> Assembler {
        Assembler {
            instructions
        }
    }

    pub fn process(&self) -> Result<Vec<u8>, &'static str> {}

    pub fn process_first_phase(&self) {}

    pub fn process_second_phase(&self) {}
}
