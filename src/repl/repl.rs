use crate::assembler::assembler::Assembler;
use crate::assembler::instructions_parser::InstructionParser;
use crate::repl::repl::ReplMode::Assembly;
use crate::vm::vm::VM;
use std::fs;
use std::io::Write;
use std::num::ParseIntError;

pub struct REPL {
    command_buffer: Vec<String>,
    pub(crate) vm: VM,
    mode: ReplMode,
}

#[derive(Debug, PartialEq)]
pub enum ReplMode {
    Assembly,
    Instruction,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            command_buffer: Vec::new(),
            vm: VM::new(),
            mode: Assembly,
        }
    }

    fn parse_hex(&mut self, buf: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = buf.split(" ").collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];

        for hex_str in split {
            let byte = u8::from_str_radix(&hex_str, 16);
            match byte {
                Ok(result) => results.push(result),
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(results)
    }

    pub fn run_asm_file(&mut self, filename: &str) {
        let src = fs::read_to_string(filename);
        match src {
            Ok(asm_src) => {
                println!("Loading ASM : \n{}\nASM loaded.", asm_src);

                let mut assembler = Assembler::new();
                let instructions = assembler.process(&asm_src);
                match instructions {
                    Ok(ins) => {
                        self.vm.load_program(ins);
                        self.vm.set_ro_data(assembler.ro_section);
                        self.vm.run();
                    }
                    Err(errs) => {
                        for e in errs {
                            println!("Error in assemble : {:#?}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("Asm load failed: {:#?}", e);
            }
        }
    }

    pub fn run(&mut self) {
        println!("Du.rs release_0.1.0 (default, Oct 15 2019, 23:12:15)");
        println!("Type \".help\" for more information, \".exit\" to quit.");
        loop {
            let mut buffer = String::new();
            let stdin = std::io::stdin();

            print!(">>> ");
            std::io::stdout().flush().expect("Unable to flush stdout.");

            stdin.read_line(&mut buffer).expect("Unable to read line.");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());
            let mut commands = buffer.split_ascii_whitespace().peekable();
            if commands.peek().is_some() {
                if commands
                    .peek()
                    .map_or(false, |w| (*w == ".quit") || (*w == ".exit"))
                {
                    println!("Bye, have a nice day.");
                    std::process::exit(0);
                } else if commands.peek().map_or(false, |w| (*w == ".load_asm")) {
                    commands.next();
                    match commands.peek() {
                        Some(filepath) => {
                            self.run_asm_file(filepath);
                        }
                        None => println!("No input: need a file path for asm code."),
                    }
                } else if commands.peek().map_or(false, |w| (*w == ".load_elf")) {
                    // todo : load elf file to execute.
                } else if commands.peek().map_or(false, |w| (*w == ".output_elf")) {
                    // todo : output elf file.
                } else if commands.peek().map_or(false, |w| (*w == ".mode")) {
                    commands.next();
                    match commands.peek() {
                        Some(mode) => match mode {
                            &"Assembly" => {
                                self.mode = ReplMode::Assembly;
                                println!("Mode change to Assembly.");
                            }
                            &"Instruction" => {
                                self.mode = ReplMode::Instruction;
                                println!("Mode change to Instruction.");
                            }
                            _ => {
                                println!("Expect mode: Assembly/Instruction");
                            }
                        },
                        None => println!("Need a mode Assembly/Instruction."),
                    }
                } else if commands.peek().map_or(false, |w| (*w == ".history")) {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                } else if commands.peek().map_or(false, |w| (*w == ".program")) {
                    println!("Listing instructions currently in VM's program vector:");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                    println!("End of Program Listing.")
                } else if commands.peek().map_or(false, |w| (*w == ".clear")) {
                    println!("Clearing in VM's program vector:");
                    let len = self.vm.program.len();
                    self.vm.program.clear();
                    println!("  {} instructions cleared.", len)
                } else if commands.peek().map_or(false, |w| (*w == ".reset")) {
                    println!("Resetting vm:");
                    self.vm.registers = [0; 32];
                    println!("  registers reset.");
                    self.vm.program.clear();
                    println!("  program reset.");
                    self.vm.ro_data.clear();
                    println!("  read-only data reset.");
                    println!("  vm reset.")
                } else if commands.peek().map_or(false, |w| (*w == ".registers")) {
                    println!("Listing registers and all contents:");
                    println!("{:#?}", self.vm.registers);
                    println!("End of Registers Listing.")
                } else if commands.peek().map_or(false, |w| (*w == ".help")) {
                    println!("Command Usage:");
                    println!("  .load_asm   : Load asm file and run. e.g. .load_asm xxx.asm");
                    println!("  .history    : Command history");
                    println!("  .registers  : Registers and content in current vm");
                    println!("  .program    : Program in current vm");
                    println!("  .clear      : Clear vm program memory");
                    println!("  .reset      : Reset vm");
                    println!(
                        "  .mode       : Change to mode of REPL between Assembly and Instruction"
                    );
                } else {
                    match &self.mode {
                        ReplMode::Assembly => {
                            let mut instruction_parser = InstructionParser::new(buffer);
                            let input_instruction = instruction_parser.parse_assembly_line();
                            match input_instruction {
                                Ok(ins) => {
                                    if ins.token.is_some() & &ins.label.is_none() {
                                        for byte in ins.to_bytes() {
                                            self.vm.program.push(byte);
                                        }
                                    }
                                    self.vm.run_once();
                                }
                                Err(e) => {
                                    println!("Error: {:?}", e);
                                }
                            }
                        }
                        ReplMode::Instruction => {
                            let ins_bytes = &self.parse_hex(buffer);
                            match ins_bytes {
                                Ok(ins) => {
                                    for byte in ins {
                                        self.vm.program.push(*byte);
                                    }
                                    self.vm.run_once();
                                }
                                Err(e) => {
                                    println!("Error: {:?}", e);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
