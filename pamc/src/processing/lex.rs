use crate::data::token::{Token, TokenKind};

#[derive(Clone, Debug)]
pub enum LexError {
    UnexpectedEoi,
    UnexpectedAsciiDigit,
    UnexpectedCharacter(char),
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
            PendingTokenKind::Equal => {
                state.tokens.push(pending_token.into());
                state.pending_token = None;
            }

            PendingTokenKind::StandardIdentifier => {
                state.tokens.push(pending_token.into());
                state.pending_token = None;
            }
        }
    }

    Ok(state.tokens)
}

#[derive(Clone, Debug)]
struct LexState {
    tokens: Vec<Token>,
    pending_token: Option<PendingToken>,
}

/// Pending tokens can only have a limited
/// subset of the possible token kinds.
/// Thus, we create a separate `PendingToken` struct
/// (and accompanying `PendingTokenKind` enum)
/// to represent this constraint.
#[derive(Clone, Debug)]
struct PendingToken {
    pub start_index: usize,
    pub content: String,
    pub kind: PendingTokenKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PendingTokenKind {
    Equal,
    StandardIdentifier,
}

impl From<PendingToken> for Token {
    fn from(pending_token: PendingToken) -> Self {
        let PendingToken {
            start_index,
            content,
            kind,
        } = pending_token;
        let kind = kind.into();
        Token {
            start_index,
            content,
            kind,
        }
    }
}

impl From<PendingTokenKind> for TokenKind {
    fn from(pending_token_kind: PendingTokenKind) -> Self {
        match pending_token_kind {
            PendingTokenKind::Equal => TokenKind::Equal,
            PendingTokenKind::StandardIdentifier => TokenKind::StandardIdentifier,
        }
    }
}

fn handle_char(state: &mut LexState, c: char, i: usize) -> Option<LexError> {
    match &mut state.pending_token {
        None => {
            if c == '=' {
                state.pending_token = Some(PendingToken {
                    start_index: i,
                    content: c.into(),
                    kind: PendingTokenKind::Equal,
                });
                None
            } else if let Some(kind) = get_token_kind_of_special_non_underscore_character(c) {
                state.tokens.push(Token {
                    start_index: i,
                    content: c.into(),
                    kind,
                });
                None
            } else if c.is_whitespace() {
                state.tokens.push(Token {
                    start_index: i,
                    content: c.into(),
                    kind: TokenKind::Whitespace,
                });
                None
            } else if c.is_ascii_digit() {
                Some(LexError::UnexpectedAsciiDigit)
            } else if does_character_category_permit_it_to_be_used_in_identifier_name(c) {
                state.pending_token = Some(PendingToken {
                    start_index: i,
                    content: c.into(),
                    kind: PendingTokenKind::StandardIdentifier,
                });
                None
            } else {
                Some(LexError::UnexpectedCharacter(c))
            }
        }
        Some(pending_token) => match pending_token.kind {
            PendingTokenKind::Equal => {
                if c == '>' {
                    state.tokens.push(Token {
                        start_index: pending_token.start_index,
                        content: "=>".into(),
                        kind: TokenKind::FatArrow,
                    });
                    state.pending_token = None;
                    None
                } else {
                    state.tokens.push(pending_token.clone().into());
                    state.pending_token = None;
                    handle_char(state, c, i)
                }
            }

            PendingTokenKind::StandardIdentifier => {
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
                    } else if pending_token.content == "check" {
                        Token {
                            start_index: pending_token.start_index,
                            content: pending_token.content.clone(),
                            kind: TokenKind::Check,
                        }
                    } else if pending_token.content == "goal" {
                        Token {
                            start_index: pending_token.start_index,
                            content: pending_token.content.clone(),
                            kind: TokenKind::Goal,
                        }
                    } else if pending_token.content == "_" {
                        Token {
                            start_index: pending_token.start_index,
                            content: pending_token.content.clone(),
                            kind: TokenKind::Underscore,
                        }
                    } else {
                        pending_token.clone().into()
                    });
                    state.pending_token = None;
                    handle_char(state, c, i)
                }
            }
        },
    }
}

fn is_valid_non_initial_identifier_character(c: char) -> bool {
    !c.is_whitespace()
        && get_token_kind_of_special_non_underscore_character(c).is_none()
        && does_character_category_permit_it_to_be_used_in_identifier_name(c)
}

/// If this character is a special character that is not an underscore, returns `Some`.
/// Otherwise, returns `None`.
fn get_token_kind_of_special_non_underscore_character(c: char) -> Option<TokenKind> {
    match c {
        ';' => Some(TokenKind::Semicolon),
        ':' => Some(TokenKind::Colon),
        ',' => Some(TokenKind::Comma),
        '.' => Some(TokenKind::Dot),
        '@' => Some(TokenKind::At),
        '=' => Some(TokenKind::Equal),
        '-' => Some(TokenKind::Dash),
        '?' => Some(TokenKind::Question),
        '~' => Some(TokenKind::Tilde),
        '(' => Some(TokenKind::LParen),
        ')' => Some(TokenKind::RParen),
        '[' => Some(TokenKind::LSquare),
        ']' => Some(TokenKind::RSquare),
        '{' => Some(TokenKind::LCurly),
        '}' => Some(TokenKind::RCurly),
        '<' => Some(TokenKind::LAngle),
        '>' => Some(TokenKind::RAngle),
        _ => None,
    }
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
