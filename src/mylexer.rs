#![allow(unused)]
use core::fmt;

macro_rules! KW {
    () => {
        "proc" | "in" | "end"
    };
}

pub struct Lexer {
    loc: Loc,
    data: Vec<u8>,
    pos: usize,
    max_pos: usize,
}

impl Lexer {
    pub fn new(file_path: String, data: String) -> Lexer {
        let data = data.into_bytes();
        let max_pos = data.len();
        Lexer {
            data,
            pos: 0,
            max_pos,
            loc: Loc::new(file_path, 1, 1),
        }
    }

    fn advance(&mut self) {
        if self.pos < self.max_pos {
            self.pos += 1;
            self.loc.next(self.curr_char());
        }
    }

    fn curr_char(&self) -> u8 {
        if self.pos < self.max_pos {
            self.data[self.pos]
        } else {
            0
        }
    }

    fn peek_char(&self, offset: usize) -> u8 {
        if self.pos + offset < self.max_pos {
            self.data[self.pos + offset]
        } else {
            0
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        self.skip_comment();
        match self.curr_char() {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.identfier(),
            b'0'..=b'9' => self.integer(),
            b'+' => self.simple_token_advance(TokenKind::Operator, "+"),
            b'-' => {
                if self.peek_char(1) == b'-' {
                    self.pos += 2;
                }
                self.simple_token_advance(TokenKind::Punct, "--")
            }
            c => {
                let loc = self.loc.clone();
                if self.pos >= self.max_pos {
                    return Token::eof(loc);
                }
                let mut value = String::from(c as char);
                let mut kind = TokenKind::Invalid;
                self.token_advance(kind, value, loc)
            }
        }
    }

    fn token_advance(&mut self, kind: TokenKind, value: String, loc: Loc) -> Token {
        self.advance();
        Token { kind, value, loc }
    }

    fn simple_token_advance(&mut self, kind: TokenKind, value: &str) -> Token {
        let loc = self.loc.clone();
        let value = String::from(value);
        self.advance();
        Token { kind, value, loc }
    }

    fn skip_comment(&mut self) {
        match self.curr_char() {
            b'/' => {
                if self.peek_char(1) == b'/' {
                    self.pos += 2;
                }
                loop {
                    if matches!(self.curr_char(), b'\n' | b'\0') {
                        break;
                    }
                    self.advance();
                }
                self.skip_whitespace();
            }
            _ => {}
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.curr_char();
            if !c.is_ascii_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn identfier(&mut self) -> Token {
        let loc = self.loc.clone();
        let mut value = String::new();
        loop {
            let c = self.curr_char();
            self.loc.next(c);
            match c {
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => value.push(c as char),
                _ => break,
            }
            self.advance();
        }
        let kind = match value.as_str() {
            KW!() => TokenKind::Keyword,
            _ => TokenKind::Identfier,
        };
        Token { kind, value, loc }
    }

    fn integer(&mut self) -> Token {
        let loc = self.loc.clone();
        let mut value = String::new();
        loop {
            let c = self.curr_char();
            self.loc.next(c);
            match c {
                b'0'..=b'9' => value.push(c as char),
                _ => break,
            }
            self.advance();
        }
        let kind = TokenKind::Integer;
        Token { kind, value, loc }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Invalid,
    EOF,
    Integer,
    Identfier,
    Keyword,
    Operator,
    Punct,
    // TODO: Add any type of quotes for strings
    DQString,
    SQString,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub loc: Loc,
}

impl Token {
    pub fn eof(loc: Loc) -> Self {
        Token {
            kind: TokenKind::EOF,
            value: "\0".to_string(),
            loc,
        }
    }
    pub fn val(&self) -> &str {
        self.value.as_str()
    }

}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}({})", self.kind, self.value)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Default)]
pub struct Loc {
    file_path: String,
    line: usize,
    col: usize,
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file_path, self.line, self.col)
    }
}

impl Loc {
    pub fn new(file_path: String, line: usize, col: usize) -> Self {
        Self {
            file_path,
            line,
            col,
        }
    }
    pub fn next_column(&mut self) {
        self.col += 1;
    }
    pub fn next_line(&mut self) {
        self.line += 1;
        self.col = 1;
    }
    pub fn next(&mut self, c: u8) {
        match c {
            b'\n' => self.next_line(),
            b'\t' => {
                let ts = 8;
                self.line = self.line;
                self.col = (self.col / ts) * ts + ts;
            }
            c if (c as char).is_control() => {}
            _ => self.next_column(),
        }
    }
}
