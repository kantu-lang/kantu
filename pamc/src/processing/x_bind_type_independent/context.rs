use super::*;

use rustc_hash::FxHashMap;

#[derive(Clone, Debug)]
pub struct Context {
    stack: Vec<ContextEntry>,
}

/// Instead of implementing this as an enum,
/// we could have just as easily implemented it
/// as a struct with a `is_restricted: bool` field.
///
/// However, then developers might access the other
/// fields without first checking the `is_restricted` field,
/// making it really easy for bugs to sneak in.
///
/// By using an enum, developers are forced to use a
/// `match` statement, which forces acknowledgement of
/// the is-restricted status.
#[derive(Clone, Debug)]
pub enum ContextEntry {
    Restricted {
        name_components: Vec<IdentifierName>,
        source: OwnedSymbolSource,
    },
    Unrestricted {
        name_components: Vec<IdentifierName>,
        source: OwnedSymbolSource,
    },
}

/// Instead of implementing this as an enum,
/// we could have just as easily implemented it
/// as a struct with a `is_restricted: bool` field.
///
/// However, then developers might access the other
/// fields without first checking the `is_restricted` field,
/// making it really easy for bugs to sneak in.
///
/// By using an enum, developers are forced to use a
/// `match` statement, which forces acknowledgement of
/// the is-restricted status.
#[derive(Clone, Debug)]
pub enum ContextEntryWithDbIndex {
    Restricted {
        name_components: Vec<IdentifierName>,
        source: OwnedSymbolSource,
        db_index: DbIndex,
    },
    Unrestricted {
        name_components: Vec<IdentifierName>,
        source: OwnedSymbolSource,
        db_index: DbIndex,
    },
}

#[derive(Clone, Debug)]
pub enum OwnedSymbolSource {
    Identifier(ub::Identifier),
    Builtin,
}

impl Context {
    pub fn with_builtins() -> Self {
        let type1_entry = ContextEntry::Unrestricted {
            name_components: vec![],
            source: OwnedSymbolSource::Builtin,
        };
        let type0_entry = ContextEntry::Unrestricted {
            name_components: vec![IdentifierName::Reserved(
                ReservedIdentifierName::TypeTitleCase,
            )],
            source: OwnedSymbolSource::Builtin,
        };
        Self {
            stack: vec![type1_entry, type0_entry],
        }
    }
}

impl Context {
    pub fn push(&mut self, entry: ContextEntry) {
        self.stack.push(entry);
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
}

impl Context {
    pub fn level_to_index(&self, level: DbLevel) -> DbIndex {
        DbIndex(self.len() - level.0 - 1)
    }

    pub fn index_to_level(&self, index: DbIndex) -> DbLevel {
        DbLevel(self.len() - index.0 - 1)
    }
}

impl Context {
    pub fn get_symbol(&self, identifier: &ub::Identifier) -> Result<DbIndex, NameNotFoundError> {
        if let Some((
            ContextEntry::Unrestricted {
                name_components,
                source,
            },
            db_index,
        )) = self.lookup_name(&[&identifier.name])
        {
        } else {
            Err(NameNotFoundError {
                name: identifier.clone().into(),
            })
        }
    }

    fn lookup_name(&self, name_components: &[&IdentifierName]) -> Option<(&ContextEntry, DbIndex)> {
        self.stack
            .iter()
            .enumerate()
            .rev()
            .find_map(|(raw_index, entry)| {
                if entry
                    .name_components
                    .iter()
                    .eq(name_components.iter().copied())
                {
                    let db_level = DbLevel(raw_index);
                    let db_index = self.level_to_index(db_level);
                    Some((entry, db_index))
                } else {
                    None
                }
            })
    }

    pub fn add_unrestricted_unqualified_name_to_scope(
        &mut self,
        identifier: &ub::Identifier,
    ) -> Result<(), NameClashError> {
        self.check_for_name_clash(identifier)?;

        self.push(ContextEntry {
            name_components: vec![identifier.name.clone()],
            source: OwnedSymbolSource::Identifier(identifier.clone()),
            is_restricted: false,
        });

        Ok(())
    }

    fn check_for_name_clash(&self, identifier: &ub::Identifier) -> Result<(), NameClashError> {
        if let Some((entry, _)) = self.lookup_name(&[&identifier.name]) {
            return Err(NameClashError {
                old: entry.source.clone(),
                new: OwnedSymbolSource::Identifier(identifier.clone()),
            });
        } else {
            Ok(())
        }
    }
}

impl Context {
    pub fn add_restricted_dot_target_to_scope(
        &mut self,
        input: (Symbol, IdentifierName),
        output: (Symbol, OwnedSymbolSource),
    ) -> Result<(), DotExpressionRhsClashError> {
        self.check_for_dot_target_clash(&input, &output.1)?;

        self.scope_stack
            .last_mut()
            .expect("Tried to declare name in a zero-scope state.")
            .insert_restricted_dot_target(input, output);

        Ok(())
    }

