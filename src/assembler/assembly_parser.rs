use std::iter::Peekable;
use std::str::{SplitWhitespace, Lines};
use crate::assembler::token::Token::{Op, Register, IntegerOperand, Directive, LabelDeclaration, LabelUsage, IrString};
use crate::vm::instruction::OpCode::*;
use crate::assembler::assembler_instruction::AssemblerInstruction;
use crate::vm::instruction::OpCode;
use crate::assembler::instructions_parser::InstructionParser;

pub struct AssemblyProgramParser<'a> {
    instructions: Peekable<Lines<'a>>,
}

impl<'a> AssemblyProgramParser<'a> {
    pub fn new(str: &str) -> AssemblyProgramParser {
        AssemblyProgramParser {
            instructions: str.lines().peekable()
        }
    }

    pub fn parse_program(&mut self) -> Result<Vec<AssemblerInstruction>, &'static str> {
        let mut assembler_instructions: Vec<AssemblerInstruction> = Vec::new();
        while self.instructions.peek().is_some() {
            match self.instructions.peek() {
                Some(instruction_str) => {
                    let mut instruction_parser = InstructionParser::new(instruction_str);
                    let instruction = instruction_parser.parse_assembly_line();
                    match instruction {
                        Ok(ins) => {
                            assembler_instructions.push(ins);
                            self.instructions.next();
                        }
                        Err(e) => { return Err(e); }
                    }
                }
                _ => { break; }
            }
        }
        return Ok(assembler_instructions);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_instruction_list_when_give_assembly_code() {
        let mut assembler = AssemblyProgramParser::new(
            "hlt\n\
            .code\n\
                main:\n\
                    load $1 #300\n\
                    add $0 $1 $2\n\
                    sub $0 $1 $2\n\
                    mul $0 $1     $2\n\
                    div $0 $1 $2\n\
                hello: jmp $1\n\
                    jmp_f $1\n\
                    jmp_b $1\n\
                    eq $1 $2\n\
                    jeq $1\n\
            .data\n\
                hw: .asciz \"hello,World\"\n\
                about: .asciz \"hello, I am Nero Yang\"");
        let instructions = assembler.parse_program().unwrap();
        assert_eq!(instructions[0], AssemblerInstruction {
            token: Some(Op { opcode: HLT }),
            label: None,
            directive: None,
            operand1: None,
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[1], AssemblerInstruction {
            token: None,
            label: None,
            directive: Some(Directive { name: "code".to_string() }),
            operand1: None,
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[2], AssemblerInstruction {
            token: None,
            label: Some(LabelDeclaration { name: "main".to_string() }),
            directive: None,
            operand1: None,
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[3], AssemblerInstruction {
            token: Some(Op { opcode: LOAD }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(IntegerOperand { value: 300 }),
            operand3: None,
        });
        assert_eq!(instructions[4], AssemblerInstruction {
            token: Some(Op { opcode: ADD }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
        assert_eq!(instructions[5], AssemblerInstruction {
            token: Some(Op { opcode: SUB }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
        assert_eq!(instructions[6], AssemblerInstruction {
            token: Some(Op { opcode: MUL }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
        assert_eq!(instructions[7], AssemblerInstruction {
            token: Some(Op { opcode: DIV }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 0 }),
            operand2: Some(Register { reg_num: 1 }),
            operand3: Some(Register { reg_num: 2 }),
        });
        assert_eq!(instructions[8], AssemblerInstruction {
            token: Some(Op { opcode: JMP }),
            label: Some(LabelDeclaration { name: "hello".to_string() }),
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[9], AssemblerInstruction {
            token: Some(Op { opcode: JMP_F }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[10], AssemblerInstruction {
            token: Some(Op { opcode: JMP_B }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[11], AssemblerInstruction {
            token: Some(Op { opcode: EQ }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(Register { reg_num: 2 }),
            operand3: None,
        });
        assert_eq!(instructions[12], AssemblerInstruction {
            token: Some(Op { opcode: JEQ }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
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
            directive: Some(Directive { name: "asciz".to_string() }),
            operand1: Some(IrString { name: "hello,World".to_string() }),
            operand2: None,
            operand3: None,
        });
        assert_eq!(instructions[15], AssemblerInstruction {
            token: None,
            label: Some(LabelDeclaration { name: "about".to_string() }),
            directive: Some(Directive { name: "asciz".to_string() }),
            operand1: Some(IrString { name: "hello, I am Nero Yang".to_string() }),
            operand2: None,
            operand3: None,
        });
    }
}
