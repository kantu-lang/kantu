use super::*;

pub(in crate::processing::type_check) fn get_type_of_name(
    state: &mut State,
    name_id: &'a NameExpression<'a>,
) -> NormalFormId {
    let name = state.registry.get(name_id);
    state.context.get_type(name.db_index, state.registry)
}
