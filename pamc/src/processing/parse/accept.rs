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
            UnfinishedStackItem::Variant(variant) => variant.accept(item, file_id),
            UnfinishedStackItem::UnfinishedDelimitedExpression(expression) => {
                expression.accept(item, file_id)
            }
            UnfinishedStackItem::Fun(fun) => fun.accept(item, file_id),
            UnfinishedStackItem::Match(match_) => match_.accept(item, file_id),
            UnfinishedStackItem::Forall(forall) => forall.accept(item, file_id),
            UnfinishedStackItem::Check(check) => check.accept(item, file_id),
            UnfinishedStackItem::CheckAssertions(assertions) => assertions.accept(item, file_id),
            UnfinishedStackItem::CheckAssertion(assertion) => assertion.accept(item, file_id),
            UnfinishedStackItem::UnfinishedDelimitedGoalKwOrExpression(expression) => {
                expression.accept(item, file_id)
            }
            UnfinishedStackItem::UnfinishedDelimitedQuestionMarkOrExpression(expression) => {
                expression.accept(item, file_id)
            }
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
                    TokenKind::StandardIdentifier => {
                        let name = Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::Standard(token.content.clone()),
                        };
                        *self = UnfinishedTypeStatement::Name(type_kw.clone(), name);
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                other_item => unexpected_finished_item(&other_item),
            },
            UnfinishedTypeStatement::Name(type_kw, name) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::LParen => {
                        AcceptResult::Push(UnfinishedStackItem::Params(UnfinishedParams {
                            first_token: token,
                            maximum_dashed_params_allowed: 0,
                            pending_dash: None,
                            params: vec![],
                        }))
                    }
                    TokenKind::LCurly => {
                        *self = UnfinishedTypeStatement::Variants(
                            type_kw.clone(),
                            name.clone(),
                            vec![],
                            vec![],
                        );
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                FinishedStackItem::Params(_, params) => {
                    *self = UnfinishedTypeStatement::Params(type_kw.clone(), name.clone(), params);
                    AcceptResult::ContinueToNextToken
                }
                other_item => unexpected_finished_item(&other_item),
            },
            UnfinishedTypeStatement::Params(type_kw, name, params) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::LCurly => {
                        *self = UnfinishedTypeStatement::Variants(
                            type_kw.clone(),
                            name.clone(),
                            params.clone(),
                            vec![],
                        );
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                other_item => unexpected_finished_item(&other_item),
            },
            UnfinishedTypeStatement::Variants(type_kw, name, params, variants) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Dot => AcceptResult::Push(UnfinishedStackItem::Variant(
                        UnfinishedVariant::Dot(token),
                    )),
                    TokenKind::RCurly => {
                        AcceptResult::PopAndContinueReducing(FinishedStackItem::Type(
                            type_kw.clone(),
                            TypeStatement {
                                span: span_range_including_end(file_id, &type_kw, &token),
                                name: name.clone(),
                                params: params.clone(),
                                variants: variants.clone(),
                            },
                        ))
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                FinishedStackItem::Variant(_, variant, end_delimiter) => {
                    variants.push(variant);
                    match end_delimiter.raw().kind {
                        TokenKind::Comma => AcceptResult::ContinueToNextToken,
                        TokenKind::RCurly => {
                            AcceptResult::PopAndContinueReducing(FinishedStackItem::Type(
                                type_kw.clone(),
                                TypeStatement {
                                    span: span_range_including_end(
                                        file_id,
                                        &type_kw,
                                        end_delimiter.raw(),
                                    ),
                                    name: name.clone(),
                                    params: params.clone(),
                                    variants: variants.clone(),
                                },
                            ))
                        }
                        _other_end_delimiter => AcceptResult::Error(ParseError::UnexpectedToken(
                            end_delimiter.into_raw(),
                        )),
                    }
                }
                other_item => unexpected_finished_item(&other_item),
            },
        }
    }
}

