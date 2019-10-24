/*
 * Copyright (c) 2019. NeroYang
 */

#[cfg(test)]
mod tests {
    use crate::assembler::assembler::Assembler;
    use crate::assembler::assembler_instruction::AssemblerInstruction;
    use crate::assembler::assembler_section::AssemblerSection::{Code, Data};
    use crate::assembler::assembly_parser::AssemblyProgramParser;
    use crate::assembler::symbol_table::{Symbol, SymbolType};
    use crate::assembler::token::Token::{
        Directive, IntegerOperand, LabelDeclaration, LabelUsage, Op, Register,
    };
    use crate::vm::instruction::OpCode;
    use std::string::ToString;

    #[test]
    fn should_write_elf_header() {
        let assembler = Assembler::new();
        let header = assembler.write_delf_header();
        assert_eq!(
            header,
            vec![
                0x64, 0x65, 0x6C, 0x66, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            ]
        )
    }

    #[test]
    fn should_process_program() {
        let mut assembler = Assembler::new();
        assembler.process(
            ".code\n\
             main:   load $1 #300\n\
             add $0 $1 $2\n\
             sub $0 $1 $2\n\
             mul $0 $1     $2\n\
             div $0 $1 $2\n\
             hello:  jmp $0\n\
             jmpf $1\n\
             jmpb $1\n\
             eq $1 $2\n\
             je $0\n\
             hlt\n\
             .data\n\
             hw:     .asciiz \"hello,World\"\n\
             about:  .asciiz \"hello, I am Nero Yang\"",
        );

        assert_eq!(
            assembler.current_section,
            Some(Data {
                instruction_starting: None
            })
        );
        assert_eq!(
            assembler.symbol_table.get_symbol("main"),
            Some(&Symbol::new("main".to_string(), 0, SymbolType::Label))
        );
        assert_eq!(
            assembler.symbol_table.get_symbol("hello"),
            Some(&Symbol::new("hello".to_string(), 20, SymbolType::Label))
        );
        assert_eq!(
            assembler.symbol_table.get_symbol("hw"),
            Some(&Symbol::new("hw".to_string(), 0, SymbolType::Label))
        );
        assert_eq!(
            assembler.symbol_table.get_symbol("about"),
            Some(&Symbol::new("about".to_string(), 12, SymbolType::Label))
        );

        assert_eq!(
            assembler.ro_section,
            vec![
                0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x00, 0x68, 0x65,
                0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x49, 0x20, 0x61, 0x6D, 0x20, 0x4E, 0x65, 0x72, 0x6F,
                0x20, 0x59, 0x61, 0x6E, 0x67, 0x00
            ]
        );
    }

