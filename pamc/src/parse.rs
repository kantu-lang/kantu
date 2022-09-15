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
        Params(UnfinishedParams),
        Param(UnfinishedParam),
        Constructor(UnfinishedConstructor),
        UnfinishedExpression(UnfinishedExpression),
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
        Constructors(Token, Identifier, Vec<Param>, Vec<Constructor>),
    }

    #[derive(Clone, Debug)]
    pub enum UnfinishedLetStatement {
        Keyword(Token),
        Name(Token, Identifier),
    }

    #[derive(Clone, Debug)]
    pub struct UnfinishedParams {
        pub first_token: Token,
        pub params: Vec<Param>,
    }

    #[derive(Clone, Debug)]
    pub enum UnfinishedParam {
        Empty,
        Name(Token, Identifier),
    }

    #[derive(Clone, Debug)]
    pub enum UnfinishedConstructor {
        Dot(Token),
        Name(Token, Identifier),
        Params(Token, Identifier, Vec<Param>),
    }

    #[derive(Clone, Debug)]
    pub enum UnfinishedExpression {
        Empty,
        Valid(Expression, Token),
    }
}

use finished::*;
mod finished {
    use super::*;

    #[derive(Clone, Debug)]
    pub enum FinishedStackItem {
        Token(Token),
        Type(
            /// First token ("type")
            Token,
            TypeStatement,
        ),
        Let(
            /// First token ("let")
            Token,
            LetStatement,
        ),
        Params(
            /// First token ("(")
            Token,
            Vec<Param>,
        ),
        Param(
            /// First token
            Token,
            Param,
            ExpressionEndDelimiter,
        ),
        Constructor(
            /// First token (".")
            Token,
            Constructor,
            ExpressionEndDelimiter,
        ),
        Expression(
            /// First token (".")
            Token,
            Expression,
            ExpressionEndDelimiter,
        ),
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct ExpressionEndDelimiter(pub Token);
}

mod first_token {
    use super::*;

    impl FinishedStackItem {
        pub fn first_token(&self) -> &Token {
            match self {
                FinishedStackItem::Token(token) => &token,
                FinishedStackItem::Type(token, _) => &token,
                FinishedStackItem::Let(token, _) => &token,
                FinishedStackItem::Params(token, _) => &token,
                FinishedStackItem::Param(token, _, _) => &token,
                FinishedStackItem::Constructor(token, _, _) => &token,
                FinishedStackItem::Expression(token, _, _) => &token,
            }
        }
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

    fn unexpected_finished_item(item: &FinishedStackItem) -> AcceptResult {
        AcceptResult::Error(ParseError::UnexpectedToken(item.first_token().clone()))
    }

    impl Accept for UnfinishedStackItem {
        fn accept(&mut self, item: FinishedStackItem) -> AcceptResult {
            match self {
                UnfinishedStackItem::File(file) => file.accept(item),
                UnfinishedStackItem::Type(type_) => type_.accept(item),
                UnfinishedStackItem::Let(let_) => let_.accept(item),
                UnfinishedStackItem::Params(params) => params.accept(item),
                UnfinishedStackItem::Param(param) => param.accept(item),
                UnfinishedStackItem::Constructor(constructor) => constructor.accept(item),
                UnfinishedStackItem::UnfinishedExpression(expression) => expression.accept(item),
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
                other_item => unexpected_finished_item(&other_item),
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
                        _other_token_kind => {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    },
                    other_item => unexpected_finished_item(&other_item),
                },
                UnfinishedTypeStatement::Name(type_kw, name) => match item {
                    FinishedStackItem::Token(token) => match token.kind {
                        TokenKind::LParen => {
                            AcceptResult::Push(UnfinishedStackItem::Params(UnfinishedParams {
                                first_token: token,
                                params: vec![],
                            }))
                        }
                        _other_token_kind => {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    },
                    FinishedStackItem::Params(_, params) => {
                        *self =
                            UnfinishedTypeStatement::Params(type_kw.clone(), name.clone(), params);
                        AcceptResult::ContinueToNextToken
                    }
                    other_item => unexpected_finished_item(&other_item),
                },
                UnfinishedTypeStatement::Params(type_kw, name, params) => match item {
                    FinishedStackItem::Token(token) => match token.kind {
                        TokenKind::LCurly => {
                            *self = UnfinishedTypeStatement::Constructors(
                                type_kw.clone(),
                                name.clone(),
                                params.clone(),
                                vec![],
                            );
                            AcceptResult::ContinueToNextToken
                        }
                        _other_token_kind => {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    },
                    other_item => unexpected_finished_item(&other_item),
                },
                UnfinishedTypeStatement::Constructors(type_kw, name, params, constructors) => {
                    match item {
                        FinishedStackItem::Token(token) => match token.kind {
                            TokenKind::Dot => AcceptResult::Push(UnfinishedStackItem::Constructor(
                                UnfinishedConstructor::Dot(token),
                            )),
                            TokenKind::RCurly => {
                                AcceptResult::PopAndContinueReducing(FinishedStackItem::Type(
                                    type_kw.clone(),
                                    TypeStatement {
                                        name: name.clone(),
                                        params: params.clone(),
                                        constructors: constructors.clone(),
                                    },
                                ))
                            }
                            _other_token_kind => {
                                AcceptResult::Error(ParseError::UnexpectedToken(token))
                            }
                        },
                        FinishedStackItem::Constructor(_, constructor, end_delimiter) => {
                            constructors.push(constructor);
                            match end_delimiter.0.kind {
                                TokenKind::Comma => AcceptResult::ContinueToNextToken,
                                TokenKind::RCurly => {
                                    AcceptResult::PopAndContinueReducing(FinishedStackItem::Type(
                                        type_kw.clone(),
                                        TypeStatement {
                                            name: name.clone(),
                                            params: params.clone(),
                                            constructors: constructors.clone(),
                                        },
                                    ))
                                }
                                _other_end_delimiter => AcceptResult::Error(
                                    ParseError::UnexpectedToken(end_delimiter.0),
                                ),
                            }
                        }
                        other_item => unexpected_finished_item(&other_item),
                    }
                }
            }
        }
    }

