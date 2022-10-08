use crate::data::{
    node_registry::{NodeId, NodeRegistry},
    registered_ast::*,
    symbol_database::{Symbol, SymbolDatabase},
};

use rustc_hash::FxHasher;

// Do **NOT** write `use std::hash::Hash` (however, `use std::hash::Hasher` is okay).
// To see why, please read the comment attached to the
// `hash_explicit_type` submodule.
use std::hash::Hasher;

/// To speed up equality checks, we assign each node a "structural identity hash" (SIH).
/// This hash accounts for symbols, but ignores `id` and its position in the AST.
/// In this way, two nodes will have the same SIH if and only if they are "structurally equal".
///
/// To avoid recomputing the SIH of a node every time we need it, we cache the SIH of each node
/// using this struct.
#[derive(Clone, Debug)]
pub struct NodeStructuralIdentityHashCache {
    wrapped_expression_hashes: Vec<Option<u64>>,
}

impl NodeStructuralIdentityHashCache {
    pub fn empty() -> Self {
        Self {
            wrapped_expression_hashes: Vec::new(),
        }
    }
}

impl NodeStructuralIdentityHashCache {
    pub fn get_structural_identity_hash(
        &mut self,
        node_id: NodeId<WrappedExpression>,
        node_info: (&NodeRegistry, &SymbolDatabase),
    ) -> u64 {
        if let Some(cached) = self.get_cached_structural_identity_hash(node_id) {
            cached
        } else {
            self.compute_hash_and_cache(node_id, node_info)
        }
    }

    fn get_cached_structural_identity_hash(
        &mut self,
        node_id: NodeId<WrappedExpression>,
    ) -> Option<u64> {
        self.wrapped_expression_hashes
            .get(node_id.raw)
            .copied()
            .flatten()
    }

    fn compute_hash_and_cache(
        &mut self,
        node_id: NodeId<WrappedExpression>,
        node_info: (&NodeRegistry, &SymbolDatabase),
    ) -> u64 {
        let hash = self.compute_wrapped_expression_hash(node_id, node_info);
        self.set_wrapped_expression_hash(node_id, hash);
        hash
    }

    fn compute_wrapped_expression_hash(
        &mut self,
        node_id: NodeId<WrappedExpression>,
        node_info: (&NodeRegistry, &SymbolDatabase),
    ) -> u64 {
        let (registry, _) = node_info;
        let node = registry.wrapped_expression(node_id);
        match &node.expression {
            Expression::Identifier(identifier) => {
                self.compute_identifier_hash(identifier.id, node_info)
            }
            Expression::Dot(dot) => self.compute_dot_hash(dot.id, node_info),
            Expression::Call(call) => self.compute_call_hash(call.id, node_info),
            Expression::Fun(fun) => self.compute_fun_hash(fun.id, node_info),
            Expression::Match(match_) => self.compute_match_hash(match_.id, node_info),
            Expression::Forall(forall) => self.compute_forall_hash(forall.id, node_info),
        }
    }

    fn compute_identifier_hash(
        &mut self,
        node_id: NodeId<Identifier>,
        node_info: (&NodeRegistry, &SymbolDatabase),
    ) -> u64 {
        let (registry, symbol_db) = node_info;
        let identifier = registry.identifier(node_id);
        let symbol = symbol_db.identifier_symbols.get(identifier.id);

        let mut hasher = FxHasher::default();
        hash_symbol(symbol, &mut hasher);
        hasher.finish()
    }

    fn compute_dot_hash(
        &mut self,
        node_id: NodeId<Dot>,
        node_info: (&NodeRegistry, &SymbolDatabase),
    ) -> u64 {
        let (registry, symbol_db) = node_info;
        let dot = registry.dot(node_id);
        let left_hash = self.get_structural_identity_hash(dot.left_id, node_info);
        let right_symbol = symbol_db.identifier_symbols.get(dot.right_id);

        let mut hasher = FxHasher::default();
        hash_u64(left_hash, &mut hasher);
        hash_symbol(right_symbol, &mut hasher);
        hasher.finish()
    }

    fn compute_call_hash(
        &mut self,
        node_id: NodeId<Call>,
        node_info: (&NodeRegistry, &SymbolDatabase),
    ) -> u64 {
        let (registry, _) = node_info;
        let call = registry.call(node_id);
        let callee_hash = self.get_structural_identity_hash(call.callee_id, node_info);
        let arg_ids = registry.wrapped_expression_list(call.arg_list_id);

        let mut hasher = FxHasher::default();
        hash_u64(callee_hash, &mut hasher);
        for arg_id in arg_ids {
            let arg_hash = self.get_structural_identity_hash(*arg_id, node_info);
            hash_u64(arg_hash, &mut hasher);
        }
        hasher.finish()
    }

