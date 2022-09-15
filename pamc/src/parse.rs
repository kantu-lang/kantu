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
        UnfinishedDelimitedExpression(UnfinishedDelimitedExpression),
        Fun(UnfinishedFun),
        Match(UnfinishedMatch),
        Forall(UnfinishedForall),
        Dot(UnfinishedDot),
        Call(UnfinishedCall),
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
        Matchee(Token, Expression),
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
    pub enum UnfinishedCall {
        Callee(Token, Expression),
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
                UnfinishedStackItem::UnfinishedDelimitedExpression(expression) => {
                    expression.accept(item)
                }
                UnfinishedStackItem::Fun(fun) => fun.accept(item),
                UnfinishedStackItem::Match(match_) => match_.accept(item),
                UnfinishedStackItem::Forall(forall) => forall.accept(item),
                UnfinishedStackItem::Dot(dot) => dot.accept(item),
                UnfinishedStackItem::Call(call) => call.accept(item),
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
            match self {
                UnfinishedParam::Empty => match item {
                    FinishedStackItem::Token(token) => match token.kind {
                        TokenKind::Identifier => {
                            let name = Identifier {
                                start_index: token.start_index,
                                content: token.content.clone(),
                            };
                            *self = UnfinishedParam::Name(token, name);
                            AcceptResult::ContinueToNextToken
                        }
                        _other_token_kind => {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    },
                    other_item => unexpected_finished_item(&other_item),
                },
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
        fn accept(&mut self, item: FinishedStackItem) -> AcceptResult {
            match self {
                UnfinishedConstructor::Dot(dot) => match item {
                    FinishedStackItem::Token(token) => match token.kind {
                        TokenKind::Identifier => {
                            let name = Identifier {
                                start_index: token.start_index,
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
        fn accept(&mut self, item: FinishedStackItem) -> AcceptResult {
            match self {
                UnfinishedDelimitedExpression::Empty => match item {
                    FinishedStackItem::Token(token) => match token.kind {
                        TokenKind::TypeTitleCase => {
                            let expression = Expression::QuasiIdentifier(QuasiIdentifier {
                                start_index: token.start_index,
                                kind: QuasiIdentifierKind::TypeTitleCase,
                            });
                            *self = UnfinishedDelimitedExpression::WaitingForEndDelimiter(
                                token, expression,
                            );
                            AcceptResult::ContinueToNextToken
                        }
                        TokenKind::Underscore => {
                            let expression = Expression::QuasiIdentifier(QuasiIdentifier {
                                start_index: token.start_index,
                                kind: QuasiIdentifierKind::Underscore,
                            });
                            *self = UnfinishedDelimitedExpression::WaitingForEndDelimiter(
                                token, expression,
                            );
                            AcceptResult::ContinueToNextToken
                        }
                        TokenKind::Identifier => {
                            let expression = Expression::Identifier(Identifier {
                                start_index: token.start_index,
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
                        TokenKind::Match => AcceptResult::Push(UnfinishedStackItem::Match(
                            UnfinishedMatch::Keyword(token),
                        )),
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
                                let unfinished = UnfinishedStackItem::Call(UnfinishedCall::Callee(
                                    first_token.clone(),
                                    expression.clone(),
                                ));
                                *self = UnfinishedDelimitedExpression::Empty;
                                AcceptResult::Push(unfinished)
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
        fn accept(&mut self, item: FinishedStackItem) -> AcceptResult {
            match self {
                UnfinishedFun::Keyword(fun_kw) => match item {
                    FinishedStackItem::Token(token) => match token.kind {
                        TokenKind::Identifier => {
                            let name = Identifier {
                                start_index: token.start_index,
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
        fn accept(&mut self, item: FinishedStackItem) -> AcceptResult {
            unimplemented!();
        }
    }

    impl Accept for UnfinishedForall {
        fn accept(&mut self, item: FinishedStackItem) -> AcceptResult {
            unimplemented!();
        }
    }

    impl Accept for UnfinishedDot {
        fn accept(&mut self, item: FinishedStackItem) -> AcceptResult {
            unimplemented!();
        }
    }

    impl Accept for UnfinishedCall {
        fn accept(&mut self, item: FinishedStackItem) -> AcceptResult {
            unimplemented!();
        }
    }
}