    impl Accept for UnfinishedLetStatement {
        fn accept(&mut self, item: FinishedStackItem) -> AcceptResult {
            match self {
                UnfinishedLetStatement::Keyword(let_kw) => match item {
                    FinishedStackItem::Token(token) => match token.kind {
                        TokenKind::Identifier => {
                            let name = Identifier {
                                start_index: token.start_index,
                                content: token.content.clone(),
                            };
                            *self = UnfinishedLetStatement::Name(let_kw.clone(), name);
                            AcceptResult::ContinueToNextToken
                        }
                        _other_token_kind => {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    },
                    other_item => unexpected_finished_item(&other_item),
                },
                UnfinishedLetStatement::Name(let_kw, name) => match item {
                    FinishedStackItem::Token(token) => match token.kind {
                        TokenKind::Equal => AcceptResult::Push(
                            UnfinishedStackItem::UnfinishedExpression(UnfinishedExpression::Empty),
                        ),
                        _other_token_kind => {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    },
                    FinishedStackItem::Expression(_, expression, _) => {
                        AcceptResult::PopAndContinueReducing(FinishedStackItem::Let(
                            let_kw.clone(),
                            LetStatement {
                                name: name.clone(),
                                value: expression,
                            },
                        ))
                    }
                    other_item => unexpected_finished_item(&other_item),
                },
            }
        }
    }

    impl Accept for UnfinishedParams {
        fn accept(&mut self, item: FinishedStackItem) -> AcceptResult {
            match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Comma => {
                        if self.params.is_empty() {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        } else {
                            AcceptResult::Push(UnfinishedStackItem::Param(UnfinishedParam::Empty))
                        }
                    }
                    TokenKind::Identifier => {
                        let name = Identifier {
                            start_index: token.start_index,
                            content: token.content.clone(),
                        };
                        AcceptResult::Push(UnfinishedStackItem::Param(UnfinishedParam::Name(
                            token, name,
                        )))
                    }
                    TokenKind::RParen => {
                        if self.params.is_empty() {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        } else {
                            AcceptResult::PopAndContinueReducing(FinishedStackItem::Params(
                                self.first_token.clone(),
                                self.params.clone(),
                            ))
                        }
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                FinishedStackItem::Param(_, param, end_delimiter) => {
                    self.params.push(param);
                    match end_delimiter.0.kind {
                        TokenKind::Comma => {
                            AcceptResult::Push(UnfinishedStackItem::Param(UnfinishedParam::Empty))
                        }
                        TokenKind::RParen => {
                            AcceptResult::PopAndContinueReducing(FinishedStackItem::Params(
                                self.first_token.clone(),
                                self.params.clone(),
                            ))
                        }
                        _other_end_delimiter => {
                            AcceptResult::Error(ParseError::UnexpectedToken(end_delimiter.0))
                        }
                    }
                }
                other_item => unexpected_finished_item(&other_item),
            }
        }
    }

    impl Accept for UnfinishedParam {
        fn accept(&mut self, item: FinishedStackItem) -> AcceptResult {
            unimplemented!();
        }
    }

    impl Accept for UnfinishedConstructor {
        fn accept(&mut self, item: FinishedStackItem) -> AcceptResult {
            unimplemented!()
        }
    }

    impl Accept for UnfinishedExpression {
        fn accept(&mut self, item: FinishedStackItem) -> AcceptResult {
            unimplemented!()
        }
    }
}
