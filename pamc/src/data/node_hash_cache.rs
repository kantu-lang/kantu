use crate::data::{
    node_registry::{NodeId, NodeRegistry},
    registered_sst::*,
    symbol_database::{Symbol, SymbolDatabase},
};

// TODO: There's too much logic in this file.
// We should just make caches generic and implement
// the hashing in a separate file (as a submodule of `processing`).

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
    name_expression_hashes: Vec<Option<u64>>,
    call_hashes: Vec<Option<u64>>,
    fun_hashes: Vec<Option<u64>>,
    match_hashes: Vec<Option<u64>>,
    forall_hashes: Vec<Option<u64>>,
}

impl NodeStructuralIdentityHashCache {
    pub fn empty() -> Self {
        Self {
            name_expression_hashes: Vec::new(),
            call_hashes: Vec::new(),
            fun_hashes: Vec::new(),
            match_hashes: Vec::new(),
            forall_hashes: Vec::new(),
        }
    }
}

impl NodeStructuralIdentityHashCache {
    pub fn get_structural_identity_hash(
        &mut self,
        node_id: ExpressionId,
        node_info: (&NodeRegistry, &SymbolDatabase),
    ) -> u64 {
        if let Some(cached) = self.get_cached_structural_identity_hash(node_id) {
            cached
        } else {
            self.compute_hash_and_cache(node_id, node_info)
        }
    }

    fn get_cached_structural_identity_hash(&mut self, node_id: ExpressionId) -> Option<u64> {
        match node_id {
            ExpressionId::Name(id) => self.name_expression_hashes[id.raw],
            ExpressionId::Call(id) => self.call_hashes[id.raw],
            ExpressionId::Fun(id) => self.fun_hashes[id.raw],
            ExpressionId::Match(id) => self.match_hashes[id.raw],
            ExpressionId::Forall(id) => self.forall_hashes[id.raw],
        }
    }

    fn compute_hash_and_cache(
        &mut self,
        node_id: ExpressionId,
        node_info: (&NodeRegistry, &SymbolDatabase),
    ) -> u64 {
        let hash = self.compute_wrapped_expression_hash(node_id, node_info);
        self.set_wrapped_expression_hash(node_id, hash);
        hash
    }

    fn compute_wrapped_expression_hash(
        &mut self,
        node_id: ExpressionId,
        node_info: (&NodeRegistry, &SymbolDatabase),
    ) -> u64 {
        match node_id {
            ExpressionId::Name(id) => self.compute_name_expression_hash(id, node_info),
            ExpressionId::Call(id) => self.compute_call_hash(id, node_info),
            ExpressionId::Fun(id) => self.compute_fun_hash(id, node_info),
            ExpressionId::Match(id) => self.compute_match_hash(id, node_info),
            ExpressionId::Forall(id) => self.compute_forall_hash(id, node_info),
        }
    }

    fn compute_name_expression_hash(
        &mut self,
        node_id: NodeId<NameExpression>,
        node_info: (&NodeRegistry, &SymbolDatabase),
    ) -> u64 {
        let (registry, symbol_db) = node_info;
        let name = registry.name_expression(node_id);
        let component_ids = registry.identifier_list(name.component_list_id);

        let mut hasher = FxHasher::default();
        for component_id in component_ids {
            let identifier = registry.identifier(*component_id);
            let symbol = symbol_db.identifier_symbols.get(identifier.id);
            hash_symbol(symbol, &mut hasher);
        }
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
        let arg_ids = registry.expression_list(call.arg_list_id);

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

    fn set_wrapped_expression_hash(&mut self, node_id: ExpressionId, hash: u64) {
        let (index, vec) = match node_id {
            ExpressionId::Name(id) => (id.raw, &mut self.name_expression_hashes),
            ExpressionId::Call(id) => (id.raw, &mut self.call_hashes),
            ExpressionId::Fun(id) => (id.raw, &mut self.fun_hashes),
            ExpressionId::Match(id) => (id.raw, &mut self.match_hashes),
            ExpressionId::Forall(id) => (id.raw, &mut self.forall_hashes),
        };
        let min_len = index + 1;
        if vec.len() < min_len {
            vec.resize(min_len, None);
        }
        vec[index] = Some(hash);
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
