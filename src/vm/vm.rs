use crate::vm::instruction::OpCode;
use std::str::from_utf8;
use crate::assembler::elf::ELF_HEADER_PREFIX;
use std::f64::EPSILON;

pub const DEFAULT_STACK_SIZE: usize = 2097152;
pub const TMP_REGISTER: u8 = 0x20 - 1;

#[derive(Debug)]
pub struct VM {
    /* 8bits for opcode , 8bits for register number , 16 bits for numbers just 2<<16 = 65536(unsigned) */
    pub registers: [i32; 32],
    pub float_registers: [f64; 32],
    /* program counter */
    pub(crate) pc: usize,

    /* program memory */
    pub program: Vec<u8>,
    pub ro_data: Vec<u8>,
    pub heap: Vec<u8>,

    pub(crate) sp: usize,
    pub(crate) stack: Vec<i32>,

    pub(crate) bp: usize,

    pub(crate) remainder: u32,
    pub(crate) comparison_flag: bool,

}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            float_registers: [0.0; 32],
            pc: 0,
            program: Vec::new(),
            ro_data: Vec::new(),
            heap: Vec::new(),

            sp: 0,
            stack: Vec::with_capacity(DEFAULT_STACK_SIZE),

            bp: 0,

            remainder: 0,
            comparison_flag: false,
        }
    }

    fn decode_opcode(&mut self) -> OpCode {
        let opcode = OpCode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }

    /* read next 8bits from program */
    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }

    /* read next 16bits from program */
    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        return result;
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> Result<bool, &'static str> {
        // fly away
        if self.pc >= self.program.len() {
            return Ok(true);
        }

        let code = self.decode_opcode();
        match code {
            OpCode::LOAD => {
                /* LOAD reg numberH numberL*/
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;
                self.registers[register] = number as i32;
            }
            OpCode::ADD => {
                /* ADD reg1 reg2 regTarget */
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            }
            OpCode::SUB => {
                /* SUB reg1 reg2 regTarget */
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = (register1 - register2) as i32;
            }
            OpCode::MUL => {
                /* MUL reg1 reg2 regTarget */
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = (register1 * register2) as i32;
            }
            OpCode::DIV => {
                /* DIV reg1 reg2 regTarget */
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = (register1 / register2) as i32;
                self.remainder = (register1 % register2) as u32;
            }
            OpCode::JMP => {
                /* JMP regTarget */
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            }
            OpCode::JMPF => {
                /* JMPF regTarget */
                let step = self.registers[self.next_8_bits() as usize];
                self.pc += step as usize;
            }
            OpCode::JMPB => {
                /* JMPB regTarget */
                let step = self.registers[self.next_8_bits() as usize];
                self.pc -= step as usize;
            }
            OpCode::EQ => {
                /* EQ reg0 reg1 */
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                if register1 == register2 {
                    self.comparison_flag = true;
                } else {
                    self.comparison_flag = false;
                }
            }
            OpCode::LT => {
                /* LT reg0 reg1 */
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                if register1 < register2 {
                    self.comparison_flag = true;
                } else {
                    self.comparison_flag = false;
                }
            }
            OpCode::LTE => {
                /* LTE reg0 reg1 */
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                if register1 <= register2 {
                    self.comparison_flag = true;
                } else {
                    self.comparison_flag = false;
                }
            }
            OpCode::GT => {
                /* LT reg0 reg1 */
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                if register1 > register2 {
                    self.comparison_flag = true;
                } else {
                    self.comparison_flag = false;
                }
            }
            OpCode::GTE => {
                /* GTE reg0 reg1 */
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                if register1 >= register2 {
                    self.comparison_flag = true;
                } else {
                    self.comparison_flag = false;
                }
            }
            OpCode::JE => {
                /* JE regTarget */
                let target = self.registers[self.next_8_bits() as usize];
                if self.comparison_flag {
                    self.pc = target as usize;
                }
            }
            OpCode::JNE => {
                /* JNE regTarget */
                let target = self.registers[self.next_8_bits() as usize];
                if !self.comparison_flag {
                    self.pc = target as usize;
                }
            }
            OpCode::JL => {
                /* JL regTarget */
                let target = self.registers[self.next_8_bits() as usize];
                if self.comparison_flag {
                    self.pc = target as usize;
                }
            }
            OpCode::JG => {
                /* JG regTarget */
                let target = self.registers[self.next_8_bits() as usize];
                if self.comparison_flag {
                    self.pc = target as usize;
                }
            }
            OpCode::ALOC => {
                let register = self.next_8_bits() as usize;
                let bytes = self.registers[register];
                let new_end = self.heap.len() as i32 + bytes;
                self.heap.resize(new_end as usize, 0);
            }
            OpCode::INC => {
                /* INC reg */
                self.registers[self.next_8_bits() as usize] += 1;
            }
            OpCode::DEC => {
                /* DEC reg */
                self.registers[self.next_8_bits() as usize] -= 1;
            }
            OpCode::LOADF64 => {
                /* LOADF64 reg numberH numberL*/
                let register = self.next_8_bits() as usize;
                let number = f64::from(self.next_16_bits());
                self.float_registers[register] = number;
            }
            OpCode::ADDF64 => {
                /* ADDF64 reg1 reg2 regTarget */
                let register1 = self.float_registers[self.next_8_bits() as usize];
                let register2 = self.float_registers[self.next_8_bits() as usize];
                self.float_registers[self.next_8_bits() as usize] = register1 + register2;
            }
            OpCode::SUBF64 => {
                /* ADDF64 reg1 reg2 regTarget */
                let register1 = self.float_registers[self.next_8_bits() as usize];
                let register2 = self.float_registers[self.next_8_bits() as usize];
                self.float_registers[self.next_8_bits() as usize] = register1 - register2;
            }
            OpCode::MULF64 => {
                /* ADDF64 reg1 reg2 regTarget */
                let register1 = self.float_registers[self.next_8_bits() as usize];
                let register2 = self.float_registers[self.next_8_bits() as usize];
                self.float_registers[self.next_8_bits() as usize] = register1 * register2;
            }
            OpCode::DIVF64 => {
                /* DIVF64 reg1 reg2 regTarget */
                let register1 = self.float_registers[self.next_8_bits() as usize];
                let register2 = self.float_registers[self.next_8_bits() as usize];
                self.float_registers[self.next_8_bits() as usize] = register1 / register2;
            }
            OpCode::EQF64 => {
                /* EQF64 reg1 reg2 regTarget */
                let register1 = self.float_registers[self.next_8_bits() as usize];
                let register2 = self.float_registers[self.next_8_bits() as usize];
                self.comparison_flag = (register1 - register2).abs() < EPSILON;
            }
            OpCode::NEQF64 => {
                /* NEQF64 reg1 reg2 regTarget */
                let register1 = self.float_registers[self.next_8_bits() as usize];
                let register2 = self.float_registers[self.next_8_bits() as usize];
                self.comparison_flag = (register1 - register2).abs() > EPSILON;
            }
            OpCode::GTF64 => {
                /* GTF64 reg1 reg2 regTarget */
                let register1 = self.float_registers[self.next_8_bits() as usize];
                let register2 = self.float_registers[self.next_8_bits() as usize];
                self.comparison_flag = register1 > register2;
            }
            OpCode::GTEF64 => {
                /* GTEF64 reg1 reg2 regTarget */
                let register1 = self.float_registers[self.next_8_bits() as usize];
                let register2 = self.float_registers[self.next_8_bits() as usize];
                self.comparison_flag = register1 >= register2;
            }
            OpCode::LTF64 => {
                /* LTF64 reg1 reg2 regTarget */
                let register1 = self.float_registers[self.next_8_bits() as usize];
                let register2 = self.float_registers[self.next_8_bits() as usize];
                self.comparison_flag = register1 < register2;
            }
            OpCode::LTEF64 => {
                /* LTEF64 reg1 reg2 regTarget */
                let register1 = self.float_registers[self.next_8_bits() as usize];
                let register2 = self.float_registers[self.next_8_bits() as usize];
                self.comparison_flag = register1 <= register2;
            }
            OpCode::AND => {
                /* AND reg1 reg2 regTarget */
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 & register2;
            }
            OpCode::OR => {
                /* OR reg1 reg2 regTarget */
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 | register2;
            }
            OpCode::XOR => {
                /* XOR reg1 reg2 regTarget */
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 ^ register2;
            }
            OpCode::NOT => {
                /* NOT reg1 regTarget */
                let register1 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = !register1;
            }
            OpCode::PUSH => {
                /* PUSH reg1 */
                let register1 = self.registers[self.next_8_bits() as usize];
                self.stack.push(register1);
                self.sp += 1;
            }
            OpCode::POP => {
                /* POP reg1 */
                let register1 = self.next_8_bits() as usize;
                self.registers[register1] = self.stack.pop().unwrap();
                self.sp -= 1;
            }
            OpCode::CALL => {
                /* CALL label_usage */
                let ret_dest = self.pc + 1;

                let function = self.registers[self.next_8_bits() as usize];

                self.stack.push(ret_dest as i32);
                self.stack.push(self.bp as i32);

                if self.stack.len() > DEFAULT_STACK_SIZE {
                    return Err("Error: Stack Overflow.");
                }
                self.bp = self.sp;

                self.pc = function as usize;
            }
            OpCode::RET => {
                /* RET */
                self.sp = self.bp;
                self.bp = self.stack.pop().unwrap() as usize;
                self.pc = self.stack.pop().unwrap() as usize;
            }
            OpCode::HLT => {
                println!("\nexit(0)");
                return Ok(true);
            }
            OpCode::PRTS => {
                /* PRTS reg */
                let register = self.next_8_bits() as usize;
                let start_offset = self.registers[register] as usize;
                let mut end_offset = start_offset;

                let slice = self.ro_data.as_slice();
                while slice[end_offset] != 0 {
                    end_offset += 1;
                }

                let result = from_utf8(&slice[start_offset..end_offset]);
                match result {
                    Ok(str) => {
                        print!("{}", str);
                    }
                    Err(e) => {
                        println!("Error decoding string constant for PTRS instruction:{:#?}", e)
                    }
                }
            }
            OpCode::IGL => {
                print!("Unrecognized opcode {} found! Terminating...", code);
                return Err("Unrecognized opcode found, Terminated.");
            }
        }
        return Ok(false);
    }

    pub fn run(&mut self) {
        let mut terminated = false;
        while !terminated {
            terminated = self.execute_instruction().unwrap();
        }
    }

    fn verify_header(&mut self) -> bool {
        if self.program[0..4] != ELF_HEADER_PREFIX.to_owned() {
            return false;
        }
        true
    }

    pub fn load_program(&mut self, program: Vec<u8>) {
        self.program = program;
        if !self.verify_header() {
            println!("Not ELF file.")
        }
        let pro: Vec<u8> = self.program[64..].to_owned();
        self.program = pro;
    }

    pub fn set_ro_data(&mut self, ro_section: Vec<u8>) {
        self.ro_data = ro_section;
    }
}
