use super::*;

pub(super) fn type_check_file_item_dirty(
    state: &mut State,
    item: FileItemNodeId,
) -> Result<PushWarning, Tainted<TypeCheckError>> {
    match item {
        FileItemNodeId::Type(type_statement) => {
            type_check_type_statement_dirty(state, type_statement)
        }
        FileItemNodeId::Let(let_statement) => type_check_let_statement_dirty(state, let_statement),
    }
}

pub(super) fn type_check_type_statement_dirty(
    state: &mut State,
    type_statement_id: NodeId<TypeStatement>,
) -> Result<PushWarning, Tainted<TypeCheckError>> {
    type_check_type_constructor_dirty(state, type_statement_id)??;

    let type_statement = state.registry.get(type_statement_id);
    let variant_ids = state
        .registry
        .get_possibly_empty_list(type_statement.variant_list_id)
        .to_vec();
    for variant_id in variant_ids {
        type_check_type_variant_dirty(state, variant_id)??;
    }

    Ok(with_push_warning(()))
}

pub(super) fn type_check_type_constructor_dirty(
    state: &mut State,
    type_statement_id: NodeId<TypeStatement>,
) -> Result<PushWarning, Tainted<TypeCheckError>> {
    let type_statement = state.registry.get(type_statement_id).clone();
    let arity = type_statement.param_list_id.len();
    let normalized_param_list_id = normalize_optional_params_and_leave_params_in_context_dirty(
        state,
        type_statement.param_list_id,
    )??;
    let type_constructor_type_id = NormalFormId::unchecked_new(
        PossiblyNullaryForall {
            id: dummy_id(),
            span: None,
            param_list_id: normalized_param_list_id,
            output_id: type0_expression(state).raw(),
        }
        .into_id(state.registry)
        .without_spans(state.registry),
    );
    state.context.pop_n(arity);

    let variant_name_list_id = {
        let variant_ids = state
            .registry
            .get_possibly_empty_list(type_statement.variant_list_id);
        let variant_name_ids: Vec<_> = variant_ids
            .iter()
            .map(|&variant_id| state.registry.get(variant_id).name_id)
            .collect();
        state.registry.add_possibly_empty_list(variant_name_ids)
    };
    Ok(state.context.push(ContextEntry {
        type_id: type_constructor_type_id,
        definition: ContextEntryDefinition::Adt {
            variant_name_list_id,
        },
    }))
}

pub(in crate::processing::type_check) fn type_check_unlabeled_param_dirty(
    state: &mut State,
    param_id: NodeId<UnlabeledParam>,
) -> Result<PushWarning, Tainted<TypeCheckError>> {
    let param = state.registry.get(param_id).clone();
    let param_type_type_id = get_type_of_expression_dirty(state, None, param.type_id)?;
    if !is_term_equal_to_type0_or_type1(state, param_type_type_id) {
        return tainted_err(TypeCheckError::IllegalTypeExpression(param.type_id));
    }

    let normalized_type_id = evaluate_well_typed_expression(state, param.type_id);
    Ok(state.context.push(ContextEntry {
        type_id: normalized_type_id,
        definition: ContextEntryDefinition::Uninterpreted,
    }))
}

pub(in crate::processing::type_check) fn type_check_labeled_param_dirty(
    state: &mut State,
    param_id: NodeId<LabeledParam>,
) -> Result<PushWarning, Tainted<TypeCheckError>> {
    let param = state.registry.get(param_id).clone();
    let param_type_type_id = get_type_of_expression_dirty(state, None, param.type_id)?;
    if !is_term_equal_to_type0_or_type1(state, param_type_type_id) {
        return tainted_err(TypeCheckError::IllegalTypeExpression(param.type_id));
    }

    let normalized_type_id = evaluate_well_typed_expression(state, param.type_id);
    Ok(state.context.push(ContextEntry {
        type_id: normalized_type_id,
        definition: ContextEntryDefinition::Uninterpreted,
    }))
}

pub(super) fn type_check_type_variant_dirty(
    state: &mut State,
    variant_id: NodeId<Variant>,
) -> Result<PushWarning, Tainted<TypeCheckError>> {
    let variant = state.registry.get(variant_id).clone();
    let arity = variant.param_list_id.len();
    let normalized_param_list_id =
        normalize_optional_params_and_leave_params_in_context_dirty(state, variant.param_list_id)??;
    type_check_expression_dirty(state, None, variant.return_type_id)?;
    let return_type_id = evaluate_well_typed_expression(state, variant.return_type_id);
    let type_id = NormalFormId::unchecked_new(
        PossiblyNullaryForall {
            id: dummy_id(),
            span: None,
            param_list_id: normalized_param_list_id,
            output_id: return_type_id.raw(),
        }
        .into_id(state.registry)
        .without_spans(state.registry),
    );
    state.context.pop_n(arity);
    Ok(state.context.push(ContextEntry {
        type_id,
        definition: ContextEntryDefinition::Variant {
            name_id: variant.name_id,
        },
    }))
}

pub(super) fn type_check_let_statement_dirty(
    state: &mut State,
    let_statement_id: NodeId<LetStatement>,
) -> Result<PushWarning, Tainted<TypeCheckError>> {
    let let_statement = state.registry.get(let_statement_id).clone();
    let type_id = get_type_of_expression_dirty(state, None, let_statement.value_id)?;
    verify_expression_is_visible_from(state, type_id.raw(), let_statement.visibility)
        .map_err(Tainted::new)?;
    let normalized_value_id = evaluate_well_typed_expression(state, let_statement.value_id);
    Ok(state.context.push(ContextEntry {
        type_id,
        definition: ContextEntryDefinition::Alias {
            value_id: normalized_value_id,
        },
    }))
}
