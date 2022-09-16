use crate::{
    lex::{Token, TokenKind},
    unbound_ast::*,
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
        UnfinishedDelimitedExpression(UnfinishedDelimitedExpression),
        Fun(UnfinishedFun),
        Match(UnfinishedMatch),
        Forall(UnfinishedForall),
        Dot(UnfinishedDot),
        Call(UnfinishedCall),
        MatchCase(UnfinishedMatchCase),
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
        Name(Token, Identifier),
    }

    #[derive(Clone, Debug)]
    pub enum UnfinishedConstructor {
        Dot(Token),
        Name(Token, Identifier),
        Params(Token, Identifier, Vec<Param>),
    }

    #[derive(Clone, Debug)]
    pub enum UnfinishedDelimitedExpression {
        Empty,
        WaitingForEndDelimiter(Token, Expression),
    }

    #[derive(Clone, Debug)]
    pub enum UnfinishedFun {
        Keyword(Token),
        Name(Token, Identifier),
        Params(Token, Identifier, Vec<Param>),
        ReturnType(Token, Identifier, Vec<Param>, Expression),
    }

    #[derive(Clone, Debug)]
    pub enum UnfinishedMatch {
        Keyword(Token),
        Cases(Token, Expression, Vec<MatchCase>),
    }

    #[derive(Clone, Debug)]
    pub enum UnfinishedForall {
        Keyword(Token),
        Params(Token, Vec<Param>),
    }

    #[derive(Clone, Debug)]
    pub struct UnfinishedDot {
        pub first_token: Token,
        pub left: Expression,
    }

    #[derive(Clone, Debug)]
    pub struct UnfinishedCall {
        pub first_token: Token,
        pub callee: Expression,
        pub args: Vec<Expression>,
    }

    #[derive(Clone, Debug)]
    pub enum UnfinishedMatchCase {
        Dot(Token),
        ConstructorName(Token, Identifier),
        ParamsInProgress(Token, Identifier, Vec<Identifier>, CurrentlyHasEndingComma),
        AwaitingOutput(Token, Identifier, Vec<Identifier>),
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct CurrentlyHasEndingComma(pub bool);
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
        DelimitedExpression(
            /// First token (".")
            Token,
            Expression,
            ExpressionEndDelimiter,
        ),
        UndelimitedExpression(
            /// First token (".")
            Token,
            Expression,
        ),
        MatchCase(
            /// First token (".")
            Token,
            MatchCase,
            ExpressionEndDelimiter,
        ),
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    /// Can be `,;{})`
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
                FinishedStackItem::DelimitedExpression(token, _, _) => &token,
                FinishedStackItem::UndelimitedExpression(token, _) => &token,
                FinishedStackItem::MatchCase(token, _, _) => &token,
            }
        }
    }
}

use accept::*;
mod accept {
    use super::*;

    pub trait Accept {
        fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult;
    }

    #[derive(Clone, Debug)]
    pub enum AcceptResult {
        ContinueToNextToken,
        PopAndContinueReducing(FinishedStackItem),
        Push(UnfinishedStackItem),
        Push2(UnfinishedStackItem, UnfinishedStackItem),
        PushAndContinueReducingWithNewTop(UnfinishedStackItem, FinishedStackItem),
        Error(ParseError),
    }

    fn unexpected_finished_item(item: &FinishedStackItem) -> AcceptResult {
        AcceptResult::Error(ParseError::UnexpectedToken(item.first_token().clone()))
    }

