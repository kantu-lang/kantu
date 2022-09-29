use crate::data::{
    node_registry::{ListId, NodeId, NodeRegistry},
    registered_ast::*,
    symbol_database::{Symbol, SymbolDatabase, SymbolSource},
    type_map::{NormalFormNodeId, TypeMap},
    variant_return_type::VariantReturnTypeDatabase,
};

#[derive(Clone, Debug)]
pub enum TypeError {
    IllegalParamType {
        param: NodeId<Param>,
        type_type: NodeId<WrappedExpression>,
    },
    CalleeNotAFunction {
        callee: NodeId<WrappedExpression>,
        callee_type: NodeId<WrappedExpression>,
    },
    WrongNumberOfArguments {
        call: NodeId<Call>,
        param_arity: usize,
        arg_arity: usize,
    },
    WrongArgumentType {
        arg_id: NodeId<WrappedExpression>,
        param_type: NodeId<WrappedExpression>,
        arg_type: NodeId<WrappedExpression>,
    },
}

pub fn type_check_file(
    registry: &mut NodeRegistry,
    symbol_db: &mut SymbolDatabase,
    variant_return_type_args: &VariantReturnTypeDatabase,
    file_id: NodeId<File>,
) -> Result<TypeMap, TypeError> {
    let file = registry.file(file_id);
    let file_item_ids = registry.file_item_list(file.item_list_id).to_vec();
    let mut state = TypeCheckState {
        registry,
        symbol_db,
        variant_return_type_args,
        context: TypeCheckContext::new(),
        type0_identifier_id: todo(),
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

fn todo<T>() -> T {
    unimplemented!()
}

fn type_check_type_statement(
    state: &mut TypeCheckState,
    type_statement_id: NodeId<TypeStatement>,
) -> Result<(), TypeError> {
    let type_statement = state.registry.type_statement(type_statement_id);
    let param_ids = state
        .registry
        .param_list(type_statement.param_list_id)
        .to_vec();
    for param_id in &param_ids {
        type_check_param(state, *param_id)?;
    }

    let type_name_type_id = if param_ids.is_empty() {
        state.type0_identifier_id
    } else {
        let normalized_param_list_id = normalize_params(state, param_ids.iter().copied())?;
        let forall_with_dummy_id = Forall {
            id: dummy_id(),
            param_list_id: normalized_param_list_id,
            output_id: state.type0_identifier_id.0,
        };
        let forall_id = state
            .registry
            .add_forall_and_overwrite_its_id(forall_with_dummy_id);
        let registered_forall = state.registry.forall(forall_id).clone();
        let wrapped_with_dummy_id = WrappedExpression {
            id: dummy_id(),
            expression: Expression::Forall(Box::new(registered_forall)),
        };
        let wrapped_id = state
            .registry
            .add_wrapped_expression_and_overwrite_its_id(wrapped_with_dummy_id);
        NormalFormNodeId(wrapped_id)
    };
    let type_statement = state.registry.type_statement(type_statement_id);
    let type_name_symbol = state
        .symbol_db
        .identifier_symbols
        .get(type_statement.name_id);
    state
        .context
        .insert_new(type_name_symbol, type_name_type_id);

    let variant_ids: Vec<NodeId<Variant>> = state
        .registry
        .variant_list(type_statement.variant_list_id)
        .to_vec();
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
    variant_id: NodeId<Variant>,
    type_identifier_id: NodeId<Identifier>,
) -> Result<(), TypeError> {
    let variant = state.registry.variant(variant_id);
    let param_ids = state.registry.param_list(variant.param_list_id).to_vec();
    for param_id in &param_ids {
        type_check_param(state, *param_id)?;
    }

    let normalized_param_list_id = normalize_params(state, param_ids.iter().copied())?;

    let return_type_arg_list_id = state.variant_return_type_args.get(variant_id);
    let return_type_arg_ids = state
        .registry
        .wrapped_expression_list(return_type_arg_list_id)
        .to_vec();
    for return_type_arg_id in &return_type_arg_ids {
        type_check_expression(state, *return_type_arg_id)?;
    }
    // let normalized_return_type_arg_ids: Vec<NodeId<WrappedExpression>> = return_type_arg_ids
    //     .iter()
    //     .map(|id| Ok(evaluate_well_typed_expression(state, *id)?.0))
    //     .collect::<Result<Vec<_>, TypeError>>()?;
    // let normalized_return_type_arg_list_id = state
    //     .registry
    //     .add_wrapped_expression_list(normalized_return_type_arg_ids);

    let variant_type_id = if param_ids.is_empty() {
        let call_id = state.registry.add_call_and_overwrite_its_id(Call {
            id: dummy_id(),
            callee_id: type_identifier_id,
            arg_list_id: normalized_return_type_arg_list_id,
        });
    } else {
        unimplemented!();
    };

    // We need to add the type for the declared variant to the context.
    // If `variant` is a variant of type T, then the type of `variant` is either
    // `T` or `forall() { T }`.
    unimplemented!();
    Ok(())
}

fn type_check_param(state: &mut TypeCheckState, param_id: NodeId<Param>) -> Result<(), TypeError> {
    // TODO Review
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
        // TODO: Should we really be this strict?
        // Answer: No. Otherwise `List(T)` would be disallowed.
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
) -> Result<NormalFormNodeId, TypeError> {
    match &state.registry.wrapped_expression(id).expression {
        Expression::Identifier(identifier) => {
            let symbol = state.symbol_db.identifier_symbols.get(identifier.id);
            let type_id = get_normalized_type(state, symbol)?;
            Ok(type_id)
        }
        Expression::Dot(dot) => {
            let symbol = state.symbol_db.identifier_symbols.get(dot.right_id);
            let type_id = get_normalized_type(state, symbol)?;
            Ok(type_id)
        }
        Expression::Call(call) => {
            let call_id = call.id;
            let callee_id = call.callee_id;
            let arg_list_id = call.arg_list_id;
            let callee_type_id = type_check_expression(state, callee_id)?;
            let callee_type: Forall = match &state
                .registry
                .wrapped_expression(callee_type_id.0)
                .expression
            {
                Expression::Forall(forall) => (**forall).clone(),
                _ => {
                    return Err(TypeError::CalleeNotAFunction {
                        callee: callee_id,
                        callee_type: callee_type_id.0,
                    })
                }
            };
            let param_ids = state.registry.param_list(callee_type.param_list_id);
            let arg_ids = state.registry.wrapped_expression_list(arg_list_id);
            if param_ids.len() != arg_ids.len() {
                return Err(TypeError::WrongNumberOfArguments {
                    call: call_id,
                    param_arity: param_ids.len(),
                    arg_arity: arg_ids.len(),
                });
            }

            let arg_ids_and_arg_type_ids: Vec<(NodeId<WrappedExpression>, NormalFormNodeId)> =
                arg_ids
                    .to_vec()
                    .iter()
                    .map(|arg_id| -> Result<(NodeId<WrappedExpression>, NormalFormNodeId), TypeError> {
                        Ok((*arg_id, type_check_expression(state, *arg_id)?))
                    })
                    .collect::<Result<Vec<_>, TypeError>>()?;

            let param_ids = state
                .registry
                .param_list(callee_type.param_list_id)
                .to_vec();

            for (param_id, (arg_id, arg_type_id)) in param_ids
                .iter()
                .copied()
                .zip(arg_ids_and_arg_type_ids.iter().copied())
            {
                let param = state.registry.param(param_id);
                let param_symbol = state.symbol_db.identifier_symbols.get(param.name_id);
                let param_type_id = get_normalized_type(state, param_symbol)?;
                if !are_types_equal(state, param_type_id, arg_type_id) {
                    return Err(TypeError::WrongArgumentType {
                        arg_id,
                        param_type: param_type_id.0,
                        arg_type: arg_type_id.0,
                    });
                }
            }

            let substitutions: Vec<Substitution> = param_ids
                .iter()
                .copied()
                .zip(arg_ids_and_arg_type_ids.iter().copied())
                .map(
                    |(param_id, (arg_id, _))| -> Result<Substitution, TypeError> {
                        let normalized_arg_id = evaluate_well_typed_expression(state, arg_id)?;
                        let param = state.registry.param(param_id);
                        let param_symbol = state.symbol_db.identifier_symbols.get(param.name_id);
                        Ok(Substitution {
                            from: param_symbol,
                            to: normalized_arg_id,
                        })
                    },
                )
                .collect::<Result<Vec<_>, TypeError>>()?;
            let unnormalized_return_type_id = apply_substitutions(
                &mut state.registry,
                &state.symbol_db,
                callee_type.output_id,
                substitutions,
            );
            let return_type_id =
                evaluate_well_typed_expression(state, unnormalized_return_type_id)?;

            Ok(return_type_id)
        }
        _ => unimplemented!(),
    }
}

fn evaluate_well_typed_expression(
    _state: &mut TypeCheckState,
    _expression: NodeId<WrappedExpression>,
) -> Result<NormalFormNodeId, TypeError> {
    unimplemented!();
}

fn get_normalized_type(
    state: &mut TypeCheckState,
    symbol: Symbol,
) -> Result<NormalFormNodeId, TypeError> {
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
    evaluate_well_typed_expression(state, unnormalized_type_id)
}

// TODO: This should just lookup the param symbol in the context,
// since this should only be called after the param has been type checked.
// We should probably remove this entirely.

/// Every param id yielded by `param_ids` **must** be a param that has been
/// type checked.
fn normalize_params(
    state: &mut TypeCheckState,
    param_ids: impl IntoIterator<Item = NodeId<Param>>,
) -> Result<ListId<NodeId<Param>>, TypeError> {
    let normalized_param_ids: Vec<NodeId<Param>> = param_ids
        .into_iter()
        .map(|id| -> Result<NodeId<Param>, TypeError> {
            let param = state.registry.param(id);
            let param_type_id = param.type_id;
            let normalized_param_with_dummy_id = Param {
                id: dummy_id(),
                is_dashed: param.is_dashed,
                name_id: param.name_id,
                // It's safe to call `evaluate_well_typed_expression`
                // because we type-checked it above.
                type_id: evaluate_well_typed_expression(state, param_type_id)?.0,
            };
            Ok(state
                .registry
                .add_param_and_overwrite_its_id(normalized_param_with_dummy_id))
        })
        .collect::<Result<Vec<_>, TypeError>>()?;
    Ok(state.registry.add_param_list(normalized_param_ids))
}

fn are_types_equal(_state: &TypeCheckState, _a: NormalFormNodeId, _b: NormalFormNodeId) -> bool {
    unimplemented!()
}

fn dummy_id<T>() -> NodeId<T> {
    NodeId::new(0)
}

#[derive(Debug)]
struct TypeCheckState<'a> {
    registry: &'a mut NodeRegistry,
    symbol_db: &'a mut SymbolDatabase,
    variant_return_type_args: &'a VariantReturnTypeDatabase,
    context: TypeCheckContext,
    type0_identifier_id: NormalFormNodeId,
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
        pub to: NormalFormNodeId,
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
