use super::*;

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
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
            UnfinishedFun::Name(fun_kw, name) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::LParen => {
                        AcceptResult::Push(UnfinishedStackItem::Params(UnfinishedParams {
                            first_token: token.clone(),
                            maximum_dashed_params_allowed: 1,
                            pending_tilde: None,
                            pending_dash: None,
                            params: vec![],
                        }))
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                FinishedStackItem::Params(_, params) => {
                    *self = UnfinishedFun::Params(fun_kw.clone(), name.clone(), params);
                    AcceptResult::ContinueToNextToken
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
            UnfinishedFun::Params(fun_kw, name, params) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Colon => {
                        AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                            UnfinishedDelimitedExpression::Empty,
                        ))
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
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
                        _other_end_delimiter => AcceptResult::Error(ParseError::unexpected_token(
                            end_delimiter.into_raw(),
                        )),
                    }
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
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
                        _other_end_delimiter => AcceptResult::Error(ParseError::unexpected_token(
                            end_delimiter.into_raw(),
                        )),
                    }
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
        }
    }
}
