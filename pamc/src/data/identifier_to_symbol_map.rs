use crate::data::{node_registry::NodeId, registered_ast::*};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SymbolId(pub usize);

#[derive(Clone, Debug)]
pub struct IdentifierToSymbolMap {
    map: Vec<SymbolId>,
}

impl IdentifierToSymbolMap {
    pub fn dense(map: Vec<SymbolId>) -> Self {
        Self { map }
    }
}

impl IdentifierToSymbolMap {
    pub fn get(&self, identifier: NodeId<Identifier>) -> SymbolId {
        self.map[identifier.raw]
    }
}
