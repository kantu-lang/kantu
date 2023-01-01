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

fn wrapped_unexpected_finished_item_err(item: &FinishedStackItem) -> AcceptResult {
    AcceptResult::Error(unexpected_finished_item_err(item))
}

impl Accept for UnfinishedStackItem {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedStackItem::File(file) => file.accept(item, file_id),
            UnfinishedStackItem::ParenthesizedQuasiAncestor(ancestor) => {
                ancestor.accept(item, file_id)
            }
            UnfinishedStackItem::Use(use_) => use_.accept(item, file_id),
            UnfinishedStackItem::Mod(mod_) => mod_.accept(item, file_id),
            UnfinishedStackItem::Type(type_) => type_.accept(item, file_id),
            UnfinishedStackItem::Let(let_) => let_.accept(item, file_id),
            UnfinishedStackItem::Params(params) => params.accept(item, file_id),
            UnfinishedStackItem::Param(param) => param.accept(item, file_id),
            UnfinishedStackItem::Variant(variant) => variant.accept(item, file_id),
            UnfinishedStackItem::UnfinishedDelimitedExpression(expression) => {
                expression.accept(item, file_id)
            }
            UnfinishedStackItem::UnfinishedDelimitedCallArg(arg) => arg.accept(item, file_id),
            UnfinishedStackItem::Fun(fun) => fun.accept(item, file_id),
            UnfinishedStackItem::Match(match_) => match_.accept(item, file_id),
            UnfinishedStackItem::MatchCaseParam(param) => param.accept(item, file_id),
            UnfinishedStackItem::UnfinishedDelimitedTripleDot(triple_dot) => {
                triple_dot.accept(item, file_id)
            }
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
            UnfinishedStackItem::UnfinishedDelimitedImpossibleKwOrExpression(expression) => {
                expression.accept(item, file_id)
            }
            UnfinishedStackItem::Dot(dot) => dot.accept(item, file_id),
            UnfinishedStackItem::Call(call) => call.accept(item, file_id),
            UnfinishedStackItem::MatchCase(match_case) => match_case.accept(item, file_id),
        }
    }
}

mod impl_accept;
