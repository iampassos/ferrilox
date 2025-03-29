#[derive(Debug)]
pub enum TokenType {
    // SINGLE-CHARACTER TOKENS
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // ONE OR TWO CHARACTER TOKENS
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // LITERALS
    Identifier,
    String(String),
    Number(f64),

    // KEYWORDS
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

#[derive(Debug)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            ttype,
            lexeme,
            line,
        }
    }
}
