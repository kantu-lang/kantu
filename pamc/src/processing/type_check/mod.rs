use crate::data::{
    fun_recursion_validation_result::FunRecursionValidated,
    light_ast::*,
    node_equality_checker::NodeEqualityChecker,
    node_registry::{ListId, NodeId, NodeRegistry},
    TextSpan,
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
    NoGoal {
        goal_kw_start: TextSpan,
    },
    // TODO: Rename "missing" to "incomplete"?
    MissingCheckeeType {
        question_mark_start: TextSpan,
    },
    UntypecheckableExpression(InvalidExpressionId),
    IllTypedCheckeeType(ExpressionId, TypeCheckError),
    IncorrectCheckeeType {
        checkee_type_id: ExpressionId,
        expected_id: NormalFormId,
        actual_id: NormalFormId,
    },
    MissingCheckeeValue {
        question_mark_start: TextSpan,
    },
    IllTypedCheckeeValue(ExpressionId, TypeCheckError),
    IncorrectCheckeeValue {
        checkee_type_id: ExpressionId,
        expected_id: NormalFormId,
        actual_id: NormalFormId,
    },
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
