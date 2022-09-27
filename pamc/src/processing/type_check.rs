use crate::data::{
    node_registry::{NodeId, NodeRegistry},
    registered_ast::*,
    symbol_database::SymbolDatabase,
    type_map::{NormalFormId, TypeMap},
};

#[derive(Clone, Debug)]
pub enum TypeError {
    IllegalParamType {
        param: NodeId<Param>,
        type_type: NodeId<WrappedExpression>,
    },
}

pub fn type_check_file(
    registry: &mut NodeRegistry,
    symbol_db: &mut SymbolDatabase,
    file_id: NodeId<File>,
) -> Result<TypeMap, TypeError> {
    let file = registry.file(file_id);
    let file_item_ids = file.item_ids.clone();
    let mut state = TypeCheckState {
        registry,
        symbol_db,
        context: TypeCheckContext::new(),
    };
    for item_id in file_item_ids {
        match item_id {
            FileItemNodeId::Type(type_id) => {
                type_check_type_statement(&mut state, type_id)?;
            }
            FileItemNodeId::Let(function_id) => {
                type_check_let_statement(&mut state, function_id)?;
            }
        }
    }
    Ok(state.context.bottom_type_map())
}

fn type_check_type_statement(
    state: &mut TypeCheckState,
    type_statement_id: NodeId<TypeStatement>,
) -> Result<(), TypeError> {
    let variant_ids: Vec<NodeId<Variant>> = state
        .registry
        .type_statement(type_statement_id)
        .variant_ids
        .clone();
    for variant_id in variant_ids {
        type_check_variant(state, variant_id)?;
    }
    Ok(())
}

fn type_check_let_statement(
    _state: &mut TypeCheckState,
    _let_statement: NodeId<LetStatement>,
) -> Result<(), TypeError> {
    // TODO: Actually implement (or remove) type_check_let_statement
    Ok(())
}

fn type_check_variant(
    state: &mut TypeCheckState,
    variant: NodeId<Variant>,
) -> Result<(), TypeError> {
    let param_ids: Vec<NodeId<Param>> = state.registry.variant(variant).param_ids.clone();
    for param_id in param_ids {
        type_check_param(state, param_id)?;
    }
    Ok(())
}

fn type_check_param(state: &mut TypeCheckState, param_id: NodeId<Param>) -> Result<(), TypeError> {
    let type_id = state.registry.param(param_id).type_id;
    let type_type_id = type_check_expression(state, type_id)?.0;
    let type_type = state.registry.wrapped_expression(type_type_id);
    match &type_type.expression {
        Expression::Identifier(identifier) => {
            let symbol = state.symbol_db.identifier_symbols.get(identifier.id);
            if !(symbol == state.symbol_db.provider.type0_symbol()
                || symbol == state.symbol_db.provider.type1_symbol())
            {
                return Err(TypeError::IllegalParamType {
                    param: param_id,
                    type_type: type_type_id,
                });
            }
        }
        _other_type_type => {
            return Err(TypeError::IllegalParamType {
                param: param_id,
                type_type: type_type_id,
            })
        }
    }

    let param_name_id = state.registry.param(param_id).name_id;
    let param_symbol = state.symbol_db.identifier_symbols.get(param_name_id);
    let type_normal_form_id = evaluate_well_typed_expression(state, type_id)?;
    state.context.insert_new(param_symbol, type_normal_form_id);

    Ok(())
}

