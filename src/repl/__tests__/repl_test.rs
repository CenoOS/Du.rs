/*
 * Copyright (c) 2019. NeroYang
 */
#[cfg(test)]
mod tests {
    use crate::repl::repl::REPL;

    #[test]
    fn should_run_asm() {
        let mut repl = REPL::new();
        repl.run_asm_file("asm/for_each.asm");
        assert_eq!(
            repl.vm.program,
            vec![
                0x01, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00, 0x32, 0x01, 0x02, 0x00, 0x00, 0x09, 0x00,
                0x01, 0x01, 0x1F, 0x00, 0x00, 0x0E, 0x1F, 0x0D, 0x01, 0x0C, 0x02, 0x01, 0x1F, 0x00,
                0x0C, 0x0F, 0x1F, 0x01, 0x1F, 0x00, 0x0F, 0x0E, 0x1F, 0x00
            ]
        );

        assert_eq!(
            repl.vm.ro_data,
            vec![
                72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 46, 10, 0, 79, 107, 44, 32,
                53, 48, 32, 116, 105, 109, 101, 115, 32, 112, 114, 105, 110, 116, 32, 112, 97, 115,
                115, 101, 100, 46, 0
            ]
        );
    }
}
