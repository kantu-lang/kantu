use super::*;

use rustc_hash::FxHashMap;

#[derive(Clone, Debug)]
pub struct DotGraph {
    edge_maps:
        FxHashMap<DotGraphNode, FxHashMap<IdentifierName, (DotGraphNode, OwnedSymbolSource)>>,
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
        end: DotGraphNode,
        source: OwnedSymbolSource,
    ) -> Result<(), OwnedSymbolSource> {
        let old_entry = self
            .edge_maps
            .entry(start.clone())
            .or_default()
            .insert(label.clone(), (end, source));

        if let Some(old_entry) = old_entry {
            let old_source = old_entry.1.clone();
            self.edge_maps
                .entry(start)
                .or_default()
                .insert(label.clone(), old_entry);
            return Err(old_source);
        }

        Ok(())
    }

    pub fn get_edge_dest(
        &self,
        start: DotGraphNode,
        label: &IdentifierName,
    ) -> Option<(DotGraphNode, &OwnedSymbolSource)> {
        self.edge_maps
            .get(&start)
            .and_then(|map| map.get(label))
            .map(|(node, source)| (*node, source))
    }

    pub fn get_edges(
        &self,
        node: DotGraphNode,
    ) -> Vec<(&IdentifierName, DotGraphNode, &OwnedSymbolSource)> {
        let Some(edges) = self.edge_maps.get(&node) else {
            return vec![];
        };
        let mut out = Vec::with_capacity(edges.len());
        for (label, (end, source)) in edges {
            out.push((label, *end, source));
        }
        out
    }
}
