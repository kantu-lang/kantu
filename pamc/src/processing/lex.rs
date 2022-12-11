use crate::data::token::{Token, TokenKind};

use std::convert::TryFrom;

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
        handle_char(&mut state, c, i)?;
    }

    if let Some(pending_token) = state.pending_token {
        let Ok(token) = pending_token.try_into() else {
            return Err(LexError::UnexpectedEoi);
        };
        state.tokens.push(token);
        state.pending_token = None;
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
    Slash,
    SingleLineComment,
    MultiLineComment {
        left_delimiter_count: usize,
        last_char_changed_delimiter_count: bool,
    },
}

impl TryFrom<PendingToken> for Token {
    type Error = ();

    fn try_from(pending_token: PendingToken) -> Result<Self, ()> {
        let PendingToken {
            start_index,
            content,
            kind,
        } = pending_token;
        let kind = kind.try_into()?;
        Ok(Token {
            start_index,
            content,
            kind,
        })
    }
}

impl TryFrom<PendingTokenKind> for TokenKind {
    type Error = ();

    fn try_from(pending_token_kind: PendingTokenKind) -> Result<Self, Self::Error> {
        match pending_token_kind {
            PendingTokenKind::Equal => Ok(TokenKind::Equal),
            PendingTokenKind::StandardIdentifier => Ok(TokenKind::StandardIdentifier),
            PendingTokenKind::Slash => Ok(TokenKind::Slash),
            PendingTokenKind::SingleLineComment => Ok(TokenKind::SingleLineComment),
            PendingTokenKind::MultiLineComment {
                left_delimiter_count,
                ..
            } => {
                if left_delimiter_count == 0 {
                    Ok(TokenKind::MultiLineComment)
                } else {
                    Err(())
                }
            }
        }
    }
}

fn handle_char(state: &mut LexState, c: char, i: usize) -> Result<(), LexError> {
    match &mut state.pending_token {
        None => {
            if c == '=' {
                state.pending_token = Some(PendingToken {
                    start_index: i,
                    content: c.into(),
                    kind: PendingTokenKind::Equal,
                });
                Ok(())
            } else if c == '/' {
                state.pending_token = Some(PendingToken {
                    start_index: i,
                    content: c.into(),
                    kind: PendingTokenKind::Slash,
                });
                Ok(())
            } else if let Some(kind) = get_token_kind_of_special_non_underscore_character(c) {
                state.tokens.push(Token {
                    start_index: i,
                    content: c.into(),
                    kind,
                });
                Ok(())
            } else if c.is_whitespace() {
                state.tokens.push(Token {
                    start_index: i,
                    content: c.into(),
                    kind: TokenKind::Whitespace,
                });
                Ok(())
            } else if c.is_ascii_digit() {
                Err(LexError::UnexpectedAsciiDigit)
            } else if does_character_category_permit_it_to_be_used_in_identifier_name(c) {
                state.pending_token = Some(PendingToken {
                    start_index: i,
                    content: c.into(),
                    kind: PendingTokenKind::StandardIdentifier,
                });
                Ok(())
            } else {
                Err(LexError::UnexpectedCharacter(c))
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
                    Ok(())
                } else {
                    let Ok(token) = pending_token.clone().try_into() else {
                        return Err(LexError::UnexpectedCharacter(c));
                    };
                    state.tokens.push(token);
                    state.pending_token = None;
                    handle_char(state, c, i)
                }
            }

            PendingTokenKind::StandardIdentifier => {
                if is_valid_non_initial_identifier_character(c) {
                    pending_token.content.push(c);
                    Ok(())
                } else {
                    state.tokens.push(
                        if let Some(kind) =
                            get_token_kind_of_non_underscore_keyword(&pending_token.content)
                        {
                            Token {
                                start_index: pending_token.start_index,
                                content: pending_token.content.clone(),
                                kind,
                            }
                        } else if pending_token.content == "_" {
                            Token {
                                start_index: pending_token.start_index,
                                content: pending_token.content.clone(),
                                kind: TokenKind::Underscore,
                            }
                        } else {
                            let Ok(token) = pending_token.clone().try_into() else {
                                return Err(LexError::UnexpectedCharacter(c));
                            };
                            token
                        },
                    );
                    state.pending_token = None;
                    handle_char(state, c, i)
                }
            }

            PendingTokenKind::Slash => {
                if c == '/' {
                    state.pending_token = Some(PendingToken {
                        start_index: pending_token.start_index,
                        content: "//".into(),
                        kind: PendingTokenKind::SingleLineComment,
                    });
                    Ok(())
                } else if c == '*' {
                    state.pending_token = Some(PendingToken {
                        start_index: pending_token.start_index,
                        content: "/*".into(),
                        kind: PendingTokenKind::MultiLineComment {
                            left_delimiter_count: 1,
                            last_char_changed_delimiter_count: true,
                        },
                    });
                    Ok(())
                } else {
                    let Ok(token) = pending_token.clone().try_into() else {
                        return Err(LexError::UnexpectedCharacter(c));
                    };
                    state.tokens.push(token);
                    state.pending_token = None;
                    handle_char(state, c, i)
                }
            }

            PendingTokenKind::SingleLineComment => {
                pending_token.content.push(c);
                if c == '\n' {
                    state.tokens.push(Token {
                        start_index: pending_token.start_index,
                        content: pending_token.content.clone(),
                        kind: TokenKind::SingleLineComment,
                    });
                    state.pending_token = None;
                }
                Ok(())
            }

            PendingTokenKind::MultiLineComment {
                left_delimiter_count,
                last_char_changed_delimiter_count,
            } => {
                pending_token.content.push(c);
                set_last_char_changed_delimiter_count_or_panic(pending_token, false);

                if pending_token.content.ends_with("*/") && !last_char_changed_delimiter_count {
                    let new_left_delimiter_count = left_delimiter_count - 1;
                    if new_left_delimiter_count == 0 {
                        state.tokens.push(Token {
                            start_index: pending_token.start_index,
                            content: pending_token.content.clone(),
                            kind: TokenKind::MultiLineComment,
                        });
                        state.pending_token = None;
                    } else {
                        state.pending_token = Some(PendingToken {
                            start_index: pending_token.start_index,
                            content: pending_token.content.clone(),
                            kind: PendingTokenKind::MultiLineComment {
                                left_delimiter_count: new_left_delimiter_count,
                                last_char_changed_delimiter_count: true,
                            },
                        });
                    }
                } else if pending_token.content.ends_with("/*")
                    && !last_char_changed_delimiter_count
                {
                    let new_left_delimiter_count = left_delimiter_count + 1;
                    state.pending_token = Some(PendingToken {
                        start_index: pending_token.start_index,
                        content: pending_token.content.clone(),
                        kind: PendingTokenKind::MultiLineComment {
                            left_delimiter_count: new_left_delimiter_count,
                            last_char_changed_delimiter_count: true,
                        },
                    });
                }
                Ok(())
            }
        },
    }
}

