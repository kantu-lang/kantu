#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub start_index: usize,
    pub content: String,
    pub kind: TokenKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Eoi,

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
    Slash,
    LParen,
    RParen,
    LSquare,
    RSquare,
    LCurly,
    RCurly,
    LAngle,
    RAngle,

    SingleLineComment,
    MultiLineComment,

    FatArrow,
    TripleDot,

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
    Todo,

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
