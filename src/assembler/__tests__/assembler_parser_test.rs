/*
 * Copyright (c) 2019. NeroYang
 */


#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::assembly_parser::AssemblyProgramParser;
    use crate::assembler::assembler_instruction::AssemblerInstruction;
    use crate::assembler::token::Token::{Directive, LabelDeclaration, Op, Register, IntegerOperand, IrString, LabelUsage};
    use crate::vm::instruction::OpCode::*;

    #[test]
    fn should_return_instruction_list_when_give_assembly_code() {
        let mut assembler = AssemblyProgramParser::new(
            ".code\n\
                    main:   load $1 #300\n\
                            add $0 $1 $2\n\
                            sub $0 $1 $2\n\
                            mul $0 $1     $2\n\
                            div $0 $1 $2\n\
                    hello:  jmp $1\n\
                            jmpf $1\n\
                            jmpb $1\n\
                            eq $1 $2\n\
                            je $1\n\
                            hlt\n\
                 .data\n\
                    hw:     .asciiz \"hello,World\"\n\
                    about:  .asciiz \"hello, I am Nero Yang\"");
        let instructions = assembler.parse_program().unwrap();
        assert_eq!(instructions[0], AssemblerInstruction {
            token: None,
            label: None,
            directive: Some(Directive { name: "code".to_string() }),
            operand1: None,
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[1], AssemblerInstruction {
            token: Some(Op { opcode: LOAD }),
            label: Some(LabelDeclaration { name: "main".to_string() }),
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(IntegerOperand { value: 300 }),
            operand3: None,
        });
        assert_eq!(instructions[2], AssemblerInstruction {
            token: Some(Op { opcode: ADD }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
        assert_eq!(instructions[3], AssemblerInstruction {
            token: Some(Op { opcode: SUB }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
        assert_eq!(instructions[4], AssemblerInstruction {
            token: Some(Op { opcode: MUL }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
        assert_eq!(instructions[5], AssemblerInstruction {
            token: Some(Op { opcode: DIV }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
        assert_eq!(instructions[6], AssemblerInstruction {
            token: Some(Op { opcode: JMP }),
            label: Some(LabelDeclaration { name: "hello".to_string() }),
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[7], AssemblerInstruction {
            token: Some(Op { opcode: JMPF }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[8], AssemblerInstruction {
            token: Some(Op { opcode: JMPB }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[9], AssemblerInstruction {
            token: Some(Op { opcode: EQ }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        });
        assert_eq!(instructions[10], AssemblerInstruction {
            token: Some(Op { opcode: JE }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[11], AssemblerInstruction {
            token: Some(Op { opcode: HLT }),
            label: None,
            directive: None,
            operand1: None,
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[12], AssemblerInstruction {
            token: None,
            label: None,
            directive: Some(Directive { name: "data".to_string() }),
            operand1: None,
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[13], AssemblerInstruction {
            token: None,
            label: Some(LabelDeclaration { name: "hw".to_string() }),
            directive: Some(Directive { name: "asciiz".to_string() }),
            operand1: Some(IrString { name: "hello,World".to_string() }),
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[14], AssemblerInstruction {
            token: None,
            label: Some(LabelDeclaration { name: "about".to_string() }),
            directive: Some(Directive { name: "asciiz".to_string() }),
            operand1: Some(IrString { name: "hello, I am Nero Yang".to_string() }),
            operand2: None,
            operand3: None,
        });
    }

    #[test]
    fn should_return_instruction_list_when_give_assembly_code_with_label() {
        let mut assembler = AssemblyProgramParser::new(
            ".code\n\
                    main:   load $1 @bar\n\
                            add $0 $1 $2\n\
                            sub $0 $1 $2\n\
                            mul $0 $1     $2\n\
                            div $0 $1 $2\n\
                    hello:  jmp @foo\n\
                            jmpf $1\n\
                            jmpb $1\n\
                            eq $1 $2\n\
                            je @flag\n\
                            prts @hw\n\
                            hlt\n\
                 .data\n\
                    hw:     .asciiz \"hello,World\"\n\
                    about:  .asciiz \"hello, I am Nero Yang\"");
        let instructions = assembler.parse_program().unwrap();
        assert_eq!(instructions[0], AssemblerInstruction {
            token: None,
            label: None,
            directive: Some(Directive { name: "code".to_string() }),
            operand1: None,
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[1], AssemblerInstruction {
            token: Some(Op { opcode: LOAD }),
            label: Some(LabelDeclaration { name: "main".to_string() }),
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(LabelUsage { name: "bar".to_string() }),
            operand3: None,
        });
        assert_eq!(instructions[2], AssemblerInstruction {
            token: Some(Op { opcode: ADD }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
        assert_eq!(instructions[3], AssemblerInstruction {
            token: Some(Op { opcode: SUB }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
        assert_eq!(instructions[4], AssemblerInstruction {
            token: Some(Op { opcode: MUL }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
        assert_eq!(instructions[5], AssemblerInstruction {
            token: Some(Op { opcode: DIV }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
        assert_eq!(instructions[6], AssemblerInstruction {
            token: Some(Op { opcode: JMP }),
            label: Some(LabelDeclaration { name: "hello".to_string() }),
            directive: None,
            operand1: Some(LabelUsage { name: "foo".to_string() }),
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[7], AssemblerInstruction {
            token: Some(Op { opcode: JMPF }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[8], AssemblerInstruction {
            token: Some(Op { opcode: JMPB }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[9], AssemblerInstruction {
            token: Some(Op { opcode: EQ }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        });
        assert_eq!(instructions[10], AssemblerInstruction {
            token: Some(Op { opcode: JE }),
            label: None,
            directive: None,
            operand1: Some(LabelUsage { name: "flag".to_string() }),
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[11], AssemblerInstruction {
            token: Some(Op { opcode: PRTS }),
            label: None,
            directive: None,
            operand1: Some(LabelUsage { name: "hw".to_string() }),
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[12], AssemblerInstruction {
            token: Some(Op { opcode: HLT }),
            label: None,
            directive: None,
            operand1: None,
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[13], AssemblerInstruction {
            token: None,
            label: None,
            directive: Some(Directive { name: "data".to_string() }),
            operand1: None,
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[14], AssemblerInstruction {
            token: None,
            label: Some(LabelDeclaration { name: "hw".to_string() }),
            directive: Some(Directive { name: "asciiz".to_string() }),
            operand1: Some(IrString { name: "hello,World".to_string() }),
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[15], AssemblerInstruction {
            token: None,
            label: Some(LabelDeclaration { name: "about".to_string() }),
            directive: Some(Directive { name: "asciiz".to_string() }),
            operand1: Some(IrString { name: "hello, I am Nero Yang".to_string() }),
            operand2: None,
            operand3: None,
        });
    }
}
