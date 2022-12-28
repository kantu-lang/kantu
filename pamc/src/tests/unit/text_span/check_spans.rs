use crate::{data::TextSpan, processing::lex::LexError};

use super::*;

pub fn check_spans_in_file(src: &str, file: &File) {
    file.deep_check_spans(src);
}

trait DeepCheckSpans {
    fn deep_check_spans(&self, src: &str);
}

trait ShallowCheckOwnSpan {
    fn shallow_check_own_span(&self, src: &str);
}

trait DeepCheckChildSpans {
    fn deep_check_child_spans(&self, src: &str);
}

impl<T> DeepCheckSpans for Option<T>
where
    T: DeepCheckSpans,
{
    fn deep_check_spans(&self, src: &str) {
        if let Some(x) = self {
            x.deep_check_spans(src);
        }
    }
}

impl<T> DeepCheckSpans for T
where
    T: ShallowCheckOwnSpan + DeepCheckChildSpans,
{
    fn deep_check_spans(&self, src: &str) {
        self.shallow_check_own_span(src);
        self.deep_check_child_spans(src);
    }
}

impl ShallowCheckOwnSpan for File {
    fn shallow_check_own_span(&self, src: &str) {
        let spanned_src = get_spanned_slice(src, self.span).expect("Span should be valid");
        let reconstructed: File = parse_str(spanned_src)
            .expect("Should be able to reconstruct a copy using the spanned slice.");
        assert_eq!(
            self.clone().replace_spans_and_file_ids_with_dummies(),
            reconstructed.replace_spans_and_file_ids_with_dummies()
        );
    }
}
impl DeepCheckChildSpans for File {
    fn deep_check_child_spans(&self, src: &str) {
        for item in &self.items {
            item.deep_check_spans(src);
        }
    }
}

impl DeepCheckSpans for FileItem {
    fn deep_check_spans(&self, src: &str) {
        match self {
            FileItem::Mod(x) => x.deep_check_spans(src),
            FileItem::Type(x) => x.deep_check_spans(src),
            FileItem::Let(x) => x.deep_check_spans(src),
        }
    }
}

impl ShallowCheckOwnSpan for ModStatement {
    fn shallow_check_own_span(&self, _src: &str) {
        // Do nothing, since we haven't implemented `Parse` for `ModStatement` yet.
        // TODO: Implement `Parse` for `ModStatement` and use it here.
    }
}
impl DeepCheckChildSpans for ModStatement {
    fn deep_check_child_spans(&self, src: &str) {
        self.visibility.deep_check_spans(src);
        self.name.deep_check_spans(src);
    }
}

impl ShallowCheckOwnSpan for PubClause {
    fn shallow_check_own_span(&self, _src: &str) {
        // Do nothing, since we haven't implemented `Parse` for `PubClause` yet.
        // TODO: Implement `Parse` for `PubClause` and use it here.
    }
}
impl DeepCheckChildSpans for PubClause {
    fn deep_check_child_spans(&self, src: &str) {
        self.ancestor.deep_check_spans(src);
    }
}

impl ShallowCheckOwnSpan for ParenthesizedWeakAncestor {
    fn shallow_check_own_span(&self, _src: &str) {
        // Do nothing, since we haven't implemented `Parse` for `WeakAncestor` yet.
        // TODO: Implement `Parse` for `WeakAncestor` and use it here.
    }
}
impl DeepCheckChildSpans for ParenthesizedWeakAncestor {
    fn deep_check_child_spans(&self, src: &str) {
        self.kind.deep_check_spans(src);
    }
}

impl ShallowCheckOwnSpan for WeakAncestorKind {
    fn shallow_check_own_span(&self, _src: &str) {
        // Do nothing, since `WeakAncestorKind` doesn't have its own span.
    }
}
impl DeepCheckChildSpans for WeakAncestorKind {
    fn deep_check_child_spans(&self, src: &str) {
        match self {
            WeakAncestorKind::Global => {}
            WeakAncestorKind::Mod => {}
            WeakAncestorKind::Super(_) => {}
            WeakAncestorKind::PackageRelative { path_after_pack_kw } => {
                path_after_pack_kw.deep_check_spans(src);
            }
        }
    }
}

