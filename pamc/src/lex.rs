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

#[derive(Clone, Debug)]
pub enum LexError {
    UnexpectedEoi,
    UnexpectedAsciiDigit,
    UnexpectedCharacter(char),
}

#[derive(Clone, Debug)]
struct LexState {
    tokens: Vec<Token>,
    pending_token: Option<Token>,
}

fn handle_char(state: &mut LexState, c: char, i: usize) -> Option<LexError> {
    match &mut state.pending_token {
        None => {
            if c.is_whitespace() {
                state.tokens.push(Token {
                    start_index: i,
                    content: c.into(),
                    kind: TokenKind::Whitespace,
                });
                None
            } else if c == '(' {
                state.tokens.push(Token {
                    start_index: i,
                    content: c.into(),
                    kind: TokenKind::LParen,
                });
                None
            } else if c == ')' {
                state.tokens.push(Token {
                    start_index: i,
                    content: c.into(),
                    kind: TokenKind::RParen,
                });
                None
            } else if c == '[' {
                state.tokens.push(Token {
                    start_index: i,
                    content: c.into(),
                    kind: TokenKind::LSquare,
                });
                None
            } else if c == ']' {
                state.tokens.push(Token {
                    start_index: i,
                    content: c.into(),
                    kind: TokenKind::RSquare,
                });
                None
            } else if c == '{' {
                state.tokens.push(Token {
                    start_index: i,
                    content: c.into(),
                    kind: TokenKind::LCurly,
                });
                None
            } else if c == '}' {
                state.tokens.push(Token {
                    start_index: i,
                    content: c.into(),
                    kind: TokenKind::RCurly,
                });
                None
            } else if c == '@' {
                state.tokens.push(Token {
                    start_index: i,
                    content: c.into(),
                    kind: TokenKind::At,
                });
                None
            } else if c == ':' {
                state.tokens.push(Token {
                    start_index: i,
                    content: c.into(),
                    kind: TokenKind::Colon,
                });
                None
            } else if c == ',' {
                state.tokens.push(Token {
                    start_index: i,
                    content: c.into(),
                    kind: TokenKind::Comma,
                });
                None
            } else if c == '.' {
                state.tokens.push(Token {
                    start_index: i,
                    content: c.into(),
                    kind: TokenKind::Dot,
                });
                None
            } else if c == '=' {
                state.pending_token = Some(Token {
                    start_index: i,
                    content: c.into(),
                    kind: TokenKind::Equal,
                });
                None
            } else if c.is_ascii_digit() {
                Some(LexError::UnexpectedAsciiDigit)
            } else if does_character_category_permit_it_to_be_used_in_identifier_name(c) {
                state.pending_token = Some(Token {
                    start_index: i,
                    content: c.into(),
                    kind: TokenKind::Identifier,
                });
                None
            } else {
                Some(LexError::UnexpectedCharacter(c))
            }
        }
        Some(pending_token) => match pending_token.kind {
            TokenKind::Whitespace
            | TokenKind::LParen
            | TokenKind::RParen
            | TokenKind::LSquare
            | TokenKind::RSquare
            | TokenKind::LCurly
            | TokenKind::RCurly
            | TokenKind::LAngle
            | TokenKind::RAngle
            | TokenKind::Colon
            | TokenKind::Comma
            | TokenKind::Dot
            | TokenKind::At
            | TokenKind::Arrow
            | TokenKind::TypeLowerCase
            | TokenKind::TypeTitleCase
            | TokenKind::Let
            | TokenKind::Fun
            | TokenKind::Match
            | TokenKind::Forall
            | TokenKind::Exists
            | TokenKind::Underscore => unreachable!(),

            TokenKind::Equal => {
                if c == '>' {
                    state.tokens.push(Token {
                        start_index: pending_token.start_index,
                        content: "=>".into(),
                        kind: TokenKind::Arrow,
                    });
                    state.pending_token = None;
                    None
                } else {
                    state.tokens.push(pending_token.clone());
                    state.pending_token = None;
                    handle_char(state, c, i)
                }
            }

            TokenKind::Identifier => {
                if is_valid_non_initial_identifier_character(c) {
                    pending_token.content.push(c);
                    None
                } else {
                    state.tokens.push(if pending_token.content == "type" {
                        Token {
                            start_index: pending_token.start_index,
                            content: pending_token.content.clone(),
                            kind: TokenKind::TypeLowerCase,
                        }
                    } else if pending_token.content == "Type" {
                        Token {
                            start_index: pending_token.start_index,
                            content: pending_token.content.clone(),
                            kind: TokenKind::TypeTitleCase,
                        }
                    } else if pending_token.content == "let" {
                        Token {
                            start_index: pending_token.start_index,
                            content: pending_token.content.clone(),
                            kind: TokenKind::Let,
                        }
                    } else if pending_token.content == "fun" {
                        Token {
                            start_index: pending_token.start_index,
                            content: pending_token.content.clone(),
                            kind: TokenKind::Fun,
                        }
                    } else if pending_token.content == "match" {
                        Token {
                            start_index: pending_token.start_index,
                            content: pending_token.content.clone(),
                            kind: TokenKind::Match,
                        }
                    } else if pending_token.content == "forall" || pending_token.content == "∀" {
                        Token {
                            start_index: pending_token.start_index,
                            content: pending_token.content.clone(),
                            kind: TokenKind::Forall,
                        }
                    } else if pending_token.content == "exists" || pending_token.content == "∃" {
                        Token {
                            start_index: pending_token.start_index,
                            content: pending_token.content.clone(),
                            kind: TokenKind::Exists,
                        }
                    } else if pending_token.content == "_" {
                        Token {
                            start_index: pending_token.start_index,
                            content: pending_token.content.clone(),
                            kind: TokenKind::Underscore,
                        }
                    } else {
                        pending_token.clone()
                    });
                    state.pending_token = None;
                    handle_char(state, c, i)
                }
            }
        },
    }
}

