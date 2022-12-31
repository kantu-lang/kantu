use super::*;

use ub::Identifier;

#[derive(Debug)]
pub struct Context<'a, 'b> {
    data: &'a mut ContextData<'b>,
    current_file_id: FileId,
}

impl Context<'_, '_> {
    pub fn current_file_id(&self) -> FileId {
        self.current_file_id
    }
}

#[derive(Clone, Debug)]
pub struct ContextData<'a> {
    stack: Vec<ContextEntry>,
    graph: DotGraph,
    file_tree: &'a FileTree,
}

#[derive(Clone, Debug)]
pub enum ContextEntry {
    Placeholder,
    Accessible(AccessibleEntry),
}

#[derive(Clone, Debug)]
pub enum AccessibleEntry {
    Builtin(IdentifierName),
    Local(Identifier),
}

impl ContextData<'_> {
    pub fn with_builtins(file_tree: &FileTree) -> ContextData {
        let type1_entry = ContextEntry::Placeholder;
        let type0_entry = ContextEntry::Accessible(AccessibleEntry::Builtin(
            IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase),
        ));
        ContextData {
            stack: vec![type1_entry, type0_entry],
            graph: DotGraph::empty(),
            file_tree,
        }
    }
}

impl<'b> ContextData<'b> {
    pub fn create_context_for_mod<'a>(&'a mut self, mod_id: FileId) -> Context<'a, 'b> {
        Context {
            data: self,
            current_file_id: mod_id,
        }
    }
}

impl Context<'_, '_> {
    /// Panics if `n > self.len()`.
    pub fn pop_n(&mut self, n: usize) {
        self.data.pop_n(n)
    }

    /// Panics if `new_len > self.len()`.
    pub fn truncate(&mut self, new_len: usize) {
        self.data.truncate(new_len)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}
impl ContextData<'_> {
    /// Panics if `n > self.len()`.
    fn pop_n(&mut self, n: usize) {
        if n > self.len() {
            panic!(
                "Tried to pop {} elements from a context with only {} elements",
                n,
                self.len()
            );
        }
        self.stack.truncate(self.len() - n);
    }

    /// Panics if `new_len > self.len()`.
    fn truncate(&mut self, new_len: usize) {
        if new_len > self.len() {
            panic!(
                "Tried to truncate a context with only {} elements to {} elements",
                self.len(),
                new_len
            );
        }
        self.stack.truncate(new_len);
    }

    fn len(&self) -> usize {
        self.stack.len()
    }
}

impl Context<'_, '_> {
    pub fn index_to_level(&self, level: DbIndex) -> DbLevel {
        self.data.index_to_level(level)
    }
}
impl ContextData<'_> {
    fn level_to_index(&self, level: DbLevel) -> DbIndex {
        DbIndex(self.len() - level.0 - 1)
    }

    fn index_to_level(&self, level: DbIndex) -> DbLevel {
        DbLevel(self.len() - level.0 - 1)
    }
}

impl Context<'_, '_> {
    /// If the DB index lookup fails, then there are 2 possibilities:
    /// 1. The name is not in scope, in which case the `Err` variant is `None`.
    /// 2. The name is in scope, but it refers to a mod rather than a leaf item.
    ///    In this case, the `Err` variant is `Some(file_id)` where `file_id` is
    ///    the id of the mod's file.
    pub fn get_db_index<'a, N>(&self, name_components: N) -> Result<DbIndex, Option<FileId>>
    where
        N: Clone + Iterator<Item = &'a IdentifierName>,
    {
        self.data
            .get_db_index(self.current_file_id, name_components)
    }

