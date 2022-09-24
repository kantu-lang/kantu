#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub start_index: usize,
    pub content: String,
    pub kind: TokenKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Whitespace,
    StandardIdentifier,
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
    Dash,
    FatArrow,

    TypeLowerCase,
    TypeTitleCase,
    Let,
    Fun,
    Match,
    Forall,
    Exists,
    Underscore,
}
