use crate::assembler::elf::ELF_HEADER_PREFIX;
use crate::vm::instruction::OpCode;
use std::f64::EPSILON;
use std::str::from_utf8;

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
            OpCode::LOAD => self.handle_load(),
            OpCode::ADD => self.handle_add(),
            OpCode::SUB => self.handle_sub(),
            OpCode::MUL => self.handle_mul(),
            OpCode::DIV => self.handle_div(),
            OpCode::JMP => self.handle_jmp(),
            OpCode::JMPF => self.handle_jmpf(),
            OpCode::JMPB => self.handle_jmpb(),
            OpCode::EQ => self.handle_eq(),
            OpCode::LT => self.handle_lt(),
            OpCode::LTE => self.handle_lte(),
            OpCode::GT => self.handle_gt(),
            OpCode::GTE => self.handle_gte(),
            OpCode::JE => self.handle_je(),
            OpCode::JNE => self.handle_jne(),
            OpCode::JL => self.handle_jl(),
            OpCode::JG => self.handle_jg(),
            OpCode::ALOC => self.handle_aloc(),
            OpCode::INC => self.handle_inc(),
            OpCode::DEC => self.handle_dec(),
            OpCode::LOADF64 => self.handle_load_f64(),
            OpCode::ADDF64 => self.handle_add_f64(),
            OpCode::SUBF64 => self.handle_sub_f64(),
            OpCode::MULF64 => self.handle_mul_f64(),
            OpCode::DIVF64 => self.handle_div_f64(),
            OpCode::EQF64 => self.handle_eq_f64(),
            OpCode::NEQF64 => self.handle_neq_f64(),
            OpCode::GTF64 => self.handle_gt_f64(),
            OpCode::GTEF64 => self.handle_gte_f64(),
            OpCode::LTF64 => self.handle_lt_f64(),
            OpCode::LTEF64 => self.handle_lte_f64(),
            OpCode::AND => self.handle_and(),
            OpCode::OR => self.handle_or(),
            OpCode::XOR => self.handle_xor(),
            OpCode::NOT => self.handle_not(),
            OpCode::PUSH => self.handle_push(),
            OpCode::POP => self.handle_pop(),
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
            OpCode::RET => self.handle_ret(),
            OpCode::HLT => {
                println!("\nexit(0)");
                return Ok(true);
            }
            OpCode::PRTS => self.handel_prts(),
            OpCode::IGL => {
                print!("Unrecognized opcode {} found! Terminating...", code);
                return Err("Unrecognized opcode found, Terminated.");
            }
        }
        return Ok(false);
    }

    fn handel_prts(&mut self) -> () {
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
            Err(e) => println!(
                "Error decoding string constant for PTRS instruction:{:#?}",
                e
            ),
        }
    }

    fn handle_ret(&mut self) -> () {
        /* RET */
        self.sp = self.bp;
        self.bp = self.stack.pop().unwrap() as usize;
        self.pc = self.stack.pop().unwrap() as usize;
    }

    fn handle_pop(&mut self) -> () {
        /* POP reg1 */
        let register1 = self.next_8_bits() as usize;
        self.registers[register1] = self.stack.pop().unwrap();
        self.sp -= 1;
    }

    fn handle_push(&mut self) -> () {
        /* PUSH reg1 */
        let register1 = self.registers[self.next_8_bits() as usize];
        self.stack.push(register1);
        self.sp += 1;
    }

    fn handle_not(&mut self) -> () {
        /* NOT reg1 regTarget */
        let register1 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = !register1;
    }

    fn handle_xor(&mut self) -> () {
        /* XOR reg1 reg2 regTarget */
        let register1 = self.registers[self.next_8_bits() as usize];
        let register2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = register1 ^ register2;
    }

    fn handle_or(&mut self) -> () {
        /* OR reg1 reg2 regTarget */
        let register1 = self.registers[self.next_8_bits() as usize];
        let register2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = register1 | register2;
    }

    fn handle_and(&mut self) -> () {
        /* AND reg1 reg2 regTarget */
        let register1 = self.registers[self.next_8_bits() as usize];
        let register2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = register1 & register2;
    }

    fn handle_lte_f64(&mut self) -> () {
        /* LTEF64 reg1 reg2 regTarget */
        let register1 = self.float_registers[self.next_8_bits() as usize];
        let register2 = self.float_registers[self.next_8_bits() as usize];
        self.comparison_flag = register1 <= register2;
    }

    fn handle_lt_f64(&mut self) -> () {
        /* LTF64 reg1 reg2 regTarget */
        let register1 = self.float_registers[self.next_8_bits() as usize];
        let register2 = self.float_registers[self.next_8_bits() as usize];
        self.comparison_flag = register1 < register2;
    }

    fn handle_gte_f64(&mut self) -> () {
        /* GTEF64 reg1 reg2 regTarget */
        let register1 = self.float_registers[self.next_8_bits() as usize];
        let register2 = self.float_registers[self.next_8_bits() as usize];
        self.comparison_flag = register1 >= register2;
    }

    fn handle_gt_f64(&mut self) -> () {
        /* GTF64 reg1 reg2 regTarget */
        let register1 = self.float_registers[self.next_8_bits() as usize];
        let register2 = self.float_registers[self.next_8_bits() as usize];
        self.comparison_flag = register1 > register2;
    }

    fn handle_neq_f64(&mut self) -> () {
        /* NEQF64 reg1 reg2 regTarget */
        let register1 = self.float_registers[self.next_8_bits() as usize];
        let register2 = self.float_registers[self.next_8_bits() as usize];
        self.comparison_flag = (register1 - register2).abs() > EPSILON;
    }

    fn handle_eq_f64(&mut self) -> () {
        /* EQF64 reg1 reg2 regTarget */
        let register1 = self.float_registers[self.next_8_bits() as usize];
        let register2 = self.float_registers[self.next_8_bits() as usize];
        self.comparison_flag = (register1 - register2).abs() < EPSILON;
    }

    fn handle_div_f64(&mut self) -> () {
        /* DIVF64 reg1 reg2 regTarget */
        let register1 = self.float_registers[self.next_8_bits() as usize];
        let register2 = self.float_registers[self.next_8_bits() as usize];
        self.float_registers[self.next_8_bits() as usize] = register1 / register2;
    }

    fn handle_mul_f64(&mut self) -> () {
        /* ADDF64 reg1 reg2 regTarget */
        let register1 = self.float_registers[self.next_8_bits() as usize];
        let register2 = self.float_registers[self.next_8_bits() as usize];
        self.float_registers[self.next_8_bits() as usize] = register1 * register2;
    }

    fn handle_sub_f64(&mut self) -> () {
        /* ADDF64 reg1 reg2 regTarget */
        let register1 = self.float_registers[self.next_8_bits() as usize];
        let register2 = self.float_registers[self.next_8_bits() as usize];
        self.float_registers[self.next_8_bits() as usize] = register1 - register2;
    }

    fn handle_add_f64(&mut self) -> () {
        /* ADDF64 reg1 reg2 regTarget */
        let register1 = self.float_registers[self.next_8_bits() as usize];
        let register2 = self.float_registers[self.next_8_bits() as usize];
        self.float_registers[self.next_8_bits() as usize] = register1 + register2;
    }

    fn handle_load_f64(&mut self) -> () {
        /* LOADF64 reg numberH numberL*/
        let register = self.next_8_bits() as usize;
        let number = f64::from(self.next_16_bits());
        self.float_registers[register] = number;
    }

    fn handle_dec(&mut self) -> () {
        /* DEC reg */
        self.registers[self.next_8_bits() as usize] -= 1;
    }

    fn handle_inc(&mut self) -> () {
        /* INC reg */
        self.registers[self.next_8_bits() as usize] += 1;
    }

    fn handle_aloc(&mut self) -> () {
        let register = self.next_8_bits() as usize;
        let bytes = self.registers[register];
        let new_end = self.heap.len() as i32 + bytes;
        self.heap.resize(new_end as usize, 0);
    }

    fn handle_jg(&mut self) -> () {
        /* JG regTarget */
        let target = self.registers[self.next_8_bits() as usize];
        if self.comparison_flag {
            self.pc = target as usize;
        }
    }

    fn handle_jl(&mut self) -> () {
        /* JL regTarget */
        let target = self.registers[self.next_8_bits() as usize];
        if self.comparison_flag {
            self.pc = target as usize;
        }
    }

    fn handle_jne(&mut self) -> () {
        /* JNE regTarget */
        let target = self.registers[self.next_8_bits() as usize];
        if !self.comparison_flag {
            self.pc = target as usize;
        }
    }

    fn handle_je(&mut self) -> () {
        /* JE regTarget */
        let target = self.registers[self.next_8_bits() as usize];
        if self.comparison_flag {
            self.pc = target as usize;
        }
    }

    fn handle_gte(&mut self) -> () {
        /* GTE reg0 reg1 */
        let register1 = self.registers[self.next_8_bits() as usize];
        let register2 = self.registers[self.next_8_bits() as usize];

        if register1 >= register2 {
            self.comparison_flag = true;
        } else {
            self.comparison_flag = false;
        }
    }

    fn handle_gt(&mut self) -> () {
        /* LT reg0 reg1 */
        let register1 = self.registers[self.next_8_bits() as usize];
        let register2 = self.registers[self.next_8_bits() as usize];

        if register1 > register2 {
            self.comparison_flag = true;
        } else {
            self.comparison_flag = false;
        }
    }

    fn handle_lte(&mut self) -> () {
        /* LTE reg0 reg1 */
        let register1 = self.registers[self.next_8_bits() as usize];
        let register2 = self.registers[self.next_8_bits() as usize];

        if register1 <= register2 {
            self.comparison_flag = true;
        } else {
            self.comparison_flag = false;
        }
    }

    fn handle_lt(&mut self) -> () {
        /* LT reg0 reg1 */
        let register1 = self.registers[self.next_8_bits() as usize];
        let register2 = self.registers[self.next_8_bits() as usize];

        if register1 < register2 {
            self.comparison_flag = true;
        } else {
            self.comparison_flag = false;
        }
    }

    fn handle_eq(&mut self) -> () {
        /* EQ reg0 reg1 */
        let register1 = self.registers[self.next_8_bits() as usize];
        let register2 = self.registers[self.next_8_bits() as usize];

        if register1 == register2 {
            self.comparison_flag = true;
        } else {
            self.comparison_flag = false;
        }
    }

    fn handle_jmpb(&mut self) -> () {
        /* JMPB regTarget */
        let step = self.registers[self.next_8_bits() as usize];
        self.pc -= step as usize;
    }

    fn handle_jmpf(&mut self) -> () {
        /* JMPF regTarget */
        let step = self.registers[self.next_8_bits() as usize];
        self.pc += step as usize;
    }

    fn handle_jmp(&mut self) -> () {
        /* JMP regTarget */
        let target = self.registers[self.next_8_bits() as usize];
        self.pc = target as usize;
    }

    fn handle_div(&mut self) -> () {
        /* DIV reg1 reg2 regTarget */
        let register1 = self.registers[self.next_8_bits() as usize];
        let register2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = (register1 / register2) as i32;
        self.remainder = (register1 % register2) as u32;
    }

    fn handle_mul(&mut self) -> () {
        /* MUL reg1 reg2 regTarget */
        let register1 = self.registers[self.next_8_bits() as usize];
        let register2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = (register1 * register2) as i32;
    }

    fn handle_sub(&mut self) -> () {
        /* SUB reg1 reg2 regTarget */
        let register1 = self.registers[self.next_8_bits() as usize];
        let register2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = (register1 - register2) as i32;
    }

    fn handle_add(&mut self) -> () {
        /* ADD reg1 reg2 regTarget */
        let register1 = self.registers[self.next_8_bits() as usize];
        let register2 = self.registers[self.next_8_bits() as usize];
        self.registers[self.next_8_bits() as usize] = register1 + register2;
    }

    fn handle_load(&mut self) -> () {
        /* LOAD reg numberH numberL*/
        let register = self.next_8_bits() as usize;
        let number = self.next_16_bits() as u16;
        self.registers[register] = number as i32;
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
