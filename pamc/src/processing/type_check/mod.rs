use crate::data::{
    light_ast::*,
    node_equality_checker::NodeEqualityChecker,
    node_registry::{ListId, NodeId, NodeRegistry},
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

#[derive(Debug)]
struct State<'a> {
    context: &'a mut Context,
    registry: &'a mut NodeRegistry,
    equality_checker: &'a mut NodeEqualityChecker,
}

impl<'a> State<'_> {
    fn without_context(&'a mut self) -> ContextlessState<'a> {
        ContextlessState {
            registry: self.registry,
            equality_checker: self.equality_checker,
        }
    }
}

#[derive(Debug)]
struct ContextlessState<'a> {
    registry: &'a mut NodeRegistry,
    equality_checker: &'a mut NodeEqualityChecker,
}
