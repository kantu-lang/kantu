use super::*;

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
