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

#[derive(Clone, Debug)]
pub struct NameComponentNotFoundError {
    pub index: usize,
    pub kind: NameComponentNotFoundErrorKind,
}

#[derive(Clone, Debug)]
pub enum NameComponentNotFoundErrorKind {
    NotFound,
    Private(Visibility),
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
    /// 1. The name cannot be legally accessed (e.g., it cannot be found in scope, or it's private)
    ///    in which case this returns `Err(Err(err))` where `err` is the reason
    ///    why the name cannot be legally accessed.
    /// 2. The name can be legally accessed, but it refers to a mod rather than a leaf item.
    ///    In this case, this returns `Err(Ok(file_id))` where `file_id` is
    ///    the id of the mod's file.
    pub fn get_db_index<'a, N>(
        &self,
        name_components: N,
    ) -> Result<DbIndex, Result<FileId, NameComponentNotFoundError>>
    where
        N: Clone + Iterator<Item = &'a IdentifierName>,
    {
        self.data
            .get_db_index(self.current_file_id, name_components)
    }

    pub fn lookup_name<'a, N>(
        &self,
        name_components: N,
    ) -> Result<DotGraphEntry, NameComponentNotFoundError>
    where
        N: Clone + Iterator<Item = &'a IdentifierName>,
    {
        self.data.lookup_name(self.current_file_id, name_components)
    }
}
impl ContextData<'_> {
    /// If the DB index lookup fails, then there are 2 possibilities:
    /// 1. The name cannot be legally accessed (e.g., it cannot be found in scope, or it's private)
    ///    in which case this returns `Err(Err(err))` where `err` is the reason
    ///    why the name cannot be legally accessed.
    /// 2. The name can be legally accessed, but it refers to a mod rather than a leaf item.
    ///    In this case, this returns `Err(Ok(file_id))` where `file_id` is
    ///    the id of the mod's file.
    fn get_db_index<'a, N>(
        &self,
        current_file_id: FileId,
        name_components: N,
    ) -> Result<DbIndex, Result<FileId, NameComponentNotFoundError>>
    where
        N: Clone + Iterator<Item = &'a IdentifierName>,
    {
        let lookup_result = self
            .lookup_name(current_file_id, name_components)
            .map(|entry| entry.node);
        match lookup_result {
            Ok(DotGraphNode::LeafItem(level)) => Ok(self.level_to_index(level)),
            Ok(DotGraphNode::Mod(file_id)) => Err(Ok(file_id)),
            Err(err) => Err(Err(err)),
        }
    }

    fn lookup_name<'a, N>(
        &self,
        current_file_id: FileId,
        name_components: N,
    ) -> Result<DotGraphEntry, NameComponentNotFoundError>
    where
        N: Clone + Iterator<Item = &'a IdentifierName>,
    {
        self.lookup_name_with_customizable_visibility_enforcement::<N, true>(
            current_file_id,
            name_components,
        )
    }

    fn lookup_name_with_customizable_visibility_enforcement<
        'a,
        N,
        const SHOULD_ENFORCE_VISIBILITY: bool,
    >(
        &self,
        current_file_id: FileId,
        name_components: N,
    ) -> Result<DotGraphEntry, NameComponentNotFoundError>
    where
        N: Clone + Iterator<Item = &'a IdentifierName>,
    {
        let mut remaining = name_components;
        let first = remaining
            .next()
            .expect("name_components must not be empty.");

        let current = self
            .lookup_builtin(first)
            .or_else(|| self.lookup_local_name_component(first))
            .or_else(|| self.lookup_mod_item(current_file_id, first))
            .or_else(|| self.resolve_component_kw_if_applicable(current_file_id, first));
        let Some(mut current) = current else {
            return Err(NameComponentNotFoundError {
                index: 0,
                kind: NameComponentNotFoundErrorKind::NotFound,
            });
        };

        for (index_in_remaining, component) in remaining.enumerate() {
            let next = self.graph.get_edge_dest(current.node, component).cloned();
            let Some(next) = next else {
                return Err(NameComponentNotFoundError {
                    index: index_in_remaining + 1,
                    kind: NameComponentNotFoundErrorKind::NotFound,
                });
            };
            if SHOULD_ENFORCE_VISIBILITY
                && !is_visible_to_mod(self.file_tree, next.visibility, current_file_id)
            {
                return Err(NameComponentNotFoundError {
                    index: index_in_remaining + 1,
                    kind: NameComponentNotFoundErrorKind::Private(next.visibility),
                });
            }
            current = next;
        }

        Ok(current)
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
                            visibility: Visibility::Global,
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
                            // Local names will never be accessed from outside the current file,
                            // so we don't need to place any visibility restrictions on them.
                            // Thus, we give them global visibility.
                            visibility: Visibility::Global,
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
                self.get_n_supers(current_file_id, 0)
            }
            IdentifierName::Reserved(ReservedIdentifierName::Super) => {
                self.get_n_supers(current_file_id, 1)
            }
            IdentifierName::Reserved(ReservedIdentifierName::Super2) => {
                self.get_n_supers(current_file_id, 2)
            }
            IdentifierName::Reserved(ReservedIdentifierName::Super3) => {
                self.get_n_supers(current_file_id, 3)
            }
            IdentifierName::Reserved(ReservedIdentifierName::Super4) => {
                self.get_n_supers(current_file_id, 4)
            }
            IdentifierName::Reserved(ReservedIdentifierName::Super5) => {
                self.get_n_supers(current_file_id, 5)
            }
            IdentifierName::Reserved(ReservedIdentifierName::Super6) => {
                self.get_n_supers(current_file_id, 6)
            }
            IdentifierName::Reserved(ReservedIdentifierName::Super7) => {
                self.get_n_supers(current_file_id, 7)
            }
            IdentifierName::Reserved(ReservedIdentifierName::Super8) => {
                self.get_n_supers(current_file_id, 8)
            }
            IdentifierName::Reserved(ReservedIdentifierName::Pack) => {
                let root_id = self.file_tree.root();
                Some(DotGraphEntry {
                    node: DotGraphNode::Mod(root_id),
                    def: OwnedSymbolSource::Mod(root_id),
                    visibility: Visibility::Global,
                })
            }

            _ => None,
        }
    }
}