impl Accept for UnfinishedLetStatement {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedLetStatement::Keyword(let_kw) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::StandardIdentifier => {
                        let name = Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::Standard(token.content.clone()),
                        };
                        *self = UnfinishedLetStatement::Name(let_kw.clone(), name);
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
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
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                FinishedStackItem::DelimitedExpression(_, expression, end_delimiter) => {
                    match end_delimiter.raw().kind {
                        TokenKind::Semicolon => {
                            AcceptResult::PopAndContinueReducing(FinishedStackItem::Let(
                                let_kw.clone(),
                                LetStatement {
                                    span: span_range_including_end(
                                        file_id,
                                        &let_kw,
                                        end_delimiter.raw(),
                                    ),
                                    name: name.clone(),
                                    value: expression,
                                },
                            ))
                        }
                        _ => AcceptResult::Error(ParseError::UnexpectedToken(
                            end_delimiter.into_raw(),
                        )),
                    }
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
                TokenKind::Dash => {
                    if self.maximum_dashed_params_allowed == 0 || self.pending_dash.is_some() {
                        AcceptResult::Error(ParseError::UnexpectedToken(token))
                    } else {
                        self.maximum_dashed_params_allowed -= 1;
                        self.pending_dash = Some(token);
                        AcceptResult::ContinueToNextToken
                    }
                }
                TokenKind::StandardIdentifier => {
                    let name = Identifier {
                        span: span_single(file_id, &token),
                        name: IdentifierName::Standard(token.content.clone()),
                    };
                    let is_dashed = self.pending_dash.is_some();
                    let pending_dash = self.pending_dash.take();
                    AcceptResult::Push(UnfinishedStackItem::Param(UnfinishedParam::Name(
                        pending_dash.unwrap_or(token),
                        is_dashed,
                        name,
                    )))
                }
                TokenKind::Underscore => {
                    let name = Identifier {
                        span: span_single(file_id, &token),
                        name: IdentifierName::Reserved(ReservedIdentifierName::Underscore),
                    };
                    let is_dashed = self.pending_dash.is_some();
                    let pending_dash = self.pending_dash.take();
                    AcceptResult::Push(UnfinishedStackItem::Param(UnfinishedParam::Name(
                        pending_dash.unwrap_or(token),
                        is_dashed,
                        name,
                    )))
                }
                TokenKind::RParen => {
                    if self.params.is_empty() || self.pending_dash.is_some() {
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
                match end_delimiter.raw().kind {
                    TokenKind::Comma => AcceptResult::ContinueToNextToken,
                    TokenKind::RParen => AcceptResult::PopAndContinueReducing(
                        FinishedStackItem::Params(self.first_token.clone(), self.params.clone()),
                    ),
                    _other_end_delimiter => {
                        AcceptResult::Error(ParseError::UnexpectedToken(end_delimiter.into_raw()))
                    }
                }
            }
            other_item => unexpected_finished_item(&other_item),
        }
    }
}

