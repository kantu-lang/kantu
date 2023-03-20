use super::*;

impl Accept for UnfinishedCheckAssertions {
    fn accept(&mut self, item: FinishedStackItem, _: FileId) -> AcceptResult {
        match item {
            FinishedStackItem::Token(token) => match token.kind {
                TokenKind::RParen => match NonEmptyVec::try_from(self.assertions.clone()) {
                    Ok(assertions) => AcceptResult::PopAndContinueReducing(
                        FinishedStackItem::CheckAssertions(self.first_token.clone(), assertions),
                    ),
                    Err(_) => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
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
                    AcceptResult::Error(ParseError::unexpected_token(end_delimiter.into_raw()))
                }
            },

            FinishedStackItem::CheckAssertion(_, assertion, end_delimiter) => {
                match end_delimiter.raw().kind {
                    TokenKind::RParen => {
                        let assertions =
                            NonEmptyVec::from_pushed(self.assertions.clone(), assertion);
                        AcceptResult::PopAndContinueReducing(FinishedStackItem::CheckAssertions(
                            self.first_token.clone(),
                            assertions,
                        ))
                    }
                    TokenKind::Comma => {
                        self.assertions.push(assertion);
                        AcceptResult::ContinueToNextToken
                    }
                    _other_end_delimiter => {
                        AcceptResult::Error(ParseError::unexpected_token(end_delimiter.into_raw()))
                    }
                }
            }

            other_item => return wrapped_unexpected_finished_item_err(&other_item),
        }
    }
}
