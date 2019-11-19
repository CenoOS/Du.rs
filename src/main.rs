use Dulang::repl::repl::REPL;
use Dulang::repl::terminal::terminal::Terminal;

fn main() {
    let mut repl = REPL::new();
    repl.run();

//    let term = Terminal::new();
//        print!("T{}", term.read_line().unwrap());
}

