use super::*;

#[derive(Clone, Debug)]
pub struct DotGraph {
    edges: FxHashMap<DotGraphNode, FxHashMap<UnreservedIdentifierName, DotGraphNode>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DotGraphNode {
    LeafItem(DbLevel),
    Mod(FileId),
}

impl DotGraph {
    pub fn empty() -> Self {
        Self {
            edges: FxHashMap::default(),
        }
    }
}
