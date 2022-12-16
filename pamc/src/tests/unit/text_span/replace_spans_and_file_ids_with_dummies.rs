use crate::data::TextSpan;

use super::*;

pub trait ReplaceSpansAndFileIdsWithDummies {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self;
}

impl<T> ReplaceSpansAndFileIdsWithDummies for Option<T>
where
    T: ReplaceSpansAndFileIdsWithDummies,
{
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        self.map(T::replace_spans_and_file_ids_with_dummies)
    }
}

impl ReplaceSpansAndFileIdsWithDummies for File {
    fn replace_spans_and_file_ids_with_dummies(mut self) -> Self {
        self.span = dummy_span();
        self.id = dummy_id();

        self.items = self
            .items
            .into_iter()
            .map(FileItem::replace_spans_and_file_ids_with_dummies)
            .collect();
        self
    }
}

impl ReplaceSpansAndFileIdsWithDummies for FileItem {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        match self {
            FileItem::Type(item) => FileItem::Type(item.replace_spans_and_file_ids_with_dummies()),
            FileItem::Let(item) => FileItem::Let(item.replace_spans_and_file_ids_with_dummies()),
        }
    }
}

impl ReplaceSpansAndFileIdsWithDummies for TypeStatement {
    fn replace_spans_and_file_ids_with_dummies(mut self) -> Self {
        self.span = dummy_span();

        self.name = self.name.replace_spans_and_file_ids_with_dummies();
        self.params = self.params.replace_spans_and_file_ids_with_dummies();
        self.variants = self
            .variants
            .into_iter()
            .map(Variant::replace_spans_and_file_ids_with_dummies)
            .collect();
        self
    }
}

impl ReplaceSpansAndFileIdsWithDummies for NonEmptyVec<Param> {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        self.into_mapped(Param::replace_spans_and_file_ids_with_dummies)
    }
}

impl ReplaceSpansAndFileIdsWithDummies for Param {
    fn replace_spans_and_file_ids_with_dummies(mut self) -> Self {
        self.span = dummy_span();

        self.name = self.name.replace_spans_and_file_ids_with_dummies();
        self.type_ = self.type_.replace_spans_and_file_ids_with_dummies();
        self
    }
}

impl ReplaceSpansAndFileIdsWithDummies for Variant {
    fn replace_spans_and_file_ids_with_dummies(mut self) -> Self {
        self.span = dummy_span();

        self.name = self.name.replace_spans_and_file_ids_with_dummies();
        self.params = self.params.replace_spans_and_file_ids_with_dummies();
        self.return_type = self.return_type.replace_spans_and_file_ids_with_dummies();
        self
    }
}

impl ReplaceSpansAndFileIdsWithDummies for LetStatement {
    fn replace_spans_and_file_ids_with_dummies(mut self) -> Self {
        self.span = dummy_span();

        self.name = self.name.replace_spans_and_file_ids_with_dummies();
        self.value = self.value.replace_spans_and_file_ids_with_dummies();
        self
    }
}

impl ReplaceSpansAndFileIdsWithDummies for Identifier {
    fn replace_spans_and_file_ids_with_dummies(mut self) -> Self {
        self.span = dummy_span();
        self
    }
}

impl ReplaceSpansAndFileIdsWithDummies for Expression {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        match self {
            Expression::Identifier(identifier) => {
                Expression::Identifier(identifier.replace_spans_and_file_ids_with_dummies())
            }
            Expression::Dot(dot) => {
                Expression::Dot(Box::new(dot.replace_spans_and_file_ids_with_dummies()))
            }
            Expression::Call(call) => {
                Expression::Call(Box::new(call.replace_spans_and_file_ids_with_dummies()))
            }
            Expression::Fun(fun) => {
                Expression::Fun(Box::new(fun.replace_spans_and_file_ids_with_dummies()))
            }
            Expression::Match(match_) => {
                Expression::Match(Box::new(match_.replace_spans_and_file_ids_with_dummies()))
            }
            Expression::Forall(forall) => {
                Expression::Forall(Box::new(forall.replace_spans_and_file_ids_with_dummies()))
            }
            Expression::Check(check) => {
                Expression::Check(Box::new(check.replace_spans_and_file_ids_with_dummies()))
            }
        }
    }
}

impl ReplaceSpansAndFileIdsWithDummies for Dot {
    fn replace_spans_and_file_ids_with_dummies(mut self) -> Self {
        self.span = dummy_span();

        self.left = self.left.replace_spans_and_file_ids_with_dummies();
        self.right = self.right.replace_spans_and_file_ids_with_dummies();
        self
    }
}

