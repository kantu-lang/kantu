use super::*;

pub(super) fn verify_expression_is_visible_from(
    state: &State,
    expression_id: ExpressionId,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    match expression_id {
        ExpressionId::Name(id) => verify_name_expression_is_visible_from(state, id, perspective),
        ExpressionId::Todo(_) => Ok(()),
        ExpressionId::Call(id) => verify_call_is_visible_from(state, id, perspective),
        ExpressionId::Fun(id) => verify_fun_is_visible_from(state, id, perspective),
        ExpressionId::Match(id) => verify_match_is_visible_from(state, id, perspective),
        ExpressionId::Forall(id) => verify_forall_is_visible_from(state, id, perspective),
        ExpressionId::Check(id) => verify_check_expression_is_visible_from(state, id, perspective),
    }
}

fn verify_name_expression_is_visible_from(
    state: &State,
    id: NodeId<NameExpression>,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    let name = state.registry.get(id);
    let visibility = state.context.get_visibility(name.db_index);
    if !is_left_at_least_as_permissive_as_right(state.file_tree, visibility.0, perspective.0) {
        return Err(id);
    }
    Ok(())
}

fn verify_call_is_visible_from(
    state: &State,
    id: NodeId<Call>,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    let call = state.registry.get(id);
    verify_expression_is_visible_from(state, call.callee_id, perspective)?;
    verify_arg_list_is_visible_from(state, call.arg_list_id, perspective)?;
    Ok(())
}

fn verify_arg_list_is_visible_from(
    state: &State,
    id: NonEmptyCallArgListId,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    match id {
        NonEmptyCallArgListId::Unlabeled(id) => {
            verify_expression_list_is_visible_from(state, id, perspective)
        }
        NonEmptyCallArgListId::UniquelyLabeled(id) => {
            verify_labeled_call_arg_list_is_visible_from(state, id, perspective)
        }
    }
}

fn verify_expression_list_is_visible_from(
    state: &State,
    list_id: NonEmptyListId<ExpressionId>,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    let list = state.registry.get_list(list_id);
    for &id in list.iter() {
        verify_expression_is_visible_from(state, id, perspective)?;
    }
    Ok(())
}

fn verify_labeled_call_arg_list_is_visible_from(
    state: &State,
    list_id: NonEmptyListId<LabeledCallArgId>,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    let list = state.registry.get_list(list_id);
    for &id in list.iter() {
        verify_labeled_call_arg_is_visible_from(state, id, perspective)?;
    }
    Ok(())
}

fn verify_labeled_call_arg_is_visible_from(
    state: &State,
    id: LabeledCallArgId,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    match id {
        LabeledCallArgId::Explicit {
            label_id: _,
            value_id,
        } => verify_expression_is_visible_from(state, value_id, perspective),
        LabeledCallArgId::Implicit {
            label_id: _,
            db_index: _,
            value_id,
        } => verify_name_expression_is_visible_from(state, value_id, perspective),
    }
}

fn verify_fun_is_visible_from(
    state: &State,
    id: NodeId<Fun>,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    unimplemented!()
}

fn verify_match_is_visible_from(
    state: &State,
    id: NodeId<Match>,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    unimplemented!()
}

fn verify_forall_is_visible_from(
    state: &State,
    id: NodeId<Forall>,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    unimplemented!()
}

fn verify_check_expression_is_visible_from(
    state: &State,
    id: NodeId<Check>,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    unimplemented!()
}
