use crate::vm::instruction::OpCode;
use std::str::from_utf8;

pub const TMP_REGISTER: u8 = 0x20 - 1;

#[derive(Debug)]
pub struct VM {
    /* 8bits for opcode , 8bits for register number , 16 bits for numbers just 2<<16 = 65536(unsigned) */
    pub registers: [i32; 32],
    /* program counter */
    pc: usize,
    /* program memory */
    pub program: Vec<u8>,
    pub ro_data: Vec<u8>,
    pub heap: Vec<u8>,
    remainder: u32,
    eq_flag: bool,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            pc: 0,
            program: Vec::new(),
            ro_data: Vec::new(),
            heap: Vec::new(),
            remainder: 0,
            eq_flag: false,
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

        match self.decode_opcode() {
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
            OpCode::JMP_F => {
                /* JMP_F regTarget */
                let step = self.registers[self.next_8_bits() as usize];
                self.pc += step as usize;
            }
            OpCode::JMP_B => {
                /* JMP_B regTarget */
                let step = self.registers[self.next_8_bits() as usize];
                self.pc -= step as usize;
            }
            OpCode::EQ => {
                /* EQ reg0 reg1 */
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                if register1 == register2 {
                    self.eq_flag = true;
                } else {
                    self.eq_flag = false;
                }

                self.next_8_bits();
            }
            OpCode::JEQ => {
                let target = self.registers[self.next_8_bits() as usize];
                if self.eq_flag {
                    self.pc = target as usize;
                }
            }
            OpCode::ALOC => {
                let register = self.next_8_bits() as usize;
                let bytes = self.registers[register];
                let new_end = self.heap.len() as i32 + bytes;
                self.heap.resize(new_end as usize, 0);
            }
            OpCode::HLT => {
                print!("HLT");
                return true;
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
                let start_offset = self.registers[TMP_REGISTER as usize] as usize;
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
            _ => {
                print!("Unrecognized opcode found! Terminating...");
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

    pub fn set_program(&mut self, program: Vec<u8>) {
        self.program = program;
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
    fn should_opcode_igl() {
        let mut vm = VM::new();
        vm.program = vec![200, 0, 0, 0];
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
                          6, 0, 0, 0];  /*JMP 0; */
        vm.run_once();
        vm.run_once();
        assert_eq!(vm.pc, 1);
    }

    #[test]
    fn should_jmp_f() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 0, 3, /*LOAD 0 #3; */
                          7, 0, 0, 0];  /*JMP_F 0; */
        vm.run_once();
        assert_eq!(vm.pc, 4);
        vm.run_once();
        assert_eq!(vm.pc, 9);
    }

    #[test]
    fn should_jmp_b() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 0, 3, /*LOAD 0 #3; */
                          8, 0, 0, 0];  /*JMP_F 0; */
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
                          9, 0, 1, 0, /*EQ 0 1; */
                          1, 2, 1, 244, /*LOAD 2 #500; */
                          1, 3, 0, 244, /*LOAD 3 #244; */
                          9, 2, 3, 0];  /*EQ 2 3; */

        vm.run();
        assert_eq!(vm.eq_flag, false);
    }

    #[test]
    fn should_jeq() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 1, 244, /*LOAD 0 #500; */
                          1, 1, 1, 244, /*LOAD 1 #500; */
                          9, 0, 1, 0, /*EQ 0 1; */
                          1, 2, 0, 3, /*LOAD 2 #3; */
                          10, 2, 0, 0];  /*JEQ 2; */

        vm.run_once(); /*LOAD 0 #500; */
        vm.run_once(); /*LOAD 1 #500; */
        vm.run_once(); /*EQ 0 1; */
        assert_eq!(vm.eq_flag, true);
        vm.run_once(); /*LOAD 2 #3; */
        vm.run_once(); /*JEQ 2; */
        assert_eq!(vm.pc, 3);
    }

    #[test]
    fn should_inc() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 1, 244, /*LOAD 0 #500; */
                          12, 0, 0, 0]; /*INC $0; */

        vm.run_once(); /*LOAD 0 #500; */
        vm.run_once(); /*INC $0; */
        assert_eq!(vm.registers[0], 501);
    }

    #[test]
    fn should_dec() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 1, 244, /*LOAD 0 #500; */
                          13, 0, 0, 0]; /*DEC $0; */

        vm.run_once(); /*LOAD 0 #500; */
        vm.run_once(); /*DEC $0; */
        assert_eq!(vm.registers[0], 499);
    }


    #[test]
    fn should_aloc() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 1, 244, /*LOAD 0 #500; */
                          11, 0, 0, 0]; /*ALOC $0; */

        vm.run_once(); /*LOAD 0 #500; */
        vm.run_once(); /*ALOC $0; */
        assert_eq!(vm.heap.len(), 500);
    }
}
