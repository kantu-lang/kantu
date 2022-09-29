use crate::data::{
    node_registry::{ListId, NodeId},
    registered_ast::*,
};

// TODO: Delete this.
// check_variant_return_types_for_file should probably
// return Result<(), ...> instead.
#[derive(Clone, Debug)]
pub struct VariantReturnTypeDatabase {
    map: Vec<Option<VariantReturnType>>,
}

#[derive(Clone, Debug)]
pub enum VariantReturnType {
    Identifier {
        identifier_id: NodeId<WrappedExpression>,
    },
    Call {
        callee_id: NodeId<WrappedExpression>,
        arg_list_id: ListId<NodeId<WrappedExpression>>,
    },
}

impl VariantReturnTypeDatabase {
    pub fn empty() -> Self {
        VariantReturnTypeDatabase { map: Vec::new() }
    }
}

impl VariantReturnTypeDatabase {
    /// Panics if no entry is found for the provided variant.
    pub fn get(&self, variant: NodeId<Variant>) -> VariantReturnType {
        self.try_get(variant)
            .expect(&format!("Return type could not be found for {:?}", variant))
    }

    fn try_get(&self, variant: NodeId<Variant>) -> Option<VariantReturnType> {
        if variant.raw >= self.map.len() {
            None
        } else {
            self.map[variant.raw]
        }
    }

    /// Panics if an entry already exists for the provided variant.
    pub fn insert_new(&mut self, variant: NodeId<Variant>, return_type: VariantReturnType) {
        if self.contains(variant) {
            panic!("Return type already exists for variant {:?}", variant);
        }
        self.insert(variant, return_type);
    }

    fn contains(&self, variant: NodeId<Variant>) -> bool {
        self.try_get(variant).is_some()
    }

    fn insert(&mut self, variant: NodeId<Variant>, return_type: VariantReturnType) {
        let min_len = variant.raw + 1;
        if self.map.len() < min_len {
            self.map.resize(min_len, None);
        }
        self.map[variant.raw] = Some(return_type);
    }
}
