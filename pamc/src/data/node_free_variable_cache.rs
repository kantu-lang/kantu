use crate::data::{
    node_registry::NodeRegistry,
    registered_sst::*,
    symbol_database::{Symbol, SymbolDatabase},
};

#[derive(Clone, Debug)]
pub struct NodeFreeVariableCache {
    name_expression_hashes: Vec<Option<FreeVariableSet>>,
    call_hashes: Vec<Option<FreeVariableSet>>,
    fun_hashes: Vec<Option<FreeVariableSet>>,
    match_hashes: Vec<Option<FreeVariableSet>>,
    forall_hashes: Vec<Option<FreeVariableSet>>,
}

impl NodeFreeVariableCache {
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
        node_id: ExpressionId,
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
        (id1, id2): (ExpressionId, ExpressionId),
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

    fn get_cached_free_variables(&self, node_id: ExpressionId) -> Option<&FreeVariableSet> {
        match node_id {
            ExpressionId::Name(id) => self
                .name_expression_hashes
                .get(id.raw)
                .and_then(Option::as_ref),
            ExpressionId::Call(id) => self.call_hashes.get(id.raw).and_then(Option::as_ref),
            ExpressionId::Fun(id) => self.fun_hashes.get(id.raw).and_then(Option::as_ref),
            ExpressionId::Match(id) => self.match_hashes.get(id.raw).and_then(Option::as_ref),
            ExpressionId::Forall(id) => self.forall_hashes.get(id.raw).and_then(Option::as_ref),
        }
    }

    fn compute_free_variables_and_cache(
        &mut self,
        node_id: ExpressionId,
        node_info: (&NodeRegistry, &SymbolDatabase),
    ) -> &FreeVariableSet {
        let free_variables = self.compute_free_variables(node_id, node_info);
        self.set_free_variables(node_id, free_variables)
    }

    fn compute_free_variables(
        &mut self,
        _node_id: ExpressionId,
        _node_info: (&NodeRegistry, &SymbolDatabase),
    ) -> FreeVariableSet {
        unimplemented!()
    }

    fn set_free_variables(
        &mut self,
        node_id: ExpressionId,
        free_variables: FreeVariableSet,
    ) -> &FreeVariableSet {
        let (index, vec) = match node_id {
            ExpressionId::Name(id) => (id.raw, &mut self.name_expression_hashes),
            ExpressionId::Call(id) => (id.raw, &mut self.call_hashes),
            ExpressionId::Fun(id) => (id.raw, &mut self.fun_hashes),
            ExpressionId::Match(id) => (id.raw, &mut self.match_hashes),
            ExpressionId::Forall(id) => (id.raw, &mut self.forall_hashes),
        };

        if index >= vec.len() {
            vec.resize_with(index + 1, || None);
        }
        vec[index] = Some(free_variables);
        vec[index]
            .as_ref()
            .expect("Value should be Some(_), since we just set it to that.")
    }
}

impl FreeVariableSet {
    pub fn contains(&self, symbol: Symbol) -> bool {
        self.raw.contains(&symbol)
    }
}
