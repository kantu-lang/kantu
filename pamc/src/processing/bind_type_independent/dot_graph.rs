use super::*;

use rustc_hash::FxHashMap;

#[derive(Clone, Debug)]
pub struct DotGraph {
    edge_maps: FxHashMap<DotGraphNode, FxHashMap<IdentifierName, DotGraphEntry>>,
}

#[derive(Clone, Debug)]
pub struct DotGraphEntry {
    pub node: DotGraphNode,
    pub def: OwnedSymbolSource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DotGraphNode {
    LeafItem(DbLevel),
    Mod(FileId),
}

impl DotGraph {
    pub fn empty() -> Self {
        Self {
            edge_maps: FxHashMap::default(),
        }
    }
}

impl DotGraph {
    pub fn add_edge(
        &mut self,
        start: DotGraphNode,
        label: &IdentifierName,
        end: DotGraphEntry,
    ) -> Result<(), DotGraphEntry> {
        let old_entry = self
            .edge_maps
            .entry(start.clone())
            .or_default()
            .insert(label.clone(), end);

        if let Some(old_entry) = old_entry {
            self.edge_maps
                .entry(start)
                .or_default()
                .insert(label.clone(), old_entry.clone());
            return Err(old_entry);
        }

        Ok(())
    }

    pub fn get_edge_dest(
        &self,
        start: DotGraphNode,
        label: &IdentifierName,
    ) -> Option<&DotGraphEntry> {
        self.edge_maps.get(&start).and_then(|map| map.get(label))
    }

    pub fn get_edges(&self, node: DotGraphNode) -> Vec<(&IdentifierName, &DotGraphEntry)> {
        let Some(edges) = self.edge_maps.get(&node) else {
            return vec![];
        };
        let mut out = Vec::with_capacity(edges.len());
        for (label, end) in edges {
            out.push((label, end));
        }
        out
    }
}