    #[test]
    fn should_process_prts_to_machine_code() {
        let mut assembler = Assembler::new();
        let assembly = ".code\n\
                        main:   load $1 #500\n\
                        prts    @hw\n\
                        prts    @about\n\
                        .data\n\
                        hw:     .asciiz \"hello,World\"\n\
                        about:  .asciiz \"hello, I am Nero Yang\"";
        let mut parser = AssemblyProgramParser::new(assembly);
        let ins = parser.parse_program().unwrap();

        assert_eq!(
            ins[0],
            AssemblerInstruction {
                token: None,
                label: None,
                directive: Some(Directive {
                    name: "code".to_string()
                }),
                operand1: None,
                operand2: None,
                operand3: None,
            }
        );

        assert_eq!(
            ins[1],
            AssemblerInstruction {
                token: Some(Op {
                    opcode: OpCode::LOAD
                }),
                label: Some(LabelDeclaration {
                    name: "main".to_string()
                }),
                directive: None,
                operand1: Some(Register { reg_num: 1 }),
                operand2: Some(IntegerOperand { value: 500 }),
                operand3: None,
            }
        );

        assert_eq!(
            ins[2],
            AssemblerInstruction {
                token: Some(Op {
                    opcode: OpCode::PRTS
                }),
                label: None,
                directive: None,
                operand1: Some(LabelUsage {
                    name: "hw".to_string()
                }),
                operand2: None,
                operand3: None,
            }
        );

        assert_eq!(
            ins[3],
            AssemblerInstruction {
                token: Some(Op {
                    opcode: OpCode::PRTS
                }),
                label: None,
                directive: None,
                operand1: Some(LabelUsage {
                    name: "about".to_string()
                }),
                operand2: None,
                operand3: None,
            }
        );

        let result = assembler.process_instructions(&ins);

        assert_eq!(
            assembler.current_section,
            Some(Data {
                instruction_starting: None
            })
        );
        assert_eq!(
            assembler.symbol_table.get_symbol("main"),
            Some(&Symbol::new("main".to_string(), 0, SymbolType::Label))
        );
        assert_eq!(
            assembler.symbol_table.get_symbol("hw"),
            Some(&Symbol::new("hw".to_string(), 0, SymbolType::Label))
        );
        assert_eq!(
            assembler.symbol_table.get_symbol("about"),
            Some(&Symbol::new("about".to_string(), 12, SymbolType::Label))
        );

        assert_eq!(
            assembler.ro_section,
            vec![
                0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x00, 0x68, 0x65,
                0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x49, 0x20, 0x61, 0x6D, 0x20, 0x4E, 0x65, 0x72, 0x6F,
                0x20, 0x59, 0x61, 0x6E, 0x67, 0x00
            ]
        );

        assert_eq!(
            result.unwrap(),
            vec![
                0x64, 0x65, 0x6C, 0x66, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, 0x01, 0x01, 0xF4, 0x01, 0x1F,
                0x00, 0x00, 0x0E, 0x1F, 0x01, 0x1F, 0x00, 0x0C, 0x0E, 0x1F
            ]
        );
    }

    #[test]
    fn should_process_program_to_machine_code() {
        let mut assembler = Assembler::new();
        let result = assembler.process(
            ".code\n\
             main:   load    $1  #500\n\
             add     $0  $1  $2\n\
             sub     $0  $1  $2\n\
             mul     $0  $1  $2\n\
             div     $0  $1  $2\n\
             hello:  jmp     $0\n\
             jmpf   $1\n\
             jmpb   $1\n\
             eq $1   $2\n\
             je     @hello\n\
             prts    @hw\n\
             prts    @about\n\
             foo:    load    $0  #500\n\
             add     $0  $1  $0\n\
             jmp     @foo\n\
             hlt\n\
             .data\n\
             hw:     .asciiz \"hello,World\"\n\
             about:  .asciiz \"hello, I am Nero Yang\"",
        );

        assert_eq!(
            assembler.current_section,
            Some(Data {
                instruction_starting: None
            })
        );
        assert_eq!(
            assembler.symbol_table.get_symbol("main"),
            Some(&Symbol::new("main".to_string(), 0, SymbolType::Label))
        );
        assert_eq!(
            assembler.symbol_table.get_symbol("hello"),
            Some(&Symbol::new("hello".to_string(), 20, SymbolType::Label))
        );
        assert_eq!(
            assembler.symbol_table.get_symbol("foo"),
            Some(&Symbol::new("foo".to_string(), 47, SymbolType::Label))
        );
        assert_eq!(
            assembler.symbol_table.get_symbol("hw"),
            Some(&Symbol::new("hw".to_string(), 0, SymbolType::Label))
        );
        assert_eq!(
            assembler.symbol_table.get_symbol("about"),
            Some(&Symbol::new("about".to_string(), 12, SymbolType::Label))
        );

        assert_eq!(
            assembler.ro_section,
            vec![
                0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x00, 0x68, 0x65,
                0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x49, 0x20, 0x61, 0x6D, 0x20, 0x4E, 0x65, 0x72, 0x6F,
                0x20, 0x59, 0x61, 0x6E, 0x67, 0x00
            ]
        );

        assert_eq!(
            result.unwrap(),
            vec![
                0x64, 0x65, 0x6C, 0x66, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, 0x01, 0x01, 0xF4, 0x02, 0x00,
                0x01, 0x02, 0x03, 0x00, 0x01, 0x02, 0x04, 0x00, 0x01, 0x02, 0x05, 0x00, 0x01, 0x02,
                0x06, 0x00, 0x07, 0x01, 0x08, 0x01, 0x09, 0x01, 0x02, 0x01, 0x1F, 0x00, 0x14, 0x0A,
                0x1F, 0x01, 0x1F, 0x00, 0x00, 0x0E, 0x1F, 0x01, 0x1F, 0x00, 0x0C, 0x0E, 0x1F, 0x01,
                0x00, 0x01, 0xF4, 0x02, 0x00, 0x01, 0x00, 0x01, 0x1F, 0x00, 0x2F, 0x06, 0x1F, 0x00
            ]
        );
    }

