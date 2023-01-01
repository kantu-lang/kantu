use super::*;

pub(super) fn verify_expression_is_visible_from(
    state: &State,
    expression_id: ExpressionId,
    perspective: Visibility,
) -> Result<(), TypeCheckError> {
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
    expression_id: NodeId<NameExpression>,
    perspective: Visibility,
) -> Result<(), TypeCheckError> {
    unimplemented!()
}

fn verify_call_is_visible_from(
    state: &State,
    expression_id: NodeId<Call>,
    perspective: Visibility,
) -> Result<(), TypeCheckError> {
    unimplemented!()
}

fn verify_fun_is_visible_from(
    state: &State,
    expression_id: NodeId<Fun>,
    perspective: Visibility,
) -> Result<(), TypeCheckError> {
    unimplemented!()
}

fn verify_match_is_visible_from(
    state: &State,
    expression_id: NodeId<Match>,
    perspective: Visibility,
) -> Result<(), TypeCheckError> {
    unimplemented!()
}

fn verify_forall_is_visible_from(
    state: &State,
    expression_id: NodeId<Forall>,
    perspective: Visibility,
) -> Result<(), TypeCheckError> {
    unimplemented!()
}

fn verify_check_expression_is_visible_from(
    state: &State,
    expression_id: NodeId<Check>,
    perspective: Visibility,
) -> Result<(), TypeCheckError> {
    unimplemented!()
}
