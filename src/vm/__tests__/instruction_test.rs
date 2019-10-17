/*
 * Copyright (c) 2019. NeroYang
 */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::instruction::OpCode::{IGL, HLT};
    use crate::vm::instruction::Instruction;

    #[test]
    fn should_create_opcode() {
        let opcode = HLT;
        assert_eq!(opcode, HLT);
    }

    #[test]
    fn should_create_instruction() {
        let instruction = Instruction::new(HLT);
        assert_eq!(instruction.opcode, HLT);
    }
}
