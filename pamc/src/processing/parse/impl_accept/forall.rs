use super::*;

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
