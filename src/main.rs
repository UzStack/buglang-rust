mod compiler;
mod chunk;
mod value;
mod scanner;
use compiler::Parser;
use std::fs;

fn main() {
    let mut code = fs::read_to_string("code.lox").expect("Unable to read file");
    code.push('\0');
    let mut parser = Parser::new(&code);
    parser.compile();
}
