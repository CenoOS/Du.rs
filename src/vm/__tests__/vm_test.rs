/*
 * Copyright (c) 2019. NeroYang
 */

#[cfg(test)]
mod tests {
    use crate::vm::vm::VM;

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
    fn should_push() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 1, 244, /*LOAD 0 #500; */
                          37, 0, /*PUSH $0; */
                          1, 1, 1, 243, /*LOAD 1 #499; */
                          37, 1];       /*PUSH $1; */

        vm.run();
        assert_eq!(vm.stack.len(), 2);
        assert_eq!(vm.sp, 2);
        assert_eq!(vm.stack, vec![500, 499]);
    }

    #[test]
    fn should_pop() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 1, 244, /*LOAD 0 #500; */
                          37, 0, /*PUSH $0; */
                          1, 1, 1, 243, /*LOAD 1 #499; */
                          37, 1, /*PUSH $1; */
                          38, 2];        /*POP $2*/

        vm.run_once();
        vm.run_once();
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack, vec![500]);
        assert_eq!(vm.sp, 1);
        vm.run_once();
        vm.run_once();
        assert_eq!(vm.stack.len(), 2);
        assert_eq!(vm.stack, vec![500, 499]);
        assert_eq!(vm.sp, 2);
        vm.run_once();
        assert_eq!(vm.registers[2], 499);
        assert_eq!(vm.sp, 1);
    }

    #[test]
    #[should_panic(expected = "Error: Stack Overflow.")]
    fn should_stack_overflow_when_recursion_call() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 1, 244, /*LOAD 0 #500; */
                          37, 0, /*PUSH $0; */
                          1, 3, 0, 10, /*LOAD $3 10*/
                          39, 3]; /*call $3*/
        vm.run();
    }


    #[test]
    fn should_call() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 0, 10, /*LOAD $0 10*/
                          39, 0, /*call $0*/
                          1, 1, 1, 243, /*LOAD 1 #499; */
                          1, 2, 1, 242, /*LOAD 1 #498; */
                          1, 3, 1, 241, /*LOAD 1 #497; */
                          1, 4, 1, 240, /*LOAD 1 #496; */
                          1, 5, 1, 239]; /*LOAD 1 #495; */
        vm.run();
        assert_eq!(vm.stack.len(), 2);
        assert_eq!(vm.bp, 0);
        assert_eq!(vm.stack, vec![6, 0]);
        assert_eq!(vm.registers[0], 10);
        assert_eq!(vm.registers[1], 0);
        assert_eq!(vm.registers[2], 498);
        assert_eq!(vm.registers[3], 497);
        assert_eq!(vm.registers[4], 496);
        assert_eq!(vm.registers[5], 495);
    }


    #[test]
    fn should_ret() {
        let mut vm = VM::new();
        vm.program = vec![1, 0, 0, 11, /*LOAD $0 11*/
                          39, 0, /*call $0*/
                          1, 1, 1, 243, /*LOAD 1 #499; */
                          0, /*hlt */
                          1, 1, 1, 243, /*LOAD 1 #499; */
                          1, 2, 1, 242, /*LOAD 2 #498; */
                          1, 3, 1, 241, /*LOAD 3 #497; */
                          1, 4, 1, 240, /*LOAD 4 #496; */
                          1, 5, 1, 239, /*LOAD 5 #495; */
                          40];/*ret*/
        vm.run();
        assert_eq!(vm.stack.len(), 0);
        assert_eq!(vm.bp, 0);
        assert_eq!(vm.registers[0], 11);
        assert_eq!(vm.registers[1], 499);
        assert_eq!(vm.registers[2], 498);
        assert_eq!(vm.registers[3], 497);
        assert_eq!(vm.registers[4], 496);
        assert_eq!(vm.registers[5], 495);
    }


    #[test]
    #[should_panic(expected = "Unrecognized opcode found, Terminated.")]
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
