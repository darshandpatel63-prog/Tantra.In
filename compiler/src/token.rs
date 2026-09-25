use crate::diagnostic::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    Integer(String),
    Float(String),
    String(String),
    Character(char),

    True,
    False,
    Null,
    Const,
    Mut,
    Fn,
    Return,
    If,
    Else,
    Then,
    For,
    Each,
    In,
    While,
    Break,
    Continue,
    Import,
    From,
    Type,
    Try,
    Error,
    Throw,
    Async,
    Await,
    Capability,
    Public,
    Private,
    Module,

    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Power,
    Bang,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    ShiftLeft,
    ShiftRight,
    Ampersand,
    Caret,
    Pipe,
    AndAnd,
    OrOr,
    NullCoalesce,
    Question,
    Colon,
    Arrow,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Semicolon,

    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, start: usize, end: usize) -> Self {
        Self {
            kind,
            span: Span::new(start, end),
        }
    }
}
