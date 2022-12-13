use crate::data::{
    token::{Token, TokenKind},
    unsimplified_ast::*,
    FileId, TextSpan,
};

// TODO: Check first token location logic.
// We don't want to give incorrect error messages!
// (E.g., "Error at index 234" but it's actually at index 864.)

// TODO: Make errors more informative.
// For example, if possible, it would be possible
// to include "expected token <x>" information.
// Or even encode hints into the error type
// (e.g., "Nullary functions are not permitted.").

#[derive(Clone, Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEndOfInput,
}

pub trait Parse: Sized {
    fn from_empty_str(file_id: FileId) -> Result<Self, ParseError>;

    fn initial_stack(file_id: FileId, first_token: Token) -> Vec<UnfinishedStackItem>;

    fn before_handle_token(
        _file_id: FileId,
        _token: &Token,
        _stack: &[UnfinishedStackItem],
    ) -> Result<(), ParseError> {
        Ok(())
    }

    fn finish(
        file_id: FileId,
        top_item: UnfinishedStackItem,
        remaining_stack: Vec<UnfinishedStackItem>,
    ) -> Result<Self, ParseError>;
}

pub fn parse_file(tokens: Vec<Token>, file_id: FileId) -> Result<File, ParseError> {
    parse(tokens, file_id)
}

pub fn parse<T: Parse>(tokens: Vec<Token>, file_id: FileId) -> Result<T, ParseError> {
    let first_token = if let Some(t) = tokens.iter().find(is_not_whitespace_or_comment_ref) {
        t.clone()
    } else {
        return T::from_empty_str(file_id);
    };
    let mut stack: Vec<UnfinishedStackItem> = T::initial_stack(file_id, first_token);

    for token in tokens.into_iter().filter(is_not_whitespace_or_comment) {
        T::before_handle_token(file_id, &token, &stack)?;
        handle_token(token, &mut stack, file_id)?;
    }

    let top_unfinished = stack.pop().expect("Stack should never be empty");
    T::finish(file_id, top_unfinished, stack)
}

fn is_not_whitespace_or_comment(token: &Token) -> bool {
    !matches!(
        token.kind,
        TokenKind::Whitespace | TokenKind::SingleLineComment | TokenKind::MultiLineComment
    )
}

fn is_not_whitespace_or_comment_ref(token: &&Token) -> bool {
    is_not_whitespace_or_comment(*token)
}

fn handle_token(
    token: Token,
    stack: &mut Vec<UnfinishedStackItem>,
    file_id: FileId,
) -> Result<(), ParseError> {
    let mut finished = FinishedStackItem::Token(token);
    while stack.len() >= 1 {
        let top_unfinished = stack.last_mut().unwrap();
        let accept_result = top_unfinished.accept(finished, file_id);
        match accept_result {
            AcceptResult::ContinueToNextToken => break,
            AcceptResult::PopAndContinueReducing(new_finished) => {
                stack.pop();
                finished = new_finished;
                continue;
            }
            AcceptResult::Push(item) => {
                stack.push(item);
                break;
            }
            AcceptResult::Push2(item1, item2) => {
                stack.push(item1);
                stack.push(item2);
                break;
            }
            AcceptResult::PushAndContinueReducingWithNewTop(item, new_finished) => {
                stack.push(item);
                finished = new_finished;
                continue;
            }
            AcceptResult::Error(err) => return Err(err),
        }
    }
    Ok(())
}

fn span_single(file_id: FileId, token: &Token) -> TextSpan {
    let start = token.start_index;
    TextSpan {
        file_id,
        start,
        end: start + token.content.len(),
    }
}

fn span_range_including_end(file_id: FileId, start: &Token, end: &Token) -> TextSpan {
    let start = start.start_index;
    let end = end.start_index + end.content.len();

    if end < start {
        panic!("End of span is before start of span.");
    }

    TextSpan {
        file_id,
        start,
        end,
    }
}

fn span_range_excluding_end(file_id: FileId, start: &Token, end: &Token) -> TextSpan {
    let start = start.start_index;
    let end = end.start_index;

    if end < start {
        panic!("End of span is before start of span.");
    }

    TextSpan {
        file_id,
        start,
        end,
    }
}

use unfinished::*;
mod unfinished;

use finished::*;
mod finished;

use accept::*;
mod accept;

mod impl_parse;
