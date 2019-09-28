use crate::vm::VM;
use std::io::{Write, Read};

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

    pub fn run(&mut self) {
        loop {
            let mut buffer = String::new();
            let stdin = std::io::stdin();

            print!(">>> ");
            std::io::stdout().flush().expect("Unable to flush stdout.");

            std::io::stdin().read_line(&mut buffer).expect("Unable to read line.");
            let buffer = buffer.trim();

            match buffer {
                ".quit" => {
                    println!("Bye, have a nice day.");
                    std::process::exit(0);
                }
                _ => {
                    println!("Invalid Input.");
                }
            }
        }
    }
}
