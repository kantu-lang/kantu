use super::*;

pub use files::*;
mod files;

pub(in crate::processing::type_check) use file_item::*;
mod file_item;

pub(in crate::processing::type_check) use name_expression::*;
mod name_expression;

pub(in crate::processing::type_check) use todo_expression::*;
mod todo_expression;

pub(in crate::processing::type_check) use call::*;
mod call;

pub(in crate::processing::type_check) use fun::*;
mod fun;

pub(in crate::processing::type_check) use match_::*;
mod match_;

pub(in crate::processing::type_check) use forall::*;
mod forall;

pub(in crate::processing::type_check) use check::*;
mod check;

fn type_check_expression_dirty(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    expression: ExpressionId,
) -> Result<(), Tainted<TypeCheckError>> {
    // In the future, we could implement a version of this that skips the
    // allocations required by `get_type_of_expression`, since we don't
    // actually use the returned type.
    // But for now, we'll just reuse the existing code, for the sake of
    // simplicity.
    get_type_of_expression_dirty(state, coercion_target_id, expression).map(std::mem::drop)
}

fn get_type_of_expression(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    id: ExpressionId,
) -> Result<NormalFormId, TypeCheckError> {
    fn f(
        state: &mut State,
        (coercion_target_id, id): (Option<NormalFormId>, ExpressionId),
    ) -> Result<NormalFormId, Tainted<TypeCheckError>> {
        get_type_of_expression_dirty(state, coercion_target_id, id)
    }

    untaint_err(state, (coercion_target_id, id), f)
}

pub(in crate::processing::type_check) fn get_type_of_expression_dirty(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    id: ExpressionId,
) -> Result<NormalFormId, Tainted<TypeCheckError>> {
    match id {
        ExpressionId::Name(name) => Ok(get_type_of_name(state, name)),
        ExpressionId::Todo(todo) => get_type_of_todo_dirty(state, coercion_target_id, todo),
        ExpressionId::Call(call) => get_type_of_call_dirty(state, call),
        ExpressionId::Fun(fun) => get_type_of_fun_dirty(state, fun),
        ExpressionId::Match(match_) => get_type_of_match_dirty(state, coercion_target_id, match_),
        ExpressionId::Forall(forall) => get_type_of_forall_dirty(state, forall),
        ExpressionId::Check(check) => {
            get_type_of_check_expression_dirty(state, coercion_target_id, check)
        }
    }
}
