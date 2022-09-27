use crate::data::{node_registry::NodeId, registered_ast::*, symbol_database::Symbol};
use rustc_hash::FxHashMap;

#[derive(Clone, Copy, Debug)]
pub struct NormalFormNodeId(pub NodeId<WrappedExpression>);

#[derive(Clone, Debug)]
pub struct TypeMap {
    raw: FxHashMap<Symbol, NormalFormNodeId>,
}

impl TypeMap {
    pub fn empty() -> Self {
        Self {
            raw: FxHashMap::default(),
        }
    }
}

impl TypeMap {
    pub fn insert_new(&mut self, symbol: Symbol, type_id: NormalFormNodeId) {
        if let Some(existing_type_id) = self.raw.get(&symbol) {
            panic!("Tried to insert new entry ({:?}, {:?}) into a type map, when it already contained the entry ({:?}, {:?}).", symbol, type_id, symbol, existing_type_id);
        }
        self.raw.insert(symbol, type_id);
    }

    pub fn get(&self, symbol: Symbol) -> NormalFormNodeId {
        self.try_get(symbol).expect(&format!(
            "Tried to get the type of {:?}, but it was not in the type map.",
            symbol
        ))
    }

    pub fn try_get(&self, symbol: Symbol) -> Option<NormalFormNodeId> {
        self.raw.get(&symbol).copied()
    }
}
