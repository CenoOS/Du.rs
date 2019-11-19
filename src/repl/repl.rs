use crate::assembler::assembler::Assembler;
use crate::assembler::instructions_parser::InstructionParser;
use crate::repl::color_print::ColorPrint;
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
                ColorPrint::println_light_blue(format!("{}\n", asm_src).as_str());

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
                            ColorPrint::println_light_red(
                                format!("Error in assemble : {:#?}", e).as_str(),
                            );
                        }
                    }
                }
            }
            Err(e) => {
                ColorPrint::println_light_red(format!("Asm load failed: {:#?}", e).as_str());
            }
        }
    }

    pub fn run(&mut self) {
        REPL::printSplash();
        loop {
            let mut buffer = String::new();
            let stdin = std::io::stdin();
            ColorPrint::print_light_green("du> ");
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
                    ColorPrint::println_light_green("Bye, have a nice day.");
                    std::process::exit(0);
                } else if commands.peek().map_or(false, |w| (*w == ".load_asm")) {
                    commands.next();
                    match commands.peek() {
                        Some(filepath) => {
                            self.run_asm_file(filepath);
                        }
                        None => ColorPrint::println_light_red(
                            "No input: need a file path for asm code.",
                        ),
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
                                ColorPrint::println_light_purple("Mode change to Assembly.");
                            }
                            &"Instruction" => {
                                self.mode = ReplMode::Instruction;
                                ColorPrint::println_light_purple("Mode change to Instruction.");
                            }
                            _ => {
                                ColorPrint::println_light_red("Expect mode: Assembly/Instruction");
                            }
                        },
                        None => ColorPrint::println_light_red("Need a mode Assembly/Instruction."),
                    }
                } else if commands.peek().map_or(false, |w| (*w == ".history")) {
                    for command in &self.command_buffer {
                        ColorPrint::println_light_blue(format!("{}", command).as_str());
                    }
                } else if commands.peek().map_or(false, |w| (*w == ".program")) {
                    ColorPrint::println_light_green(
                        "Listing instructions currently in VM's program vector:",
                    );
                    for instruction in &self.vm.program {
                        ColorPrint::print_light_blue(format!("{:?}", instruction).as_str());
                    }
                    ColorPrint::println_light_green("");
                    ColorPrint::println_light_green("End of Program Listing.")
                } else if commands.peek().map_or(false, |w| (*w == ".clear")) {
                    ColorPrint::println_light_green("Clearing in VM's program vector:");
                    let len = self.vm.program.len();
                    self.vm.program.clear();
                    ColorPrint::println_light_green(
                        format!("  {} instructions cleared.", len).as_str(),
                    )
                } else if commands.peek().map_or(false, |w| (*w == ".reset")) {
                    ColorPrint::println_light_green("Resetting vm:");
                    self.vm.registers = [0; 32];
                    ColorPrint::println_light_green("  registers reset.");
                    self.vm.program.clear();
                    ColorPrint::println_light_green("  program reset.");
                    self.vm.ro_data.clear();
                    ColorPrint::println_light_green("  read-only data reset.");
                    ColorPrint::println_light_green("  vm reset.")
                } else if commands.peek().map_or(false, |w| (*w == ".registers")) {
                    ColorPrint::println_light_green("Listing registers and all contents:");
                    ColorPrint::println_light_purple(format!("PC: {:?}", self.vm.pc).as_str());
                    ColorPrint::println_light_purple(format!("SP: {:?}", self.vm.sp).as_str());
                    ColorPrint::println_light_purple(format!("BP: {:?}", self.vm.bp).as_str());
                    ColorPrint::println_light_purple(
                        format!("CF: {:?}", self.vm.comparison_flag).as_str(),
                    );
                    ColorPrint::println_light_purple(
                        format!("RE: {:?}", self.vm.remainder).as_str(),
                    );
                    ColorPrint::println_light_blue(
                        format!("R0-R31 {:?}", self.vm.registers).as_str(),
                    );
                    ColorPrint::println_light_green("End of Registers Listing.")
                } else if commands.peek().map_or(false, |w| (*w == ".help")) {
                    ColorPrint::println_light_green("Command Usage:");
                    ColorPrint::println_light_blue(
                        "  .load_asm   : Load asm file and run. e.g. .load_asm xxx.asm",
                    );
                    ColorPrint::println_light_blue("  .history    : Command history");
                    ColorPrint::println_light_blue(
                        "  .registers  : Registers and content in current vm",
                    );
                    ColorPrint::println_light_blue("  .program    : Program in current vm");
                    ColorPrint::println_light_blue("  .clear      : Clear vm program memory");
                    ColorPrint::println_light_blue("  .reset      : Reset vm");
                    ColorPrint::println_light_blue(
                        "  .mode       : Change to mode of REPL between Assembly and Instruction",
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
                                    ColorPrint::println_light_red(
                                        format!("[ERROR]: {:?}", e).as_str(),
                                    );
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
                                    ColorPrint::println_light_red(
                                        format!("[ERROR]: {:?}", e).as_str(),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn printSplash() {
        ColorPrint::println_light_blue("");
        ColorPrint::println_light_blue("                   |   Du.rs release_0.1.0");
        ColorPrint::println_light_purple("             __    |   (default, Oct 15 2019, 23:12:15)");
        ColorPrint::println_light_purple("  ____      |  |   |   ");
        ColorPrint::println_light_purple(
            " |    \\  ___|  |   |   Type \".help\" for more information.",
        );
        ColorPrint::println_light_purple(" |  |  | |  |__|   |        \".exit\" to quit.");
        ColorPrint::println_light_purple(" |____/|____|__|   |");
        ColorPrint::println_light_purple("                   |   More information on: ");
        ColorPrint::println_light_blue("                   |       http://github.com/CenoOS/Du.rs");
        ColorPrint::println_light_purple("");
    }
}
