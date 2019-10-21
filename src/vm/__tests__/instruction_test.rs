/*
 * Copyright (c) 2019. NeroYang
 */

#[cfg(test)]
mod tests {
    use crate::vm::instruction::Instruction;
    use crate::vm::instruction::OpCode::HLT;

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
