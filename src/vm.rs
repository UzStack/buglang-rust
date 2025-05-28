use crate::chunk::Chunk;
use crate::compiler::{OpCode, Parser};
use crate::value::{Value, ValueType};

pub struct VM {
    stack: Vec<Value>,
    stack_top: u8,
    chunk: Chunk,
    ip: u8,
}

impl VM {
    pub fn new(source: &str) -> VM {
        let mut parser = Parser::new(source);
        parser.compile();
        VM {
            stack: Vec::new(),
            stack_top: 0,
            ip: 0,
            chunk: parser.get_chunk(),
        }
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
        self.stack_top += 1;
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.chunk.code[self.ip as usize];
        self.ip += 1;
        byte
    }

    fn read_constant(&mut self) -> Value {
        let index = self.read_byte() as usize;
        self.chunk.constants[index].clone()
    }

    fn add(&mut self) {
        let val2 = self.pop();
        let val1 = self.pop();
        self.push(Value::new(ValueType::Number, val1.value + val2.value));
    }

    fn derive(&mut self) {
        let val2 = self.pop();
        let val1 = self.pop();
        self.push(Value::new(ValueType::Number, val1.value / val2.value));
    }

    fn subtract(&mut self) {
        let val2 = self.pop();
        let val1 = self.pop();
        self.push(Value::new(ValueType::Number, val1.value - val2.value));
    }

    fn multiply(&mut self) {
        let val2 = self.pop();
        let val1 = self.pop();
        self.push(Value::new(ValueType::Number, val1.value * val2.value));
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    fn print_stack(&mut self) {
        print!("Stack: [");
        for i in &self.stack {
            print!(" {:?} ", i.value);
        }
        println!("]");
    }

    pub fn run(&mut self) {
        loop {
            self.chunk.disassemble_instruction(self.ip as usize);
            let instruction = self.read_byte();
            match instruction {
                x if x == OpCode::Constant as u8 => {
                    let constant = self.read_constant();
                    self.push(constant);
                }
                x if x == OpCode::Add as u8 => self.add(),
                x if x == OpCode::Divide as u8 => self.derive(),
                x if x == OpCode::Subtract as u8 => self.subtract(),
                x if x == OpCode::Multiply as u8 => self.multiply(),
                x if x == OpCode::Return as u8 => {
                    println!("Return value: {}", self.pop().value);
                    break;
                }
                _ => {
                    println!("Return value: {}", self.pop().value);
                }
            }
            self.print_stack();
        }
    }
}