    fn compute_fun_hash(
        &mut self,
        node_id: NodeId<Fun>,
        node_info: (&NodeRegistry, &SymbolDatabase),
    ) -> u64 {
        let (registry, symbol_db) = node_info;
        let fun = registry.fun(node_id);
        let param_ids = registry.param_list(fun.param_list_id);
        let body_hash = self.get_structural_identity_hash(fun.body_id, node_info);

        let mut hasher = FxHasher::default();
        for param_id in param_ids {
            let param = registry.param(*param_id);
            let param_symbol = symbol_db.identifier_symbols.get(param.name_id);
            hash_symbol(param_symbol, &mut hasher);
        }
        hash_u64(body_hash, &mut hasher);
        hasher.finish()
    }

    fn compute_match_hash(
        &mut self,
        node_id: NodeId<Match>,
        node_info: (&NodeRegistry, &SymbolDatabase),
    ) -> u64 {
        let (registry, _) = node_info;
        let match_ = registry.match_(node_id);
        let matchee_hash = self.get_structural_identity_hash(match_.matchee_id, node_info);
        let case_ids = registry.match_case_list(match_.case_list_id);

        let mut hasher = FxHasher::default();
        hash_u64(matchee_hash, &mut hasher);
        for case_id in case_ids {
            let case_hash = self.compute_match_case_hash(*case_id, node_info);
            hash_u64(case_hash, &mut hasher);
        }
        hasher.finish()
    }

    fn compute_match_case_hash(
        &mut self,
        node_id: NodeId<MatchCase>,
        node_info: (&NodeRegistry, &SymbolDatabase),
    ) -> u64 {
        let (registry, symbol_db) = node_info;
        let case = registry.match_case(node_id);
        let variant_name: &IdentifierName = &registry.identifier(case.variant_name_id).name;
        let param_ids = registry.identifier_list(case.param_list_id);
        let output_hash = self.get_structural_identity_hash(case.output_id, node_info);

        let mut hasher = FxHasher::default();
        hash_identifier_name(variant_name, &mut hasher);
        for param_id in param_ids {
            let param_symbol = symbol_db.identifier_symbols.get(*param_id);
            hash_symbol(param_symbol, &mut hasher);
        }
        hash_u64(output_hash, &mut hasher);
        hasher.finish()
    }

    fn compute_forall_hash(
        &mut self,
        node_id: NodeId<Forall>,
        node_info: (&NodeRegistry, &SymbolDatabase),
    ) -> u64 {
        let (registry, symbol_db) = node_info;
        let forall = registry.forall(node_id);
        let param_ids = registry.param_list(forall.param_list_id);
        let output_hash = self.get_structural_identity_hash(forall.output_id, node_info);

        let mut hasher = FxHasher::default();
        for param_id in param_ids {
            let param = registry.param(*param_id);
            let param_symbol = symbol_db.identifier_symbols.get(param.name_id);
            hash_symbol(param_symbol, &mut hasher);
        }
        hash_u64(output_hash, &mut hasher);
        hasher.finish()
    }

    fn set_wrapped_expression_hash(&mut self, node_id: NodeId<WrappedExpression>, hash: u64) {
        let min_len = node_id.raw + 1;
        if self.wrapped_expression_hashes.len() < min_len {
            self.wrapped_expression_hashes.resize(min_len, None);
        }
        self.wrapped_expression_hashes[node_id.raw] = Some(hash);
    }
}

/// We use these trivial helpers functions instead
/// of simply writing `n.hash(&mut hasher)` because
/// we don't want to accidentally call `Hash::hash` on
/// the wrong type (e.g., `NodeId<_>` or some node type).
/// Currently, the nodes don't implement `Hash`, so this
/// would cause a compile error.
/// However, in case they do in the future, we want to
/// make sure that we don't accidentally call `Hash::hash`.
/// By requiring explicit calls to `hash_SO_AND_SO`, we ensure
/// we are hashing the intended type.
mod hash_explicit_type {
    use super::*;

    use std::hash::Hash;

    pub fn hash_u64(n: u64, hasher: &mut impl Hasher) {
        n.hash(hasher);
    }

    pub fn hash_symbol(n: Symbol, hasher: &mut impl Hasher) {
        n.hash(hasher);
    }

    pub fn hash_identifier_name(name: &IdentifierName, hasher: &mut impl Hasher) {
        name.hash(hasher);
    }
}
use hash_explicit_type::*;
