/*
 * Copyright (c) 2019. NeroYang
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::token::Token::{Op, Register, IntegerOperand};
    use crate::vm::instruction::OpCode::LOAD;
    use crate::assembler::assembler_instruction::AssemblerInstruction;

    #[test]
    fn should_return_bytes_when_give_an_instruction() {
        let ins = AssemblerInstruction::new(Some(Op { opcode: LOAD }),
                                            None, None,
                                            Some(Register { reg_num: 1 }),
                                            Some(IntegerOperand { value: 500 }),
                                            None);
        let results = ins.to_bytes();
        assert_eq!(results[0], 1);
        assert_eq!(results[1], 1);
        assert_eq!(results[2], 1);
        assert_eq!(results[3], 244);
    }
}