fn is_visible_to_mod(file_tree: &FileTree, visibility: Visibility, mod_id: FileId) -> bool {
    match visibility {
        Visibility::Global => true,
        Visibility::Mod(visibility_mod_id) => {
            file_tree.is_left_non_strict_descendant_of_right(mod_id, visibility_mod_id)
        }
    }
}

impl Context<'_, '_> {
    pub fn get_n_supers(&self, n: usize) -> Option<DotGraphEntry> {
        self.data.get_n_supers(self.current_file_id, n)
    }
}

impl ContextData<'_> {
    fn get_n_supers(&self, current_file_id: FileId, n: usize) -> Option<DotGraphEntry> {
        let mut current = current_file_id;
        for _ in 0..n {
            current = self.file_tree.parent(current)?;
        }
        let nth_super = current;
        Some(DotGraphEntry {
            node: DotGraphNode::Mod(nth_super),
            def: OwnedSymbolSource::Mod(nth_super),
            visibility: Visibility::Global,
        })
    }
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
    pub fn overwrite_dot_edge(
        &mut self,
        start: DotGraphNode,
        label: &IdentifierName,
        end: DotGraphEntry,
    ) {
        self.data.overwrite_dot_edge(start, label, end)
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
    fn overwrite_dot_edge(
        &mut self,
        start: DotGraphNode,
        label: &IdentifierName,
        end: DotGraphEntry,
    ) {
        self.graph.overwrite_edge(start, label, end)
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
            self.lookup_name_ignoring_visibility(current_file_id, std::iter::once(&identifier.name))
        {
            return Err(existing_entry.def.clone());
        }

        self.stack
            .push(ContextEntry::Accessible(AccessibleEntry::Local(
                identifier.clone(),
            )));

        Ok(())
    }

    fn lookup_name_ignoring_visibility<'a, N>(
        &self,
        current_file_id: FileId,
        name_components: N,
    ) -> Option<DotGraphEntry>
    where
        N: Clone + Iterator<Item = &'a IdentifierName>,
    {
        self.lookup_name_with_customizable_visibility_enforcement::<N, false>(
            current_file_id,
            name_components,
        )
        .ok()
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

impl Context<'_, '_> {
    pub fn is_left_more_permissive_than_right(&self, left: Visibility, right: Visibility) -> bool {
        match (left, right) {
            (Visibility::Global, Visibility::Global) => false,
            (Visibility::Global, Visibility::Mod(_)) => true,
            (Visibility::Mod(_), Visibility::Global) => false,
            (Visibility::Mod(left), Visibility::Mod(right)) => self
                .data
                .file_tree
                .is_left_strict_descendant_of_right(right, left),
        }
    }
}
