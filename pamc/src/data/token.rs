#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub start_index: usize,
    pub content: String,
    pub kind: TokenKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Whitespace,
    Identifier,
    LParen,
    RParen,
    LSquare,
    RSquare,
    LCurly,
    RCurly,
    LAngle,
    RAngle,
    Semicolon,
    Colon,
    Comma,
    Dot,
    At,
    Equal,
    Arrow,

    TypeLowerCase,
    TypeTitleCase,
    Let,
    Fun,
    Match,
    Forall,
    Exists,
    Underscore,
}
