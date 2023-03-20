use super::*;

impl Accept for UnfinishedDelimitedImpossibleKwOrExpression {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedDelimitedImpossibleKwOrExpression::Empty => match item {
                FinishedStackItem::Token(token) if token.kind == TokenKind::Impossible => {
                    *self = UnfinishedDelimitedImpossibleKwOrExpression::WaitingForEndDelimiter {
                        impossible_kw: token,
                    };
                    AcceptResult::ContinueToNextToken
                }

                FinishedStackItem::DelimitedExpression(first_token, expression, end_delimiter) => {
                    AcceptResult::PopAndContinueReducing(
                        FinishedStackItem::DelimitedImpossibleKwOrExpression(
                            first_token,
                            MatchCaseOutput::Some(expression),
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

            UnfinishedDelimitedImpossibleKwOrExpression::WaitingForEndDelimiter {
                impossible_kw,
            } => match item {
                FinishedStackItem::Token(token) => {
                    let token = ExpressionEndDelimiter::try_new(token);
                    match token {
                        Err(original_token) => {
                            AcceptResult::Error(ParseError::unexpected_token(original_token))
                        }
                        Ok(wrapped_token) => {
                            let output = MatchCaseOutput::ImpossibilityClaim(span_single(
                                file_id,
                                impossible_kw,
                            ));
                            let first_token = impossible_kw.clone();
                            AcceptResult::PopAndContinueReducing(
                                FinishedStackItem::DelimitedImpossibleKwOrExpression(
                                    first_token,
                                    output,
                                    wrapped_token,
                                ),
                            )
                        }
                    }
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
        }
    }
}