impl Accept for UnfinishedParam {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedParam::Name(first_token, is_dashed, name) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Colon => {
                        AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                            UnfinishedDelimitedExpression::Empty,
                        ))
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                FinishedStackItem::DelimitedExpression(_, expression, end_delimiter) => {
                    AcceptResult::PopAndContinueReducing(FinishedStackItem::Param(
                        first_token.clone(),
                        Param {
                            span: span_range_excluding_end(
                                file_id,
                                &first_token,
                                end_delimiter.raw(),
                            ),
                            is_dashed: *is_dashed,
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

impl Accept for UnfinishedVariant {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedVariant::Dot(dot) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::StandardIdentifier => {
                        let name = Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::Standard(token.content.clone()),
                        };
                        *self = UnfinishedVariant::Name(dot.clone(), name);
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                other_item => unexpected_finished_item(&other_item),
            },
            UnfinishedVariant::Name(dot, name) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::LParen => {
                        AcceptResult::Push(UnfinishedStackItem::Params(UnfinishedParams {
                            first_token: token.clone(),
                            maximum_dashed_params_allowed: 0,
                            pending_dash: None,
                            params: vec![],
                        }))
                    }
                    TokenKind::Colon => {
                        *self = UnfinishedVariant::Params(dot.clone(), name.clone(), vec![]);
                        AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                            UnfinishedDelimitedExpression::Empty,
                        ))
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                FinishedStackItem::Params(_, params) => {
                    *self = UnfinishedVariant::Params(dot.clone(), name.clone(), params);
                    AcceptResult::ContinueToNextToken
                }
                other_item => unexpected_finished_item(&other_item),
            },
            UnfinishedVariant::Params(dot, name, params) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Colon => {
                        AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                            UnfinishedDelimitedExpression::Empty,
                        ))
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                FinishedStackItem::DelimitedExpression(_, expression, end_delimiter) => {
                    AcceptResult::PopAndContinueReducing(FinishedStackItem::Variant(
                        dot.clone(),
                        Variant {
                            span: span_single(file_id, &dot).inclusive_merge(expression.span()),
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
                        let expression = Expression::Identifier(Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase),
                        });
                        *self = UnfinishedDelimitedExpression::WaitingForEndDelimiter(
                            token, expression,
                        );
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::Underscore => {
                        let expression = Expression::Identifier(Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::Reserved(ReservedIdentifierName::Underscore),
                        });
                        *self = UnfinishedDelimitedExpression::WaitingForEndDelimiter(
                            token, expression,
                        );
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::StandardIdentifier => {
                        let expression = Expression::Identifier(Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::Standard(token.content.clone()),
                        });
                        *self = UnfinishedDelimitedExpression::WaitingForEndDelimiter(
                            token, expression,
                        );
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::Fun => {
                        AcceptResult::Push(UnfinishedStackItem::Fun(UnfinishedFun::Keyword(token)))
                    }
                    TokenKind::Match => AcceptResult::Push2(
                        UnfinishedStackItem::Match(UnfinishedMatch::Keyword(token)),
                        UnfinishedStackItem::UnfinishedDelimitedExpression(
                            UnfinishedDelimitedExpression::Empty,
                        ),
                    ),
                    TokenKind::Forall => AcceptResult::Push(UnfinishedStackItem::Forall(
                        UnfinishedForall::Keyword(token),
                    )),
                    TokenKind::Check => AcceptResult::Push(UnfinishedStackItem::Check(
                        UnfinishedCheck::Keyword(token),
                    )),
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                FinishedStackItem::DelimitedExpression(first_token, expression, end_delimiter) => {
                    AcceptResult::PopAndContinueReducing(FinishedStackItem::DelimitedExpression(
                        first_token,
                        expression,
                        end_delimiter,
                    ))
                }
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
                    FinishedStackItem::Token(token) => {
                        let token = ExpressionEndDelimiter::try_new(token);
                        let token = match token {
                            Err(original_token) => original_token,
                            Ok(wrapped_token) => {
                                return AcceptResult::PopAndContinueReducing(
                                    FinishedStackItem::DelimitedExpression(
                                        first_token.clone(),
                                        expression.clone(),
                                        wrapped_token,
                                    ),
                                )
                            }
                        };
                        match token.kind {
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
                        }
                    }
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
                    TokenKind::StandardIdentifier => {
                        let name = Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::Standard(token.content.clone()),
                        };
                        *self = UnfinishedFun::Name(fun_kw.clone(), name);
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::Underscore => {
                        let name = Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::Reserved(ReservedIdentifierName::Underscore),
                        };
                        *self = UnfinishedFun::Name(fun_kw.clone(), name);
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                other_item => unexpected_finished_item(&other_item),
            },
            UnfinishedFun::Name(fun_kw, name) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::LParen => {
                        AcceptResult::Push(UnfinishedStackItem::Params(UnfinishedParams {
                            first_token: token.clone(),
                            maximum_dashed_params_allowed: 1,
                            pending_dash: None,
                            params: vec![],
                        }))
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
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
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                FinishedStackItem::DelimitedExpression(_, expression, end_delimiter) => {
                    *self = UnfinishedFun::ReturnType(
                        fun_kw.clone(),
                        name.clone(),
                        params.clone(),
                        expression,
                    );
                    match end_delimiter.raw().kind {
                        TokenKind::LCurly => {
                            AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                                UnfinishedDelimitedExpression::Empty,
                            ))
                        }
                        _other_end_delimiter => AcceptResult::Error(ParseError::UnexpectedToken(
                            end_delimiter.into_raw(),
                        )),
                    }
                }
                other_item => unexpected_finished_item(&other_item),
            },
            UnfinishedFun::ReturnType(fun_kw, name, params, return_type) => match item {
                FinishedStackItem::DelimitedExpression(_, expression, end_delimiter) => {
                    match end_delimiter.raw().kind {
                        TokenKind::RCurly => AcceptResult::PopAndContinueReducing(
                            FinishedStackItem::UndelimitedExpression(
                                fun_kw.clone(),
                                Expression::Fun(Box::new(Fun {
                                    span: span_range_including_end(
                                        file_id,
                                        &fun_kw,
                                        end_delimiter.raw(),
                                    ),
                                    name: name.clone(),
                                    params: params.clone(),
                                    return_type: return_type.clone(),
                                    body: expression,
                                })),
                            ),
                        ),
                        _other_end_delimiter => AcceptResult::Error(ParseError::UnexpectedToken(
                            end_delimiter.into_raw(),
                        )),
                    }
                }
                other_item => unexpected_finished_item(&other_item),
            },
        }
    }
}

