use crate::data::{
    node_registry::{ListId, NodeId, NodeRegistry},
    registered_ast::*,
    symbol_database::{Symbol, SymbolDatabase, SymbolSource},
    type_map::{NormalFormNodeId, TypeMap},
    variant_return_type::{VariantReturnType, VariantReturnTypeDatabase},
};

#[derive(Clone, Debug)]
pub enum TypeError {
    IllegalParamType {
        param_id: NodeId<Param>,
        type_type_id: NormalFormNodeId,
    },
    CalleeNotAFunction {
        callee_id: NodeId<WrappedExpression>,
        callee_type_id: NormalFormNodeId,
    },
    WrongNumberOfArguments {
        call_id: NodeId<Call>,
        param_arity: usize,
        arg_arity: usize,
    },
    WrongArgumentType {
        arg_id: NodeId<WrappedExpression>,
        param_type_id: NormalFormNodeId,
        arg_type_id: NormalFormNodeId,
    },
    IllegalReturnType {
        fun_id: NodeId<Fun>,
        return_type_type_id: NormalFormNodeId,
    },
    WrongBodyType {
        fun_id: NodeId<Fun>,
        normalized_return_type_id: NormalFormNodeId,
        body_type_id: NormalFormNodeId,
    },
    GoalMismatch {
        goal_id: NormalFormNodeId,
        actual_type_id: NormalFormNodeId,
    },
    IllegalMatcheeType {
        match_id: NodeId<Match>,
        matchee_type_id: NormalFormNodeId,
    },
    UnrecognizedVariant {
        adt_callee_id: NodeId<Identifier>,
        variant_name_id: NodeId<Identifier>,
    },
    DuplicateMatchCases {
        match_id: NodeId<Match>,
        first_case_id: NodeId<MatchCase>,
        second_case_id: NodeId<MatchCase>,
    },
    InconsistentMatchCases {
        match_id: NodeId<Match>,
        first_case_output_type_id: NormalFormNodeId,
        second_case_output_type_id: NormalFormNodeId,
    },
    UncoveredMatchCase {
        match_id: NodeId<Match>,
        uncovered_case: IdentifierName,
    },
    WrongNumberOfMatchCaseParams {
        case_id: NodeId<MatchCase>,
        variant_id: NodeId<Variant>,
        expected_arity: usize,
        actual_arity: usize,
    },
    IllegalForallOutputType {
        forall_id: NodeId<Forall>,
        output_type_id: NormalFormNodeId,
    },
}

pub use type_check_node::type_check_file;
mod type_check_node;

use context::*;
mod context;

use eval::*;
mod eval;

use fusion::*;
mod fusion;

use misc::*;
mod misc;

// TODO: Maybe context should be separate, since
// I feel like I'm passing `registry` and `symbol_db`
// a lot (since `context` is borrowed in such circumstances).
#[derive(Debug)]
struct TypeCheckState<'a> {
    registry: &'a mut NodeRegistry,
    symbol_db: &'a mut SymbolDatabase,
    variant_db: &'a VariantReturnTypeDatabase,
    context: TypeCheckContext,
    type0_identifier_id: NormalFormNodeId,
}