impl ReplaceSpansAndFileIdsWithDummies for Call {
    fn replace_spans_and_file_ids_with_dummies(mut self) -> Self {
        self.span = dummy_span();

        self.callee = self.callee.replace_spans_and_file_ids_with_dummies();
        self.args = self.args.replace_spans_and_file_ids_with_dummies();
        self
    }
}

impl ReplaceSpansAndFileIdsWithDummies for NonEmptyVec<Expression> {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        self.into_mapped(Expression::replace_spans_and_file_ids_with_dummies)
    }
}

impl ReplaceSpansAndFileIdsWithDummies for Fun {
    fn replace_spans_and_file_ids_with_dummies(mut self) -> Self {
        self.span = dummy_span();

        self.name = self.name.replace_spans_and_file_ids_with_dummies();
        self.params = self.params.replace_spans_and_file_ids_with_dummies();
        self.return_type = self.return_type.replace_spans_and_file_ids_with_dummies();
        self.body = self.body.replace_spans_and_file_ids_with_dummies();
        self
    }
}

impl ReplaceSpansAndFileIdsWithDummies for Match {
    fn replace_spans_and_file_ids_with_dummies(mut self) -> Self {
        self.span = dummy_span();

        self.matchee = self.matchee.replace_spans_and_file_ids_with_dummies();
        self.cases = self
            .cases
            .into_iter()
            .map(MatchCase::replace_spans_and_file_ids_with_dummies)
            .collect();
        self
    }
}

impl ReplaceSpansAndFileIdsWithDummies for MatchCase {
    fn replace_spans_and_file_ids_with_dummies(mut self) -> Self {
        self.span = dummy_span();

        self.variant_name = self.variant_name.replace_spans_and_file_ids_with_dummies();
        self.params = self.params.replace_spans_and_file_ids_with_dummies();
        self.output = self.output.replace_spans_and_file_ids_with_dummies();
        self
    }
}

impl ReplaceSpansAndFileIdsWithDummies for NonEmptyVec<Identifier> {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        self.into_mapped(Identifier::replace_spans_and_file_ids_with_dummies)
    }
}

impl ReplaceSpansAndFileIdsWithDummies for Forall {
    fn replace_spans_and_file_ids_with_dummies(mut self) -> Self {
        self.span = dummy_span();

        self.params = self.params.replace_spans_and_file_ids_with_dummies();
        self.output = self.output.replace_spans_and_file_ids_with_dummies();
        self
    }
}

impl ReplaceSpansAndFileIdsWithDummies for Check {
    fn replace_spans_and_file_ids_with_dummies(mut self) -> Self {
        self.span = dummy_span();

        self.assertions = self.assertions.replace_spans_and_file_ids_with_dummies();
        self.output = self.output.replace_spans_and_file_ids_with_dummies();
        self
    }
}

impl ReplaceSpansAndFileIdsWithDummies for NonEmptyVec<CheckAssertion> {
    fn replace_spans_and_file_ids_with_dummies(mut self) -> Self {
        self.into_mapped(CheckAssertion::replace_spans_and_file_ids_with_dummies)
    }
}

impl ReplaceSpansAndFileIdsWithDummies for CheckAssertion {
    fn replace_spans_and_file_ids_with_dummies(mut self) -> Self {
        self.span = dummy_span();

        self.left = self.left.replace_spans_and_file_ids_with_dummies();
        self.right = self.right.replace_spans_and_file_ids_with_dummies();
        self
    }
}

impl ReplaceSpansAndFileIdsWithDummies for QuestionMarkOrExpression {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        match self {
            QuestionMarkOrExpression::QuestionMark { .. } => {
                QuestionMarkOrExpression::QuestionMark { span: dummy_span() }
            }
            QuestionMarkOrExpression::Expression(expression) => {
                QuestionMarkOrExpression::Expression(
                    expression.replace_spans_and_file_ids_with_dummies(),
                )
            }
        }
    }
}

impl ReplaceSpansAndFileIdsWithDummies for GoalKwOrExpression {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        match self {
            GoalKwOrExpression::GoalKw { .. } => GoalKwOrExpression::GoalKw { span: dummy_span() },
            GoalKwOrExpression::Expression(expression) => {
                GoalKwOrExpression::Expression(expression.replace_spans_and_file_ids_with_dummies())
            }
        }
    }
}

fn dummy_id() -> FileId {
    FileId(0)
}

fn dummy_span() -> TextSpan {
    TextSpan {
        file_id: dummy_id(),
        start: 0,
        end: 0,
    }
}