impl Accept for UnfinishedMatch {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedMatch::Keyword(match_kw) => match item {
                FinishedStackItem::DelimitedExpression(_, expression, end_delimiter) => {
                    match end_delimiter.raw().kind {
                        TokenKind::LCurly => {
                            *self = UnfinishedMatch::Cases(match_kw.clone(), expression, vec![]);
                            AcceptResult::ContinueToNextToken
                        }
                        _other_end_delimiter => AcceptResult::Error(ParseError::UnexpectedToken(
                            end_delimiter.into_raw(),
                        )),
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
                                span: span_range_including_end(file_id, &match_kw, &token),
                                matchee: matchee.clone(),
                                cases: cases.clone(),
                            })),
                        ),
                    ),
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                FinishedStackItem::MatchCase(_, case, end_delimiter) => {
                    cases.push(case);
                    match end_delimiter.raw().kind {
                        TokenKind::Comma => AcceptResult::ContinueToNextToken,
                        TokenKind::RCurly => AcceptResult::PopAndContinueReducing(
                            FinishedStackItem::UndelimitedExpression(
                                match_kw.clone(),
                                Expression::Match(Box::new(Match {
                                    span: span_range_including_end(
                                        file_id,
                                        &match_kw,
                                        end_delimiter.raw(),
                                    ),
                                    matchee: matchee.clone(),
                                    cases: cases.clone(),
                                })),
                            ),
                        ),
                        _other_end_delimiter => AcceptResult::Error(ParseError::UnexpectedToken(
                            end_delimiter.into_raw(),
                        )),
                    }
                }
                other_item => unexpected_finished_item(&other_item),
            },
        }
    }
}

impl Accept for UnfinishedForall {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedForall::Keyword(forall_kw) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::LParen => {
                        AcceptResult::Push(UnfinishedStackItem::Params(UnfinishedParams {
                            first_token: token.clone(),
                            maximum_dashed_params_allowed: 0,
                            pending_dash: None,
                            params: vec![],
                        }))
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
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
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                FinishedStackItem::DelimitedExpression(_, expression, end_delimiter) => {
                    match end_delimiter.raw().kind {
                        TokenKind::RCurly => AcceptResult::PopAndContinueReducing(
                            FinishedStackItem::UndelimitedExpression(
                                forall_kw.clone(),
                                Expression::Forall(Box::new(Forall {
                                    span: span_range_including_end(
                                        file_id,
                                        &forall_kw,
                                        end_delimiter.raw(),
                                    ),
                                    params: params.clone(),
                                    output: expression,
                                })),
                            ),
                        ),
                        _other_end_delimiter => AcceptResult::Error(ParseError::UnexpectedToken(
                            end_delimiter.into_raw(),
                        )),
                    }
                }
                other_item => unexpected_finished_item(&other_item),
            },
        }
    }
}

impl Accept for UnfinishedCheck {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedCheck::Keyword(check_kw) => match item {
                FinishedStackItem::Token(token) if token.kind == TokenKind::LParen => {
                    AcceptResult::Push(UnfinishedStackItem::CheckAssertions(
                        UnfinishedCheckAssertions {
                            first_token: token,
                            assertions: vec![],
                        },
                    ))
                }

                FinishedStackItem::CheckAssertions(_, assertions) => {
                    *self = UnfinishedCheck::Assertions(check_kw.clone(), assertions);
                    AcceptResult::ContinueToNextToken
                }

                other_item => unexpected_finished_item(&other_item),
            },
            UnfinishedCheck::Assertions(check_kw, assertions) => match item {
                FinishedStackItem::Token(token) if token.kind == TokenKind::LCurly => {
                    AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                        UnfinishedDelimitedExpression::Empty,
                    ))
                }

                FinishedStackItem::DelimitedExpression(_, output, end_delimiter)
                    if end_delimiter.raw().kind == TokenKind::RCurly =>
                {
                    AcceptResult::PopAndContinueReducing(FinishedStackItem::UndelimitedExpression(
                        check_kw.clone(),
                        Expression::Check(Box::new(Check {
                            span: span_range_including_end(file_id, &check_kw, end_delimiter.raw()),
                            assertions: assertions.clone(),
                            output,
                        })),
                    ))
                }

                other_item => unexpected_finished_item(&other_item),
            },
        }
    }
}