fn is_valid_non_initial_identifier_character(c: char) -> bool {
    c != '('
        && c != ')'
        && c != '['
        && c != ']'
        && c != '{'
        && c != '}'
        && c != '<'
        && c != '>'
        && c != '='
        && c != '@'
        && c != ':'
        && c != ','
        && c != '.'
        && does_character_category_permit_it_to_be_used_in_identifier_name(c)
}

fn does_character_category_permit_it_to_be_used_in_identifier_name(c: char) -> bool {
    use unicode_general_category::{get_general_category, GeneralCategory};
    matches!(
        get_general_category(c),
        GeneralCategory::ClosePunctuation
            | GeneralCategory::ConnectorPunctuation
            | GeneralCategory::CurrencySymbol
            | GeneralCategory::DashPunctuation
            | GeneralCategory::DecimalNumber
            | GeneralCategory::FinalPunctuation
            | GeneralCategory::InitialPunctuation
            | GeneralCategory::LetterNumber
            | GeneralCategory::LowercaseLetter
            | GeneralCategory::MathSymbol
            | GeneralCategory::ModifierLetter
            | GeneralCategory::ModifierSymbol
            | GeneralCategory::OpenPunctuation
            | GeneralCategory::OtherLetter
            | GeneralCategory::OtherNumber
            | GeneralCategory::OtherPunctuation
            | GeneralCategory::OtherSymbol
            | GeneralCategory::TitlecaseLetter
            | GeneralCategory::UppercaseLetter
    )
}

pub fn lex(src: &str) -> Result<Vec<Token>, LexError> {
    let mut state = LexState {
        tokens: vec![],
        pending_token: None,
    };
    for (i, c) in src.chars().enumerate() {
        if let Some(err) = handle_char(&mut state, c, i) {
            return Err(err);
        }
    }

    if let Some(pending_token) = state.pending_token {
        match pending_token.kind {
            TokenKind::Whitespace
            | TokenKind::LParen
            | TokenKind::RParen
            | TokenKind::LSquare
            | TokenKind::RSquare
            | TokenKind::LCurly
            | TokenKind::RCurly
            | TokenKind::LAngle
            | TokenKind::RAngle
            | TokenKind::Colon
            | TokenKind::Comma
            | TokenKind::Dot
            | TokenKind::At
            | TokenKind::Arrow
            | TokenKind::TypeLowerCase
            | TokenKind::TypeTitleCase
            | TokenKind::Let
            | TokenKind::Fun
            | TokenKind::Match
            | TokenKind::Forall
            | TokenKind::Exists
            | TokenKind::Underscore => unreachable!(),

            TokenKind::Equal => {
                state.tokens.push(pending_token);
                state.pending_token = None;
            }

            TokenKind::Identifier => {
                state.tokens.push(pending_token);
                state.pending_token = None;
            }
        }
    }

    Ok(state.tokens)
}
