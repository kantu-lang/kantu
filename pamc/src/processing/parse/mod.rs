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

    fn finish(file_id: FileId, item: UnfinishedStackItem) -> Result<Self, ParseError>;
}

impl Parse for File {
    fn from_empty_str(file_id: FileId) -> Result<Self, ParseError> {
        Ok(File {
            span: TextSpan {
                file_id,
                start: 0,
                end: 0,
            },
            id: file_id,
            items: vec![],
        })
    }

    fn initial_stack(_: FileId, first_token: Token) -> Vec<UnfinishedStackItem> {
        vec![UnfinishedStackItem::File(Box::new(UnfinishedFile {
            first_token,
            items: vec![],
        }))]
    }

    fn finish(file_id: FileId, item: UnfinishedStackItem) -> Result<Self, ParseError> {
        match item {
            UnfinishedStackItem::File(file) => Ok(File {
                span: TextSpan {
                    file_id,
                    start: file.items[0].span().start,
                    end: file.items.last().expect("File should have at least one item.").span().end,
                },
                id: file_id,
                items: file.items,
            }),
            _ => panic!("The top item on the stack is not a file. This indicates a serious logic error with the parser.")
        }
    }
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
        handle_token(token, &mut stack, file_id)?;
    }

    if stack.len() != 1 {
        Err(ParseError::UnexpectedEndOfInput)
    } else {
        let top_unfinished = stack.pop().unwrap();
        T::finish(file_id, top_unfinished)
    }
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

use unfinished::*;
mod unfinished;

use finished::*;
mod finished;

use accept::*;
mod accept;