impl Accept for UnfinishedCheckAssertions {
    fn accept(&mut self, item: FinishedStackItem, _: FileId) -> AcceptResult {
        match item {
            FinishedStackItem::Token(token) => match token.kind {
                TokenKind::RParen => {
                    return AcceptResult::PopAndContinueReducing(
                        FinishedStackItem::CheckAssertions(
                            self.first_token.clone(),
                            self.assertions.clone(),
                        ),
                    )
                }
                _ => AcceptResult::PushAndContinueReducingWithNewTop(
                    UnfinishedStackItem::UnfinishedDelimitedGoalKwOrExpression(
                        UnfinishedDelimitedGoalKwOrExpression::Empty,
                    ),
                    FinishedStackItem::Token(token),
                ),
            },

            FinishedStackItem::DelimitedGoalKwOrExpression(
                first_token,
                goal_kw_or_expression,
                end_delimiter,
            ) => match end_delimiter.raw().kind {
                TokenKind::Colon => AcceptResult::Push(UnfinishedStackItem::CheckAssertion(
                    UnfinishedCheckAssertion {
                        first_token,
                        left: goal_kw_or_expression,
                        kind: CheckAssertionKind::Type,
                    },
                )),
                TokenKind::Equal => AcceptResult::Push(UnfinishedStackItem::CheckAssertion(
                    UnfinishedCheckAssertion {
                        first_token,
                        left: goal_kw_or_expression,
                        kind: CheckAssertionKind::NormalForm,
                    },
                )),
                _other_end_delimiter => {
                    AcceptResult::Error(ParseError::UnexpectedToken(end_delimiter.into_raw()))
                }
            },

            FinishedStackItem::CheckAssertion(_, assertion, end_delimiter) => {
                match end_delimiter.raw().kind {
                    TokenKind::RParen => {
                        self.assertions.push(assertion);
                        AcceptResult::PopAndContinueReducing(FinishedStackItem::CheckAssertions(
                            self.first_token.clone(),
                            self.assertions.clone(),
                        ))
                    }
                    TokenKind::Comma => {
                        self.assertions.push(assertion);
                        AcceptResult::ContinueToNextToken
                    }
                    _other_end_delimiter => {
                        AcceptResult::Error(ParseError::UnexpectedToken(end_delimiter.into_raw()))
                    }
                }
            }

            other_item => return unexpected_finished_item(&other_item),
        }
    }
}

impl Accept for UnfinishedCheckAssertion {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match item {
            FinishedStackItem::DelimitedQuestionMarkOrExpression(_, right, end_delimiter) => {
                AcceptResult::PopAndContinueReducing(FinishedStackItem::CheckAssertion(
                    self.first_token.clone(),
                    CheckAssertion {
                        span: span_single(file_id, &self.first_token).inclusive_merge(right.span()),
                        kind: self.kind,
                        left: self.left.clone(),
                        right,
                    },
                    end_delimiter,
                ))
            }

            other_item => AcceptResult::PushAndContinueReducingWithNewTop(
                UnfinishedStackItem::UnfinishedDelimitedQuestionMarkOrExpression(
                    UnfinishedDelimitedQuestionMarkOrExpression::Empty,
                ),
                other_item,
            ),
        }
    }
}

impl Accept for UnfinishedDelimitedGoalKwOrExpression {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedDelimitedGoalKwOrExpression::Empty => match item {
                FinishedStackItem::Token(token) if token.kind == TokenKind::Goal => {
                    *self = UnfinishedDelimitedGoalKwOrExpression::WaitingForEndDelimiter {
                        goal_kw: token,
                    };
                    AcceptResult::ContinueToNextToken
                }

                FinishedStackItem::DelimitedExpression(first_token, expression, end_delimiter) => {
                    AcceptResult::PopAndContinueReducing(
                        FinishedStackItem::DelimitedGoalKwOrExpression(
                            first_token,
                            GoalKwOrExpression::Expression(expression),
                            end_delimiter,
                        ),
                    )
                }

                other_item => AcceptResult::PushAndContinueReducingWithNewTop(
                    UnfinishedStackItem::UnfinishedDelimitedExpression(
                        UnfinishedDelimitedExpression::Empty,
                    ),
                    other_item,
                ),
            },

