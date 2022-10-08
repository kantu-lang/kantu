use crate::data::{
    node_registry::{NodeId, NodeRegistry},
    registered_ast::*,
    symbol_database::{Symbol, SymbolDatabase},
};

#[derive(Clone, Debug)]
pub struct FreeVariableCache {
    wrapped_expression_hashes: Vec<Option<FreeVariableSet>>,
}

impl FreeVariableCache {
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

impl FreeVariableCache {
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

    fn get_cached_free_variables(
        &mut self,
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
