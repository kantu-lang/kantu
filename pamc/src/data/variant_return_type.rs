use crate::data::{node_registry::NodeId, registered_ast::*};

#[derive(Clone, Debug)]
pub struct VariantReturnTypeTypeArgsMap {
    map: Vec<Option<Vec<NodeId<WrappedExpression>>>>,
}

impl VariantReturnTypeTypeArgsMap {
    pub fn empty() -> Self {
        VariantReturnTypeTypeArgsMap { map: Vec::new() }
    }
}

impl VariantReturnTypeTypeArgsMap {
    /// Panics if no entry is found for the provided variant.
    pub fn get(&self, variant: NodeId<Variant>) -> &[NodeId<WrappedExpression>] {
        self.try_get(variant)
            .expect(&format!("Type args could not be found for {:?}", variant))
    }

    fn try_get(&self, variant: NodeId<Variant>) -> Option<&[NodeId<WrappedExpression>]> {
        if variant.raw >= self.map.len() {
            None
        } else {
            self.map[variant.raw].as_ref().map(Vec::as_slice)
        }
    }

    /// Panics if an entry already exists for the provided variant.
    pub fn insert_new(
        &mut self,
        variant: NodeId<Variant>,
        type_args: Vec<NodeId<WrappedExpression>>,
    ) {
        if self.contains(variant) {
            panic!("Type args already exist for {:?}", variant);
        }
        self.insert(variant, type_args);
    }

    fn contains(&self, variant: NodeId<Variant>) -> bool {
        self.try_get(variant).is_some()
    }

    fn insert(&mut self, variant: NodeId<Variant>, type_args: Vec<NodeId<WrappedExpression>>) {
        let min_len = variant.raw + 1;
        if self.map.len() < min_len {
            self.map.resize(min_len, None);
        }
        self.map[variant.raw] = Some(type_args);
    }
}
