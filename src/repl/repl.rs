use std::io::Write;
use std::num::ParseIntError;
use crate::assembler::assembler::InstructionParser;
use crate::vm::vm::VM;
use crate::assembler::token::Token::LabelUsage;
use crate::repl::repl::ReplMode::{Assembly, Instruction};

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
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
                Ok(result) => { results.push(result) }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(results)
    }

    pub fn run(&mut self) {
        println!("Du.rs 0.0.23 (default, Sep 23 2019, 20:52:15)");
        println!("Type \".help\" for more information, \".exit\" to quit.");
        loop {
            let mut buffer = String::new();
            let stdin = std::io::stdin();

            print!(">>> ");
            std::io::stdout().flush().expect("Unable to flush stdout.");

            stdin.read_line(&mut buffer).expect("Unable to read line.");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());

            match buffer {
                ".quit" | ".exit" => {
                    println!("Bye, have a nice day.");
                    std::process::exit(0);
                }
                ".load_elf" => {
                    // todo : load elf file to execute.
                }
                ".output_elf" => {
                    // todo : output elf file.
                }
                ".mode" => {
                    // todo : change mode to Assembly / Instruction
                }
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                }
                ".program" => {
                    println!("Listing instructions currently in VM's program vector:");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                    println!("End of Program Listing.")
                }
                ".clear" => {
                    println!("Clearing in VM's program vector:");
                    let len = self.vm.program.len();
                    self.vm.program.clear();
                    println!("{} instructions cleared.", len)
                }
                ".registers" => {
                    println!("Listing registers and all contents:");
                    println!("{:#?}", self.vm.registers);
                    println!("End of Registers Listing.")
                }
                ".help" => {
                    println!("Command Usage:");
                    println!("  .history    : command history");
                    println!("  .program    : program in current vm");
                    println!("  .registers  : registers and content in current vm");
                    println!("Type above commands to debug.");
                }
                _ => {
                    match &self.mode {
                        ReplMode::Assembly => {
                            let mut instruction_parser = InstructionParser::new(buffer);
                            let input_instruction = instruction_parser.parse_assembly_line();
                            match input_instruction {
                                Ok(ins) => {
                                    if ins.token.is_some() && ins.label.is_none() {
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
                        _ => {
                            println!("Error: un-support repl mode :{:?}", self.mode);
                        }
                    }
                }
            }
        }
    }
}
