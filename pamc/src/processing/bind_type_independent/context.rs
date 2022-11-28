use super::*;

#[derive(Clone, Debug)]
pub struct Context {
    stack: Vec<PossiblyRestricted<ContextEntry>>,
}

#[derive(Clone, Copy, Debug)]
enum PossiblyRestricted<T> {
    PermanentlyRestricted(T),
    TemporarilyRestricted(T),
    Unrestricted(T),
}

impl<T> PossiblyRestricted<T> {
    fn map<U>(self, f: impl FnOnce(T) -> U) -> PossiblyRestricted<U> {
        match self {
            PossiblyRestricted::PermanentlyRestricted(t) => {
                PossiblyRestricted::PermanentlyRestricted(f(t))
            }
            Self::TemporarilyRestricted(t) => PossiblyRestricted::TemporarilyRestricted(f(t)),
            Self::Unrestricted(t) => PossiblyRestricted::Unrestricted(f(t)),
        }
    }

    fn as_ref(&self) -> PossiblyRestricted<&T> {
        match self {
            PossiblyRestricted::PermanentlyRestricted(t) => {
                PossiblyRestricted::PermanentlyRestricted(t)
            }
            Self::TemporarilyRestricted(t) => PossiblyRestricted::TemporarilyRestricted(&t),
            Self::Unrestricted(t) => PossiblyRestricted::Unrestricted(&t),
        }
    }

    fn as_mut(&mut self) -> PossiblyRestricted<&mut T> {
        match self {
            PossiblyRestricted::PermanentlyRestricted(t) => {
                PossiblyRestricted::PermanentlyRestricted(t)
            }
            Self::TemporarilyRestricted(t) => PossiblyRestricted::TemporarilyRestricted(t),
            Self::Unrestricted(t) => PossiblyRestricted::Unrestricted(t),
        }
    }

    fn temporarily_restricted(self) -> Option<T> {
        match self {
            PossiblyRestricted::PermanentlyRestricted(_) => None,
            Self::TemporarilyRestricted(t) => Some(t),
            Self::Unrestricted(_) => None,
        }
    }

    fn ignore_status(self) -> T {
        match self {
            PossiblyRestricted::PermanentlyRestricted(t) => t,
            Self::TemporarilyRestricted(t) => t,
            Self::Unrestricted(t) => t,
        }
    }
}

