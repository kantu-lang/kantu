use super::*;

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
                        _other_end_delimiter => AcceptResult::Error(ParseError::unexpected_token(
                            end_delimiter.into_raw(),
                        )),
                    }
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
            UnfinishedMatch::Cases(match_kw, matchee, cases) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::StandardIdentifier => {
                        AcceptResult::Push(UnfinishedStackItem::MatchCase(
                            UnfinishedMatchCase::VariantName(Identifier {
                                span: span_single(file_id, &token),
                                name: IdentifierName::new(token.content),
                            }),
                        ))
                    }
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
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
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
