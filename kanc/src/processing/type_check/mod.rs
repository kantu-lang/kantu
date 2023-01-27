use crate::data::{
    file_tree::FileTree,
    light_ast::*,
    non_empty_veclike::{NonEmptySlice, NonEmptyVec, OptionalNonEmptyVecLen},
    text_span::*,
    type_positivity_validation_result::TypePositivityValidated,
};

use eval::*;
mod eval;

use context::*;
mod context;

use misc::*;
mod misc;

use shift::*;
mod shift;

use substitute::*;
mod substitute;

use substitution_context::*;
mod substitution_context;

pub use type_check_node::type_check_file_items;
use type_check_node::*;
mod type_check_node;

mod verify_expression_is_visibility;
use verify_expression_is_visibility::*;

mod without_spans;
use without_spans::*;

#[derive(Clone, Debug)]
pub enum TypeCheckError {
    ExpectedTermOfTypeType0OrType1 {
        expression_id: ExpressionRef<'a>,
        non_type0_or_type1_type_id: NormalFormId,
    },
    IllegalCallee {
        callee_id: ExpressionRef<'a>,
        callee_type_id: NormalFormId,
    },
    WrongNumberOfArguments {
        call_id: &'a Call<'a>,
        expected: usize,
        actual: usize,
    },
    CallLabelednessMismatch {
        call_id: &'a Call<'a>,
    },
    MissingLabeledCallArgs {
        call_id: &'a Call<'a>,
        missing_label_list_id: NonEmptyListId<&'a Identifier<'a>>,
    },
    ExtraneousLabeledCallArg {
        call_id: &'a Call<'a>,
        // TODO: Make this a list
        arg_id: LabeledCallArgId,
    },
    WrongNumberOfMatchCaseParams {
        case_id: &'a MatchCase<'a>,
        expected: usize,
        actual: usize,
    },
    MatchCaseLabelednessMismatch {
        case_id: &'a MatchCase<'a>,
        param_list_id: NonEmptyMatchCaseParamListId,
    },
    MissingLabeledMatchCaseParams {
        case_id: &'a MatchCase<'a>,
        missing_label_list_id: NonEmptyListId<&'a Identifier<'a>>,
    },
    UndefinedLabeledMatchCaseParams {
        case_id: &'a MatchCase<'a>,
        case_param_list_id: NonEmptyListId<&'a LabeledMatchCaseParam<'a>>,
    },
    TypeMismatch {
        expression_id: ExpressionRef<'a>,
        expected_type_id: NormalFormId,
        actual_type_id: NormalFormId,
    },
    NonAdtMatchee {
        matchee_id: ExpressionRef<'a>,
        type_id: NormalFormId,
    },
    DuplicateMatchCase {
        existing_match_case_id: &'a MatchCase<'a>,
        new_match_case_id: &'a MatchCase<'a>,
    },
    MissingMatchCases {
        match_id: &'a Match<'a>,
        missing_variant_name_list_id: NonEmptyListId<&'a Identifier<'a>>,
    },
    ExtraneousMatchCase {
        // TODO: Make this a list
        case_id: &'a MatchCase<'a>,
    },
    AllegedlyImpossibleMatchCaseWasNotObviouslyImpossible {
        case_id: &'a MatchCase<'a>,
    },
    CannotInferTypeOfEmptyMatch {
        match_id: &'a Match<'a>,
    },
    AmbiguousMatchCaseOutputType {
        case_id: &'a MatchCase<'a>,
        non_shifted_output_type_id: NormalFormId,
    },
    CannotInferTypeOfTodoExpression(&'a TodoExpression<'a>),
    // TODO: Track explosion sources, so we can give the user a
    // specific line number to mark as `impossible`.
    UnreachableExpression(ExpressionRef<'a>),
    // TODO: Be more strict with this error, since I think it
    // only tracks the rightmost visibility (so a dot chain could
    // still have a middle component that is not visible from
    // the perspective of the `let` statement's transparency).
    // Actually, this might severely complicate things, so maybe
    // we let it slide.
    LetStatementTypeContainsPrivateName {
        let_statement_id: &'a LetStatement<'a>,
        let_statement_type_id: NormalFormId,
        name_id: &'a NameExpression<'a>,
        name_visibility: Visibility,
    },
}

#[derive(Clone, Debug)]
pub enum TypeCheckWarning {
    TypeAssertion(TypeAssertionWarning),
    NormalFormAssertion(NormalFormAssertionWarning),
    TodoExpression(&'a TodoExpression<'a>),
}

#[derive(Clone, Debug)]
pub enum TypeAssertionWarning {
    GoalLhs(&'a CheckAssertion<'a>),
    LhsTypeIsType1(&'a CheckAssertion<'a>),
    CompareeTypeCheckFailure(TypeCheckFailureReason),
    TypesDoNotMatch {
        left_id: ExpressionRef<'a>,
        rewritten_left_type_id: NormalFormId,
        original_and_rewritten_right_ids:
            Result<(ExpressionRef<'a>, NormalFormId), RhsIsQuestionMark>,
    },
}

#[derive(Clone, Copy, Debug)]
pub struct RhsIsQuestionMark;

#[derive(Clone, Debug)]
pub enum NormalFormAssertionWarning {
    NoGoalExists(&'a CheckAssertion<'a>),
    CompareeTypeCheckFailure(TypeCheckFailureReason),
    CompareesDoNotMatch {
        left_id: Result<ExpressionRef<'a>, LhsIsGoalKw>,
        rewritten_left_id: NormalFormId,
        original_and_rewritten_right_ids:
            Result<(ExpressionRef<'a>, NormalFormId), RhsIsQuestionMark>,
    },
}

#[derive(Clone, Copy, Debug)]
pub struct LhsIsGoalKw;

#[derive(Clone, Debug)]
pub enum TypeCheckFailureReason {
    CannotTypeCheck(InvalidExpressionId),
    TypeCheckError(ExpressionRef<'a>, TypeCheckError),
}

#[derive(Debug)]
struct State<'a> {
    file_tree: &'a FileTree,
    substitution_context: &'a mut SubstitutionContext,
    registry: &'a mut NodeRegistry,
    equality_checker: &'a mut NodeEqualityChecker,
    warnings: &'a mut Vec<TypeCheckWarning>,

    required_transparency_for_substitution: Option<Transparency>,

    context: &'a mut Context,
}

impl<'a> State<'_> {
    fn without_context(&'a mut self) -> ContextlessState<'a> {
        self.detach_context().1
    }

    fn detach_context(&'a mut self) -> (&mut Context, ContextlessState<'a>) {
        let contextless = ContextlessState {
            file_tree: self.file_tree,
            substitution_context: self.substitution_context,
            registry: self.registry,
            equality_checker: self.equality_checker,
            warnings: self.warnings,

            required_transparency_for_substitution: self.required_transparency_for_substitution,
        };
        (self.context, contextless)
    }
}

#[derive(Debug)]
struct ContextlessState<'a> {
    file_tree: &'a FileTree,
    substitution_context: &'a mut SubstitutionContext,
    registry: &'a mut NodeRegistry,
    equality_checker: &'a mut NodeEqualityChecker,
    warnings: &'a mut Vec<TypeCheckWarning>,

    required_transparency_for_substitution: Option<Transparency>,
}