// TODO: Make left_delimiter_count: NonZeroUsize

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
        '/' => Some(TokenKind::Slash),
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

fn get_token_kind_of_non_underscore_keyword(s: &str) -> Option<TokenKind> {
    match s {
        "type" => Some(TokenKind::TypeLowerCase),
        "let" => Some(TokenKind::Let),
        "Type" => Some(TokenKind::TypeTitleCase),
        "Type0" => Some(TokenKind::Type0),
        "Type1" => Some(TokenKind::Type1),
        "Type2" => Some(TokenKind::Type2),
        "Type3" => Some(TokenKind::Type3),
        "fun" => Some(TokenKind::Fun),
        "match" => Some(TokenKind::Match),
        "forall" => Some(TokenKind::Forall),
        "check" => Some(TokenKind::Check),
        "goal" => Some(TokenKind::Goal),
        "impossible" => Some(TokenKind::Impossible),

        "struct" => Some(TokenKind::Struct),
        "var" => Some(TokenKind::Var),
        "trait" => Some(TokenKind::Trait),

        "pub" => Some(TokenKind::Pub),
        "prot" => Some(TokenKind::Prot),
        "priv" => Some(TokenKind::Priv),
        "mod" => Some(TokenKind::Mod),
        "pack" => Some(TokenKind::Pack),
        "use" => Some(TokenKind::Use),
        "namespace" => Some(TokenKind::Namespace),

        "extern" => Some(TokenKind::Extern),
        "unsafe" => Some(TokenKind::Unsafe),
        "async" => Some(TokenKind::Async),

        "notation" => Some(TokenKind::Notation),
        "exists" => Some(TokenKind::Exists),

        "∀" => Some(TokenKind::Universal),
        "∃" => Some(TokenKind::Existential),

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

fn set_last_char_changed_delimiter_count_or_panic(token: &mut PendingToken, value: bool) {
    match &mut token.kind {
        PendingTokenKind::MultiLineComment {
            left_delimiter_count: _,
            last_char_changed_delimiter_count,
        } => {
            *last_char_changed_delimiter_count = false;
        }
        other => panic!(
            "Tried to set last_char_changed_delimiter_count to {:?}, but the token was of kind {:?}",
            value, other
        ),
    }
}