            UnfinishedDelimitedGoalKwOrExpression::WaitingForEndDelimiter {
                goal_kw: goal_kw_token,
            } => match item {
                FinishedStackItem::Token(token) => {
                    let token = ExpressionEndDelimiter::try_new(token);
                    match token {
                        Err(original_token) => {
                            AcceptResult::Error(ParseError::UnexpectedToken(original_token))
                        }
                        Ok(wrapped_token) => {
                            let goal_kw = GoalKwOrExpression::GoalKw {
                                span: span_single(file_id, &goal_kw_token),
                            };
                            let first_token = goal_kw_token.clone();
                            AcceptResult::PopAndContinueReducing(
                                FinishedStackItem::DelimitedGoalKwOrExpression(
                                    first_token,
                                    goal_kw,
                                    wrapped_token,
                                ),
                            )
                        }
                    }
                }
                other_item => unexpected_finished_item(&other_item),
            },
        }
    }
}

impl Accept for UnfinishedDelimitedQuestionMarkOrExpression {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedDelimitedQuestionMarkOrExpression::Empty => match item {
                FinishedStackItem::Token(token) if token.kind == TokenKind::Question => {
                    *self = UnfinishedDelimitedQuestionMarkOrExpression::WaitingForEndDelimiter {
                        question_mark: token,
                    };
                    AcceptResult::ContinueToNextToken
                }

                FinishedStackItem::DelimitedExpression(first_token, expression, end_delimiter) => {
                    AcceptResult::PopAndContinueReducing(
                        FinishedStackItem::DelimitedQuestionMarkOrExpression(
                            first_token,
                            QuestionMarkOrExpression::Expression(expression),
                            end_delimiter,
                        ),
                    )
                }

                other_item => AcceptResult::PushAndContinueReducingWithNewTop(
                    UnfinishedStackItem::UnfinishedDelimitedExpression(
                        UnfinishedDelimitedExpression::Empty,
                    ),
                    other_item,
                ),
            },