    fn check_for_dot_target_clash(
        &self,
        input: &(Symbol, IdentifierName),
        output_source: &OwnedSymbolSource,
    ) -> Result<(), DotExpressionRhsClashError> {
        if let Some(data) = self.get_dot_target_symbol_data((input.0, &input.1)) {
            return Err(DotExpressionRhsClashError {
                old: data.source.clone(),
                new: output_source.clone(),
            });
        } else {
            Ok(())
        }
    }

    /// Panics if there is no entry corresponding to the given input.
    pub fn lift_dot_target_restriction(&mut self, input: (Symbol, &IdentifierName)) {
        self.scope_stack
            .last_mut()
            .expect("Tried to declare name in a zero-scope state.")
            .lift_dot_target_restriction(input);
    }
}

impl Context {
    /// Returns the zero-based De Bruijn index of the given symbol,
    /// where index zero corresponds to the **top** symbol.
    pub fn get_db_index(&self, symbol: Symbol) -> Option<DbIndex> {
        let bottom_index = self.get_db_level(symbol)?;
        Some(DbIndex(self.len() - bottom_index - 1))
    }

    /// Returns the zero-based De Bruijn level of the given symbol.
    /// A "De Bruijn _level_" is like a De Bruijn _index_, except that
    /// levels count from the bottom of the stack, rather than the top.
    /// That is, level zero corresponds to the **bottom** symbol.
    fn get_db_level(&self, symbol: Symbol) -> Option<usize> {
        for (scope_index, scope) in self.scope_stack.iter().enumerate().rev() {
            if let Some(data) = scope.get_symbol_data_by_symbol(symbol) {
                let local_index = data.index_within_scope;
                let offset: usize = self.scope_stack[0..scope_index]
                    .iter()
                    .map(|scope| scope.len())
                    .sum();
                return Some(offset + local_index);
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.scope_stack.iter().map(|scope| scope.len()).sum()
    }
}

use scope::*;
mod scope {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct Scope {
        /// Unqualified names are names that can be accessed without a dot.
        /// For example, in
        ///
        /// ```pamlihu
        /// type Nat {
        ///    .O: Nat,
        ///    .S(n: Nat): Nat,
        /// }
        /// ```
        ///
        /// ...`Nat` is the only unqualified name (excluding the implicitly defined `Type`, of course).
        unqualified_names: FxHashMap<IdentifierName, SymbolData>,

        dot_targets: FxHashMap<(Symbol, IdentifierName), (IsRestricted, SymbolData)>,
    }

    #[derive(Clone, Debug)]
    pub struct SymbolData {
        pub source: OwnedSymbolSource,
        pub symbol: Symbol,
        pub index_within_scope: usize,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct IsRestricted(bool);

    impl Scope {
        pub fn empty() -> Self {
            Self {
                unqualified_names: FxHashMap::default(),
                dot_targets: FxHashMap::default(),
            }
        }
    }

    impl Scope {
        pub fn get_symbol_data_by_name(&self, name: &IdentifierName) -> Option<&SymbolData> {
            self.unqualified_names.get(name)
        }

        pub fn get_symbol_data_by_symbol(&self, symbol: Symbol) -> Option<&SymbolData> {
            self.unqualified_names
                .values()
                .chain(self.dot_targets.values().map(|(_, data)| data))
                .find(|data| data.symbol == symbol)
        }

        pub fn len(&self) -> usize {
            self.unqualified_names.len() + self.dot_targets.len()
        }

        /// Panics if the name is already in the scope.
        pub fn insert_unqualified_name(
            &mut self,
            input: IdentifierName,
            output: (Symbol, OwnedSymbolSource),
        ) {
            let (symbol, source) = output;
            if self.unqualified_names.contains_key(&input) {
                panic!("Tried to insert a name that already exists in the scope.");
            }
            let data = SymbolData {
                source,
                symbol,
                index_within_scope: self.len(),
            };
            self.unqualified_names.insert(input, data);
        }

        /// Panics if the name is already in the scope.
        pub fn insert_restricted_dot_target(
            &mut self,
            input: (Symbol, IdentifierName),
            output: (Symbol, OwnedSymbolSource),
        ) {
            let data = SymbolData {
                symbol: output.0,
                source: output.1,
                index_within_scope: self.len(),
            };
            self.dot_targets.insert(input, (IsRestricted(true), data));
        }

        pub fn lift_dot_target_restriction(&mut self, input: (Symbol, &IdentifierName)) {
            if let Some(entry) = self.dot_targets.get_mut(&(input.0, input.1.clone())) {
                entry.0 = IsRestricted(false);
            } else {
                panic!("Tried to lift a restriction on a name that doesn't exist in the scope.");
            }
        }

        pub fn get_dot_target_symbol_data(
            &self,
            input: (Symbol, &IdentifierName),
        ) -> Option<&SymbolData> {
            Some(&self.dot_targets.get(&(input.0, input.1.clone()))?.1)
        }
    }
}
