use crate::data::{
    fun_recursion_validation_result::FunRecursionValidated,
    light_ast::*,
    node_equality_checker::NodeEqualityChecker,
    node_registry::{NodeId, NodeRegistry, NonEmptyListId},
    non_empty_vec::{NonEmptyVec, OptionalNonEmptyVecLen},
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

pub use type_check_node::type_check_files;
use type_check_node::*;
mod type_check_node;

mod without_spans;
use without_spans::*;

#[derive(Clone, Debug)]
pub enum TypeCheckError {
    IllegalTypeExpression(ExpressionId),
    IllegalCallee(ExpressionId),
    WrongNumberOfArguments {
        call_id: NodeId<Call>,
        expected: usize,
        actual: usize,
    },
    WrongNumberOfCaseParams {
        case_id: NodeId<MatchCase>,
        expected: usize,
        actual: usize,
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
    MissingMatchCase {
        variant_name_id: NodeId<Identifier>,
    },
    ExtraneousMatchCase {
        case_id: NodeId<MatchCase>,
    },
    AmbiguousOutputType {
        case_id: NodeId<MatchCase>,
    },
}

#[derive(Clone, Debug)]
pub enum TypeCheckWarning {
    TypeAssertion(TypeAssertionWarning),
    NormalFormAssertion(NormalFormAssertionWarning),
}

#[derive(Clone, Debug)]
pub enum TypeAssertionWarning {
    GoalLhs(NodeId<CheckAssertion>),
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
    context: &'a mut Context,
    substitution_context: &'a mut SubstitutionContext,
    registry: &'a mut NodeRegistry,
    equality_checker: &'a mut NodeEqualityChecker,
    warnings: &'a mut Vec<TypeCheckWarning>,
}

impl<'a> State<'_> {
    fn without_context(&'a mut self) -> ContextlessState<'a> {
        ContextlessState {
            substitution_context: self.substitution_context,
            registry: self.registry,
            equality_checker: self.equality_checker,
            warnings: self.warnings,
        }
    }
}

// TODO: Delete

#[derive(Debug)]
struct ContextlessState<'a> {
    substitution_context: &'a mut SubstitutionContext,
    registry: &'a mut NodeRegistry,
    equality_checker: &'a mut NodeEqualityChecker,
    warnings: &'a mut Vec<TypeCheckWarning>,
}