            UnfinishedDelimitedQuestionMarkOrExpression::WaitingForEndDelimiter {
                question_mark: question_mark_token,
            } => match item {
                FinishedStackItem::Token(token) => {
                    let token = ExpressionEndDelimiter::try_new(token);
                    match token {
                        Err(original_token) => {
                            AcceptResult::Error(ParseError::UnexpectedToken(original_token))
                        }
                        Ok(wrapped_token) => {
                            let question_mark = QuestionMarkOrExpression::QuestionMark {
                                span: span_single(file_id, &question_mark_token),
                            };
                            let first_token = question_mark_token.clone();
                            AcceptResult::PopAndContinueReducing(
                                FinishedStackItem::DelimitedQuestionMarkOrExpression(
                                    first_token,
                                    question_mark,
                                    wrapped_token,
                                ),
                            )
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
                TokenKind::StandardIdentifier => {
                    let right = Identifier {
                        span: span_single(file_id, &token),
                        name: IdentifierName::Standard(token.content.clone()),
                    };
                    AcceptResult::PopAndContinueReducing(FinishedStackItem::UndelimitedExpression(
                        self.first_token.clone(),
                        Expression::Dot(Box::new(Dot {
                            span: self.left.span().inclusive_merge(right.span),
                            left: self.left.clone(),
                            right,
                        })),
                    ))
                }
                _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
            },
            other_item => unexpected_finished_item(&other_item),
        }
    }
}

impl Accept for UnfinishedCall {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match item {
            FinishedStackItem::DelimitedExpression(_, arg, end_delimiter) => {
                self.args.push(arg);
                match end_delimiter.raw().kind {
                    TokenKind::Comma => AcceptResult::ContinueToNextToken,
                    TokenKind::RParen => AcceptResult::PopAndContinueReducing(
                        FinishedStackItem::UndelimitedExpression(
                            self.first_token.clone(),
                            Expression::Call(Box::new(Call {
                                span: span_range_including_end(
                                    file_id,
                                    &self.first_token,
                                    end_delimiter.raw(),
                                ),
                                callee: self.callee.clone(),
                                args: self.args.clone(),
                            })),
                        ),
                    ),
                    _other_end_delimiter => {
                        AcceptResult::Error(ParseError::UnexpectedToken(end_delimiter.into_raw()))
                    }
                }
            }
            FinishedStackItem::Token(token) => match token.kind {
                TokenKind::StandardIdentifier
                | TokenKind::Underscore
                | TokenKind::TypeTitleCase
                | TokenKind::Fun
                | TokenKind::Match
                | TokenKind::Forall
                | TokenKind::Check => AcceptResult::PushAndContinueReducingWithNewTop(
                    UnfinishedStackItem::UnfinishedDelimitedExpression(
                        UnfinishedDelimitedExpression::Empty,
                    ),
                    FinishedStackItem::Token(token),
                ),
                TokenKind::RParen => {
                    AcceptResult::PopAndContinueReducing(FinishedStackItem::UndelimitedExpression(
                        self.first_token.clone(),
                        Expression::Call(Box::new(Call {
                            span: span_range_including_end(file_id, &self.first_token, &token),
                            callee: self.callee.clone(),
                            args: self.args.clone(),
                        })),
                    ))
                }
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
                    TokenKind::StandardIdentifier => {
                        let name = Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::Standard(token.content.clone()),
                        };
                        *self = UnfinishedMatchCase::VariantName(dot_token.clone(), name);
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                other_item => unexpected_finished_item(&other_item),
            },
            UnfinishedMatchCase::VariantName(dot_token, variant_name) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::LParen => {
                        *self = UnfinishedMatchCase::ParamsInProgress(
                            dot_token.clone(),
                            variant_name.clone(),
                            vec![],
                            CurrentlyHasEndingComma(false),
                        );
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::FatArrow => {
                        *self = UnfinishedMatchCase::AwaitingOutput(
                            dot_token.clone(),
                            variant_name.clone(),
                            vec![],
                        );
                        AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                            UnfinishedDelimitedExpression::Empty,
                        ))
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                other_item => unexpected_finished_item(&other_item),
            },
            UnfinishedMatchCase::ParamsInProgress(
                dot_token,
                variant_name,
                params,
                currently_has_ending_comma,
            ) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::StandardIdentifier => {
                        let can_accept_identifier =
                            params.is_empty() || currently_has_ending_comma.0;
                        if can_accept_identifier {
                            let name = Identifier {
                                span: span_single(file_id, &token),
                                name: IdentifierName::Standard(token.content.clone()),
                            };
                            params.push(name);
                            currently_has_ending_comma.0 = false;
                            AcceptResult::ContinueToNextToken
                        } else {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    }
                    TokenKind::Underscore => {
                        let can_accept_identifier =
                            params.is_empty() || currently_has_ending_comma.0;
                        if can_accept_identifier {
                            let name = Identifier {
                                span: span_single(file_id, &token),
                                name: IdentifierName::Reserved(ReservedIdentifierName::Underscore),
                            };
                            params.push(name);
                            currently_has_ending_comma.0 = false;
                            AcceptResult::ContinueToNextToken
                        } else {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    }
                    TokenKind::Comma => {
                        let can_accept_comma = !currently_has_ending_comma.0 && !params.is_empty();
                        if can_accept_comma {
                            currently_has_ending_comma.0 = true;
                            AcceptResult::ContinueToNextToken
                        } else {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    }
                    TokenKind::RParen => {
                        if params.len() == 0 {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        } else {
                            *self = UnfinishedMatchCase::AwaitingOutput(
                                dot_token.clone(),
                                variant_name.clone(),
                                params.clone(),
                            );
                            AcceptResult::ContinueToNextToken
                        }
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                other_item => unexpected_finished_item(&other_item),
            },
            UnfinishedMatchCase::AwaitingOutput(dot_token, variant_name, params) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::FatArrow => {
                        AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                            UnfinishedDelimitedExpression::Empty,
                        ))
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                FinishedStackItem::DelimitedExpression(_, expression, end_delimiter) => {
                    match end_delimiter.raw().kind {
                        TokenKind::Comma | TokenKind::RCurly => {
                            AcceptResult::PopAndContinueReducing(FinishedStackItem::MatchCase(
                                dot_token.clone(),
                                MatchCase {
                                    span: span_range_excluding_end(
                                        file_id,
                                        &dot_token,
                                        end_delimiter.raw(),
                                    ),
                                    variant_name: variant_name.clone(),
                                    params: params.clone(),
                                    output: expression,
                                },
                                end_delimiter,
                            ))
                        }
                        _other_end_delimiter => AcceptResult::Error(ParseError::UnexpectedToken(
                            end_delimiter.into_raw(),
                        )),
                    }
                }
                other_item => unexpected_finished_item(&other_item),
            },
        }
    }
}
