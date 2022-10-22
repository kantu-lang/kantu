use crate::data::{node_registry::NodeId, registered_sst::*};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Symbol(pub usize);

#[derive(Clone, Debug)]
pub struct SymbolDatabase {
    pub provider: SymbolProvider,
    pub identifier_symbols: IdentifierToSymbolMap,
    pub symbol_dot_targets: SymbolToDotTargetsMap,
    pub symbol_sources: SymbolSourceMap,
}

pub use symbol_provider::*;
mod symbol_provider {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct SymbolProvider {
        type0_symbol: Symbol,
        type1_symbol: Symbol,
        lowest_available_symbol: Symbol,
    }

    impl SymbolProvider {
        pub fn new() -> Self {
            Self {
                type0_symbol: Symbol(0),
                type1_symbol: Symbol(1),
                lowest_available_symbol: Symbol(2),
            }
        }
    }

    impl Default for SymbolProvider {
        fn default() -> Self {
            Self::new()
        }
    }

    impl SymbolProvider {
        pub fn new_symbol(&mut self) -> Symbol {
            let symbol = self.lowest_available_symbol;
            self.lowest_available_symbol.0 += 1;
            symbol
        }
    }

    impl SymbolProvider {
        pub fn type0_symbol(&self) -> Symbol {
            self.type0_symbol
        }

        pub fn type1_symbol(&self) -> Symbol {
            self.type1_symbol
        }
    }
}

pub use identifier_to_symbol_map::*;
mod identifier_to_symbol_map {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct IdentifierToSymbolMap {
        map: Vec<Option<Symbol>>,
    }

    impl IdentifierToSymbolMap {
        pub fn empty() -> Self {
            Self { map: Vec::new() }
        }
    }

    impl IdentifierToSymbolMap {
        pub fn get(&self, identifier_id: NodeId<Identifier>) -> Symbol {
            self.try_get(identifier_id).expect(&format!(
                "Symbol could not be found for {:?}",
                identifier_id
            ))
        }

        pub fn try_get(&self, identifier_id: NodeId<Identifier>) -> Option<Symbol> {
            if identifier_id.raw >= self.map.len() {
                None
            } else {
                self.map[identifier_id.raw]
            }
        }

        pub fn contains(&self, identifier_id: NodeId<Identifier>) -> bool {
            self.try_get(identifier_id).is_some()
        }

        pub fn insert(&mut self, identifier_id: NodeId<Identifier>, symbol: Symbol) -> bool {
            let is_newly_inserted = !self.contains(identifier_id);

            if identifier_id.raw >= self.map.len() {
                self.map.resize(identifier_id.raw + 1, None);
            }
            self.map[identifier_id.raw] = Some(symbol);

            is_newly_inserted
        }
    }
}

pub use dot_targets::*;
mod dot_targets {
    use super::*;

    use rustc_hash::FxHashMap;

    #[derive(Clone, Debug)]
    pub struct SymbolToDotTargetsMap(FxHashMap<Symbol, FxHashMap<IdentifierName, Symbol>>);

    impl SymbolToDotTargetsMap {
        pub fn empty() -> Self {
            SymbolToDotTargetsMap(FxHashMap::default())
        }
    }

    impl SymbolToDotTargetsMap {
        pub fn insert(
            &mut self,
            symbol: Symbol,
            target_name: IdentifierName,
            target_symbol: Symbol,
        ) {
            if let Some(targets) = self.0.get_mut(&symbol) {
                targets.insert(target_name, target_symbol);
            } else {
                let mut targets = FxHashMap::default();
                targets.insert(target_name, target_symbol);
                self.0.insert(symbol, targets);
            }
        }

        pub fn get(&self, symbol: Symbol, target_name: &IdentifierName) -> Option<Symbol> {
            self.0
                .get(&symbol)
                .and_then(|targets| targets.get(target_name))
                .copied()
        }

        pub fn get_all(&self, symbol: Symbol) -> Option<impl Iterator<Item = &IdentifierName>> {
            self.0
                .get(&symbol)
                .map(std::collections::hash_map::HashMap::keys)
        }
    }
}

pub use symbol_source::*;
mod symbol_source {
    use super::*;

    use rustc_hash::FxHashMap;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SymbolSource {
        Type(NodeId<TypeStatement>),
        Variant(NodeId<Variant>),
        TypedParam(NodeId<Param>),
        MatchCaseParam(NodeId<Identifier>, NodeId<MatchCase>, NodeId<Match>),
        Let(NodeId<LetStatement>),
        Fun(NodeId<Fun>),
        BuiltinTypeTitleCase,
    }

    // TODO: Wrap this so end users are not exposed to a third-party
    // API (i.e., FxHashMap, in this case).
    pub type SymbolSourceMap = FxHashMap<Symbol, SymbolSource>;
}
