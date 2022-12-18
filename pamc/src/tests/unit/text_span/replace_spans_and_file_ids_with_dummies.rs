use crate::data::TextSpan;

use super::*;

pub trait ReplaceSpansAndFileIdsWithDummies {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self;
}

impl ReplaceSpansAndFileIdsWithDummies for TextSpan {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        dummy_span()
    }
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
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        let items = self
            .items
            .into_iter()
            .map(FileItem::replace_spans_and_file_ids_with_dummies)
            .collect();
        Self {
            span: dummy_span(),
            id: dummy_id(),
            items,
        }
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
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        let name = self.name.replace_spans_and_file_ids_with_dummies();
        let params = self.params.replace_spans_and_file_ids_with_dummies();
        let variants = self
            .variants
            .into_iter()
            .map(Variant::replace_spans_and_file_ids_with_dummies)
            .collect();
        Self {
            span: dummy_span(),
            name,
            params,
            variants,
        }
    }
}

impl ReplaceSpansAndFileIdsWithDummies for NonEmptyVec<Param> {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        self.into_mapped(Param::replace_spans_and_file_ids_with_dummies)
    }
}

impl ReplaceSpansAndFileIdsWithDummies for Param {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        let label = self.label.replace_spans_and_file_ids_with_dummies();
        let name = self.name.replace_spans_and_file_ids_with_dummies();
        let type_ = self.type_.replace_spans_and_file_ids_with_dummies();
        Self {
            span: dummy_span(),
            label,
            is_dashed: self.is_dashed,
            name,
            type_,
        }
    }
}

impl ReplaceSpansAndFileIdsWithDummies for ParamLabel {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        match self {
            ParamLabel::Implicit => ParamLabel::Implicit,
            ParamLabel::Explicit(label) => {
                ParamLabel::Explicit(label.replace_spans_and_file_ids_with_dummies())
            }
        }
    }
}

impl ReplaceSpansAndFileIdsWithDummies for Variant {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        let name = self.name.replace_spans_and_file_ids_with_dummies();
        let params = self.params.replace_spans_and_file_ids_with_dummies();
        let return_type = self.return_type.replace_spans_and_file_ids_with_dummies();
        Self {
            span: dummy_span(),
            name,
            params,
            return_type,
        }
    }
}

impl ReplaceSpansAndFileIdsWithDummies for LetStatement {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        let name = self.name.replace_spans_and_file_ids_with_dummies();
        let value = self.value.replace_spans_and_file_ids_with_dummies();
        Self {
            span: dummy_span(),
            name,
            value,
        }
    }
}

impl ReplaceSpansAndFileIdsWithDummies for Identifier {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        Self {
            span: dummy_span(),
            name: self.name,
        }
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
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        let left = self.left.replace_spans_and_file_ids_with_dummies();
        let right = self.right.replace_spans_and_file_ids_with_dummies();
        Self {
            span: dummy_span(),
            left,
            right,
        }
    }
}

impl ReplaceSpansAndFileIdsWithDummies for Call {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        let callee = self.callee.replace_spans_and_file_ids_with_dummies();
        let args = self.args.replace_spans_and_file_ids_with_dummies();
        Self {
            span: dummy_span(),
            callee,
            args,
        }
    }
}

impl ReplaceSpansAndFileIdsWithDummies for NonEmptyVec<CallArg> {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        self.into_mapped(CallArg::replace_spans_and_file_ids_with_dummies)
    }
}

impl ReplaceSpansAndFileIdsWithDummies for CallArg {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        let value = self.value.replace_spans_and_file_ids_with_dummies();
        let label = self.label.replace_spans_and_file_ids_with_dummies();
        Self {
            span: dummy_span(),
            label,
            value,
        }
    }
}

impl ReplaceSpansAndFileIdsWithDummies for Fun {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        let name = self.name.replace_spans_and_file_ids_with_dummies();
        let params = self.params.replace_spans_and_file_ids_with_dummies();
        let return_type = self.return_type.replace_spans_and_file_ids_with_dummies();
        let body = self.body.replace_spans_and_file_ids_with_dummies();
        Self {
            span: dummy_span(),
            name,
            params,
            return_type,
            body,
        }
    }
}

impl ReplaceSpansAndFileIdsWithDummies for Match {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        let matchee = self.matchee.replace_spans_and_file_ids_with_dummies();
        let cases = self.cases.replace_spans_and_file_ids_with_dummies();
        Self {
            span: dummy_span(),
            matchee,
            cases,
        }
    }
}

impl ReplaceSpansAndFileIdsWithDummies for Vec<MatchCase> {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        self.into_iter()
            .map(MatchCase::replace_spans_and_file_ids_with_dummies)
            .collect()
    }
}

impl ReplaceSpansAndFileIdsWithDummies for MatchCase {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        let variant_name = self.variant_name.replace_spans_and_file_ids_with_dummies();
        let params = self.params.replace_spans_and_file_ids_with_dummies();
        let triple_dot = self.triple_dot.replace_spans_and_file_ids_with_dummies();
        let output = self.output.replace_spans_and_file_ids_with_dummies();
        Self {
            span: dummy_span(),
            variant_name,
            params,
            triple_dot,
            output,
        }
    }
}

impl ReplaceSpansAndFileIdsWithDummies for NonEmptyVec<MatchCaseParam> {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        self.into_mapped(MatchCaseParam::replace_spans_and_file_ids_with_dummies)
    }
}

impl ReplaceSpansAndFileIdsWithDummies for MatchCaseParam {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        let label = self.label.replace_spans_and_file_ids_with_dummies();
        let name = self.name.replace_spans_and_file_ids_with_dummies();
        Self {
            span: dummy_span(),
            label,
            name,
        }
    }
}

impl ReplaceSpansAndFileIdsWithDummies for Forall {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        let params = self.params.replace_spans_and_file_ids_with_dummies();
        let output = self.output.replace_spans_and_file_ids_with_dummies();
        Self {
            span: dummy_span(),
            params,
            output,
        }
    }
}

impl ReplaceSpansAndFileIdsWithDummies for Check {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        let assertions = self.assertions.replace_spans_and_file_ids_with_dummies();
        let output = self.output.replace_spans_and_file_ids_with_dummies();
        Self {
            span: dummy_span(),
            assertions,
            output,
        }
    }
}

impl ReplaceSpansAndFileIdsWithDummies for NonEmptyVec<CheckAssertion> {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        self.into_mapped(CheckAssertion::replace_spans_and_file_ids_with_dummies)
    }
}

impl ReplaceSpansAndFileIdsWithDummies for CheckAssertion {
    fn replace_spans_and_file_ids_with_dummies(self) -> Self {
        let left = self.left.replace_spans_and_file_ids_with_dummies();
        let right = self.right.replace_spans_and_file_ids_with_dummies();
        Self {
            span: dummy_span(),
            kind: self.kind,
            left,
            right,
        }
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
