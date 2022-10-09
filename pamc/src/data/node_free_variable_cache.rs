use crate::data::{
    node_registry::{NodeId, NodeRegistry},
    registered_ast::*,
    symbol_database::{Symbol, SymbolDatabase},
};

#[derive(Clone, Debug)]
pub struct NodeFreeVariableCache {
    wrapped_expression_hashes: Vec<Option<FreeVariableSet>>,
}

impl NodeFreeVariableCache {
    pub fn empty() -> Self {
        Self {
            wrapped_expression_hashes: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FreeVariableSet {
    raw: Vec<Symbol>,
}

impl FreeVariableSet {
    pub fn empty() -> Self {
        Self { raw: Vec::new() }
    }
}

impl NodeFreeVariableCache {
    pub fn get_free_variables(
        &mut self,
        node_id: NodeId<WrappedExpression>,
        node_info: (&NodeRegistry, &SymbolDatabase),
    ) -> &FreeVariableSet {
        // The obvious implementation is:
        //
        // ```rust
        // if let Some(cached) = self.get_cached_free_variables(node_id) {
        //     return cached;
        // } else {
        //     self.compute_free_variables_and_cache(node_id, node_info)
        // }
        // ```
        //
        // But currently, the borrow checker is too strict to allow this.
        // So we have to do this instead:

        if self.get_cached_free_variables(node_id).is_none() {
            self.compute_free_variables_and_cache(node_id, node_info);
        }

        self.get_cached_free_variables(node_id)
            .expect("We just cached this, so it should be Some(_).")
    }

    pub fn get_free_variables_2(
        &mut self,
        (id1, id2): (NodeId<WrappedExpression>, NodeId<WrappedExpression>),
        node_info: (&NodeRegistry, &SymbolDatabase),
    ) -> (&FreeVariableSet, &FreeVariableSet) {
        if self.get_cached_free_variables(id1).is_none() {
            self.compute_free_variables_and_cache(id1, node_info);
        }

        if self.get_cached_free_variables(id2).is_none() {
            self.compute_free_variables_and_cache(id2, node_info);
        }

        let set1 = self
            .get_cached_free_variables(id1)
            .expect("We just cached this, so it should be Some(_).");
        let set2 = self
            .get_cached_free_variables(id2)
            .expect("We just cached this, so it should be Some(_).");
        (set1, set2)
    }

    fn get_cached_free_variables(
        &self,
        node_id: NodeId<WrappedExpression>,
    ) -> Option<&FreeVariableSet> {
        self.wrapped_expression_hashes
            .get(node_id.raw)
            .map(Option::as_ref)
            .flatten()
    }

    fn compute_free_variables_and_cache(
        &mut self,
        node_id: NodeId<WrappedExpression>,
        node_info: (&NodeRegistry, &SymbolDatabase),
    ) -> &FreeVariableSet {
        let free_variables = self.compute_free_variables(node_id, node_info);
        self.set_free_variables(node_id, free_variables)
    }

    fn compute_free_variables(
        &mut self,
        node_id: NodeId<WrappedExpression>,
        node_info: (&NodeRegistry, &SymbolDatabase),
    ) -> FreeVariableSet {
        unimplemented!()
    }

    fn set_free_variables(
        &mut self,
        node_id: NodeId<WrappedExpression>,
        free_variables: FreeVariableSet,
    ) -> &FreeVariableSet {
        let node_id = node_id.raw;
        if node_id >= self.wrapped_expression_hashes.len() {
            self.wrapped_expression_hashes
                .resize_with(node_id + 1, || None);
        }
        self.wrapped_expression_hashes[node_id] = Some(free_variables);
        self.wrapped_expression_hashes[node_id]
            .as_ref()
            .expect("Value should be Some(_), since we just set it to that.")
    }
}

impl FreeVariableSet {
    pub fn contains(&self, symbol: Symbol) -> bool {
        self.raw.contains(&symbol)
    }
}
