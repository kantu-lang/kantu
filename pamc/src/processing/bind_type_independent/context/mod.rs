use super::*;

use ub::Identifier;

#[derive(Clone, Debug)]
pub struct Context {
    stack: Vec<ContextEntry>,
    graph: DotGraph,
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

impl Context {
    pub fn with_builtins() -> Self {
        let type1_entry = ContextEntry::Placeholder;
        let type0_entry = ContextEntry::Accessible(AccessibleEntry::Builtin(
            IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase),
        ));
        Self {
            stack: vec![type1_entry, type0_entry],
            graph: DotGraph::empty(),
        }
    }
}

impl Context {
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

impl Context {
    pub fn level_to_index(&self, level: DbLevel) -> DbIndex {
        DbIndex(self.len() - level.0 - 1)
    }

    pub fn index_to_level(&self, level: DbIndex) -> DbLevel {
        DbLevel(self.len() - level.0 - 1)
    }
}

impl Context {
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

    fn lookup_name<'a, N>(
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
            .or_else(|| self.lookup_mod_item(current_file_id, first))?;

        for component in remaining {
            current = clone_tuple_1(self.graph.get_edge_target(current.0, component)?);
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
        Some(clone_tuple_1(self.graph.get_edge_target(
            DotGraphNode::Mod(current_file_id),
            component,
        )?))
    }
}

fn clone_tuple_1<A, B>((a, b): (A, &B)) -> (A, B)
where
    B: Clone,
{
    (a, b.clone())
}

impl Context {
    pub fn push_placeholder(&mut self) -> DbLevel {
        self.stack.push(ContextEntry::Placeholder);
        DbLevel(self.len() - 1)
    }
}

impl Context {
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

impl Context {
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

// TODO: Delete
// impl AccessibleEntry {
//     fn source(&self) -> OwnedSymbolSource {
//         match self {
//             AccessibleEntry::Builtin(_) => OwnedSymbolSource::Builtin,
//             AccessibleEntry::Local(identifier) => OwnedSymbolSource::Identifier(identifier.clone()),
//         }
//     }
// }

// impl AccessibleEntry {
//     fn matches_name(&self, name_components: &[Identifier]) -> bool {
//         match self {
//             AccessibleEntry::Builtin(self_components) => self_components
//                 .iter()
//                 .eq(name_components.iter().map(|c| &c.name)),
//             AccessibleEntry::GlobalName(self_entry) => self_entry.matches_name(name_components),
//             AccessibleEntry::LocalName(self_entry) => self_entry.matches_name(name_components),
//         }
//     }
// }

// impl GlobalNameEntry {
//     fn matches_name(&self, name_components: &[Identifier]) -> bool {
//         self.remaining_name_components
//             .iter()
//             .eq(name_components.iter().map(|c| &c.name))
//     }
// }

// impl LocalNameEntry {
//     fn matches_name(&self, name_components: &[Identifier]) -> bool {
//         self.name_components
//             .iter()
//             .eq(name_components.iter().map(|c| &c.name))
//     }
// }

// impl Context {
//     pub fn add_local_name_to_scope_unless_underscore(
//         &mut self,
//         current_file_id: FileId,
//         identifier: &Identifier,
//     ) -> Result<(), NameClashError> {
//         if let IdentifierName::Reserved(ReservedIdentifierName::Underscore) = &identifier.name {
//             self.stack.push(ContextEntry::Placeholder);
//             return Ok(());
//         }

//         self.check_for_name_clash(std::iter::once(&identifier.name), identifier)?;

//         self.push_accessible(AccessibleEntry::LocalName(LocalNameEntry {
//             name_components: vec![identifier.name.clone()],
//             source: identifier.clone(),
//         }));

//         Ok(())
//     }

//     fn check_for_name_clash<'a, N>(
//         &self,
//         name_components: N,
//         source: &Identifier,
//     ) -> Result<(), NameClashError>
//     where
//         N: Clone + IntoIterator<Item = &'a IdentifierName>,
//     {
//         if let Some((_, entry)) = self.lookup_name(name_components) {
//             return Err(NameClashError {
//                 old: to_owned_src(entry),
//                 new: OwnedSymbolSource::Identifier(source.clone()),
//             });
//         } else {
//             Ok(())
//         }
//     }

//     fn push_accessible(&mut self, entry: AccessibleEntry) {
//         self.stack.push(ContextEntry::Accessible(entry));
//     }
// }

// impl Context {
//     pub fn add_temporarily_restricted_name_to_scope_unless_singleton_underscore<'a, N>(
//         &mut self,
//         name_components: N,
//         source: &Identifier,
//     ) -> Result<(), NameClashError>
//     where
//         N: Clone + IntoIterator<Item = &'a IdentifierName>,
//     {
//         {
//             let mut name_components = name_components.clone().into_iter();
//             match (name_components.next(), name_components.next()) {
//                 // Detect if `name_components` is a singleton underscore.
//                 (Some(IdentifierName::Reserved(ReservedIdentifierName::Underscore)), None) => {
//                     self.push_permanently_restricted(ContextEntry {
//                         name_components: name_components.cloned().collect(),
//                         source: OwnedSymbolSource::Identifier(source.clone()),
//                     });
//                     return Ok(());
//                 }
//                 _ => {}
//             }
//         }

//         // Observe that we check for the name clash
//         // _after_ we check the singleton underscore case.
//         // This is important because otherwise,
//         // singleton underscores would cause name clash errors.
//         self.check_for_name_clash(name_components.clone(), source)?;

//         self.push_temporarily_restricted(ContextEntry {
//             name_components: name_components.into_iter().cloned().collect(),
//             source: OwnedSymbolSource::Identifier(source.clone()),
//         });

//         Ok(())
//     }

//     /// Panics if there is no entry corresponding to the given input.
//     pub fn lift_dot_target_restriction(&mut self, name_components: &[&IdentifierName]) {
//         let wrapped_entry: &mut PossiblyRestricted<_> = self
//             .get_mut_wrapped_entry(name_components)
//             .expect("Tried to lift a restriction on a name that doesn't exist");
//         *wrapped_entry = ContextEntry::Name(
//             wrapped_entry
//                 .as_mut()
//                 .temporarily_restricted()
//                 .take()
//                 .expect("Tried to lift restriction on an unrestricted entry.")
//                 .clone(),
//         );
//     }

//     fn get_mut_wrapped_entry(
//         &mut self,
//         name_components: &[&IdentifierName],
//     ) -> Option<&mut PossiblyRestricted<ContextEntry>> {
//         self.stack.iter_mut().rev().find_map(
//             |entry| -> Option<&mut PossiblyRestricted<ContextEntry>> {
//                 if entry
//                     .as_ref()
//                     .ignore_status()
//                     .name_components
//                     .iter()
//                     .eq(name_components.iter().copied())
//                 {
//                     Some(entry)
//                 } else {
//                     None
//                 }
//             },
//         )
//     }
// }
