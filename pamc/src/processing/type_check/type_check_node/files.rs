use super::*;

pub fn type_check_files(
    registry: &mut NodeRegistry,
    file_ids: &[TypePositivityValidated<NodeId<File>>],
) -> Result<Vec<TypeCheckWarning>, TypeCheckError> {
    let mut context = Context::with_builtins(registry);
    let mut substitution_context = SubstitutionContext::empty();
    let mut equality_checker = NodeEqualityChecker::new();
    let mut warnings = vec![];
    let mut state = State {
        context: &mut context,
        substitution_context: &mut substitution_context,
        registry,
        equality_checker: &mut equality_checker,
        warnings: &mut warnings,
    };
    for &id in file_ids {
        type_check_file(&mut state, id.raw())?;
    }
    Ok(warnings)
}

pub(super) fn type_check_file(
    state: &mut State,
    file_id: NodeId<File>,
) -> Result<(), TypeCheckError> {
    untaint_err(state, file_id, type_check_file_dirty)
}

pub(super) fn type_check_file_dirty(
    state: &mut State,
    file_id: NodeId<File>,
) -> Result<(), Tainted<TypeCheckError>> {
    let file = state.registry.get(file_id);
    let items = state
        .registry
        .get_possibly_empty_list(file.item_list_id)
        .to_vec();
    for &item_id in &items {
        type_check_file_item_dirty(state, item_id)??;
    }
    state.context.pop_n(items.len());
    Ok(())
}