impl ShallowCheckOwnSpan for Vec<Identifier> {
    fn shallow_check_own_span(&self, _src: &str) {
        // Do nothing, since `Vec<Identifier>` doesn't have its own span.
    }
}
impl DeepCheckChildSpans for Vec<Identifier> {
    fn deep_check_child_spans(&self, src: &str) {
        for ident in self {
            ident.deep_check_spans(src);
        }
    }
}

impl ShallowCheckOwnSpan for TypeStatement {
    fn shallow_check_own_span(&self, src: &str) {
        let spanned_src = get_spanned_slice(src, self.span).expect("Span should be valid");
        let reconstructed: Self = parse_str(spanned_src)
            .expect("Should be able to reconstruct a copy using the spanned slice.");
        assert_eq!(
            self.clone().replace_spans_and_file_ids_with_dummies(),
            reconstructed.replace_spans_and_file_ids_with_dummies()
        );
    }
}
impl DeepCheckChildSpans for TypeStatement {
    fn deep_check_child_spans(&self, src: &str) {
        self.visibility.deep_check_spans(src);
        self.name.deep_check_spans(src);
        self.params.deep_check_spans(src);
        self.variants.deep_check_spans(src);
    }
}

impl ShallowCheckOwnSpan for NonEmptyVec<Param> {
    fn shallow_check_own_span(&self, _src: &str) {
        // Do nothing, since `NonEmptyVec<Param>` doesn't have its own span.
    }
}
impl DeepCheckChildSpans for NonEmptyVec<Param> {
    fn deep_check_child_spans(&self, src: &str) {
        for param in self {
            param.deep_check_spans(src);
        }
    }
}

impl ShallowCheckOwnSpan for Identifier {
    fn shallow_check_own_span(&self, src: &str) {
        let spanned_src = get_spanned_slice(src, self.span).expect("Span should be valid");
        let reconstructed: Expression = parse_str(spanned_src)
            .expect("Should be able to reconstruct a copy using the spanned slice.");
        assert_eq!(
            Expression::Identifier(self.clone()).replace_spans_and_file_ids_with_dummies(),
            reconstructed.replace_spans_and_file_ids_with_dummies()
        );
    }
}
impl DeepCheckChildSpans for Identifier {
    fn deep_check_child_spans(&self, _src: &str) {
        // Do nothing, since `Identifier` doesn't have any children.
    }
}

impl ShallowCheckOwnSpan for Param {
    fn shallow_check_own_span(&self, src: &str) {
        let spanned_src = get_spanned_slice(src, self.span).expect("Span should be valid");
        let reconstructed: Param = parse_str(spanned_src)
            .expect("Should be able to reconstruct a copy using the spanned slice.");
        assert_eq!(
            self.clone().replace_spans_and_file_ids_with_dummies(),
            reconstructed.replace_spans_and_file_ids_with_dummies()
        );
    }
}
impl DeepCheckChildSpans for Param {
    fn deep_check_child_spans(&self, src: &str) {
        self.name.deep_check_spans(src);
        self.type_.deep_check_spans(src);
    }
}

impl ShallowCheckOwnSpan for Vec<Variant> {
    fn shallow_check_own_span(&self, _src: &str) {
        // Do nothing, since `Vec<Variant>` doesn't have its own span.
    }
}
impl DeepCheckChildSpans for Vec<Variant> {
    fn deep_check_child_spans(&self, src: &str) {
        for variant in self {
            variant.deep_check_spans(src);
        }
    }
}

impl ShallowCheckOwnSpan for Variant {
    fn shallow_check_own_span(&self, src: &str) {
        let spanned_src = get_spanned_slice(src, self.span).expect("Span should be valid");
        let reconstructed: Variant = parse_str(spanned_src)
            .expect("Should be able to reconstruct a copy using the spanned slice.");
        assert_eq!(
            self.clone().replace_spans_and_file_ids_with_dummies(),
            reconstructed.replace_spans_and_file_ids_with_dummies()
        );
    }
}
impl DeepCheckChildSpans for Variant {
    fn deep_check_child_spans(&self, src: &str) {
        self.name.deep_check_spans(src);
        self.params.deep_check_spans(src);
        self.return_type.deep_check_spans(src);
    }
}

