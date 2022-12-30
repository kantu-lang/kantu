use super::*;

use ub::Identifier;

#[derive(Clone, Debug)]
pub struct Context<'a> {
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

impl Context<'_> {
    pub fn with_builtins(file_tree: &FileTree) -> Context {
        let type1_entry = ContextEntry::Placeholder;
        let type0_entry = ContextEntry::Accessible(AccessibleEntry::Builtin(
            IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase),
        ));
        Context {
            stack: vec![type1_entry, type0_entry],
            graph: DotGraph::empty(),
            file_tree,
        }
    }
}

impl Context<'_> {
    /// Panics if `n > self.len()`.
    pub fn pop_n(&mut self, n: usize) {
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
    pub fn truncate(&mut self, new_len: usize) {
        if new_len > self.len() {
            panic!(
                "Tried to truncate a context with only {} elements to {} elements",
                self.len(),
                new_len
            );
        }
        self.stack.truncate(new_len);
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }
}

impl Context<'_> {
    pub fn level_to_index(&self, level: DbLevel) -> DbIndex {
        DbIndex(self.len() - level.0 - 1)
    }

    pub fn index_to_level(&self, level: DbIndex) -> DbLevel {
        DbLevel(self.len() - level.0 - 1)
    }
}

impl Context<'_> {
    /// If the DB index lookup fails, then there are 2 possibilities:
    /// 1. The name is not in scope, in which case the `Err` variant is `None`.
    /// 2. The name is in scope, but it refers to a mod rather than a leaf item.
    ///    In this case, the `Err` variant is `Some(file_id)` where `file_id` is
    ///    the id of the mod's file.
    pub fn get_db_index<'a, N>(
        &self,
        current_file_id: FileId,
        name_components: N,
    ) -> Result<DbIndex, Option<FileId>>
    where
        N: Clone + Iterator<Item = &'a IdentifierName>,
    {
        let lookup_result = self
            .lookup_name(current_file_id, name_components)
            .map(|(index, _)| index);
        match lookup_result {
            Some(DotGraphNode::LeafItem(level)) => Ok(self.level_to_index(level)),
            Some(DotGraphNode::Mod(file_id)) => Err(Some(file_id)),
            None => Err(None),
        }
    }

    pub fn lookup_name<'a, N>(
        &self,
        current_file_id: FileId,
        name_components: N,
    ) -> Option<(DotGraphNode, OwnedSymbolSource)>
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
            current = clone_tuple_1(self.graph.get_edge_dest(current.0, component)?);
        }

        Some(current)
    }

    fn lookup_builtin(
        &self,
        component: &IdentifierName,
    ) -> Option<(DotGraphNode, OwnedSymbolSource)> {
        self.stack
            .iter()
            .enumerate()
            .find_map(|(raw_index, entry)| match entry {
                ContextEntry::Accessible(AccessibleEntry::Builtin(builtin)) => {
                    if builtin == component {
                        let level = DbLevel(raw_index);
                        let source = OwnedSymbolSource::Builtin;
                        return Some((DotGraphNode::LeafItem(level), source));
                    }
                    None
                }
                _ => None,
            })
    }

    fn lookup_local_name_component(
        &self,
        component: &IdentifierName,
    ) -> Option<(DotGraphNode, OwnedSymbolSource)> {
        self.stack
            .iter()
            .enumerate()
            .find_map(|(raw_index, entry)| {
                if let ContextEntry::Accessible(AccessibleEntry::Local(local)) = entry {
                    if &local.name == component {
                        let level = DbLevel(raw_index);
                        let source = OwnedSymbolSource::Identifier(local.clone());
                        return Some((DotGraphNode::LeafItem(level), source));
                    }
                }
                None
            })
    }

    fn lookup_mod_item(
        &self,
        current_file_id: FileId,
        component: &IdentifierName,
    ) -> Option<(DotGraphNode, OwnedSymbolSource)> {
        Some(clone_tuple_1(self.graph.get_edge_dest(
            DotGraphNode::Mod(current_file_id),
            component,
        )?))
    }

    fn resolve_component_kw_if_applicable(
        &self,
        current_file_id: FileId,
        component: &IdentifierName,
    ) -> Option<(DotGraphNode, OwnedSymbolSource)> {
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
                Some((DotGraphNode::Mod(root_id), OwnedSymbolSource::Mod(root_id)))
            }

            _ => None,
        }
    }
}

fn clone_tuple_1<A, B>((a, b): (A, &B)) -> (A, B)
where
    B: Clone,
{
    (a, b.clone())
}

fn get_n_supers(
    tree: &FileTree,
    current_file_id: FileId,
    n: usize,
) -> Option<(DotGraphNode, OwnedSymbolSource)> {
    let mut current = current_file_id;
    for _ in 0..n {
        current = tree.parent(current)?;
    }
    let nth_super = current;
    Some((
        DotGraphNode::Mod(nth_super),
        OwnedSymbolSource::Mod(nth_super),
    ))
}

impl Context<'_> {
    pub fn push_placeholder(&mut self) -> DbLevel {
        self.stack.push(ContextEntry::Placeholder);
        DbLevel(self.len() - 1)
    }
}

impl Context<'_> {
    pub fn add_dot_edge(
        &mut self,
        start: DotGraphNode,
        label: &IdentifierName,
        end: DotGraphNode,
        source: OwnedSymbolSource,
    ) -> Result<(), OwnedSymbolSource> {
        self.graph.add_edge(start, label, end, source)
    }
}

impl Context<'_> {
    pub fn push_local(
        &mut self,
        current_file_id: FileId,
        identifier: &Identifier,
    ) -> Result<(), OwnedSymbolSource> {
        if let Some((_, existing_source)) =
            self.lookup_name(current_file_id, std::iter::once(&identifier.name))
        {
            return Err(existing_source.clone());
        }

        self.stack
            .push(ContextEntry::Accessible(AccessibleEntry::Local(
                identifier.clone(),
            )));

        Ok(())
    }
}
