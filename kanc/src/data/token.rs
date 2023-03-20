use crate::data::text_span::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Token {
    pub start: TextPosition,
    pub content: String,
    pub kind: TokenKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
    Star,
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
    TypeTitleCase,
    Let,
    Def,
    Fun,
    Match,
    Forall,
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
    Super,
    Pack,
    Use,
    As,
    Namespace,

    Extern,
    Unsafe,
    Async,

    Notation,
    Exists,
}
