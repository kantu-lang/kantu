use crate::data::{
    token::{Token, TokenKind},
    unsimplified_ast::*,
    FileId, TextPosition,
};

// TODO: Check first token location logic.
// We don't want to give incorrect error messages!
// (E.g., "Error at index 234" but it's actually at index 864.)

#[derive(Clone, Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEndOfInput,
}

pub fn parse_file(tokens: Vec<Token>, file_id: FileId) -> Result<File, ParseError> {
    let first_token = if let Some(t) = tokens.iter().find(is_not_whitespace_ref) {
        t.clone()
    } else {
        return Ok(File {
            id: file_id,
            items: vec![],
        });
    };
    let mut stack: Vec<UnfinishedStackItem> =
        vec![UnfinishedStackItem::File(Box::new(UnfinishedFile {
            first_token,
            items: vec![],
        }))];

    for token in tokens.into_iter().filter(is_not_whitespace) {
        handle_token(token, &mut stack, file_id)?;
    }

    if stack.len() != 1 {
        Err(ParseError::UnexpectedEndOfInput)
    } else {
        let top_unfinished = stack.pop().unwrap();
        match top_unfinished {
            UnfinishedStackItem::File(file) => Ok(File {
                id: file_id,
                items: file.items,
            }),
            _ => panic!("The top item on the stack is not a file. This indicates a serious logic error with the parser.")
        }
    }
}

fn is_not_whitespace(token: &Token) -> bool {
    token.kind != TokenKind::Whitespace
}

fn is_not_whitespace_ref(token: &&Token) -> bool {
    token.kind != TokenKind::Whitespace
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
