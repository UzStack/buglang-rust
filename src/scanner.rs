#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub line: i32,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, lexeme: &'a str, line: i32) -> Token<'a> {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    Plus,
    Minus,
    Slash,
    Star,
    Eof,
    OpenParen,
    CloseParen,
    Number,
    String,
    Identifier,
    Error,
}

pub struct Scanner<'a> {
    source: &'a str,
    start: i32,
    current: i32,
    line: i32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn peek(&mut self) -> Option<char> {
        return self.source[self.current as usize..].chars().nth(0);
    }

    fn peek_next(&mut self) -> Option<char> {
        return self.source[(self.current + 1) as usize..].chars().nth(0);
    }

    fn make_token(&mut self, token_type: TokenType) -> Option<Token<'a>> {
        let lexeme = &self.source[self.start as usize..self.current as usize];
        return Some(Token::new(token_type, lexeme, self.line));
    }

    fn is_digit(&mut self, ch: char) -> bool {
        return ch >= '0' && ch <= '9';
    }

    fn adnance(&mut self) -> Option<char> {
        self.current += 1;
        return self.source[(self.current - 1) as usize..].chars().nth(0);
    }
    fn is_end(&mut self) -> bool {
        if let Some(x) = self.peek() {
            return x == '\0';
        }
        return false;
    }

    fn is_alpha(&mut self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    fn number(&mut self) -> Option<Token<'a>> {
        while let Some(x) = self.peek() {
            if self.is_digit(x) {
                self.adnance();
            } else {
                break;
            }
        }
        if let (Some('.'), Some(x)) = (self.peek(), self.peek_next()) {
            if self.is_digit(x) {
                self.adnance();
                self.number();
            }
        }
        return self.make_token(TokenType::Number);
    }

    fn string(&mut self) -> Option<Token<'a>> {
        while let Some(x) = self.peek() {
            if x == '"' || self.is_end() {
                break;
            } else {
                self.adnance();
            }
        }
        self.adnance();
        return self.make_token(TokenType::String);
    }

    fn identifier(&mut self) -> Option<Token<'a>> {
        while let Some(x) = self.peek() {
            if self.is_alpha(x) {
                self.adnance();
            } else {
                break;
            }
        }
        return self.make_token(TokenType::Identifier);
    }

    fn skip_whitespace(&mut self) {
        while let Some(x) = self.peek() {
            if x == '\n' {
                self.line += 1;
                self.current += 1;
            } else if x == ' ' || x == '\r' || x == '\t' {
                self.current += 1;
            } else {
                break;
            }
        }
    }

    pub fn scan_token(&mut self) -> Option<Token<'a>> {
        self.skip_whitespace();
        self.start = self.current;
        if let Some(ch) = self.adnance() {
            if self.is_digit(ch) {
                return self.number();
            };
            if self.is_alpha(ch) {
                return self.identifier();
            }
            if ch == '\0' {
                return self.make_token(TokenType::Eof);
            }
            let token = match ch {
                '+' => self.make_token(TokenType::Plus),
                '"' => self.string(),
                '-' => self.make_token(TokenType::Minus),
                '/' => self.make_token(TokenType::Slash),
                '*' => self.make_token(TokenType::Star),
                '(' => self.make_token(TokenType::OpenParen),
                ')' => self.make_token(TokenType::CloseParen),
                _ => self.make_token(TokenType::Number),
            };
            return token;
        }
        return self.make_token(TokenType::Eof);
    }
}
