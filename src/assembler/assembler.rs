use crate::assembler::assembler_phase::AssemblerPhase;
use crate::assembler::assembly_parser::AssemblyProgramParser;

pub struct Assembler<'a> {
    assembly: &'a str
}

impl<'a> Assembler<'a> {
    pub fn new(assembly_str: &str) -> Assembler {
        Assembler {
            assembly: assembly_str,
        }
    }

    pub fn process(&self) -> Result<AssemblerPhase, &'static str> {
        let parser = AssemblyProgramParser::new(self.assembly);
        return Err("Need Implement.");
    }

    pub fn process_first_phase(&self) {}

    pub fn process_second_phase(&self) {}
}
