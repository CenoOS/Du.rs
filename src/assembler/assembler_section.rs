use crate::assembler::assembler_phase::AssemblerPhase;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AssemblerSection {
    Data { instruction_starting: Option<u32> },
    Code { instruction_starting: Option<u32> },
    UnKnown,
}

impl<'a> From<&'a str> for AssemblerSection {
    fn from(name: &str) -> AssemblerSection {
        match name {
            "code" => {
                return AssemblerSection::Code { instruction_starting: None };
            }
            "data" => {
                return AssemblerSection::Data { instruction_starting: None };
            }
            _ => {
                return AssemblerSection::UnKnown;
            }
        }
    }
}
