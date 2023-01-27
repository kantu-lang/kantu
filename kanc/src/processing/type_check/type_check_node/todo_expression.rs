use super::*;

pub(in crate::processing::type_check) fn get_type_of_todo_dirty(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    id: &'a TodoExpression<'a>,
) -> Result<NormalFormId, Tainted<TypeCheckError>> {
    state.warnings.push(TypeCheckWarning::TodoExpression(id));

    if let Some(coercion_target_id) = coercion_target_id {
        Ok(coercion_target_id)
    } else {
        tainted_err(TypeCheckError::CannotInferTypeOfTodoExpression(id))
    }
}