impl<T> PossiblyRestricted<Option<T>> {
    fn transpose(self) -> Option<PossiblyRestricted<T>> {
        match self {
            PossiblyRestricted::PermanentlyRestricted(t) => {
                t.map(PossiblyRestricted::PermanentlyRestricted)
            }
            Self::TemporarilyRestricted(t) => t.map(PossiblyRestricted::TemporarilyRestricted),
            Self::Unrestricted(t) => t.map(PossiblyRestricted::Unrestricted),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ContextEntry {
    name_components: Vec<IdentifierName>,
    source: OwnedSymbolSource,
}

#[derive(Clone, Debug)]
pub enum OwnedSymbolSource {
    Identifier(ub::Identifier),
    Builtin,
}

impl Context {
    pub fn with_builtins() -> Self {
        let type1_entry = PossiblyRestricted::Unrestricted(ContextEntry {
            name_components: vec![],
            source: OwnedSymbolSource::Builtin,
        });
        let type0_entry = PossiblyRestricted::Unrestricted(ContextEntry {
            name_components: vec![IdentifierName::Reserved(
                ReservedIdentifierName::TypeTitleCase,
            )],
            source: OwnedSymbolSource::Builtin,
        });
        Self {
            stack: vec![type1_entry, type0_entry],
        }
    }
}

impl Context {
    fn push_unrestricted(&mut self, entry: ContextEntry) {
        self.stack.push(PossiblyRestricted::Unrestricted(entry));
    }

    fn push_temporarily_restricted(&mut self, entry: ContextEntry) {
        self.stack
            .push(PossiblyRestricted::TemporarilyRestricted(entry));
    }

    fn push_permanently_restricted(&mut self, entry: ContextEntry) {
        self.stack
            .push(PossiblyRestricted::PermanentlyRestricted(entry));
    }

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

    pub fn len(&self) -> usize {
        self.stack.len()
    }
}

impl Context {
    pub fn level_to_index(&self, level: DbLevel) -> DbIndex {
        DbIndex(self.len() - level.0 - 1)
    }
}

impl Context {
    pub fn get_db_index(
        &self,
        name_components: &[ub::Identifier],
    ) -> Result<DbIndex, NameNotFoundError> {
        if let Some(PossiblyRestricted::Unrestricted((_, db_index))) =
            self.lookup_name(name_components.iter().map(|identifier| &identifier.name))
        {
            Ok(db_index)
        } else {
            Err(NameNotFoundError {
                name_components: name_components.iter().cloned().collect(),
            })
        }
    }

    fn lookup_name<'a, N>(
        &self,
        name_components: N,
    ) -> Option<PossiblyRestricted<(&ContextEntry, DbIndex)>>
    where
        N: Clone + IntoIterator<Item = &'a IdentifierName>,
    {
        self.stack.iter().enumerate().rev().find_map(
            |(raw_index, entry)| -> Option<PossiblyRestricted<(&ContextEntry, DbIndex)>> {
                entry
                    .as_ref()
                    .map(|entry| -> Option<(&ContextEntry, DbIndex)> {
                        if entry
                            .name_components
                            .iter()
                            .eq(name_components.clone().into_iter())
                        {
                            let db_level = DbLevel(raw_index);
                            let db_index = self.level_to_index(db_level);
                            Some((entry, db_index))
                        } else {
                            None
                        }
                    })
                    .transpose()
            },
        )
    }

    pub fn add_unrestricted_unqualified_name_to_scope_unless_underscore(
        &mut self,
        identifier: &ub::Identifier,
    ) -> Result<(), NameClashError> {
        if let IdentifierName::Reserved(ReservedIdentifierName::Underscore) = identifier.name {
            self.push_permanently_restricted(ContextEntry {
                name_components: vec![identifier.name.clone()],
                source: OwnedSymbolSource::Identifier(identifier.clone()),
            });
            return Ok(());
        }

        // Observe that we check for the name clash
        // _after_ we check the underscore case.
        // This is important because otherwise,
        // underscores would cause name clash errors.
        self.check_for_name_clash(std::iter::once(&identifier.name), identifier)?;

        self.push_unrestricted(ContextEntry {
            name_components: vec![identifier.name.clone()],
            source: OwnedSymbolSource::Identifier(identifier.clone()),
        });

        Ok(())
    }

    fn check_for_name_clash<'a, N>(
        &self,
        name_components: N,
        source: &ub::Identifier,
    ) -> Result<(), NameClashError>
    where
        N: Clone + IntoIterator<Item = &'a IdentifierName>,
    {
        if let Some((entry, _)) = self
            .lookup_name(name_components)
            .map(PossiblyRestricted::ignore_status)
        {
            return Err(NameClashError {
                old: entry.source.clone(),
                new: OwnedSymbolSource::Identifier(source.clone()),
            });
        } else {
            Ok(())
        }
    }
}

impl Context {
    pub fn add_temporarily_restricted_name_to_scope_unless_singleton_underscore<'a, N>(
        &mut self,
        name_components: N,
        source: &ub::Identifier,
    ) -> Result<(), NameClashError>
    where
        N: Clone + IntoIterator<Item = &'a IdentifierName>,
    {
        {
            let mut name_components = name_components.clone().into_iter();
            match (name_components.next(), name_components.next()) {
                // Detect if `name_components` is a singleton underscore.
                (Some(IdentifierName::Reserved(ReservedIdentifierName::Underscore)), None) => {
                    self.push_permanently_restricted(ContextEntry {
                        name_components: name_components.cloned().collect(),
                        source: OwnedSymbolSource::Identifier(source.clone()),
                    });
                    return Ok(());
                }
                _ => {}
            }
        }

        // Observe that we check for the name clash
        // _after_ we check the singleton underscore case.
        // This is important because otherwise,
        // singleton underscores would cause name clash errors.
        self.check_for_name_clash(name_components.clone(), source)?;

        self.push_temporarily_restricted(ContextEntry {
            name_components: name_components.into_iter().cloned().collect(),
            source: OwnedSymbolSource::Identifier(source.clone()),
        });

        Ok(())
    }

    /// Panics if there is no entry corresponding to the given input.
    pub fn lift_dot_target_restriction(&mut self, name_components: &[&IdentifierName]) {
        let wrapped_entry: &mut PossiblyRestricted<_> = self
            .get_mut_wrapped_entry(name_components)
            .expect("Tried to lift a restriction on a name that doesn't exist");
        *wrapped_entry = PossiblyRestricted::Unrestricted(
            wrapped_entry
                .as_mut()
                .temporarily_restricted()
                .take()
                .expect("Tried to lift restriction on an unrestricted entry.")
                .clone(),
        );
    }

    fn get_mut_wrapped_entry(
        &mut self,
        name_components: &[&IdentifierName],
    ) -> Option<&mut PossiblyRestricted<ContextEntry>> {
        self.stack.iter_mut().rev().find_map(
            |entry| -> Option<&mut PossiblyRestricted<ContextEntry>> {
                if entry
                    .as_ref()
                    .ignore_status()
                    .name_components
                    .iter()
                    .eq(name_components.iter().copied())
                {
                    Some(entry)
                } else {
                    None
                }
            },
        )
    }
}
