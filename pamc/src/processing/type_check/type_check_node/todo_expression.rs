use super::*;

pub(in crate::processing::type_check) fn get_type_of_todo_dirty(
    _state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    id: NodeId<TodoExpression>,
) -> Result<NormalFormId, Tainted<TypeCheckError>> {
    if let Some(coercion_target_id) = coercion_target_id {
        Ok(coercion_target_id)
    } else {
        tainted_err(TypeCheckError::CannotInferTypeOfTodoExpression(id))
    }
}
