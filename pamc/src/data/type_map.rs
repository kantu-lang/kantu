use crate::data::{registered_ast::*, symbol_database::Symbol};
use rustc_hash::FxHashMap;

#[derive(Clone, Copy, Debug)]
pub struct NormalFormNodeId(pub ExpressionId);

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

    pub fn update(&mut self, symbol: Symbol, type_id: NormalFormNodeId) {
        if self.raw.get(&symbol).is_none() {
            panic!("Tried to update existing entry to ({:?}, {:?}) into a type map, but no existing entry was found.", symbol, type_id);
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

    pub fn keys(&self) -> impl Iterator<Item = Symbol> + '_ {
        self.raw.keys().copied()
    }
}
