use super::*;

use rustc_hash::FxHashMap;

#[derive(Clone, Debug)]
pub struct State {
    provider: SymbolProvider,
    scope_stack: Vec<Scope>,
}

#[derive(Clone, Debug)]
pub enum OwnedSymbolSource {
    Identifier(ub::Identifier),
    Builtin(ReservedIdentifierName),
}

impl State {
    pub fn with_builtins() -> Self {
        let provider = SymbolProvider::new();
        let mut bottom_scope = Scope::empty();
        bottom_scope.insert_unqualified_name(
            IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase),
            (
                provider.type0_symbol(),
                OwnedSymbolSource::Builtin(ReservedIdentifierName::TypeTitleCase),
            ),
        );

        Self {
            provider,
            scope_stack: vec![bottom_scope],
        }
    }
}

impl State {
    pub fn push_scope(&mut self) {
        self.scope_stack.push(Scope::empty());
    }

    pub fn pop_scope_or_panic(&mut self) {
        self.scope_stack
            .pop()
            .expect("Tried to pop scope from empty stack");
    }
}

impl State {
    pub fn get_symbol(&self, identifier: &Identifier) -> Result<Symbol, NameNotFoundError> {
        if let Some(data) = self.get_symbol_data_for_name(&identifier.name) {
            Ok(data.symbol)
        } else {
            Err(NameNotFoundError {
                name: identifier.clone(),
            })
        }
    }

    fn get_symbol_data_for_name(&self, name: &IdentifierName) -> Option<&SymbolData> {
        for scope in self.scope_stack.iter().rev() {
            if let Some(data) = scope.get_symbol_data_by_name(name) {
                return Some(data);
            }
        }
        None
    }

    pub fn add_name_to_scope(
        &mut self,
        identifier: &ub::Identifier,
    ) -> Result<Symbol, NameClashError> {
        self.check_for_name_clash(identifier)?;

        let symbol = self.provider.new_symbol();
        self.scope_stack
            .last_mut()
            .expect("Tried to declare name in a zero-scope state.")
            .insert_unqualified_name(
                identifier.name.clone(),
                (symbol, OwnedSymbolSource::Identifier(identifier.clone())),
            );

        Ok(symbol)
    }

    fn check_for_name_clash(&self, identifier: &ub::Identifier) -> Result<(), NameClashError> {
        if let Some(data) = self.get_symbol_data_for_name(&identifier.name) {
            return Err(NameClashError {
                old: data.source.clone(),
                new: OwnedSymbolSource::Identifier(identifier.clone()),
            });
        } else {
            Ok(())
        }
    }

    pub fn new_symbol(&mut self) -> Symbol {
        self.provider.new_symbol()
    }
}

impl State {
    pub fn get_dot_target_symbol(
        &self,
        input: (Symbol, &Identifier),
    ) -> Result<Symbol, InvalidDotExpressionRhsError> {
        if let Some(data) = self.get_dot_target_symbol_data((input.0, &input.1.name)) {
            Ok(data.symbol)
        } else {
            Err(InvalidDotExpressionRhsError {
                rhs: input.1.clone(),
            })
        }
    }

    pub fn get_dot_target_symbol_data(
        &self,
        input: (Symbol, &IdentifierName),
    ) -> Option<&SymbolData> {
        for scope in self.scope_stack.iter().rev() {
            if let Some(symbol) = scope.get_dot_target_symbol_data(input) {
                return Some(symbol);
            }
        }
        None
    }

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
        if let Some(data) = self.get_symbol_data_for_name(&input.1) {
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

impl State {
    /// Returns the zero-based De Bruijn index of the given symbol,
    /// where index zero corresponds to the **top** symbol.
    pub fn get_db_index(&self, symbol: Symbol) -> Option<usize> {
        let bottom_index = self.get_db_level(symbol)?;
        Some(self.len() - bottom_index - 1)
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

impl State {
    pub fn into_provider_and_dot_targets(self) -> (SymbolProvider, SymbolToDotTargetsMap) {
        let dot_targets = {
            let mut out = SymbolToDotTargetsMap::empty();
            for scope in self.scope_stack {
                for ((left, name), data) in scope.into_dot_targets() {
                    out.insert(left, name, data.symbol);
                }
            }
            out
        };
        (self.provider, dot_targets)
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

    impl Scope {
        pub fn into_dot_targets(
            self,
        ) -> impl IntoIterator<Item = ((Symbol, IdentifierName), SymbolData)> {
            self.dot_targets
                .into_iter()
                .map(|(left, (_, data))| (left, data))
        }
    }
}
