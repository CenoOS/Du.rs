/*
 * Copyright (c) 2019. NeroYang
 */


#[cfg(test)]
mod tests {
    use crate::assembler::instructions_parser::InstructionParser;
    use crate::assembler::assembler_instruction::AssemblerInstruction;
    use crate::vm::instruction::OpCode;
    use crate::assembler::token::Token::{Op, LabelDeclaration, Register, LabelUsage, IntegerOperand, Directive, IrString};
    use crate::vm::instruction::OpCode::*;

    #[test]
    fn should_return_hlt_when_give_hlt() {
        let mut token_parser = InstructionParser::new("hlt");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: HLT }),
            label: None,
            directive: None,
            operand1: None,
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_load_when_give_load() {
        let mut token_parser = InstructionParser::new("load $1 #300");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: LOAD }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(IntegerOperand { value: 300 }),
            operand3: None,
        });
    }

    #[test]
    fn should_return_add_when_give_add() {
        let mut token_parser = InstructionParser::new("add $0 $1 $2");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: ADD }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
    }

    #[test]
    fn should_return_sub_when_give_sub() {
        let mut token_parser = InstructionParser::new("sub $0 $1 $2");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: SUB }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
    }

    #[test]
    fn should_return_mul_when_give_mul() {
        let mut token_parser = InstructionParser::new("mul $0 $1     $2");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: MUL }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
    }

    #[test]
    fn should_return_div_when_give_div() {
        let mut token_parser = InstructionParser::new("div $0 $1 $2");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: DIV }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
    }

    #[test]
    fn should_return_jmp_when_give_jmp() {
        let mut token_parser = InstructionParser::new("jmp $1");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: JMP }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_jmpf_when_give_jmpf() {
        let mut token_parser = InstructionParser::new("jmpf $1");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: JMPF }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_jmpb_when_give_jmpb() {
        let mut token_parser = InstructionParser::new("jmpb $1");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: JMPB }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_eq_when_give_eq() {
        let mut token_parser = InstructionParser::new("eq $1 $2");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: EQ }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        });
    }

    #[test]
    fn should_return_lt_when_give_lt() {
        let mut token_parser = InstructionParser::new("lt $1 $2");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: LT }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        });
    }

    #[test]
    fn should_return_lte_when_give_lte() {
        let mut token_parser = InstructionParser::new("lte $1 $2");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: LTE }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        });
    }

    #[test]
    fn should_return_gt_when_give_gt() {
        let mut token_parser = InstructionParser::new("gt $1 $2");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: GT }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        });
    }

    #[test]
    fn should_return_gte_when_give_gte() {
        let mut token_parser = InstructionParser::new("gte $1 $2");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: GTE }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        });
    }

    #[test]
    fn should_return_jeq_when_give_jeq() {
        let mut token_parser = InstructionParser::new("je $1");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: JE }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_jne_when_give_jne() {
        let mut token_parser = InstructionParser::new("jne $1");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: JNE }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_jl_when_give_jl() {
        let mut token_parser = InstructionParser::new("jl $1");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: JL }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_jg_when_give_jg() {
        let mut token_parser = InstructionParser::new("jg $1");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: JG }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_inc_when_give_inc() {
        let mut token_parser = InstructionParser::new("inc $1");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: INC }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_dec_when_give_dec() {
        let mut token_parser = InstructionParser::new("dec $1");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: DEC }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
    }


    #[test]
    fn should_return_and_when_give_and() {
        let mut token_parser = InstructionParser::new("and $1 $2 $0");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: AND }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: Some(Register { reg_num: 0 }),
        });
    }

    #[test]
    fn should_return_or_when_give_or() {
        let mut token_parser = InstructionParser::new("or $1 $2 $0");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: OR }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: Some(Register { reg_num: 0 }),
        });
    }

    #[test]
    fn should_return_xor_when_give_xor() {
        let mut token_parser = InstructionParser::new("xor $1 $2 $0");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: XOR }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: Some(Register { reg_num: 0 }),
        });
    }

    #[test]
    fn should_return_not_when_give_not() {
        let mut token_parser = InstructionParser::new("not $1 $2");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: NOT }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        });
    }


    #[test]
    fn should_return_addf64_when_give_addf64() {
        let mut token_parser = InstructionParser::new("addf64 $0 $1 $2");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: ADDF64 }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
    }

    #[test]
    fn should_return_subf64_when_give_subf64() {
        let mut token_parser = InstructionParser::new("subf64 $0 $1 $2");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: SUBF64 }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
    }

    #[test]
    fn should_return_mulf64_when_give_mulf64() {
        let mut token_parser = InstructionParser::new("mulf64 $0 $1     $2");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: MULF64 }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
    }

    #[test]
    fn should_return_divf64_when_give_divf64() {
        let mut token_parser = InstructionParser::new("divf64 $0 $1 $2");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: DIVF64 }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
    }

    #[test]
    fn should_return_eqf64_when_give_eqf64() {
        let mut token_parser = InstructionParser::new("eqf64 $1 $2");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: EQF64 }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        });
    }

    #[test]
    fn should_return_ltf64_when_give_ltf64() {
        let mut token_parser = InstructionParser::new("ltf64 $1 $2");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: LTF64 }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        });
    }

    #[test]
    fn should_return_ltef64_when_give_ltef64() {
        let mut token_parser = InstructionParser::new("ltef64 $1 $2");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: LTEF64 }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        });
    }

    #[test]
    fn should_return_gtf64_when_give_gtf64() {
        let mut token_parser = InstructionParser::new("gtf64 $1 $2");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: GTF64 }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        });
    }

    #[test]
    fn should_return_gtef64_when_give_gtef64() {
        let mut token_parser = InstructionParser::new("gtef64 $1 $2");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: GTEF64 }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        });
    }




    #[test]
    fn should_return_push_when_give_push() {
        let mut token_parser = InstructionParser::new("push $1");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: PUSH }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_pop_when_give_pop() {
        let mut token_parser = InstructionParser::new("pop $1");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: POP }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_call_when_give_call() {
        let mut token_parser = InstructionParser::new("call @foo");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: CALL }),
            label: None,
            directive: None,
            operand1: Some(LabelUsage { name: "foo".to_string() }),
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_ret_when_give_ret() {
        let mut token_parser = InstructionParser::new("ret");
        let token = token_parser.parse_instruction();
        assert_eq!(token.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: RET }),
            label: None,
            directive: None,
            operand1: None,
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_error_when_give_unexpected_token() {
        let mut token_parser = InstructionParser::new("xxx $1");
        let token = token_parser.parse_instruction();
        assert_eq!(token.is_err(), true);
    }

    #[test]
    fn should_return_label_declaration_when_parse_label_declaration() {
        let mut instruction_parser = InstructionParser::new("hello:");
        let label = instruction_parser.parse_label_declaration();
        assert_eq!(label.unwrap(), AssemblerInstruction {
            token: None,
            label: Some(LabelDeclaration { name: "hello".to_string() }),
            directive: None,
            operand1: None,
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_label_declaration_when_parse_label_declaration_with_directive() {
        let mut instruction_parser = InstructionParser::new("hello: .asciiz \"Hello, World!\"");
        let label = instruction_parser.parse_label_declaration();
        assert_eq!(label.unwrap(), AssemblerInstruction {
            token: None,
            label: Some(LabelDeclaration { name: "hello".to_string() }),
            directive: Some(Directive { name: "asciiz".to_string() }),
            operand1: Some(IrString { name: "Hello, World!".to_string() }),
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_label_declaration_when_parse_label_declaration_with_instruction() {
        let mut instruction_parser = InstructionParser::new("hello: JMP $0");
        let label = instruction_parser.parse_label_declaration();
        assert_eq!(label.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: OpCode::JMP }),
            label: Some(LabelDeclaration { name: "hello".to_string() }),
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_label_declaration_when_parse_label_declaration_with_instruction_and_use_label_as_register() {
        let mut instruction_parser = InstructionParser::new("hello: JMP @foo");
        let label = instruction_parser.parse_label_declaration();
        assert_eq!(label.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: OpCode::JMP }),
            label: Some(LabelDeclaration { name: "hello".to_string() }),
            directive: None,
            operand1: Some(LabelUsage { name: "foo".to_string() }),
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_instruction_when_give_assembly_with_three_label_as_register() {
        let mut instruction_parser = InstructionParser::new("add @bar @foo @sum");
        let label = instruction_parser.parse_assembly_line();
        assert_eq!(label.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: OpCode::ADD }),
            label: None,
            directive: None,
            operand1: Some(LabelUsage { name: "bar".to_string() }),
            operand2: Some(LabelUsage { name: "foo".to_string() }),
            operand3: Some(LabelUsage { name: "sum".to_string() }),
        });
    }


    #[test]
    fn should_return_instruction_when_give_assembly_with_two_label_as_register() {
        let mut instruction_parser = InstructionParser::new("load @age @age_constant");
        let label = instruction_parser.parse_assembly_line();
        assert_eq!(label.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: OpCode::LOAD }),
            label: None,
            directive: None,
            operand1: Some(LabelUsage { name: "age".to_string() }),
            operand2: Some(LabelUsage { name: "age_constant".to_string() }),
            operand3: None,
        });
    }

    #[test]
    fn should_return_load_instruction_when_give_assembly_with_label_as_register_and_immediate() {
        let mut instruction_parser = InstructionParser::new("load @age #300");
        let label = instruction_parser.parse_assembly_line();
        assert_eq!(label.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: OpCode::LOAD }),
            label: None,
            directive: None,
            operand1: Some(LabelUsage { name: "age".to_string() }),
            operand2: Some(IntegerOperand { value: 300 }),
            operand3: None,
        });
    }

    #[test]
    fn should_return_load_instruction_when_give_assembly_with_register_and_label_as_immediate() {
        let mut instruction_parser = InstructionParser::new("load $0 @foo");
        let label = instruction_parser.parse_assembly_line();
        assert_eq!(label.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: OpCode::LOAD }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(LabelUsage { name: "foo".to_string() }),
            operand3: None,
        });
    }

    #[test]
    fn should_return_prts_instruction_when_give_assembly_with_label_of_constant_string() {
        let mut instruction_parser = InstructionParser::new("prts @hw");
        let label = instruction_parser.parse_assembly_line();
        assert_eq!(label.unwrap(), AssemblerInstruction {
            token: Some(Op { opcode: OpCode::PRTS }),
            label: None,
            directive: None,
            operand1: Some(LabelUsage { name: "hw".to_string() }),
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_directive_when_parse_directive() {
        let mut instruction_parser = InstructionParser::new(".asciiz \"Hello, World!\"");
        let label = instruction_parser.parse_directive();
        assert_eq!(label.unwrap(), AssemblerInstruction {
            token: None,
            label: None,
            directive: Some(Directive { name: "asciiz".to_string() }),
            operand1: Some(IrString { name: "Hello, World!".to_string() }),
            operand2: None,
            operand3: None,
        });
    }


    #[test]
    fn should_return_instructions_when_parse_assembly_line() {
        let mut instruction_parser = InstructionParser::new("hello: .asciiz \"Hello, World!\"");
        let label = instruction_parser.parse_assembly_line();
        assert_eq!(label.unwrap(), AssemblerInstruction {
            token: None,
            label: Some(LabelDeclaration { name: "hello".to_string() }),
            directive: Some(Directive { name: "asciiz".to_string() }),
            operand1: Some(IrString { name: "Hello, World!".to_string() }),
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_string_constant_when_parse_assembly_line() {
        let mut instruction_parser = InstructionParser::new("hello: .asciiz \"Hello, World!\"");
        let label = instruction_parser.parse_assembly_line().unwrap();
        assert_eq!("Hello, World!".to_string(), label.get_string_constant().unwrap());
    }
}
