use crate::chunk::Chunk;
use crate::scanner::{Scanner, Token, TokenType};
use crate::value::{Value, ValueType};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Precedence {
    None,       // no precedence
    Assignment, // =
    Or,         // ||
    And,        // &&
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // .
}

impl Precedence {
    pub fn next(&self) -> Precedence {
        use Precedence::*;
        match *self {
            None => Assignment,
            Assignment => Or,
            Or => And,
            And => Equality,
            Equality => Comparison,
            Comparison => Term,
            Term => Factor,
            Factor => Unary,
            Unary => Call,
            Call => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OpCode {
    Return,   // return
    Constant, // constant
    Add,      // +
    Subtract, // -
    Multiply, // *
    Divide,   // /
}

impl OpCode {
    pub fn from_u8(byte: u8) -> Option<OpCode> {
        use OpCode::*;
        match byte {
            0 => Some(Return),
            1 => Some(Constant),
            2 => Some(Add),
            3 => Some(Subtract),
            4 => Some(Multiply),
            5 => Some(Divide),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct ParseRule<'a> {
    prefix: Option<fn(&mut Parser<'a>)>,
    infix: Option<fn(&mut Parser<'a>)>,
    precedence: Precedence,
}

impl<'a> ParseRule<'a> {
    fn new(
        prefix: Option<fn(&mut Parser<'a>)>,
        infix: Option<fn(&mut Parser<'a>)>,
        precedence: Precedence,
    ) -> ParseRule<'a> {
        ParseRule {
            prefix,
            infix,
            precedence,
        }
    }
}

pub struct Parser<'a> {
    current: Option<Token<'a>>,
    previous: Option<Token<'a>>,
    source: &'a str,
    scanner: Scanner<'a>,
    rules: HashMap<TokenType, ParseRule<'a>>,
    pub chunk: Chunk,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Parser<'a> {
        let chunk = Chunk::new();
        let mut parser = Parser {
            current: Some(Token::new(TokenType::Eof, "", 0)),
            previous: Some(Token::new(TokenType::Eof, "", 0)),
            scanner: Scanner::new(source),
            source,
            rules: HashMap::new(),
            chunk,
        };
        parser.init_rules();
        parser
    }

    fn init_rules(&mut self) {
        self.rules.insert(
            TokenType::Minus,
            ParseRule::new(Some(Parser::unary), Some(Parser::binary), Precedence::Term),
        );
        self.rules.insert(
            TokenType::Plus,
            ParseRule::new(None, Some(Parser::binary), Precedence::Term),
        );
        self.rules.insert(
            TokenType::Star,
            ParseRule::new(None, Some(Parser::binary), Precedence::Factor),
        );
        self.rules.insert(
            TokenType::Number,
            ParseRule::new(Some(Parser::number), None, Precedence::None),
        );
    }

    pub fn unary(&mut self) {
        println!("Unary expression parsed");
    }

    pub fn binary(&mut self) {
        let operator_type = self.previous.as_ref().unwrap().token_type;
        let rule = self.get_rule(operator_type).unwrap();
        self.parse_precedence(rule.precedence.next());
        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::Subtract as u8),
            TokenType::Plus => self.emit_byte(OpCode::Add as u8),
            TokenType::Slash => self.emit_byte(OpCode::Divide as u8),
            TokenType::Star => self.emit_byte(OpCode::Multiply as u8),
            _ => panic!("Unexpected operator"),
        }
    }

    pub fn number(&mut self) {
        let value = self.previous.as_ref().unwrap().lexeme.parse().unwrap();
        self.emit_constant(Value::new(ValueType::Number, value));
    }

    pub fn grouping(&mut self) {}

    pub fn literal(&mut self) {}

    pub fn get_rule(&self, key: TokenType) -> Option<&ParseRule<'a>> {
        self.rules.get(&key)
    }

    pub fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let prefix_rule = self
            .get_rule(self.previous.as_ref().unwrap().token_type)
            .and_then(|rule: &ParseRule<'a>| rule.prefix);
        if let Some(prefix_fn) = prefix_rule {
            prefix_fn(self);
        } else {
            self.error_at("Expect expression.");
            return;
        }

        while precedence
            <= self
                .get_rule(self.current.as_ref().unwrap().token_type)
                .map_or(Precedence::None, |rule: &ParseRule<'a>| rule.precedence)
        {
            self.advance();
            let infix_fn = self
                .get_rule(self.previous.as_ref().unwrap().token_type)
                .and_then(|rule| rule.infix);
            if let Some(infix_fn) = infix_fn {
                infix_fn(self);
            }
        }
    }

    fn error_at(&mut self, message: &str) {
        if let Some(current) = &self.current {
            println!("Error at {}: {}", current.line, message);
        }
    }

    pub fn advance(&mut self) {
        self.previous = self.current;
        loop {
            self.current = self.scanner.scan_token();
            if let Some(current) = &self.current {
                if current.token_type != TokenType::Error {
                    break;
                }
            }
        }
    }

    // Emits
    pub fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    pub fn emit_constant(&mut self, value: Value) {
        let index = self.chunk.add_constant(value);
        self.emit_bytes(OpCode::Constant as u8, index);
    }

    pub fn emit_byte(&mut self, byte: u8) {
        self.chunk.write(byte, self.previous.as_ref().unwrap().line);
    }

    pub fn compile(&mut self) {
        self.advance();
        self.expression();
        self.emit_byte(OpCode::Return as u8);
    }

    pub fn get_chunk(self) -> Chunk {
        self.chunk
    }
}
