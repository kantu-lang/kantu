use crate::data::{
    non_empty_vec::NonEmptyVec,
    token::{Token, TokenKind},
    unsimplified_ast::*,
    FileId, TextSpan,
};

use std::{collections::VecDeque, num::NonZeroUsize};

// TODO: Make errors more informative.
// For example, if possible, it would be possible
// to include "expected token <x>" information.
// Or even encode hints into the error type
// (e.g., "Nullary functions are not permitted.").

#[derive(Clone, Debug, PartialEq)]
pub enum ParseError {
    UnexpectedNonEoiToken(Token),
    UnexpectedEndOfInput,
}

impl ParseError {
    pub fn unexpected_token(token: Token) -> Self {
        if token.kind == TokenKind::Eoi {
            ParseError::UnexpectedEndOfInput
        } else {
            ParseError::UnexpectedNonEoiToken(token)
        }
    }
}

pub trait Parse: Sized {
    fn initial_stack(file_id: FileId, first_token: &Token) -> Vec<UnfinishedStackItem>;

    fn finish(bottom_item: FinishedStackItem) -> Result<Self, ParseError>;
}

pub fn parse_file(tokens: Vec<Token>, file_id: FileId) -> Result<File, ParseError> {
    parse(tokens, file_id)
}

pub fn parse<T: Parse>(tokens: Vec<Token>, file_id: FileId) -> Result<T, ParseError> {
    let first_token = tokens.iter().find(is_not_whitespace_or_comment_ref).expect("There should be at least one meaningful (i.e., non-whitespace non-comment) token, even if it's an EOI token.");
    let mut stack: Vec<UnfinishedStackItem> = T::initial_stack(file_id, first_token);

    for token in tokens.into_iter().filter(is_not_whitespace_or_comment) {
        if let ReductionStatus::BottomStackItemFinished(finished_bottom_item) =
            handle_token(token, &mut stack, file_id)?
        {
            return T::finish(finished_bottom_item);
        }
    }

    Err(ParseError::UnexpectedEndOfInput)
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

#[derive(Clone, Debug)]
enum ReductionStatus {
    UnfinishedItemsRemain,
    BottomStackItemFinished(FinishedStackItem),
}

/// Returns if the stack ever becomes fully reduced
/// (i.e., the last item is popped), then `Ok(Some(item))`
/// is immediately returned (where `item` is the current `FinishedStackItem`).
fn handle_token(
    token: Token,
    stack: &mut Vec<UnfinishedStackItem>,
    file_id: FileId,
) -> Result<ReductionStatus, ParseError> {
    let mut queue = VecDeque::from([FinishedStackItem::Token(token)]);
    loop {
        let Some(finished) = queue.pop_front() else {
            return Ok(ReductionStatus::UnfinishedItemsRemain);
        };

        let mut finished = finished;
        let reduction_status = loop {
            let Some(top_unfinished) = stack.last_mut() else {
                break ReductionStatus::BottomStackItemFinished(finished);
            };
            let accept_result = top_unfinished.accept(finished, file_id);
            match accept_result {
                AcceptResult::ContinueToNextToken => break ReductionStatus::UnfinishedItemsRemain,
                AcceptResult::PopAndContinueReducing(new_finished) => {
                    stack.pop();
                    finished = new_finished;
                    continue;
                }
                AcceptResult::PopAndEnqueueAndContinueReducing(new_finished, new_queue_item) => {
                    stack.pop();
                    finished = new_finished;
                    queue.push_back(new_queue_item);
                    continue;
                }
                AcceptResult::Push(item) => {
                    stack.push(item);
                    break ReductionStatus::UnfinishedItemsRemain;
                }
                AcceptResult::Push2(item1, item2) => {
                    stack.push(item1);
                    stack.push(item2);
                    break ReductionStatus::UnfinishedItemsRemain;
                }
                AcceptResult::PushAndContinueReducingWithNewTop(item, new_finished) => {
                    stack.push(item);
                    finished = new_finished;
                    continue;
                }
                AcceptResult::Error(err) => return Err(err),
            }
        };
        match reduction_status {
            ReductionStatus::BottomStackItemFinished(item) => {
                return if let Some(queued_item) = queue.pop_front() {
                    let is_non_eoi = !matches!(
                        &queued_item,
                        FinishedStackItem::Token(token) if token.kind == TokenKind::Eoi
                    );
                    if is_non_eoi {
                        Err(unexpected_finished_item_err(&queued_item))
                    } else {
                        Ok(ReductionStatus::BottomStackItemFinished(item))
                    }
                } else {
                    Ok(ReductionStatus::BottomStackItemFinished(item))
                };
            }
            ReductionStatus::UnfinishedItemsRemain => continue,
        }
    }
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

fn unexpected_finished_item_err(item: &FinishedStackItem) -> ParseError {
    // TODO: This is sometimes _last_ token.
    ParseError::unexpected_token(item.first_token().clone())
}

use unfinished::*;
mod unfinished;

use finished::*;
mod finished;

use accept::*;
mod accept;

mod impl_parse;
