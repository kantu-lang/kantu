use super::*;

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
