use crate::data::{node_registry::NodeId, registered_ast::*};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Symbol(pub usize);

#[derive(Clone, Debug)]
pub struct IdentifierToSymbolMap {
    map: Vec<Option<Symbol>>,
}

impl IdentifierToSymbolMap {
    pub fn empty() -> Self {
        Self { map: Vec::new() }
    }
}

impl IdentifierToSymbolMap {
    pub fn get(&self, identifier_id: NodeId<Identifier>) -> Symbol {
        self.try_get(identifier_id).expect(&format!(
            "Symbol could not be found for {:?}",
            identifier_id
        ))
    }

    pub fn try_get(&self, identifier_id: NodeId<Identifier>) -> Option<Symbol> {
        if identifier_id.raw >= self.map.len() {
            None
        } else {
            self.map[identifier_id.raw]
        }
    }

    pub fn contains(&self, identifier_id: NodeId<Identifier>) -> bool {
        self.try_get(identifier_id).is_some()
    }

    pub fn insert(&mut self, identifier_id: NodeId<Identifier>, symbol_id: Symbol) -> bool {
        let is_newly_inserted = !self.contains(identifier_id);

        if identifier_id.raw >= self.map.len() {
            self.map.resize(identifier_id.raw + 1, None);
        }
        self.map[identifier_id.raw] = Some(symbol_id);

        is_newly_inserted
    }
}
