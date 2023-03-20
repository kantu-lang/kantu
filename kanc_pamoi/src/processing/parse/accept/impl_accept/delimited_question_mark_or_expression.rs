use super::*;

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
                            AcceptResult::Error(ParseError::unexpected_token(original_token))
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
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
        }
    }
}