    impl Accept for UnfinishedStackItem {
        fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
            match self {
                UnfinishedStackItem::File(file) => file.accept(item, file_id),
                UnfinishedStackItem::Type(type_) => type_.accept(item, file_id),
                UnfinishedStackItem::Let(let_) => let_.accept(item, file_id),
                UnfinishedStackItem::Params(params) => params.accept(item, file_id),
                UnfinishedStackItem::Param(param) => param.accept(item, file_id),
                UnfinishedStackItem::Constructor(constructor) => constructor.accept(item, file_id),
                UnfinishedStackItem::UnfinishedDelimitedExpression(expression) => {
                    expression.accept(item, file_id)
                }
                UnfinishedStackItem::Fun(fun) => fun.accept(item, file_id),
                UnfinishedStackItem::Match(match_) => match_.accept(item, file_id),
                UnfinishedStackItem::Forall(forall) => forall.accept(item, file_id),
                UnfinishedStackItem::Dot(dot) => dot.accept(item, file_id),
                UnfinishedStackItem::Call(call) => call.accept(item, file_id),
                UnfinishedStackItem::MatchCase(match_case) => match_case.accept(item, file_id),
            }
        }
    }

    impl Accept for UnfinishedFile {
        fn accept(&mut self, item: FinishedStackItem, _: FileId) -> AcceptResult {
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
        fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
            match self {
                UnfinishedTypeStatement::Keyword(type_kw) => match item {
                    FinishedStackItem::Token(token) => match token.kind {
                        TokenKind::Identifier => {
                            let name = Identifier {
                                start: TextPosition {
                                    file_id,
                                    index: token.start_index,
                                },
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
                        TokenKind::LCurly => {
                            *self = UnfinishedTypeStatement::Constructors(
                                type_kw.clone(),
                                name.clone(),
                                vec![],
                                vec![],
                            );
                            AcceptResult::ContinueToNextToken
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
        fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
            match self {
                UnfinishedLetStatement::Keyword(let_kw) => match item {
                    FinishedStackItem::Token(token) => match token.kind {
                        TokenKind::Identifier => {
                            let name = Identifier {
                                start: TextPosition {
                                    file_id,
                                    index: token.start_index,
                                },
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
                        TokenKind::Equal => {
                            AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                                UnfinishedDelimitedExpression::Empty,
                            ))
                        }
                        _other_token_kind => {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    },
                    FinishedStackItem::DelimitedExpression(_, expression, _) => {
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
        fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
            match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Identifier => {
                        let name = Identifier {
                            start: TextPosition {
                                file_id,
                                index: token.start_index,
                            },
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
                        TokenKind::Comma => AcceptResult::ContinueToNextToken,
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
        fn accept(&mut self, item: FinishedStackItem, _: FileId) -> AcceptResult {
            match self {
                UnfinishedParam::Name(first_token, name) => match item {
                    FinishedStackItem::Token(token) => match token.kind {
                        TokenKind::Colon => {
                            AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                                UnfinishedDelimitedExpression::Empty,
                            ))
                        }
                        _other_token_kind => {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    },
                    FinishedStackItem::DelimitedExpression(_, expression, end_delimiter) => {
                        AcceptResult::PopAndContinueReducing(FinishedStackItem::Param(
                            first_token.clone(),
                            Param {
                                name: name.clone(),
                                type_: expression,
                            },
                            end_delimiter,
                        ))
                    }
                    other_item => unexpected_finished_item(&other_item),
                },
            }
        }
    }

    impl Accept for UnfinishedConstructor {
        fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
            match self {
                UnfinishedConstructor::Dot(dot) => match item {
                    FinishedStackItem::Token(token) => match token.kind {
                        TokenKind::Identifier => {
                            let name = Identifier {
                                start: TextPosition {
                                    file_id,
                                    index: token.start_index,
                                },
                                content: token.content.clone(),
                            };
                            *self = UnfinishedConstructor::Name(dot.clone(), name);
                            AcceptResult::ContinueToNextToken
                        }
                        _other_token_kind => {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    },
                    other_item => unexpected_finished_item(&other_item),
                },
                UnfinishedConstructor::Name(dot, name) => match item {
                    FinishedStackItem::Token(token) => match token.kind {
                        TokenKind::LParen => {
                            AcceptResult::Push(UnfinishedStackItem::Params(UnfinishedParams {
                                first_token: token.clone(),
                                params: vec![],
                            }))
                        }
                        TokenKind::Colon => {
                            AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                                UnfinishedDelimitedExpression::Empty,
                            ))
                        }
                        _other_token_kind => {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    },
                    FinishedStackItem::Params(_, params) => {
                        *self = UnfinishedConstructor::Params(dot.clone(), name.clone(), params);
                        AcceptResult::ContinueToNextToken
                    }
                    FinishedStackItem::DelimitedExpression(_, expression, end_delimiter) => {
                        AcceptResult::PopAndContinueReducing(FinishedStackItem::Constructor(
                            dot.clone(),
                            Constructor {
                                name: name.clone(),
                                params: vec![],
                                return_type: expression,
                            },
                            end_delimiter,
                        ))
                    }
                    other_item => unexpected_finished_item(&other_item),
                },
                UnfinishedConstructor::Params(dot, name, params) => match item {
                    FinishedStackItem::Token(token) => match token.kind {
                        TokenKind::Colon => {
                            AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                                UnfinishedDelimitedExpression::Empty,
                            ))
                        }
                        _other_token_kind => {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    },
                    FinishedStackItem::DelimitedExpression(_, expression, end_delimiter) => {
                        AcceptResult::PopAndContinueReducing(FinishedStackItem::Constructor(
                            dot.clone(),
                            Constructor {
                                name: name.clone(),
                                params: params.clone(),
                                return_type: expression,
                            },
                            end_delimiter,
                        ))
                    }
                    other_item => unexpected_finished_item(&other_item),
                },
            }
        }
    }

    impl Accept for UnfinishedDelimitedExpression {
        fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
            match self {
                UnfinishedDelimitedExpression::Empty => match item {
                    FinishedStackItem::Token(token) => match token.kind {
                        TokenKind::TypeTitleCase => {
                            let expression = Expression::QuasiIdentifier(QuasiIdentifier {
                                start: TextPosition {
                                    file_id,
                                    index: token.start_index,
                                },
                                kind: QuasiIdentifierKind::TypeTitleCase,
                            });
                            *self = UnfinishedDelimitedExpression::WaitingForEndDelimiter(
                                token, expression,
                            );
                            AcceptResult::ContinueToNextToken
                        }
                        TokenKind::Underscore => {
                            let expression = Expression::QuasiIdentifier(QuasiIdentifier {
                                start: TextPosition {
                                    file_id,
                                    index: token.start_index,
                                },
                                kind: QuasiIdentifierKind::Underscore,
                            });
                            *self = UnfinishedDelimitedExpression::WaitingForEndDelimiter(
                                token, expression,
                            );
                            AcceptResult::ContinueToNextToken
                        }
                        TokenKind::Identifier => {
                            let expression = Expression::Identifier(Identifier {
                                start: TextPosition {
                                    file_id,
                                    index: token.start_index,
                                },
                                content: token.content.clone(),
                            });
                            *self = UnfinishedDelimitedExpression::WaitingForEndDelimiter(
                                token, expression,
                            );
                            AcceptResult::ContinueToNextToken
                        }
                        TokenKind::Fun => AcceptResult::Push(UnfinishedStackItem::Fun(
                            UnfinishedFun::Keyword(token),
                        )),
                        TokenKind::Match => AcceptResult::Push2(
                            UnfinishedStackItem::Match(UnfinishedMatch::Keyword(token)),
                            UnfinishedStackItem::UnfinishedDelimitedExpression(
                                UnfinishedDelimitedExpression::Empty,
                            ),
                        ),
                        TokenKind::Forall => AcceptResult::Push(UnfinishedStackItem::Forall(
                            UnfinishedForall::Keyword(token),
                        )),
                        _other_token_kind => {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    },
                    FinishedStackItem::DelimitedExpression(
                        first_token,
                        expression,
                        end_delimiter,
                    ) => AcceptResult::PopAndContinueReducing(
                        FinishedStackItem::DelimitedExpression(
                            first_token,
                            expression,
                            end_delimiter,
                        ),
                    ),
                    FinishedStackItem::UndelimitedExpression(first_token, expression) => {
                        *self = UnfinishedDelimitedExpression::WaitingForEndDelimiter(
                            first_token,
                            expression,
                        );
                        AcceptResult::ContinueToNextToken
                    }
                    other_item => unexpected_finished_item(&other_item),
                },
                UnfinishedDelimitedExpression::WaitingForEndDelimiter(first_token, expression) => {
                    match item {
                        FinishedStackItem::Token(token) => match token.kind {
                            TokenKind::Comma
                            | TokenKind::Semicolon
                            | TokenKind::LCurly
                            | TokenKind::RCurly
                            | TokenKind::RParen => AcceptResult::PopAndContinueReducing(
                                FinishedStackItem::DelimitedExpression(
                                    first_token.clone(),
                                    expression.clone(),
                                    ExpressionEndDelimiter(token),
                                ),
                            ),
                            TokenKind::Dot => {
                                let unfinished = UnfinishedStackItem::Dot(UnfinishedDot {
                                    first_token: first_token.clone(),
                                    left: expression.clone(),
                                });
                                *self = UnfinishedDelimitedExpression::Empty;
                                AcceptResult::Push(unfinished)
                            }
                            TokenKind::LParen => {
                                let unfinished = UnfinishedStackItem::Call(UnfinishedCall {
                                    first_token: first_token.clone(),
                                    callee: expression.clone(),
                                    args: vec![],
                                });
                                *self = UnfinishedDelimitedExpression::Empty;
                                AcceptResult::Push2(
                                    unfinished,
                                    UnfinishedStackItem::UnfinishedDelimitedExpression(
                                        UnfinishedDelimitedExpression::Empty,
                                    ),
                                )
                            }
                            _other_token_kind => {
                                AcceptResult::Error(ParseError::UnexpectedToken(token))
                            }
                        },
                        other_item => unexpected_finished_item(&other_item),
                    }
                }
            }
        }
    }

    impl Accept for UnfinishedFun {
        fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
            match self {
                UnfinishedFun::Keyword(fun_kw) => match item {
                    FinishedStackItem::Token(token) => match token.kind {
                        TokenKind::Identifier => {
                            let name = Identifier {
                                start: TextPosition {
                                    file_id,
                                    index: token.start_index,
                                },
                                content: token.content.clone(),
                            };
                            *self = UnfinishedFun::Name(fun_kw.clone(), name);
                            AcceptResult::ContinueToNextToken
                        }
                        _other_token_kind => {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    },
                    other_item => unexpected_finished_item(&other_item),
                },
                UnfinishedFun::Name(fun_kw, name) => match item {
                    FinishedStackItem::Token(token) => match token.kind {
                        TokenKind::LParen => {
                            AcceptResult::Push(UnfinishedStackItem::Params(UnfinishedParams {
                                first_token: token.clone(),
                                params: vec![],
                            }))
                        }
                        _other_token_kind => {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    },
                    FinishedStackItem::Params(_, params) => {
                        *self = UnfinishedFun::Params(fun_kw.clone(), name.clone(), params);
                        AcceptResult::ContinueToNextToken
                    }
                    other_item => unexpected_finished_item(&other_item),
                },
                UnfinishedFun::Params(fun_kw, name, params) => match item {
                    FinishedStackItem::Token(token) => match token.kind {
                        TokenKind::Colon => {
                            AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                                UnfinishedDelimitedExpression::Empty,
                            ))
                        }
                        _other_token_kind => {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    },
                    FinishedStackItem::DelimitedExpression(_, expression, end_delimiter) => {
                        *self = UnfinishedFun::ReturnType(
                            fun_kw.clone(),
                            name.clone(),
                            params.clone(),
                            expression,
                        );
                        match end_delimiter.0.kind {
                            TokenKind::LCurly => AcceptResult::Push(
                                UnfinishedStackItem::UnfinishedDelimitedExpression(
                                    UnfinishedDelimitedExpression::Empty,
                                ),
                            ),
                            _other_end_delimiter => {
                                AcceptResult::Error(ParseError::UnexpectedToken(end_delimiter.0))
                            }
                        }
                    }
                    other_item => unexpected_finished_item(&other_item),
                },
                UnfinishedFun::ReturnType(fun_kw, name, params, return_type) => match item {
                    FinishedStackItem::DelimitedExpression(_, expression, end_delimiter) => {
                        match end_delimiter.0.kind {
                            TokenKind::RCurly => AcceptResult::PopAndContinueReducing(
                                FinishedStackItem::UndelimitedExpression(
                                    fun_kw.clone(),
                                    Expression::Fun(Box::new(Fun {
                                        name: name.clone(),
                                        params: params.clone(),
                                        return_type: return_type.clone(),
                                        return_value: expression,
                                    })),
                                ),
                            ),
                            _other_end_delimiter => {
                                AcceptResult::Error(ParseError::UnexpectedToken(end_delimiter.0))
                            }
                        }
                    }
                    other_item => unexpected_finished_item(&other_item),
                },
            }
        }
    }

    impl Accept for UnfinishedMatch {
        fn accept(&mut self, item: FinishedStackItem, _: FileId) -> AcceptResult {
            match self {
                UnfinishedMatch::Keyword(match_kw) => match item {
                    FinishedStackItem::DelimitedExpression(_, expression, end_delimiter) => {
                        match end_delimiter.0.kind {
                            TokenKind::LCurly => {
                                *self =
                                    UnfinishedMatch::Cases(match_kw.clone(), expression, vec![]);
                                AcceptResult::ContinueToNextToken
                            }
                            _other_end_delimiter => {
                                AcceptResult::Error(ParseError::UnexpectedToken(end_delimiter.0))
                            }
                        }
                    }
                    other_item => unexpected_finished_item(&other_item),
                },
                UnfinishedMatch::Cases(match_kw, matchee, cases) => match item {
                    FinishedStackItem::Token(token) => match token.kind {
                        TokenKind::Dot => AcceptResult::Push(UnfinishedStackItem::MatchCase(
                            UnfinishedMatchCase::Dot(token),
                        )),
                        TokenKind::RCurly => AcceptResult::PopAndContinueReducing(
                            FinishedStackItem::UndelimitedExpression(
                                match_kw.clone(),
                                Expression::Match(Box::new(Match {
                                    matchee: matchee.clone(),
                                    cases: cases.clone(),
                                })),
                            ),
                        ),
                        _other_token_kind => {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    },
                    FinishedStackItem::MatchCase(_, case, end_delimiter) => {
                        cases.push(case);
                        match end_delimiter.0.kind {
                            TokenKind::Comma => AcceptResult::ContinueToNextToken,
                            TokenKind::RCurly => AcceptResult::PopAndContinueReducing(
                                FinishedStackItem::UndelimitedExpression(
                                    match_kw.clone(),
                                    Expression::Match(Box::new(Match {
                                        matchee: matchee.clone(),
                                        cases: cases.clone(),
                                    })),
                                ),
                            ),
                            _other_end_delimiter => {
                                AcceptResult::Error(ParseError::UnexpectedToken(end_delimiter.0))
                            }
                        }
                    }
                    other_item => unexpected_finished_item(&other_item),
                },
            }
        }
    }

    impl Accept for UnfinishedForall {
        fn accept(&mut self, item: FinishedStackItem, _: FileId) -> AcceptResult {
            match self {
                UnfinishedForall::Keyword(forall_kw) => match item {
                    FinishedStackItem::Token(token) => match token.kind {
                        TokenKind::LParen => {
                            AcceptResult::Push(UnfinishedStackItem::Params(UnfinishedParams {
                                first_token: token.clone(),
                                params: vec![],
                            }))
                        }
                        _other_token_kind => {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    },
                    FinishedStackItem::Params(_, params) => {
                        *self = UnfinishedForall::Params(forall_kw.clone(), params);
                        AcceptResult::ContinueToNextToken
                    }
                    other_item => unexpected_finished_item(&other_item),
                },
                UnfinishedForall::Params(forall_kw, params) => match item {
                    FinishedStackItem::Token(token) => match token.kind {
                        TokenKind::LCurly => {
                            AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                                UnfinishedDelimitedExpression::Empty,
                            ))
                        }
                        _other_token_kind => {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    },
                    FinishedStackItem::DelimitedExpression(_, expression, end_delimiter) => {
                        match end_delimiter.0.kind {
                            TokenKind::RCurly => AcceptResult::PopAndContinueReducing(
                                FinishedStackItem::UndelimitedExpression(
                                    forall_kw.clone(),
                                    Expression::Forall(Box::new(Forall {
                                        params: params.clone(),
                                        output: expression,
                                    })),
                                ),
                            ),
                            _other_end_delimiter => {
                                AcceptResult::Error(ParseError::UnexpectedToken(end_delimiter.0))
                            }
                        }
                    }
                    other_item => unexpected_finished_item(&other_item),
                },
            }
        }
    }

    impl Accept for UnfinishedDot {
        fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
            match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Identifier => {
                        let right = Identifier {
                            start: TextPosition {
                                file_id,
                                index: token.start_index,
                            },
                            content: token.content.clone(),
                        };
                        AcceptResult::PopAndContinueReducing(
                            FinishedStackItem::UndelimitedExpression(
                                self.first_token.clone(),
                                Expression::Dot(Box::new(Dot {
                                    left: self.left.clone(),
                                    right,
                                })),
                            ),
                        )
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                other_item => unexpected_finished_item(&other_item),
            }
        }
    }

    impl Accept for UnfinishedCall {
        fn accept(&mut self, item: FinishedStackItem, _: FileId) -> AcceptResult {
            match item {
                FinishedStackItem::DelimitedExpression(first_token, expression, end_delimiter) => {
                    self.args.push(expression);
                    match end_delimiter.0.kind {
                        TokenKind::Comma => AcceptResult::ContinueToNextToken,
                        TokenKind::RParen => AcceptResult::PopAndContinueReducing(
                            FinishedStackItem::UndelimitedExpression(
                                first_token,
                                Expression::Call(Box::new(Call {
                                    callee: self.callee.clone(),
                                    args: self.args.clone(),
                                })),
                            ),
                        ),
                        _other_end_delimiter => {
                            AcceptResult::Error(ParseError::UnexpectedToken(end_delimiter.0))
                        }
                    }
                }
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Identifier
                    | TokenKind::Underscore
                    | TokenKind::TypeTitleCase
                    | TokenKind::Fun
                    | TokenKind::Match
                    | TokenKind::Forall => AcceptResult::PushAndContinueReducingWithNewTop(
                        UnfinishedStackItem::UnfinishedDelimitedExpression(
                            UnfinishedDelimitedExpression::Empty,
                        ),
                        FinishedStackItem::Token(token),
                    ),
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                other_item => unexpected_finished_item(&other_item),
            }
        }
    }

    impl Accept for UnfinishedMatchCase {
        fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
            match self {
                UnfinishedMatchCase::Dot(dot_token) => match item {
                    FinishedStackItem::Token(token) => match token.kind {
                        TokenKind::Identifier => {
                            let name = Identifier {
                                start: TextPosition {
                                    file_id,
                                    index: token.start_index,
                                },
                                content: token.content.clone(),
                            };
                            *self = UnfinishedMatchCase::ConstructorName(dot_token.clone(), name);
                            AcceptResult::ContinueToNextToken
                        }
                        _other_token_kind => {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    },
                    other_item => unexpected_finished_item(&other_item),
                },
                UnfinishedMatchCase::ConstructorName(dot_token, constructor_name) => match item {
                    FinishedStackItem::Token(token) => match token.kind {
                        TokenKind::LParen => {
                            *self = UnfinishedMatchCase::ParamsInProgress(
                                dot_token.clone(),
                                constructor_name.clone(),
                                vec![],
                                CurrentlyHasEndingComma(false),
                            );
                            AcceptResult::ContinueToNextToken
                        }
                        TokenKind::Arrow => {
                            *self = UnfinishedMatchCase::AwaitingOutput(
                                dot_token.clone(),
                                constructor_name.clone(),
                                vec![],
                            );
                            AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                                UnfinishedDelimitedExpression::Empty,
                            ))
                        }
                        _other_token_kind => {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    },
                    other_item => unexpected_finished_item(&other_item),
                },
                UnfinishedMatchCase::ParamsInProgress(
                    dot_token,
                    constructor_name,
                    params,
                    currently_has_ending_comma,
                ) => match item {
                    FinishedStackItem::Token(token) => match token.kind {
                        TokenKind::Identifier => {
                            let can_accept_identifier =
                                params.is_empty() || currently_has_ending_comma.0;
                            if can_accept_identifier {
                                let name = Identifier {
                                    start: TextPosition {
                                        file_id,
                                        index: token.start_index,
                                    },
                                    content: token.content.clone(),
                                };
                                params.push(name);
                                currently_has_ending_comma.0 = false;
                                AcceptResult::ContinueToNextToken
                            } else {
                                AcceptResult::Error(ParseError::UnexpectedToken(token))
                            }
                        }
                        TokenKind::Comma => {
                            let can_accept_comma =
                                !currently_has_ending_comma.0 && !params.is_empty();
                            if can_accept_comma {
                                currently_has_ending_comma.0 = true;
                                AcceptResult::ContinueToNextToken
                            } else {
                                AcceptResult::Error(ParseError::UnexpectedToken(token))
                            }
                        }
                        TokenKind::RParen => {
                            *self = UnfinishedMatchCase::AwaitingOutput(
                                dot_token.clone(),
                                constructor_name.clone(),
                                params.clone(),
                            );
                            AcceptResult::ContinueToNextToken
                        }
                        _other_token_kind => {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    },
                    other_item => unexpected_finished_item(&other_item),
                },
                UnfinishedMatchCase::AwaitingOutput(dot_token, constructor_name, params) => {
                    match item {
                        FinishedStackItem::Token(token) => match token.kind {
                            TokenKind::Arrow => AcceptResult::Push(
                                UnfinishedStackItem::UnfinishedDelimitedExpression(
                                    UnfinishedDelimitedExpression::Empty,
                                ),
                            ),
                            _other_token_kind => {
                                AcceptResult::Error(ParseError::UnexpectedToken(token))
                            }
                        },
                        FinishedStackItem::DelimitedExpression(_, expression, end_delimiter) => {
                            match end_delimiter.0.kind {
                                TokenKind::Comma | TokenKind::RCurly => {
                                    AcceptResult::PopAndContinueReducing(
                                        FinishedStackItem::MatchCase(
                                            dot_token.clone(),
                                            MatchCase {
                                                constructor_name: constructor_name.clone(),
                                                params: params.clone(),
                                                output: expression,
                                            },
                                            end_delimiter,
                                        ),
                                    )
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
}
