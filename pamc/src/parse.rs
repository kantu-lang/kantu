use crate::{
    lex::{Token, TokenKind},
    unbound_ast::*,
};

#[derive(Clone, Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
}

pub fn parse_file(tokens: Vec<Token>) -> Result<File, ParseError> {
    let first_token = if let Some(t) = tokens.iter().find(is_not_whitespace_ref) {
        t.clone()
    } else {
        return Ok(File(vec![]));
    };
    let mut stack: Vec<UnfinishedStackItem> =
        vec![UnfinishedStackItem::File(Box::new(UnfinishedFile {
            first_token,
            items: vec![],
        }))];

    for token in tokens.into_iter().filter(is_not_whitespace) {
        let mut finished = FinishedStackItem::Token(token);
        while stack.len() >= 1 {
            let top_unfinished = stack.last_mut().unwrap();
            let accept_result = top_unfinished.accept(finished);
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
                AcceptResult::Error(err) => return Err(err),
            }
        }
    }

    Ok(File(vec![]))
}

fn is_not_whitespace(token: &Token) -> bool {
    token.kind != TokenKind::Whitespace
}

fn is_not_whitespace_ref(token: &&Token) -> bool {
    token.kind != TokenKind::Whitespace
}

use unfinished::*;
mod unfinished {
    use super::*;

    #[derive(Clone, Debug)]
    pub enum UnfinishedStackItem {
        File(Box<UnfinishedFile>),
        Type(UnfinishedTypeStatement),
        Let(UnfinishedLetStatement),
    }

    #[derive(Clone, Debug)]
    pub struct UnfinishedFile {
        pub first_token: Token,
        pub items: Vec<FileItem>,
    }

    #[derive(Clone, Debug)]
    pub enum UnfinishedTypeStatement {
        Keyword(Token),
        Name(Token, Identifier),
        Params(Token, Identifier, Vec<Param>),
        Constructors(Token, Identifier, Vec<Constructor>),
    }

    #[derive(Clone, Debug)]
    pub enum UnfinishedLetStatement {
        Keyword(Token),
        Name(Token, Identifier),
        NameEqual(Token, Identifier),
        ValueEqual(Token, Identifier, Expression),
    }
}

mod first_token {
    use super::*;

    impl FinishedStackItem {
        pub fn first_token(&self) -> &Token {
            match self {
                FinishedStackItem::Token(token) => &token,
                FinishedStackItem::Type(token, _) => &token,
                FinishedStackItem::Let(token, _) => &token,
            }
        }
    }
}

use finished::*;
mod finished {
    use super::*;

    #[derive(Clone, Debug)]
    pub enum FinishedStackItem {
        Token(Token),
        Type(
            /// First token
            Token,
            TypeStatement,
        ),
        Let(
            /// First token
            Token,
            LetStatement,
        ),
    }
}

use accept::*;
mod accept {
    use super::*;

    pub trait Accept {
        fn accept(&mut self, item: FinishedStackItem) -> AcceptResult;
    }

    #[derive(Clone, Debug)]
    pub enum AcceptResult {
        ContinueToNextToken,
        PopAndContinueReducing(FinishedStackItem),
        Push(UnfinishedStackItem),
        Error(ParseError),
    }

    impl Accept for UnfinishedStackItem {
        fn accept(&mut self, item: FinishedStackItem) -> AcceptResult {
            match self {
                UnfinishedStackItem::File(file) => file.accept(item),
                UnfinishedStackItem::Type(type_) => type_.accept(item),
                UnfinishedStackItem::Let(let_) => let_.accept(item),
            }
        }
    }

    impl Accept for UnfinishedFile {
        fn accept(&mut self, item: FinishedStackItem) -> AcceptResult {
            match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::TypeLowerCase => AcceptResult::Push(UnfinishedStackItem::Type(
                        UnfinishedTypeStatement::Keyword(token),
                    )),
                    TokenKind::Let => AcceptResult::Push(UnfinishedStackItem::Let(
                        UnfinishedLetStatement::Keyword(token),
                    )),
                    _ => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                FinishedStackItem::Type(_, type_) => {
                    self.items.push(FileItem::Type(type_));
                    AcceptResult::ContinueToNextToken
                }
                FinishedStackItem::Let(_, let_) => {
                    self.items.push(FileItem::Let(let_));
                    AcceptResult::ContinueToNextToken
                }
            }
        }
    }

    impl Accept for UnfinishedTypeStatement {
        fn accept(&mut self, item: FinishedStackItem) -> AcceptResult {
            match self {
                UnfinishedTypeStatement::Keyword(type_kw) => match item {
                    FinishedStackItem::Token(token) => match token.kind {
                        TokenKind::Identifier => {
                            let name = Identifier {
                                start_index: token.start_index,
                                content: token.content.clone(),
                            };
                            *self = UnfinishedTypeStatement::Name(type_kw.clone(), name);
                            AcceptResult::ContinueToNextToken
                        }
                    },
                    other => AcceptResult::Error(ParseError::UnexpectedToken(
                        other.first_token().clone(),
                    )),
                },
            }
        }
    }

    impl Accept for UnfinishedLetStatement {
        fn accept(&mut self, item: FinishedStackItem) -> AcceptResult {}
    }
}
