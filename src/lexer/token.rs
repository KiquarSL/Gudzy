pub type TKind = TokenKind;

#[derive(Debug, Clone)]
pub enum TokenKind {
    Plus,  // +
    Minus, // -
    Slash, // /
    Star,  // *

    LBrace, // {
    RBrace, // }
    LParen, // (
    RParen, // )

    Comma,  // ,
    Assign, // =
    Bang,   // !

    Gt, // >
    Lt, // <
    Ge, // >=
    Le, // <=
    Eq, // ==
    Ne, // !=

    And, // &&
    Or,  // ||

    Id(String),
    StrLit(String),
    NumLit(f64),
    Bool(bool),

    // Keywords
    Fn,
    If,
    Elif,
    Else,
    While,
    Break,
    Continue,
    Return,

    // Functions
    Print,
    Input,
    Num,
    Str,

    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TKind,

    pub len: usize,
    pub pos: usize,
    pub line: usize,
    pub offset: usize,
}
