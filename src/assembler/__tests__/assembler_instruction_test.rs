/*
 * Copyright (c) 2019. NeroYang
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::token::Token::{Op, Register, IntegerOperand, LabelUsage};
    use crate::vm::instruction::OpCode::*;
    use crate::assembler::assembler_instruction::AssemblerInstruction;


    #[test]
    fn should_return_bytes_when_give_hlt() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: HLT }),
            label: None,
            directive: None,
            operand1: None,
            operand2: None,
            operand3: None,
        };
        assert_eq!(token.to_bytes(), vec![0]);
    }

    #[test]
    fn should_return_bytes_when_give_load() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: LOAD }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(IntegerOperand { value: 300 }),
            operand3: None,
        };
        assert_eq!(token.to_bytes(), vec![1, 1, 1, 44]);
    }

    #[test]
    fn should_return_bytes_when_give_add() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: ADD }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        };
        assert_eq!(token.to_bytes(), vec![2, 0, 1, 2]);
    }

    #[test]
    fn should_return_bytes_when_give_sub() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: SUB }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        };
        assert_eq!(token.to_bytes(), vec![3, 0, 1, 2]);
    }

    #[test]
    fn should_return_bytes_when_give_mul() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: MUL }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        };
        assert_eq!(token.to_bytes(), vec![4, 0, 1, 2]);
    }

    #[test]
    fn should_return_bytes_when_give_div() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: DIV }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        };
        assert_eq!(token.to_bytes(), vec![5, 0, 1, 2]);
    }

    #[test]
    fn should_return_bytes_when_give_jmp() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: JMP }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        };
        assert_eq!(token.to_bytes(), vec![6, 1]);
    }

    #[test]
    fn should_return_bytes_when_give_jmpf() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: JMPF }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        };
        assert_eq!(token.to_bytes(), vec![7, 1]);
    }

    #[test]
    fn should_return_bytes_when_give_jmpb() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: JMPB }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        };
        assert_eq!(token.to_bytes(), vec![8, 1]);
    }

    #[test]
    fn should_return_bytes_when_give_eq() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: EQ }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        };
        assert_eq!(token.to_bytes(), vec![9, 1, 2]);
    }

    #[test]
    fn should_return_bytes_when_give_lt() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: LT }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        };
        assert_eq!(token.to_bytes(), vec![18, 1, 2]);
    }

    #[test]
    fn should_return_bytes_when_give_lte() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: LTE }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        };
        assert_eq!(token.to_bytes(), vec![19, 1, 2]);
    }

    #[test]
    fn should_return_bytes_when_give_gt() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: GT }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        };
        assert_eq!(token.to_bytes(), vec![20, 1, 2]);
    }

    #[test]
    fn should_return_bytes_when_give_gte() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: GTE }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        };
        assert_eq!(token.to_bytes(), vec![21, 1, 2]);
    }

    #[test]
    fn should_return_bytes_when_give_jeq() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: JE }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        };
        assert_eq!(token.to_bytes(), vec![10, 1]);
    }

    #[test]
    fn should_return_bytes_when_give_jne() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: JNE }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        };
        assert_eq!(token.to_bytes(), vec![15, 1]);
    }

    #[test]
    fn should_return_bytes_when_give_jl() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: JL }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        };
        assert_eq!(token.to_bytes(), vec![16, 1]);
    }

    #[test]
    fn should_return_bytes_when_give_jg() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: JG }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        };
        assert_eq!(token.to_bytes(), vec![17, 1]);
    }

    #[test]
    fn should_return_bytes_when_give_inc() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: INC }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        };
        assert_eq!(token.to_bytes(), vec![12, 1]);
    }

    #[test]
    fn should_return_bytes_when_give_dec() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: DEC }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        };
        assert_eq!(token.to_bytes(), vec![13, 1]);
    }


    #[test]
    fn should_return_bytes_when_give_and() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: AND }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: Some(Register { reg_num: 0 }),
        };
        assert_eq!(token.to_bytes(), vec![33, 1, 2, 0]);
    }

    #[test]
    fn should_return_bytes_when_give_or() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: OR }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: Some(Register { reg_num: 0 }),
        };
        assert_eq!(token.to_bytes(), vec![34, 1, 2, 0]);
    }

    #[test]
    fn should_return_bytes_when_give_xor() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: XOR }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: Some(Register { reg_num: 0 }),
        };
        assert_eq!(token.to_bytes(), vec![35, 1, 2, 0]);
    }

    #[test]
    fn should_return_bytes_when_give_not() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: NOT }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        };
        assert_eq!(token.to_bytes(), vec![36, 1, 2]);
    }


    #[test]
    fn should_return_bytes_when_give_addf64() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: ADDF64 }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        };
        assert_eq!(token.to_bytes(), vec![23, 0, 1, 2]);
    }

    #[test]
    fn should_return_bytes_when_give_subf64() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: SUBF64 }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        };
        assert_eq!(token.to_bytes(), vec![24, 0, 1, 2]);
    }

    #[test]
    fn should_return_bytes_when_give_mulf64() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: MULF64 }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        };
        assert_eq!(token.to_bytes(), vec![25, 0, 1, 2]);
    }

    #[test]
    fn should_return_bytes_when_give_divf64() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: DIVF64 }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        };
        assert_eq!(token.to_bytes(), vec![26, 0, 1, 2]);
    }

    #[test]
    fn should_return_bytes_when_give_eqf64() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: EQF64 }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        };
        assert_eq!(token.to_bytes(), vec![27, 1, 2]);
    }

    #[test]
    fn should_return_bytes_when_give_ltf64() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: LTF64 }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        };
        assert_eq!(token.to_bytes(), vec![31, 1, 2]);
    }

    #[test]
    fn should_return_bytes_when_give_ltef64() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: LTEF64 }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        };
        assert_eq!(token.to_bytes(), vec![32, 1, 2]);
    }

    #[test]
    fn should_return_bytes_when_give_gtf64() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: GTF64 }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        };
        assert_eq!(token.to_bytes(), vec![29, 1, 2]);
    }

    #[test]
    fn should_return_bytes_when_give_gtef64() {
        let token =  AssemblerInstruction {
            token: Some(Op { opcode: GTEF64 }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        };
        assert_eq!(token.to_bytes(), vec![30, 1, 2]);
    }


    #[test]
    fn should_return_bytes_when_give_push() {
        let token =  AssemblerInstruction {
            token: Some(Op { opcode: PUSH }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        };
        assert_eq!(token.to_bytes(), vec![37,1]);
    }

    #[test]
    fn should_return_bytes_when_give_pop() {
        let token = AssemblerInstruction {
            token: Some(Op { opcode: POP }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        };
        assert_eq!(token.to_bytes(), vec![38,1]);
    }


    #[test]
    fn should_return_bytes_when_give_ret() {
        let token =  AssemblerInstruction {
            token: Some(Op { opcode: RET }),
            label: None,
            directive: None,
            operand1: None,
            operand2: None,
            operand3: None,
        };
        assert_eq!(token.to_bytes(), vec![40]);
    }
}
