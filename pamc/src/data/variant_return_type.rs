use crate::data::{
    node_registry::{ListId, NodeId},
    registered_ast::*,
};

#[derive(Clone, Debug)]
pub struct VariantReturnTypeTypeArgsMap {
    map: Vec<Option<ListId<NodeId<WrappedExpression>>>>,
}

impl VariantReturnTypeTypeArgsMap {
    pub fn empty() -> Self {
        VariantReturnTypeTypeArgsMap { map: Vec::new() }
    }
}

impl VariantReturnTypeTypeArgsMap {
    /// Panics if no entry is found for the provided variant.
    pub fn get(&self, variant: NodeId<Variant>) -> ListId<NodeId<WrappedExpression>> {
        self.try_get(variant)
            .expect(&format!("Type args could not be found for {:?}", variant))
    }

    fn try_get(&self, variant: NodeId<Variant>) -> Option<ListId<NodeId<WrappedExpression>>> {
        if variant.raw >= self.map.len() {
            None
        } else {
            self.map[variant.raw]
        }
    }

    /// Panics if an entry already exists for the provided variant.
    pub fn insert_new(
        &mut self,
        variant: NodeId<Variant>,
        type_arg_list_id: ListId<NodeId<WrappedExpression>>,
    ) {
        if self.contains(variant) {
            panic!("Type arg list id already exist for {:?}", variant);
        }
        self.insert(variant, type_arg_list_id);
    }

    fn contains(&self, variant: NodeId<Variant>) -> bool {
        self.try_get(variant).is_some()
    }

    fn insert(
        &mut self,
        variant: NodeId<Variant>,
        type_arg_list_id: ListId<NodeId<WrappedExpression>>,
    ) {
        let min_len = variant.raw + 1;
        if self.map.len() < min_len {
            self.map.resize(min_len, None);
        }
        self.map[variant.raw] = Some(type_arg_list_id);
    }
}
