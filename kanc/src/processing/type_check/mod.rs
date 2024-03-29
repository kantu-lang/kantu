use crate::data::{
    file_tree::FileTree,
    light_ast::*,
    node_equality_checker::NodeEqualityChecker,
    node_registry::{LabeledCallArgId, NodeId, NodeRegistry, NonEmptyListId},
    non_empty_vec::{NonEmptySlice, NonEmptyVec, OptionalNonEmptyVecLen},
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
        expression_id: ExpressionId,
        non_type0_or_type1_type_id: NormalFormId,
    },
    IllegalCallee {
        callee_id: ExpressionId,
        callee_type_id: NormalFormId,
    },
    WrongNumberOfArguments {
        call_id: NodeId<Call>,
        expected: usize,
        actual: usize,
    },
    CallLabelednessMismatch {
        call_id: NodeId<Call>,
    },
    MissingLabeledCallArgs {
        call_id: NodeId<Call>,
        missing_label_list_id: NonEmptyListId<NodeId<Identifier>>,
    },
    ExtraneousLabeledCallArg {
        call_id: NodeId<Call>,
        // TODO: Make this a list
        arg_id: LabeledCallArgId,
    },
    WrongNumberOfMatchCaseParams {
        case_id: NodeId<MatchCase>,
        expected: usize,
        actual: usize,
    },
    MatchCaseLabelednessMismatch {
        case_id: NodeId<MatchCase>,
        param_list_id: NonEmptyMatchCaseParamListId,
    },
    MissingLabeledMatchCaseParams {
        case_id: NodeId<MatchCase>,
        missing_label_list_id: NonEmptyListId<NodeId<Identifier>>,
    },
    UndefinedLabeledMatchCaseParams {
        case_id: NodeId<MatchCase>,
        case_param_list_id: NonEmptyListId<NodeId<LabeledMatchCaseParam>>,
    },
    TypeMismatch {
        expression_id: ExpressionId,
        expected_type_id: NormalFormId,
        actual_type_id: NormalFormId,
    },
    NonAdtMatchee {
        matchee_id: ExpressionId,
        type_id: NormalFormId,
    },
    DuplicateMatchCase {
        existing_match_case_id: NodeId<MatchCase>,
        new_match_case_id: NodeId<MatchCase>,
    },
    MissingMatchCases {
        match_id: NodeId<Match>,
        missing_variant_name_list_id: NonEmptyListId<NodeId<Identifier>>,
    },
    ExtraneousMatchCase {
        // TODO: Make this a list
        case_id: NodeId<MatchCase>,
    },
    AllegedlyImpossibleMatchCaseWasNotObviouslyImpossible {
        case_id: NodeId<MatchCase>,
    },
    CannotInferTypeOfEmptyMatch {
        match_id: NodeId<Match>,
    },
    AmbiguousMatchCaseOutputType {
        case_id: NodeId<MatchCase>,
        non_shifted_output_type_id: NormalFormId,
    },
    CannotInferTypeOfTodoExpression(NodeId<TodoExpression>),
    // TODO: Track explosion sources, so we can give the user a
    // specific line number to mark as `impossible`.
    UnreachableExpression(ExpressionId),
    // TODO: Be more strict with this error, since I think it
    // only tracks the rightmost visibility (so a dot chain could
    // still have a middle component that is not visible from
    // the perspective of the `let` statement's transparency).
    // Actually, this might severely complicate things, so maybe
    // we let it slide.
    LetStatementTypeContainsPrivateName {
        let_statement_id: NodeId<LetStatement>,
        let_statement_type_id: NormalFormId,
        name_id: NodeId<NameExpression>,
        name_visibility: Visibility,
    },
}

#[derive(Clone, Debug)]
pub enum TypeCheckWarning {
    TypeAssertion(TypeAssertionWarning),
    NormalFormAssertion(NormalFormAssertionWarning),
    TodoExpression(NodeId<TodoExpression>),
}

#[derive(Clone, Debug)]
pub enum TypeAssertionWarning {
    GoalLhs(NodeId<CheckAssertion>),
    LhsTypeIsType1(NodeId<CheckAssertion>),
    CompareeTypeCheckFailure(TypeCheckFailureReason),
    TypesDoNotMatch {
        left_id: ExpressionId,
        rewritten_left_type_id: NormalFormId,
        original_and_rewritten_right_ids: Result<(ExpressionId, NormalFormId), RhsIsQuestionMark>,
    },
}

#[derive(Clone, Copy, Debug)]
pub struct RhsIsQuestionMark;

#[derive(Clone, Debug)]
pub enum NormalFormAssertionWarning {
    NoGoalExists(NodeId<CheckAssertion>),
    CompareeTypeCheckFailure(TypeCheckFailureReason),
    CompareesDoNotMatch {
        left_id: Result<ExpressionId, LhsIsGoalKw>,
        rewritten_left_id: NormalFormId,
        original_and_rewritten_right_ids: Result<(ExpressionId, NormalFormId), RhsIsQuestionMark>,
    },
}

#[derive(Clone, Copy, Debug)]
pub struct LhsIsGoalKw;

#[derive(Clone, Debug)]
pub enum TypeCheckFailureReason {
    CannotTypeCheck(InvalidExpressionId),
    TypeCheckError(ExpressionId, TypeCheckError),
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
