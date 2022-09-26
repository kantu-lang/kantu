use crate::data::symbol_database::Symbol;

#[derive(Clone, Debug)]
pub struct SymbolProvider {
    type_title_case_symbol: Symbol,
    lowest_available_symbol: Symbol,
}

impl SymbolProvider {
    pub fn new() -> Self {
        Self {
            type_title_case_symbol: Symbol(0),
            lowest_available_symbol: Symbol(1),
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
    pub fn type_title_case(&self) -> Symbol {
        self.type_title_case_symbol
    }
}
