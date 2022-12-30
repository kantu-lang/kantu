use super::*;

#[derive(Clone, Debug)]
pub struct DotGraph {
    edges: FxHashMap<DotGraphNode, FxHashMap<IdentifierName, (DotGraphNode, OwnedSymbolSource)>>,
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

impl DotGraph {
    pub fn add_edge(
        &mut self,
        start: DotGraphNode,
        label: &IdentifierName,
        end: DotGraphNode,
        source: OwnedSymbolSource,
    ) -> Result<(), OwnedSymbolSource> {
        let old_entry = self
            .edges
            .entry(start.clone())
            .or_default()
            .insert(label.clone(), (end, source));

        if let Some(old_entry) = old_entry {
            let old_source = old_entry.1.clone();
            self.edges
                .entry(start)
                .or_default()
                .insert(label.clone(), old_entry);
            return Err(old_source);
        }

        Ok(())
    }
}
