/*
 * Copyright (c) 2019. NeroYang
 */
pub struct ColorPrint {}

impl ColorPrint {
    fn println_black(msg: &str) {
        println!("\x1b[30m {} \x1b[0m", msg);
    }
    fn println_dark_gray(msg: &str) {
        println!("\x1b[1;30m {} \x1b[0m", msg);
    }
    fn println_blue(msg: &str) {
        println!("\x1b[0;34m {} \x1b[0m", msg);
    }
    pub(crate) fn println_light_blue(msg: &str) {
        println!("\x1b[1;34m {} \x1b[0m", msg);
    }
    pub(crate) fn println_green(msg: &str) {
        println!("\x1b[0;32m {} \x1b[0m", msg);
    }
    pub(crate) fn println_light_green(msg: &str) {
        println!("\x1b[1;32m {} \x1b[0m", msg);
    }
    pub(crate) fn println_cyan(msg: &str) {
        println!("\x1b[0;36m {} \x1b[0m", msg);
    }
    fn println_light_cyan(msg: &str) {
        println!("\x1b[1;36m {} \x1b[0m", msg);
    }
    pub(crate) fn println_red(msg: &str) {
        println!("\x1b[0;31m {} \x1b[0m", msg);
    }
    pub(crate) fn println_light_red(msg: &str) {
        println!("\x1b[1;31m {} \x1b[0m", msg);
    }
    pub(crate) fn println_purple(msg: &str) {
        println!("\x1b[0;35m {} \x1b[0m", msg);
    }
    pub(crate) fn println_light_purple(msg: &str) {
        println!("\x1b[1;35m {} \x1b[0m", msg);
    }
    fn println_brown(msg: &str) {
        println!("\x1b[0;33m {} \x1b[0m", msg);
    }
    fn println_yellow(msg: &str) {
        println!("\x1b[1;33m {} \x1b[0m", msg);
    }
    fn println_light_gray(msg: &str) {
        println!("\x1b[0;37m {} \x1b[0m", msg);
    }
    fn println_white(msg: &str) {
        println!("\x1b[1;37m {} \x1b[0m", msg);
    }

    fn print_black(msg: &str) {
        print!("\x1b[30m {} \x1b[0m", msg);
    }
    fn print_dark_gray(msg: &str) {
        print!("\x1b[1;30m {} \x1b[0m", msg);
    }
    fn print_blue(msg: &str) {
        print!("\x1b[0;34m {} \x1b[0m", msg);
    }
    fn print_light_blue(msg: &str) {
        print!("\x1b[1;34m {} \x1b[0m", msg);
    }
    pub(crate) fn print_green(msg: &str) {
        print!("\x1b[0;32m {} \x1b[0m", msg);
    }
    pub(crate) fn print_light_green(msg: &str) {
        print!("\x1b[1;32m {} \x1b[0m", msg);
    }
    pub(crate) fn print_cyan(msg: &str) {
        print!("\x1b[0;36m {} \x1b[0m", msg);
    }
    fn print_light_cyan(msg: &str) {
        print!("\x1b[1;36m {} \x1b[0m", msg);
    }
    pub(crate) fn print_red(msg: &str) {
        print!("\x1b[0;31m {} \x1b[0m", msg);
    }
    fn print_light_red(msg: &str) {
        print!("\x1b[1;31m {} \x1b[0m", msg);
    }
    pub(crate) fn print_purple(msg: &str) {
        print!("\x1b[0;35m {} \x1b[0m", msg);
    }
    pub(crate) fn print_light_purple(msg: &str) {
        print!("\x1b[1;35m {} \x1b[0m", msg);
    }
    fn print_brown(msg: &str) {
        print!("\x1b[0;33m {} \x1b[0m", msg);
    }
    fn print_yellow(msg: &str) {
        print!("\x1b[1;33m {} \x1b[0m", msg);
    }
    fn print_light_gray(msg: &str) {
        print!("\x1b[0;37m {} \x1b[0m", msg);
    }
    fn print_white(msg: &str) {
        print!("\x1b[1;37m {} \x1b[0m", msg);
    }
}
