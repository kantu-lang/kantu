use super::*;

use crate::data::symbol_database::Symbol;

#[derive(Clone, Debug)]
pub struct TypeCheckContext {
    stack: Vec<Scope>,
}

#[derive(Clone, Copy, Debug)]
pub struct Substitution {
    pub from: SubstitutionLhs,
    pub to: NormalFormNodeId,
}

#[derive(Clone, Copy, Debug)]
pub enum SubstitutionLhs {
    Symbol(Symbol),
    Expression(NodeId<WrappedExpression>),
}

#[derive(Clone, Debug)]
struct Scope {
    map: TypeMap,
    substitutions_applied_to_previous_scopes: Vec<Substitution>,
}

impl Scope {
    fn new() -> Self {
        Self {
            map: TypeMap::empty(),
            substitutions_applied_to_previous_scopes: Vec::new(),
        }
    }
}

impl TypeCheckContext {
    pub fn new() -> Self {
        Self {
            stack: vec![Scope::new()],
        }
    }
}

impl TypeCheckContext {
    pub fn get(&self, symbol: Symbol) -> (NormalFormNodeId, Vec<&[Substitution]>) {
        self.try_get(symbol).expect(&format!(
            "Tried to get the type of {:?}, but it was not in the type map.",
            symbol
        ))
    }

    fn try_get(&self, symbol: Symbol) -> Option<(NormalFormNodeId, Vec<&[Substitution]>)> {
        let mut substitution_list_stack: Vec<&[Substitution]> = vec![];
        for scope in self.stack.iter().rev() {
            if let Some(type_id) = scope.map.try_get(symbol) {
                return Some((type_id, substitution_list_stack));
            }
            substitution_list_stack.push(&scope.substitutions_applied_to_previous_scopes);
        }
        None
    }

    pub fn insert_new(&mut self, symbol: Symbol, type_id: NormalFormNodeId) {
        if let Some((existing_type_id, substitutions)) = self.try_get(symbol) {
            panic!("Tried to insert new entry ({:?}, {:?}) into a context, when it already contained the entry ({:?}, {:?} + {} substitutions).", symbol, type_id, symbol, existing_type_id, substitutions.len());
        }
        self.stack.last_mut().expect("Error: Tried to insert an entry into a context with an empty scope stack. This indicates a serious logic error.").map.insert_new(symbol, type_id);
    }

    pub fn bottom_type_map(self) -> TypeMap {
        self.stack
                .into_iter()
                .next()
                .expect("Error: Tried to get the bottom type map from a context with an empty scope stack. This indicates a serious logic error.")
                .map
    }

    pub fn push_scope(&mut self) {
        self.stack.push(Scope::new());
    }

    pub fn pop_scope(&mut self) {
        self.stack.pop().expect("Error: Tried to pop a scope from a context with an empty scope stack. This indicates a serious logic error.");
    }

    pub fn apply_substitutions_to_top_scope(
        &mut self,
        registry: &mut NodeRegistry,
        symbol_db: &mut SymbolDatabase,
        substitutions: &[Substitution],
    ) -> Result<(), TypeError> {
        let top = self
                .stack
                .last_mut()
                .expect("Error: Tried to apply substitutions to the top scope of a context with an empty scope stack. This indicates a serious logic error.");
        apply_substitutions_to_map(registry, symbol_db, &mut top.map, substitutions)?;
        top.substitutions_applied_to_previous_scopes
            .extend(substitutions);
        Ok(())
    }
}

fn apply_substitutions_to_map(
    registry: &mut NodeRegistry,
    symbol_db: &mut SymbolDatabase,
    map: &mut TypeMap,
    substitutions: &[Substitution],
) -> Result<(), TypeError> {
    let keys = map.keys().collect::<Vec<_>>();
    for key in keys {
        let type_id = map.get(key);
        let substituted_type_id = apply_substitutions(
            registry,
            symbol_db,
            type_id.0,
            substitutions.iter().copied(),
        );
        let normalized_substituted_type_id =
            evaluate_well_typed_expression(registry, symbol_db, substituted_type_id)?;
        map.update(key, normalized_substituted_type_id);
    }

    Ok(())
}
