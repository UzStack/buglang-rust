mod chunk;
mod compiler;
mod scanner;
mod value;
mod vm;

use std::fs;

fn main() {
    let mut code = fs::read_to_string("code.lox").expect("Unable to read file");
    code.push('\0');
    let mut v = vm::VM::new(&code);
    v.run();
}
