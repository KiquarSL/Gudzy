use super::token::{TKind, Token};

pub struct Lexer<'a> {
    pos: usize,
    chars: Vec<char>,
    lines: Vec<&'a str>,
    tokens: Vec<Token>,

    line: usize,
    offset: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let lines = source.lines().collect();
        let chars = source.chars().collect();
        Self {
            lines,
            chars,
            pos: 0,
            line: 0,
            offset: 0,
            tokens: vec![],
        }
    }

    fn error(&self, msg: &str, line_num: usize, offset: usize, len: usize) -> String {
        let line = self.lines[line_num];
        let header = format!("Error in {line_num}:{offset} - {msg}");
        let err_line = format!("{line_num} | {line}");
        let point = format!(
            "{} | {}{}",
            " ".repeat(line_num.to_string().len()),
            " ".repeat(offset),
            "^".repeat(len),
        );
        format!("{header}\n{err_line}\n{point}\n")
    }

    fn peek(&self, offset: i8) -> char {
        let idx = self.pos + offset as usize;
        let c = self.chars.get(idx);
        match c {
            Some(ch) => *ch,
            None => panic!(
                "{}",
                self.error("Out of bounds index", self.line, self.offset, 1)
            ),
        }
    }

    fn advance(&mut self, offset: u8) {
        self.offset += offset as usize;
        self.pos += offset as usize;
    }

    fn push(&mut self, kind: TKind, line: usize, offset: usize, len: usize, pos: usize) {
        self.tokens.push(Token {
            kind,
            line,
            offset,
            len,
            pos,
        });
    }
}

impl Lexer<'_> {
    pub fn tokenize(&mut self) -> Vec<Token> {
        let chars_len = self.chars.len();
        while self.pos < chars_len {
            let current = self.peek(0);
            let line = self.line;
            let offset = self.offset;
            let pos = self.pos;
            match current {
                c if c.is_whitespace() => {
                    if c == '\n' {
                        self.line += 1;
                        self.advance(1);
                        self.offset = 0;
                    } else {
                        self.advance(1);
                    }
                }
                '+' => {
                    self.push(TKind::Plus, line, offset, 1, pos);
                    self.advance(1);
                }
                '-' => {
                    self.push(TKind::Minus, line, offset, 1, pos);
                    self.advance(1);
                }
                '*' => {
                    self.push(TKind::Star, line, offset, 1, pos);
                    self.advance(1);
                }
                '/' => {
                    self.push(TKind::Slash, line, offset, 1, pos);
                    self.advance(1);
                }
                '(' => {
                    self.push(TKind::LParen, line, offset, 1, pos);
                    self.advance(1);
                }
                ')' => {
                    self.push(TKind::RParen, line, offset, 1, pos);
                    self.advance(1);
                }
                '{' => {
                    self.push(TKind::LBrace, line, offset, 1, pos);
                    self.advance(1);
                }
                '}' => {
                    self.push(TKind::RBrace, line, offset, 1, pos);
                    self.advance(1);
                }
                ',' => {
                    self.push(TKind::Comma, line, offset, 1, pos);
                    self.advance(1);
                }
                '=' => {
                    if self.pos + 1 < chars_len && self.peek(1) == '=' {
                        self.push(TKind::Eq, line, offset, 2, pos);
                        self.advance(2);
                    } else {
                        self.push(TKind::Assign, line, offset, 1, pos);
                        self.advance(1);
                    }
                }
                '!' => {
                    if self.pos + 1 < chars_len && self.peek(1) == '=' {
                        self.push(TKind::Ne, line, offset, 2, pos);
                        self.advance(2);
                    } else {
                        self.push(TKind::Bang, line, offset, 1, pos);
                        self.advance(1);
                    }
                }
                '>' => {
                    if self.pos + 1 < chars_len && self.peek(1) == '=' {
                        self.push(TKind::Ge, line, offset, 2, pos);
                        self.advance(2);
                    } else {
                        self.push(TKind::Gt, line, offset, 1, pos);
                        self.advance(1);
                    }
                }
                '<' => {
                    if self.pos + 1 < chars_len && self.peek(1) == '=' {
                        self.push(TKind::Le, line, offset, 2, pos);
                        self.advance(2);
                    } else {
                        self.push(TKind::Lt, line, offset, 1, pos);
                        self.advance(1);
                    }
                }
                '&' => {
                    if self.pos + 1 < chars_len && self.peek(1) == '&' {
                        self.push(TKind::And, line, offset, 2, pos);
                        self.advance(2);
                    } else {
                        panic!("{}", self.error("Expected &&", line, offset, 1));
                    }
                }
                '|' => {
                    if self.pos + 1 < chars_len && self.peek(1) == '|' {
                        self.push(TKind::Or, line, offset, 2, pos);
                        self.advance(2);
                    } else {
                        panic!("{}", self.error("Expected ||", line, offset, 1));
                    }
                }
                c if c == '"' => self.tokenize_string(),
                c if c.is_alphabetic() => self.tokenize_ident(),
                c if c.is_digit(10) => self.tokenize_number(),
                _ => panic!("{}", self.error("Unknown char", line, offset, 1)),
            }
        }
        self.push(TKind::Eof, self.line, self.offset, 1, self.pos);
        self.tokens.clone()
    }

    fn tokenize_string(&mut self) {
        let mut buffer = String::new();
        let line = self.line;
        let offset = self.offset;
        let pos = self.pos;
        self.advance(1);

        loop {
            let current = self.peek(0);
            self.advance(1);
            if current == '"' {
                break;
            } else {
                buffer.push(current);
            }
        }
        self.push(
            TKind::StrLit(buffer.clone()),
            line,
            offset,
            buffer.len() + 2,
            pos,
        );
    }

    fn tokenize_ident(&mut self) {
        let mut buffer = String::new();
        let line = self.line;
        let offset = self.offset;
        let pos = self.pos;

        loop {
            if self.pos >= self.chars.len() {
                break;
            }
            let current = self.peek(0);
            if current.is_alphabetic() || current.is_digit(10) || current == '_' {
                buffer.push(current);
                self.advance(1);
            } else {
                break;
            }
        }
        let kind = match buffer.as_str() {
            "true" => TKind::Bool(true),
            "false" => TKind::Bool(false),

            "print" => TKind::Print,
            "input" => TKind::Print,
            "str" => TKind::Str,
            "num" => TKind::Num,

            "fn" => TKind::Fn,
            "while" => TKind::While,
            "break" => TKind::Break,
            "continue" => TKind::Continue,
            "if" => TKind::If,
            "elif" => TKind::Elif,
            "else" => TKind::Else,
            "return" => TKind::Return,

            _ => TKind::Id(buffer.clone()),
        };
        self.push(kind, line, offset, buffer.len() + 2, pos);
    }

    fn tokenize_number(&mut self) {
        let mut buffer = String::new();
        let line = self.line;
        let offset = self.offset;
        let pos = self.pos;

        loop {
            if self.pos >= self.chars.len() {
                break;
            }
            let current = self.peek(0);
            if current.is_digit(10) || current == '.' {
                buffer.push(current);
                self.advance(1);
            } else if current == '_' {
                self.advance(1);
            } else {
                break;
            }
        }
        self.push(
            TKind::NumLit(buffer.parse::<f64>().expect("Failed parse number")),
            line,
            offset,
            buffer.len(),
            pos,
        );
    }
}
