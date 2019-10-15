use Dulang::repl::repl::REPL;

fn main() {
    let mut repl = REPL::new();
//    repl.run();
    repl.run_asm_file("/Users/xingfeng.yang/project/live-code/rust/Dulang/asm/for_each.asm");
}