    pub fn lookup_name<'a, N>(&self, name_components: N) -> Option<DotGraphEntry>
    where
        N: Clone + Iterator<Item = &'a IdentifierName>,
    {
        self.data.lookup_name(self.current_file_id, name_components)
    }
}
impl ContextData<'_> {
    /// If the DB index lookup fails, then there are 2 possibilities:
    /// 1. The name is not in scope, in which case the `Err` variant is `None`.
    /// 2. The name is in scope, but it refers to a mod rather than a leaf item.
    ///    In this case, the `Err` variant is `Some(file_id)` where `file_id` is
    ///    the id of the mod's file.
    fn get_db_index<'a, N>(
        &self,
        current_file_id: FileId,
        name_components: N,
    ) -> Result<DbIndex, Option<FileId>>
    where
        N: Clone + Iterator<Item = &'a IdentifierName>,
    {
        let lookup_result = self
            .lookup_name(current_file_id, name_components)
            .map(|entry| entry.node);
        match lookup_result {
            Some(DotGraphNode::LeafItem(level)) => Ok(self.level_to_index(level)),
            Some(DotGraphNode::Mod(file_id)) => Err(Some(file_id)),
            None => Err(None),
        }
    }

    fn lookup_name<'a, N>(
        &self,
        current_file_id: FileId,
        name_components: N,
    ) -> Option<DotGraphEntry>
    where
        N: Clone + Iterator<Item = &'a IdentifierName>,
    {
        let mut remaining = name_components;
        let first = remaining
            .next()
            .expect("name_components must not be empty.");

        let mut current = self
            .lookup_builtin(first)
            .or_else(|| self.lookup_local_name_component(first))
            .or_else(|| self.lookup_mod_item(current_file_id, first))
            .or_else(|| self.resolve_component_kw_if_applicable(current_file_id, first))?;

        for component in remaining {
            current = self.graph.get_edge_dest(current.node, component)?.clone();
        }

        Some(current)
    }

    fn lookup_builtin(&self, component: &IdentifierName) -> Option<DotGraphEntry> {
        self.stack
            .iter()
            .enumerate()
            .find_map(|(raw_index, entry)| match entry {
                ContextEntry::Accessible(AccessibleEntry::Builtin(builtin)) => {
                    if builtin == component {
                        let level = DbLevel(raw_index);
                        let def = OwnedSymbolSource::Builtin;
                        return Some(DotGraphEntry {
                            node: DotGraphNode::LeafItem(level),
                            def,
                        });
                    }
                    None
                }
                _ => None,
            })
    }

    fn lookup_local_name_component(&self, component: &IdentifierName) -> Option<DotGraphEntry> {
        self.stack
            .iter()
            .enumerate()
            .find_map(|(raw_index, entry)| {
                if let ContextEntry::Accessible(AccessibleEntry::Local(local)) = entry {
                    if &local.name == component {
                        let level = DbLevel(raw_index);
                        let def = OwnedSymbolSource::Identifier(local.clone());
                        return Some(DotGraphEntry {
                            node: DotGraphNode::LeafItem(level),
                            def,
                        });
                    }
                }
                None
            })
    }

    fn lookup_mod_item(
        &self,
        current_file_id: FileId,
        component: &IdentifierName,
    ) -> Option<DotGraphEntry> {
        self.graph
            .get_edge_dest(DotGraphNode::Mod(current_file_id), component)
            .cloned()
    }

    fn resolve_component_kw_if_applicable(
        &self,
        current_file_id: FileId,
        component: &IdentifierName,
    ) -> Option<DotGraphEntry> {
        match component {
            IdentifierName::Reserved(ReservedIdentifierName::Mod) => {
                get_n_supers(self.file_tree, current_file_id, 0)
            }
            IdentifierName::Reserved(ReservedIdentifierName::Super) => {
                get_n_supers(self.file_tree, current_file_id, 1)
            }
            IdentifierName::Reserved(ReservedIdentifierName::Super2) => {
                get_n_supers(self.file_tree, current_file_id, 2)
            }
            IdentifierName::Reserved(ReservedIdentifierName::Super3) => {
                get_n_supers(self.file_tree, current_file_id, 3)
            }
            IdentifierName::Reserved(ReservedIdentifierName::Super4) => {
                get_n_supers(self.file_tree, current_file_id, 4)
            }
            IdentifierName::Reserved(ReservedIdentifierName::Super5) => {
                get_n_supers(self.file_tree, current_file_id, 5)
            }
            IdentifierName::Reserved(ReservedIdentifierName::Super6) => {
                get_n_supers(self.file_tree, current_file_id, 6)
            }
            IdentifierName::Reserved(ReservedIdentifierName::Super7) => {
                get_n_supers(self.file_tree, current_file_id, 7)
            }
            IdentifierName::Reserved(ReservedIdentifierName::Super8) => {
                get_n_supers(self.file_tree, current_file_id, 8)
            }
            IdentifierName::Reserved(ReservedIdentifierName::Pack) => {
                let root_id = self.file_tree.root();
                Some(DotGraphEntry {
                    node: DotGraphNode::Mod(root_id),
                    def: OwnedSymbolSource::Mod(root_id),
                })
            }

            _ => None,
        }
    }
}

fn get_n_supers(tree: &FileTree, current_file_id: FileId, n: usize) -> Option<DotGraphEntry> {
    let mut current = current_file_id;
    for _ in 0..n {
        current = tree.parent(current)?;
    }
    let nth_super = current;
    Some(DotGraphEntry {
        node: DotGraphNode::Mod(nth_super),
        def: OwnedSymbolSource::Mod(nth_super),
    })
}

impl Context<'_, '_> {
    pub fn push_placeholder(&mut self) -> DbLevel {
        self.data.push_placeholder()
    }
}
impl ContextData<'_> {
    fn push_placeholder(&mut self) -> DbLevel {
        self.stack.push(ContextEntry::Placeholder);
        DbLevel(self.len() - 1)
    }
}

impl Context<'_, '_> {
    pub fn add_dot_edge(
        &mut self,
        start: DotGraphNode,
        label: &IdentifierName,
        end: DotGraphEntry,
    ) -> Result<(), DotGraphEntry> {
        self.data.add_dot_edge(start, label, end)
    }
}
impl ContextData<'_> {
    fn add_dot_edge(
        &mut self,
        start: DotGraphNode,
        label: &IdentifierName,
        end: DotGraphEntry,
    ) -> Result<(), DotGraphEntry> {
        self.graph.add_edge(start, label, end)
    }
}

impl Context<'_, '_> {
    pub fn push_local(&mut self, identifier: &Identifier) -> Result<(), OwnedSymbolSource> {
        self.data.push_local(self.current_file_id, identifier)
    }
}
impl ContextData<'_> {
    fn push_local(
        &mut self,
        current_file_id: FileId,
        identifier: &Identifier,
    ) -> Result<(), OwnedSymbolSource> {
        if let Some(existing_entry) =
            self.lookup_name(current_file_id, std::iter::once(&identifier.name))
        {
            return Err(existing_entry.def.clone());
        }

        self.stack
            .push(ContextEntry::Accessible(AccessibleEntry::Local(
                identifier.clone(),
            )));

        Ok(())
    }
}

impl Context<'_, '_> {
    pub fn get_edges(&self, node: DotGraphNode) -> Vec<(&IdentifierName, &DotGraphEntry)> {
        self.data.get_edges(node)
    }
}
impl ContextData<'_> {
    fn get_edges(&self, node: DotGraphNode) -> Vec<(&IdentifierName, &DotGraphEntry)> {
        self.graph.get_edges(node)
    }
}
