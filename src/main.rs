mod formatter;

use formatter::Formatter;

use std::env;
use std::fs;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("Not enough args");
    }
    log::info!("Running on file {}", args.get(1).unwrap());
    let file = args.get(1).unwrap();

    let asm_text = fs::read_to_string(file).unwrap();
    let mut asm_formatter = Formatter::new(asm_text);
    asm_formatter.remove_double_lines();
    asm_formatter.fix_spacing();
    asm_formatter.fix_whitespace();
    asm_formatter.fix_indenting();
    asm_formatter.print();
}
