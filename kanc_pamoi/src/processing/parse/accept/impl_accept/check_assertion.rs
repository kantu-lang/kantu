use super::*;

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
