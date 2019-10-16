use crate::vm::instruction::OpCode;
use std::str::from_utf8;
use crate::assembler::elf::ELF_HEADER_PREFIX;
use std::f64::EPSILON;
use std::intrinsics::size_of_val;

pub const DEFAULT_STACK_SIZE: usize = 2097152;
pub const TMP_REGISTER: u8 = 0x20 - 1;

#[derive(Debug)]
pub struct VM {
    /* 8bits for opcode , 8bits for register number , 16 bits for numbers just 2<<16 = 65536(unsigned) */
    pub registers: [i32; 32],
    pub float_registers: [f64; 32],
    /* program counter */
    pc: usize,

    /* program memory */
    pub program: Vec<u8>,
    pub ro_data: Vec<u8>,
    pub heap: Vec<u8>,

    sp: usize,
    stack: Vec<i32>,

    bp: usize,

    remainder: u32,
    comparison_flag: bool,

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

    fn execute_instruction(&mut self) -> bool {
        // fly away
        if self.pc >= self.program.len() {
            return true;
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
                self.float_registers[self.next_8_bits() as usize] = (register1 / register2);
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
                self.comparison_flag = (register1 > register2);
            }
            OpCode::GTEF64 => {
                /* GTEF64 reg1 reg2 regTarget */
                let register1 = self.float_registers[self.next_8_bits() as usize];
                let register2 = self.float_registers[self.next_8_bits() as usize];
                self.comparison_flag = (register1 >= register2);
            }
            OpCode::LTF64 => {
                /* LTF64 reg1 reg2 regTarget */
                let register1 = self.float_registers[self.next_8_bits() as usize];
                let register2 = self.float_registers[self.next_8_bits() as usize];
                self.comparison_flag = (register1 < register2);
            }
            OpCode::LTEF64 => {
                /* LTEF64 reg1 reg2 regTarget */
                let register1 = self.float_registers[self.next_8_bits() as usize];
                let register2 = self.float_registers[self.next_8_bits() as usize];
                self.comparison_flag = (register1 <= register2);
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
                let register1 = self.registers[self.next_8_bits() as usize];
                self.registers[register1 as usize] = self.stack.pop().unwrap();
                self.sp -= 1;
            }
            OpCode::CALL => {
                /* POP label_usage */
                let ret_dest = self.pc + 1;

                let function = self.registers[self.next_8_bits() as usize];

                self.stack.push(ret_dest as i32);
                self.stack.push(bp);

                self.bp = self.sp;

                self.pc = function as usize;
            }
            OpCode::RET => {
                self.sp = self.bp;
                self.bp = self.stack.pop().unwrap() as usize;
                self.pc = self.stack.pop().unwrap() as usize;
            }
            OpCode::HLT => {
                println!("\nexit(0)");
                return true;
            }
            OpCode::IGL => {
                print!("Unrecognized opcode {} found! Terminating...", code);
                return true;
            }
        }
        return false;
    }

    pub fn run(&mut self) {
        let mut terminated = false;
        while !terminated {
            terminated = self.execute_instruction();
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_vm() {
        let vm = VM::new();
        assert_eq!(vm.registers[0], 0);
    }

    #[test]
    fn should_halt() {
        let mut vm = VM::new();
        vm.program = vec![0, 0, 0, 0];
        vm.run();
        assert_eq!(vm.pc, 1);
    }


    #[test]
    fn should_load_instruction() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 1, 244];/*LOAD 0 #500; 0b0000000111110100 = 500(oct) */
        vm.run();
        assert_eq!(vm.registers[0], 500);
    }

    #[test]
    fn should_add() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 1, 244, /*LOAD 0 #500; */
                          1, 1, 1, 244, /*LOAD 1 #500; */
                          2, 0, 1, 2];  /*ADD 0 1 2; */
        vm.run();
        assert_eq!(vm.registers[0], 500);
        assert_eq!(vm.registers[1], 500);
        assert_eq!(vm.registers[2], 1000);
    }

    #[test]
    fn should_sub() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 1, 244, /*LOAD 0 #500; */
                          1, 1, 0, 244, /*LOAD 1 #244; */
                          3, 0, 1, 2];  /*SUB 0 1 2; */
        vm.run();
        assert_eq!(vm.registers[0], 500);
        assert_eq!(vm.registers[1], 244);
        assert_eq!(vm.registers[2], 256);
    }

    #[test]
    fn should_mul() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 1, 244, /*LOAD 0 #500; */
                          1, 1, 0, 3, /*LOAD 1 #3; */
                          4, 0, 1, 2];  /*MUL 0 1 2; */
        vm.run();
        assert_eq!(vm.registers[0], 500);
        assert_eq!(vm.registers[1], 3);
        assert_eq!(vm.registers[2], 1500);
    }

    #[test]
    fn should_div() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 1, 244, /*LOAD 0 #500; */
                          1, 1, 0, 3, /*LOAD 1 #3; */
                          5, 0, 1, 2];  /*DIV 0 1 2; */
        vm.run();
        assert_eq!(vm.registers[0], 500);
        assert_eq!(vm.registers[1], 3);
        assert_eq!(vm.registers[2], 166);
        assert_eq!(vm.remainder, 2);
    }

    #[test]
    fn should_jmp() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 0, 1, /*LOAD 0 #1; */
                          6, 0];        /*JMP 0; */
        vm.run_once();
        vm.run_once();
        assert_eq!(vm.pc, 1);
    }

    #[test]
    fn should_jmpf() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 0, 3, /*LOAD 0 #3; */
                          7, 0];        /*JMPF 0; */
        vm.run_once();
        assert_eq!(vm.pc, 4);
        vm.run_once();
        assert_eq!(vm.pc, 9);
    }

    #[test]
    fn should_jmpb() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 0, 3, /*LOAD 0 #3; */
                          8, 0];        /*JMPB 0; */
        vm.run_once();
        assert_eq!(vm.pc, 4);
        vm.run_once();
        assert_eq!(vm.pc, 3);
    }

    #[test]
    fn should_eq() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 1, 244, /*LOAD 0 #500; */
                          1, 1, 1, 244, /*LOAD 1 #500; */
                          9, 0, 1, /*EQ 0 1; */
                          1, 2, 1, 244, /*LOAD 2 #500; */
                          1, 3, 0, 244, /*LOAD 3 #244; */
                          9, 2, 3];     /*EQ 2 3; */

        vm.run();
        assert_eq!(vm.comparison_flag, false);
    }

    #[test]
    fn should_jeq() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 1, 244, /*LOAD 0 #500; */
                          1, 1, 1, 244, /*LOAD 1 #500; */
                          9, 0, 1, /*EQ 0 1; */
                          1, 2, 0, 3, /*LOAD 2 #3; */
                          10, 2];       /*JEQ 2; */

        vm.run_once(); /*LOAD 0 #500; */
        vm.run_once(); /*LOAD 1 #500; */
        vm.run_once(); /*EQ 0 1; */
        assert_eq!(vm.comparison_flag, true);
        vm.run_once(); /*LOAD 2 #3; */
        vm.run_once(); /*JEQ 2; */
        assert_eq!(vm.pc, 3);
    }

    #[test]
    fn should_jne() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 1, 244, /*LOAD 0 #500; */
                          1, 1, 1, 243, /*LOAD 1 #499; */
                          9, 0, 1, /*EQ 0 1; */
                          1, 2, 0, 3, /*LOAD 2 #3; */
                          15, 2];       /*JNE 2; */

        vm.run_once(); /*LOAD 0 #500; */
        vm.run_once(); /*LOAD 1 #499; */
        vm.run_once(); /*EQ 0 1; */
        assert_eq!(vm.comparison_flag, false);
        vm.run_once(); /*LOAD 2 #3; */
        vm.run_once(); /*jne 2; */
        assert_eq!(vm.pc, 3);
    }

    #[test]
    fn should_lt() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 1, 243, /*LOAD 0 #499; */
                          1, 1, 1, 244, /*LOAD 1 #500; */
                          18, 0, 1]; /*LT 0 1; */

        vm.run();
        assert_eq!(vm.comparison_flag, true);
    }

    #[test]
    fn should_gt() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 1, 244, /*LOAD 0 #500; */
                          1, 1, 1, 243, /*LOAD 1 #499; */
                          20, 0, 1]; /*GT 0 1; */

        vm.run();
        assert_eq!(vm.comparison_flag, true);
    }

    #[test]
    fn should_jlt() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 1, 243, /*LOAD 0 #499; */
                          1, 1, 1, 244, /*LOAD 1 #500; */
                          18, 0, 1, /*LT 0 1; */
                          1, 2, 0, 3, /*LOAD 2 #3; */
                          16, 2];       /*JLT 2; */

        vm.run_once(); /*LOAD 0 #499; */
        vm.run_once(); /*LOAD 1 #500; */
        vm.run_once(); /*LT 0 1; */
        assert_eq!(vm.comparison_flag, true);
        vm.run_once(); /*LOAD 2 #3; */
        vm.run_once(); /*JLT 2; */
        assert_eq!(vm.pc, 3);
    }

    #[test]
    fn should_jgt() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 1, 244, /*LOAD 0 #500; */
                          1, 1, 1, 243, /*LOAD 1 #499; */
                          20, 0, 1, /*Gt 0 1; */
                          1, 2, 0, 3, /*LOAD 2 #3; */
                          17, 2];       /*JGT 2; */

        vm.run_once(); /*LOAD 0 #500; */
        vm.run_once(); /*LOAD 1 #499; */
        vm.run_once(); /*GT 0 1; */
        assert_eq!(vm.comparison_flag, true);
        vm.run_once(); /*LOAD 2 #3; */
        vm.run_once(); /*JGT 2; */
        assert_eq!(vm.pc, 3);
    }


    #[test]
    fn should_inc() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 1, 244, /*LOAD 0 #500; */
                          12, 0];       /*INC $0; */

        vm.run_once(); /*LOAD 0 #500; */
        vm.run_once(); /*INC $0; */
        assert_eq!(vm.registers[0], 501);
    }

    #[test]
    fn should_dec() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 1, 244, /*LOAD 0 #500; */
                          13, 0];       /*DEC $0; */

        vm.run_once(); /*LOAD 0 #500; */
        vm.run_once(); /*DEC $0; */
        assert_eq!(vm.registers[0], 499);
    }

    #[test]
    fn should_opcode_and() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 0, 3,
                          1, 1, 0, 7,
                          33, 0, 1, 2];
        vm.run();
        assert_eq!(vm.registers[2], 3 & 7);
    }

    #[test]
    fn should_opcode_or() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 0, 3,
                          1, 1, 0, 7,
                          34, 0, 1, 2];
        vm.run();
        assert_eq!(vm.registers[2], 3 | 7);
    }

    #[test]
    fn should_opcode_xor() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 0, 3,
                          1, 1, 0, 7,
                          35, 0, 1, 2];
        vm.run();
        assert_eq!(vm.registers[2], 3 ^ 7);
    }

    #[test]
    fn should_opcode_not() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 0, 3,
                          36, 0, 1];
        vm.run();
        assert_eq!(vm.registers[1], !3);
    }

    #[test]
    fn should_aloc() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 1, 244, /*LOAD 0 #500; */
                          11, 0];       /*ALOC $0; */

        vm.run_once(); /*LOAD 0 #500; */
        vm.run_once(); /*ALOC $0; */
        assert_eq!(vm.heap.len(), 500);
    }

    #[test]
    fn should_opcode_igl() {
        let mut vm = VM::new();
        vm.program = vec![200, 0, 0, 0];
        vm.run();
        assert_eq!(vm.pc, 1);
    }

    #[test]
    fn should_loop_add() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 0, 0,   // LOAD    $0  #0  #0
                          1, 1, 0, 50,  // LOAD    $1  #0  #50
                          1, 2, 0, 0,   // LOAD    $2  #0  #0
                          9, 0, 1,      // EQ      $0  $1
                          13, 1,        // DEC     $1
                          12, 2,        // INC     $2
                          1, 31, 0, 12, // LOAD    $31 #0  #12
                          15, 31];      // JNE     $31
        vm.run();
        assert_eq!(vm.pc, 25);
        assert_eq!(vm.registers[2], 51);
    }

    #[test]
    fn should_loop_add_and_print() {
        let mut vm = VM::new();
        vm.ro_data = vec![72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 46, 0,
                          79, 107, 44, 32, 53, 48, 32, 116, 105, 109, 101, 115, 32, 112, 114, 105,
                          110, 116, 32, 112, 97, 115, 115, 101, 100, 46, 0];
        vm.program = vec![0x01, 0x00, 0x00, 0x00,
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
                          0x0E, 0x1F];
        vm.run();
        assert_eq!(vm.pc, 37);
        assert_eq!(vm.registers[2], 51);
    }
}
