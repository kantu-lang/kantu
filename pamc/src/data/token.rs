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

    Semicolon,
    Colon,
    Comma,
    Dot,
    At,
    Equal,
    Dash,
    Question,
    Tilde,
    LParen,
    RParen,
    LSquare,
    RSquare,
    LCurly,
    RCurly,
    LAngle,
    RAngle,

    FatArrow,

    Underscore,

    TypeLowerCase,
    Let,
    TypeTitleCase,
    Type0,
    Type1,
    Type2,
    Type3,
    Fun,
    Match,
    Forall,
    Check,
    Goal,
    Impossible,

    // Currently unused but reserved for future use.
    Struct,
    Var,
    Trait,

    Pub,
    Prot,
    Priv,
    Mod,
    Pack,
    Use,
    Namespace,

    Extern,
    Unsafe,
    Async,

    Notation,
    Exists,

    Universal,
    Existential,
}