    #[test]
    fn should_process_section_header() {
        let mut assembler = Assembler::new();
        assembler.process_section_header("code");
        assert_eq!(
            assembler.current_section,
            Some(Code {
                instruction_starting: None
            })
        );

        assembler.process_section_header("data");
        assert_eq!(
            assembler.current_section,
            Some(Data {
                instruction_starting: None
            })
        );

        assembler.process_section_header("bss");
        assert_eq!(
            assembler.current_section,
            Some(Data {
                instruction_starting: None
            })
        );
    }

    #[test]
    fn should_assemble_for_each_prints() {
        let mut assembler = Assembler::new();
        let result = assembler.process(
            ".code                                         \n\
             main:   load    $0  #0              \n\
             load    $1  #50             \n\
             load    $2  #0             \n\
             for:    eq      $0  $1                  \n\
             prts    @hw                     \n\
             dec     $1                      \n\
             inc     $2                      \n\
             jne     @for                    \n\
             prts    @passed                 \n\
             .data                                         \n\
             hw:     .asciiz \"Hello, World.\"       \n\
             passed: .asciiz \"Ok, 50 times print passed.\"",
        );

        assert_eq!(
            assembler.ro_section,
            vec![
                72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 46, 0, 79, 107, 44, 32, 53,
                48, 32, 116, 105, 109, 101, 115, 32, 112, 114, 105, 110, 116, 32, 112, 97, 115,
                115, 101, 100, 46, 0
            ]
        );

        assert_eq!(
            result.unwrap(),
            vec![
                0x64, 0x65, 0x6C, 0x66, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, 0x00, 0x00, 0x00, 0x01, 0x01,
                0x00, 0x32, 0x01, 0x02, 0x00, 0x00, 0x09, 0x00, 0x01, 0x01, 0x1F, 0x00, 0x00, 0x0E,
                0x1F, 0x0D, 0x01, 0x0C, 0x02, 0x01, 0x1F, 0x00, 0x0C, 0x0F, 0x1F, 0x01, 0x1F, 0x00,
                0x0E, 0x0E, 0x1F
            ]
        );
    }

    #[test]
    fn should_assemble_loop_add() {
        let mut assembler = Assembler::new();
        let result = assembler.process(
            ".code                                         \n\
             main:   load    $0  #0              \n\
             load    $1  #50             \n\
             load    $2  #0              \n\
             for:    eq      $0  $1                  \n\
             dec     $1                      \n\
             inc     $2                      \n\
             jne     @for                    \n\
             .data",
        );
        assert_eq!(
            result.unwrap(),
            vec![
                0x64, 0x65, 0x6C, 0x66, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, 0x00, 0x00, 0x00, 0x01, 0x01,
                0x00, 0x32, 0x01, 0x02, 0x00, 0x00, 0x09, 0x00, 0x01, 0x0D, 0x01, 0x0C, 0x02, 0x01,
                0x1F, 0x00, 0x0C, 0x0F, 0x1F
            ]
        );
    }
}