impl ShallowCheckOwnSpan for LetStatement {
    fn shallow_check_own_span(&self, src: &str) {
        let spanned_src = get_spanned_slice(src, self.span).expect("Span should be valid");
        let reconstructed: Self = parse_str(spanned_src)
            .expect("Should be able to reconstruct a copy using the spanned slice.");
        assert_eq!(
            self.clone().replace_spans_and_file_ids_with_dummies(),
            reconstructed.replace_spans_and_file_ids_with_dummies()
        );
    }
}
impl DeepCheckChildSpans for LetStatement {
    fn deep_check_child_spans(&self, src: &str) {
        self.visibility.deep_check_spans(src);
        self.transparency.deep_check_spans(src);
        self.name.deep_check_spans(src);
        self.value.deep_check_spans(src);
    }
}

impl ShallowCheckOwnSpan for Expression {
    fn shallow_check_own_span(&self, src: &str) {
        let spanned_src = get_spanned_slice(src, self.span()).expect("Span should be valid");
        let reconstructed: Expression = parse_str(spanned_src)
            .expect("Should be able to reconstruct a copy using the spanned slice.");
        assert_eq!(
            self.clone().replace_spans_and_file_ids_with_dummies(),
            reconstructed.replace_spans_and_file_ids_with_dummies()
        );
    }
}
impl DeepCheckChildSpans for Expression {
    fn deep_check_child_spans(&self, src: &str) {
        match self {
            Expression::Identifier(id) => id.deep_check_child_spans(src),
            Expression::Todo(_) => {
                // Do nothing, since `Todo` doesn't have any children.
            }
            Expression::Dot(dot) => dot.deep_check_child_spans(src),
            Expression::Call(call) => call.deep_check_child_spans(src),
            Expression::Fun(fun) => fun.deep_check_child_spans(src),
            Expression::Match(match_) => match_.deep_check_child_spans(src),
            Expression::Forall(forall) => forall.deep_check_child_spans(src),
            Expression::Check(check) => check.deep_check_child_spans(src),
        }
    }
}

impl DeepCheckChildSpans for Dot {
    fn deep_check_child_spans(&self, src: &str) {
        self.left.deep_check_spans(src);
        self.right.deep_check_spans(src);
    }
}

impl DeepCheckChildSpans for Call {
    fn deep_check_child_spans(&self, src: &str) {
        self.callee.deep_check_spans(src);
        self.args.deep_check_spans(src);
    }
}

impl ShallowCheckOwnSpan for NonEmptyVec<CallArg> {
    fn shallow_check_own_span(&self, _src: &str) {
        // Do nothing, since `NonEmptyVec<Expression>` doesn't have its own span.
    }
}
impl DeepCheckChildSpans for NonEmptyVec<CallArg> {
    fn deep_check_child_spans(&self, src: &str) {
        for arg in self {
            arg.deep_check_spans(src);
        }
    }
}

impl ShallowCheckOwnSpan for CallArg {
    fn shallow_check_own_span(&self, _src: &str) {
        // Do nothing, since we haven't implemented `Parse` for `CallArg` yet.
        // TODO: Implement `Parse` for `CallArg` and use it here.
    }
}
impl DeepCheckChildSpans for CallArg {
    fn deep_check_child_spans(&self, src: &str) {
        if let Some(ParamLabel::Explicit(label)) = &self.label {
            label.deep_check_spans(src);
        }
        self.value.deep_check_spans(src);
    }
}

impl DeepCheckChildSpans for Fun {
    fn deep_check_child_spans(&self, src: &str) {
        self.name.deep_check_child_spans(src);
        self.params.deep_check_spans(src);
        self.return_type.deep_check_spans(src);
        self.body.deep_check_spans(src);
    }
}

impl DeepCheckChildSpans for Match {
    fn deep_check_child_spans(&self, src: &str) {
        self.matchee.deep_check_spans(src);
        self.cases.deep_check_spans(src);
    }
}

impl ShallowCheckOwnSpan for Vec<MatchCase> {
    fn shallow_check_own_span(&self, _src: &str) {
        // Do nothing, since `Vec<MatchCase>` doesn't have its own span.
    }
}
impl DeepCheckChildSpans for Vec<MatchCase> {
    fn deep_check_child_spans(&self, src: &str) {
        for case in self {
            case.deep_check_spans(src);
        }
    }
}

impl ShallowCheckOwnSpan for MatchCase {
    fn shallow_check_own_span(&self, _src: &str) {
        // Do nothing, since we haven't implemented `Parse` for `MatchCase` yet.
        // TODO: Implement `Parse` for `MatchCase` and use it here.
    }
}
impl DeepCheckChildSpans for MatchCase {
    fn deep_check_child_spans(&self, src: &str) {
        self.variant_name.deep_check_spans(src);
        self.params.deep_check_spans(src);
        self.output.deep_check_spans(src);
    }
}

