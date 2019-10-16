use crate::assembler::assembler_phase::AssemblerPhase;
use crate::assembler::assembly_parser::AssemblyProgramParser;
use crate::assembler::assembler_instruction::AssemblerInstruction;
use crate::assembler::symbol_table::{SymbolTable, Symbol, SymbolType};
use crate::assembler::elf::{ELF_HEADER_PREFIX, ELF_HEADER_LENGTH};
use crate::assembler::token::Token;
use crate::assembler::assembler_phase::AssemblerPhase::FIRST;
use crate::assembler::assembler_section::AssemblerSection;
use crate::assembler::assembler_error::AssemblerError;
use crate::assembler::assembler_error::AssemblerError::{
    NoSectionDeclarationFound,
    NoLabelNameFound,
    SymbolAlreadyDeclared,
    UnknownDirectiveFound,
    UnknownSectionFound,
};
use crate::assembler::assembler_section::AssemblerSection::UnKnown;
use crate::assembler::token::Token::{IntegerOperand, Op, Register};
use crate::vm::instruction::OpCode;
use crate::vm::vm::TMP_REGISTER;

pub struct Assembler {
    assemble_phase: AssemblerPhase,
    pub symbol_table: SymbolTable,
    pub ro_section: Vec<u8>,
    byte_code: Vec<u8>,
    ro_offset: u32,
    sections: Vec<AssemblerSection>,
    current_section: Option<AssemblerSection>,
    current_instruction: u32,
    errors: Vec<AssemblerError>,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            assemble_phase: FIRST,
            symbol_table: SymbolTable::new(),
            ro_section: Vec::new(),
            byte_code: Vec::new(),
            ro_offset: 0,
            sections: Vec::new(),
            current_section: None,
            current_instruction: 0,
            errors: Vec::new(),
        }
    }

    fn write_delf_header(&self) -> Vec<u8> {
        let mut header: Vec<u8> = Vec::<u8>::new();
        for byte in ELF_HEADER_PREFIX.into_iter() {
            header.push(byte.clone());
        }
        while header.len() < ELF_HEADER_LENGTH {
            header.push(0xFF as u8);
        }

        return header;
    }

    pub fn process(&mut self, assembly: &str) -> Result<Vec<u8>, Vec<AssemblerError>> {
        let mut parser = AssemblyProgramParser::new(assembly);
        let mut instructions = parser.parse_program();
        match instructions {
            Ok(ins) => {
                return self.process_instructions(&ins);
            }
            Err(e) => {
                return Err(vec![AssemblerError::ParseError { error: e.to_string() }]);
            }
        }
    }

    pub fn process_instructions(&mut self, instructions: &Vec<AssemblerInstruction>) -> Result<Vec<u8>, Vec<AssemblerError>> {
        let mut assembled_program: Vec<u8> = self.write_delf_header();
        self.process_first_phase(&instructions);

        if !self.errors.is_empty() {
            return Err(self.errors.clone());
        }

        if self.sections.len() < 2 { // at lease code and data section are need.
            self.errors.push(AssemblerError::InsufficientSections);
            return Err(self.errors.clone());
        }

        let mut body: Vec<u8> = self.process_second_phase(&instructions);

        assembled_program.append(&mut body);
        return Ok(assembled_program);
    }

    fn process_label_declaration(&mut self, instruction: &AssemblerInstruction) {
        match instruction.get_label_declaration_name() {
            Some(name) => {
                if self.symbol_table.get_symbol(&name).is_none() {
                    let symbol = Symbol::default(name.to_string(), SymbolType::Label);
                    self.symbol_table.add_symbol(symbol);
                } else {
                    self.errors.push(SymbolAlreadyDeclared { instruction: self.current_instruction })
                }
            }
            None => {
                self.errors.push(NoLabelNameFound { instruction: self.current_instruction })
            }
        }
    }

    fn process_label_declaration_second_phase(&mut self, instruction: &AssemblerInstruction, offset: usize) {
        if instruction.token.is_some() {
            match instruction.get_label_declaration_name() {
                Some(name) => {
                    if self.symbol_table.get_symbol(&name).is_some() {
                        self.symbol_table.set_symbol_offset(&name, offset as u32);
                    }
                }
                None => {
                    self.errors.push(NoLabelNameFound { instruction: self.current_instruction })
                }
            }
        }
    }

    fn handle_asciiz(&mut self, instruction: &AssemblerInstruction) {
        if self.assemble_phase != AssemblerPhase::FIRST { return; }
        match instruction.get_string_constant() {
            Some(s) => {
                match instruction.get_label_declaration_name() {
                    Some(name) => {
                        self.symbol_table.set_symbol_offset(&name, self.ro_offset);
                    }
                    None => { self.errors.push(AssemblerError::LabelNotFoundForStringConstant); }
                }
                for byte in s.as_bytes() {
                    if *byte == 92 {
                        let byte_addr: *const u8 = unsafe { byte as *const u8 };
                        let byte_addr = unsafe { byte_addr.add(1) };
                        if unsafe { *byte_addr } == 110 {
                            self.ro_section.push(0xA);
                            self.ro_offset += 1;
                        } else {
                            self.ro_section.push(*byte);
                            self.ro_offset += 1;
                        }
                    } else {
                        if *byte == 110 {
                            let byte_addr: *const u8 = unsafe { byte as *const u8 };
                            let byte_addr = unsafe { byte_addr.sub(1) };
                            if unsafe { *byte_addr } != 92 {
                                self.ro_section.push(*byte);
                                self.ro_offset += 1;
                            }
                        } else {
                            self.ro_section.push(*byte);
                            self.ro_offset += 1;
                        }
                    }
                }
                self.ro_section.push(0x0); // end of zero
                self.ro_offset += 1;
            }
            None => {
                self.errors.push(AssemblerError::StringConstantNotFound);
            }
        }
    }

    fn process_section_header(&mut self, header_name: &str) {
        let new_section: AssemblerSection = header_name.into();
        if new_section == AssemblerSection::UnKnown {
            self.errors.push(UnknownSectionFound { section_name: header_name.to_string() });
            return;
        }

        self.sections.push(new_section);
        self.current_section = Some(new_section);
    }

    fn process_directive(&mut self, instruction: &AssemblerInstruction) {
        if instruction.has_operands() {
            match instruction.get_directive_name().unwrap().as_ref() {
                "asciiz" => {
                    self.handle_asciiz(instruction);
                }
                _ => {
                    self.errors.push(UnknownDirectiveFound {
                        directive: instruction.get_directive_name().unwrap().clone()
                    });
                }
            }
        } else {
            self.process_section_header(instruction.get_directive_name().unwrap().as_ref());
        }
    }

    fn process_label_usage(&mut self, instruction: &AssemblerInstruction) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();

        match &instruction.operand1 {
            Some(Token::LabelUsage { name }) => { self.process_label_offset(&instruction, &mut bytes, &name) }
            _ => {}
        }

        match &instruction.operand2 {
            Some(Token::LabelUsage { name }) => { self.process_label_offset(&instruction, &mut bytes, &name) }
            _ => {}
        }

        match &instruction.operand3 {
            Some(Token::LabelUsage { name }) => { self.process_label_offset(&instruction, &mut bytes, &name) }
            _ => {}
        }

        return bytes;
    }

    fn process_label_offset(&mut self, instruction: &AssemblerInstruction, bytes: &mut Vec<u8>, name: &&String) -> () {
        let offset = self.symbol_table.get_symbol_offset(&name).unwrap();
        let save_offset_instruction = AssemblerInstruction {
            token: Some(Op { opcode: OpCode::LOAD }),
            label: None,
            directive: None,
            operand1: Some(Register { reg_num: TMP_REGISTER }),
            operand2: Some(IntegerOperand { value: offset as i32 }),
            operand3: None,
        };
        bytes.append(&mut save_offset_instruction.to_bytes());
        match &instruction.token {
            Some(Token::Op { opcode }) => match opcode {
                _ => { bytes.push(*opcode as u8) }
            },
            _ => {
                println!("None opCode found in opCode field.");
                std::process::exit(0);
            }
        }
        bytes.push(TMP_REGISTER);
    }

    // scan symbol declaration to symbol table,and sections
    fn process_first_phase(&mut self, instructions: &Vec<AssemblerInstruction>) {
        for instruction in instructions {
            if instruction.is_label_declaration() {
                if self.current_section.is_some() {
                    self.process_label_declaration(&instruction);
                } else {
                    self.errors.push(NoSectionDeclarationFound { instruction: self.current_instruction })
                }
            }
            if instruction.is_directive() {
                self.process_directive(&instruction);
            }

            self.current_instruction += 1;
        }
        self.assemble_phase = AssemblerPhase::SECOND;
    }

    // translate symbol usage to memory offset
    fn process_second_phase(&mut self, instructions: &Vec<AssemblerInstruction>) -> Vec<u8> {
        self.current_instruction = 0;
        let mut program = Vec::<u8>::new();

        for mut instruction in instructions {
            if instruction.is_label_declaration() {
                self.process_label_declaration_second_phase(&instruction, program.len());
            }
            if instruction.is_opcode() {
                if instruction.is_label_usage() {
                    let mut bytes = self.process_label_usage(&instruction);
                    program.append(&mut bytes);
                } else {
                    let mut bytes = instruction.to_bytes();
                    program.append(&mut bytes);
                }
            }

            if instruction.is_directive() {
                self.process_directive(&instruction);
            }

            self.current_instruction += 1;
        }
        return program;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::assembler_section::AssemblerSection::{Code, Data};
    use crate::assembler::token::Token::{Op, Directive, Register, LabelDeclaration, LabelUsage};
    use crate::vm::instruction::OpCode;

    #[test]
    fn should_write_elf_header() {
        let mut assembler = Assembler::new();
        let header = assembler.write_delf_header();
        assert_eq!(header, vec![0x64, 0x65, 0x6C, 0x66, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, ])
    }

    #[test]
    fn should_process_program() {
        let mut assembler = Assembler::new();
        let result = assembler.process(
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
                    about:  .asciiz \"hello, I am Nero Yang\"");

        assert_eq!(assembler.current_section, Some(Data { instruction_starting: None }));
        assert_eq!(assembler.symbol_table.get_symbol("main"), Some(&Symbol::new("main".to_string(), 0, SymbolType::Label)));
        assert_eq!(assembler.symbol_table.get_symbol("hello"), Some(&Symbol::new("hello".to_string(), 20, SymbolType::Label)));
        assert_eq!(assembler.symbol_table.get_symbol("hw"), Some(&Symbol::new("hw".to_string(), 0, SymbolType::Label)));
        assert_eq!(assembler.symbol_table.get_symbol("about"), Some(&Symbol::new("about".to_string(), 12, SymbolType::Label)));


        assert_eq!(assembler.ro_section, vec![
            0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x00,
            0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x49, 0x20, 0x61, 0x6D, 0x20, 0x4E, 0x65, 0x72, 0x6F, 0x20, 0x59, 0x61, 0x6E, 0x67, 0x00
        ]);
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


        assert_eq!(ins[0], AssemblerInstruction {
            token: None,
            label: None,
            directive: Some(Directive { name: "code".to_string() }),
            operand1: None,
            operand2: None,
            operand3: None,
        });

        assert_eq!(ins[1], AssemblerInstruction {
            token: Some(Op { opcode: OpCode::LOAD }),
            label: Some(LabelDeclaration { name: "main".to_string() }),
            directive: None,
            operand1: Some(Register { reg_num: 1 }),
            operand2: Some(IntegerOperand { value: 500 }),
            operand3: None,
        });

        assert_eq!(ins[2], AssemblerInstruction {
            token: Some(Op { opcode: OpCode::PRTS }),
            label: None,
            directive: None,
            operand1: Some(LabelUsage { name: "hw".to_string() }),
            operand2: None,
            operand3: None,
        });

        assert_eq!(ins[3], AssemblerInstruction {
            token: Some(Op { opcode: OpCode::PRTS }),
            label: None,
            directive: None,
            operand1: Some(LabelUsage { name: "about".to_string() }),
            operand2: None,
            operand3: None,
        });

        let result = assembler.process_instructions(&ins);

        assert_eq!(assembler.current_section, Some(Data { instruction_starting: None }));
        assert_eq!(assembler.symbol_table.get_symbol("main"), Some(&Symbol::new("main".to_string(), 0, SymbolType::Label)));
        assert_eq!(assembler.symbol_table.get_symbol("hw"), Some(&Symbol::new("hw".to_string(), 0, SymbolType::Label)));
        assert_eq!(assembler.symbol_table.get_symbol("about"), Some(&Symbol::new("about".to_string(), 12, SymbolType::Label)));

        assert_eq!(assembler.ro_section, vec![
            0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x00,
            0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x49, 0x20, 0x61, 0x6D, 0x20, 0x4E, 0x65, 0x72, 0x6F, 0x20, 0x59, 0x61, 0x6E, 0x67, 0x00
        ]);

        assert_eq!(result.unwrap(), vec![
            0x64, 0x65, 0x6C, 0x66, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0x01, 0x01, 0x01, 0xF4,
            0x01, 0x1F, 0x00, 0x00,
            0x0E, 0x1F,
            0x01, 0x1F, 0x00, 0x0C,
            0x0E, 0x1F
        ]);
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
                    about:  .asciiz \"hello, I am Nero Yang\"");

        assert_eq!(assembler.current_section, Some(Data { instruction_starting: None }));
        assert_eq!(assembler.symbol_table.get_symbol("main"), Some(&Symbol::new("main".to_string(), 0, SymbolType::Label)));
        assert_eq!(assembler.symbol_table.get_symbol("hello"), Some(&Symbol::new("hello".to_string(), 20, SymbolType::Label)));
        assert_eq!(assembler.symbol_table.get_symbol("foo"), Some(&Symbol::new("foo".to_string(), 47, SymbolType::Label)));
        assert_eq!(assembler.symbol_table.get_symbol("hw"), Some(&Symbol::new("hw".to_string(), 0, SymbolType::Label)));
        assert_eq!(assembler.symbol_table.get_symbol("about"), Some(&Symbol::new("about".to_string(), 12, SymbolType::Label)));

        assert_eq!(assembler.ro_section, vec![
            0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x00,
            0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x49, 0x20, 0x61, 0x6D, 0x20, 0x4E, 0x65, 0x72, 0x6F, 0x20, 0x59, 0x61, 0x6E, 0x67, 0x00
        ]);

        assert_eq!(result.unwrap(), vec![
            0x64, 0x65, 0x6C, 0x66, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0x01, 0x01, 0x01, 0xF4,
            0x02, 0x00, 0x01, 0x02,
            0x03, 0x00, 0x01, 0x02,
            0x04, 0x00, 0x01, 0x02,
            0x05, 0x00, 0x01, 0x02,
            0x06, 0x00,
            0x07, 0x01,
            0x08, 0x01,
            0x09, 0x01, 0x02,
            0x01, 0x1F, 0x00, 0x14,
            0x0A, 0x1F,
            0x01, 0x1F, 0x00, 0x00,
            0x0E, 0x1F,
            0x01, 0x1F, 0x00, 0x0C,
            0x0E, 0x1F,
            0x01, 0x00, 0x01, 0xF4,
            0x02, 0x00, 0x01, 0x00,
            0x01, 0x1F, 0x00, 0x2F,
            0x06, 0x1F,
            0x00
        ]);
    }

    #[test]
    fn should_process_section_header() {
        let mut assembler = Assembler::new();
        assembler.process_section_header("code");
        assert_eq!(assembler.current_section, Some(Code { instruction_starting: None }));

        assembler.process_section_header("data");
        assert_eq!(assembler.current_section, Some(Data { instruction_starting: None }));

        assembler.process_section_header("bss");
        assert_eq!(assembler.current_section, Some(Data { instruction_starting: None }));
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
                            passed: .asciiz \"Ok, 50 times print passed.\"");

        assert_eq!(assembler.ro_section, vec![
            72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 46, 0,
            79, 107, 44, 32, 53, 48, 32, 116, 105, 109, 101, 115, 32, 112, 114, 105,
            110, 116, 32, 112, 97, 115, 115, 101, 100, 46, 0
        ]);

        assert_eq!(result.unwrap(), vec![
            0x64, 0x65, 0x6C, 0x66, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0x01, 0x00, 0x00, 0x00,
            0x01, 0x01, 0x00, 0x32,
            0x01, 0x02, 0x00, 0x00,
            0x09, 0x00, 0x01,
            0x01, 0x1F, 0x00, 0x00,
            0x0E, 0x1F,
            0x0D, 0x01,
            0x0C, 0x02,
            0x01, 0x1F, 0x00, 0x0C,
            0x0F, 0x1F,
            0x01, 0x1F, 0x00, 0x0E,
            0x0E, 0x1F
        ]);
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
                      .data");
        assert_eq!(result.unwrap(), vec![
            0x64, 0x65, 0x6C, 0x66, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0x01, 0x00, 0x00, 0x00,
            0x01, 0x01, 0x00, 0x32,
            0x01, 0x02, 0x00, 0x00,
            0x09, 0x00, 0x01,
            0x0D, 0x01,
            0x0C, 0x02,
            0x01, 0x1F, 0x00, 0x0C,
            0x0F, 0x1F
        ]);
    }
}
