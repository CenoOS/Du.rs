use crate::vm::VM;
use std::io::Write;
use std::num::ParseIntError;
use crate::assembler::assembler::InstructionParser;

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            command_buffer: Vec::new(),
            vm: VM::new(),
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

            std::io::stdin().read_line(&mut buffer).expect("Unable to read line.");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());

            match buffer {
                ".quit" | ".exit" => {
                    println!("Bye, have a nice day.");
                    std::process::exit(0);
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
                    let mut instruction_parser = InstructionParser::new(buffer);
                    let input_instruction = instruction_parser.parse_instruction();
                    match input_instruction {
                        Ok(ins) => {
                            for byte in ins.to_bytes() {
                                self.vm.program.push(byte);
                            }
                        }
                        Err(e) => {
                            println!("Error: {:?}", e);
                        }
                    }
                    self.vm.run_once();
                }
            }
        }
    }
}
