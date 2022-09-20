use crate::data::{node_registry::NodeId, registered_ast::*};
use rustc_hash::FxHashMap;

#[derive(Clone, Debug)]
pub struct TypeMap {
    raw: FxHashMap<NodeId<WrappedExpression>, NodeId<WrappedExpression>>,
}

impl TypeMap {
    pub fn empty() -> Self {
        Self {
            raw: FxHashMap::default(),
        }
    }
}

impl TypeMap {
    pub fn insert_new_or_panic(
        &mut self,
        term_id: NodeId<WrappedExpression>,
        type_id: NodeId<WrappedExpression>,
    ) {
        if let Some(existing_type_id) = self.raw.get(&term_id) {
            panic!("Tried to insert new entry ({:?}, {:?}) into a type map, when it already contained the entry ({:?}, {:?}).", term_id, type_id, term_id, existing_type_id);
        }
        self.raw.insert(term_id, type_id);
    }

    pub fn get_or_panic(&self, term_id: NodeId<WrappedExpression>) -> NodeId<WrappedExpression> {
        if let Some(type_id) = self.raw.get(&term_id) {
            *type_id
        } else {
            panic!(
                "Tried to get the type of {:?}, but it was not in the type map.",
                term_id
            );
        }
    }

    pub fn get(&self, term_id: NodeId<WrappedExpression>) -> Option<NodeId<WrappedExpression>> {
        self.raw.get(&term_id).copied()
    }
}
