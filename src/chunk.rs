use crate::compiler::OpCode;
use crate::value::Value;


#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<i32>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: u8, line: i32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        return self.constants.len() as u8 - 1;
    }

    pub fn write_constant(&mut self, value: Value, line: i32) {
        let index = self.add_constant(value);
        self.write(index, line);
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn simple_instruction(&self, offset: usize) -> usize {
        // println!("{:?}", self.code[offset]);
        offset + 1
    }

    pub fn contant_instrcution(&self, offset: usize) -> usize {
        let constant = self.code[offset + 1];
        println!("{:?}", self.constants[constant as usize].value);
        offset + 2
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        let instruction = self.code[offset];
        println!(" --- > {:?}", OpCode::from_u8(instruction).unwrap());
        match instruction {
            val if OpCode::from_u8(val).unwrap() == OpCode::Constant => {
                return self.contant_instrcution(offset);
            }
            _ => return self.simple_instruction(offset),
        }
    }
}
