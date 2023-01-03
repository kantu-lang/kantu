use super::*;

pub(in crate::processing::type_check) fn get_type_of_forall_dirty(
    state: &mut State,
    forall_id: NodeId<Forall>,
) -> Result<NormalFormId, Tainted<TypeCheckError>> {
    let forall = state.registry.get(forall_id).clone();
    let _param_list_id =
        normalize_params_and_leave_params_in_context_dirty(state, forall.param_list_id)??;

    let output_type_id = get_type_of_expression_dirty(state, None, forall.output_id)?;
    if !is_term_equal_to_type0_or_type1(state, output_type_id) {
        return tainted_err(TypeCheckError::IllegalTypeExpression(forall.output_id));
    }

    state.context.pop_n(forall.param_list_id.len());

    Ok(type0_expression(state))
}