impl ShallowCheckOwnSpan for NonEmptyVec<MatchCaseParam> {
    fn shallow_check_own_span(&self, _src: &str) {
        // Do nothing, since `NonEmptyVec<MatchCaseParam>` doesn't have its own span.
    }
}
impl DeepCheckChildSpans for NonEmptyVec<MatchCaseParam> {
    fn deep_check_child_spans(&self, src: &str) {
        for id in self {
            id.deep_check_spans(src);
        }
    }
}

impl ShallowCheckOwnSpan for MatchCaseParam {
    fn shallow_check_own_span(&self, _src: &str) {
        // Do nothing, since we haven't implemented `Parse` for `MatchCaseParam` yet.
        // TODO: Implement `Parse` for `MatchCaseParam` and use it here.
    }
}
impl DeepCheckChildSpans for MatchCaseParam {
    fn deep_check_child_spans(&self, src: &str) {
        self.name.deep_check_spans(src);
    }
}

impl ShallowCheckOwnSpan for MatchCaseOutput {
    fn shallow_check_own_span(&self, _src: &str) {
        // Do nothing, since `MatchCaseOutput` doesn't have its own span.
    }
}
impl DeepCheckChildSpans for MatchCaseOutput {
    fn deep_check_child_spans(&self, src: &str) {
        match self {
            MatchCaseOutput::Some(expr) => expr.deep_check_spans(src),
            MatchCaseOutput::ImpossibilityClaim(kw_span) => {
                assert_eq!(Some("impossible"), get_spanned_slice(src, *kw_span));
            }
        }
    }
}

impl DeepCheckChildSpans for Forall {
    fn deep_check_child_spans(&self, src: &str) {
        self.params.deep_check_spans(src);
        self.output.deep_check_spans(src);
    }
}

impl DeepCheckChildSpans for Check {
    fn deep_check_child_spans(&self, src: &str) {
        self.assertions.deep_check_spans(src);
        self.output.deep_check_spans(src);
    }
}

impl ShallowCheckOwnSpan for NonEmptyVec<CheckAssertion> {
    fn shallow_check_own_span(&self, _src: &str) {
        // Do nothing, since `NonEmptyVec<CheckAssertion>` doesn't have its own span.
    }
}
impl DeepCheckChildSpans for NonEmptyVec<CheckAssertion> {
    fn deep_check_child_spans(&self, src: &str) {
        for assertion in self {
            assertion.deep_check_spans(src);
        }
    }
}

impl ShallowCheckOwnSpan for CheckAssertion {
    fn shallow_check_own_span(&self, _src: &str) {
        // Do nothing, since we haven't implemented `Parse` for `CheckAssertion` yet.
        // TODO: Implement `Parse` for `CheckAssertion` and use it here.
    }
}
impl DeepCheckChildSpans for CheckAssertion {
    fn deep_check_child_spans(&self, src: &str) {
        self.left.deep_check_spans(src);
        self.right.deep_check_spans(src);
    }
}

impl DeepCheckSpans for QuestionMarkOrExpression {
    fn deep_check_spans(&self, src: &str) {
        match self {
            QuestionMarkOrExpression::QuestionMark { span } => {
                assert_eq!(Some("?"), get_spanned_slice(src, *span))
            }
            QuestionMarkOrExpression::Expression(expression) => expression.deep_check_spans(src),
        }
    }
}

impl DeepCheckSpans for GoalKwOrExpression {
    fn deep_check_spans(&self, src: &str) {
        match self {
            GoalKwOrExpression::GoalKw { span } => {
                assert_eq!(Some("goal"), get_spanned_slice(src, *span))
            }
            GoalKwOrExpression::Expression(expression) => expression.deep_check_spans(src),
        }
    }
}

fn get_spanned_slice(s: &str, span: TextSpan) -> Option<&str> {
    if span.start <= s.len() && span.end <= s.len() {
        Some(&s[span.start..span.end])
    } else {
        None
    }
}

fn parse_str<T: Parse>(s: &str) -> Result<T, Result<ParseError, LexError>> {
    let tokens = lex(s).map_err(Err)?;
    parse(tokens, dummy_file_id()).map_err(Ok)
}

fn dummy_file_id() -> FileId {
    FileId(0)
}
