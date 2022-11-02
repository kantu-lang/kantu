use crate::data::{
    x_light_ast::*,
    x_node_registry::{ListId, NodeId, NodeRegistry},
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
    BadCallee(ExpressionId),
    WrongNumberOfArguments {
        call_id: NodeId<Call>,
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