fn type_check_expression(
    state: &mut TypeCheckState,
    id: NodeId<WrappedExpression>,
) -> Result<NormalFormId, TypeError> {
    match &state.registry.wrapped_expression(id).expression {
        Expression::Identifier(identifier) => {
            let symbol = state.symbol_db.identifier_symbols.get(identifier.id);
            let (unsubstituted_type_id, substitutions) = state.context.get(symbol);
            let unnormalized_type_id = apply_substitutions(
                &mut state.registry,
                &state.symbol_db,
                unsubstituted_type_id.0,
                substitutions
                    .iter()
                    .flat_map(std::ops::Deref::deref)
                    .copied(),
            );
            let type_id = evaluate_well_typed_expression(state, unnormalized_type_id)?;
            Ok(type_id)
        }
        Expression::Dot(dot) => {
            let symbol = state.symbol_db.identifier_symbols.get(dot.right_id);
            let (unsubstituted_type_id, substitutions) = state.context.get(symbol);
            let unnormalized_type_id = apply_substitutions(
                &mut state.registry,
                &state.symbol_db,
                unsubstituted_type_id.0,
                substitutions
                    .iter()
                    .flat_map(std::ops::Deref::deref)
                    .copied(),
            );
            let type_id = evaluate_well_typed_expression(state, unnormalized_type_id)?;
            Ok(type_id)
        }
        _ => unimplemented!(),
    }
}

fn evaluate_well_typed_expression(
    _state: &mut TypeCheckState,
    _expression: NodeId<WrappedExpression>,
) -> Result<NormalFormId, TypeError> {
    unimplemented!();
}

#[derive(Debug)]
struct TypeCheckState<'a> {
    registry: &'a mut NodeRegistry,
    symbol_db: &'a mut SymbolDatabase,
    context: TypeCheckContext,
}

use context::*;
mod context {
    use super::*;

    use crate::data::symbol_database::Symbol;

    #[derive(Clone, Debug)]
    pub struct TypeCheckContext {
        stack: Vec<Scope>,
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Substitution {
        pub from: Symbol,
        pub to: NormalFormId,
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
        pub fn get(&self, symbol: Symbol) -> (NormalFormId, Vec<&[Substitution]>) {
            self.try_get(symbol).expect(&format!(
                "Tried to get the type of {:?}, but it was not in the type map.",
                symbol
            ))
        }

        fn try_get(&self, symbol: Symbol) -> Option<(NormalFormId, Vec<&[Substitution]>)> {
            let mut substitution_list_stack: Vec<&[Substitution]> = vec![];
            for scope in self.stack.iter().rev() {
                if let Some(type_id) = scope.map.try_get(symbol) {
                    return Some((type_id, substitution_list_stack));
                }
                substitution_list_stack.push(&scope.substitutions_applied_to_previous_scopes);
            }
            None
        }

        pub fn insert_new(&mut self, symbol: Symbol, type_id: NormalFormId) {
            if let Some((existing_type_id, substitutions)) = self.try_get(symbol) {
                panic!("Tried to insert new entry ({:?}, {:?}) into a context, when it already contained the entry ({:?}, {:?} + {} substitutions).", symbol, type_id, symbol, existing_type_id, substitutions.len());
            }
            self.stack.last_mut().expect("Error: Tried to insert an entry into a context with an empty stack scope. This indicates a serious logic error.").map.insert_new(symbol, type_id);
        }

        pub fn bottom_type_map(self) -> TypeMap {
            self.stack
                .into_iter()
                .next()
                .expect("Error: Tried to get the bottom type map from a context with an empty stack scope. This indicates a serious logic error.")
                .map
        }
    }
}

fn apply_substitutions(
    registry: &mut NodeRegistry,
    symbol_db: &SymbolDatabase,
    type_id: NodeId<WrappedExpression>,
    substitutions: impl IntoIterator<Item = Substitution>,
) -> NodeId<WrappedExpression> {
    let mut type_id = type_id;
    for substitution in substitutions {
        type_id = apply_substitution(registry, symbol_db, type_id, substitution);
    }
    type_id
}

fn apply_substitution(
    _registry: &mut NodeRegistry,
    _symbol_db: &SymbolDatabase,
    _type_id: NodeId<WrappedExpression>,
    _substitutions: Substitution,
) -> NodeId<WrappedExpression> {
    unimplemented!()
}
