use crate::chunk::Chunk;
use crate::compiler::{OpCode, Parser};
use crate::value::{Value, ValueType};

pub struct VM {
    stack: Vec<Value>,
    chunk: Chunk,
    ip: u8,
}

impl VM {
    pub fn new(source: &str) -> VM {
        let mut parser = Parser::new(source);
        parser.compile();
        VM {
            stack: Vec::new(),
            ip: 0,
            chunk: parser.get_chunk(),
        }
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
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
            match OpCode::try_from(instruction) {
                Ok(OpCode::Constant) => {
                    let constant = self.read_constant();
                    self.push(constant);
                }
                Ok(OpCode::Add) => self.add(),
                Ok(OpCode::Divide) => self.derive(),
                Ok(OpCode::Subtract) => self.subtract(),
                Ok(OpCode::Multiply) => self.multiply(),
                Ok(OpCode::Return) => {
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
