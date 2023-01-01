use super::*;

pub fn type_check_file_items(
    file_tree: &FileTree,
    registry: &mut NodeRegistry,
    file_item_list_id: TypePositivityValidated<Option<NonEmptyListId<FileItemNodeId>>>,
) -> Result<Vec<TypeCheckWarning>, TypeCheckError> {
    let mut context = Context::with_builtins(registry);
    let mut substitution_context = SubstitutionContext::empty();
    let mut equality_checker = NodeEqualityChecker::new();
    let mut warnings = vec![];
    let mut state = State {
        file_tree: &file_tree,
        substitution_context: &mut substitution_context,
        registry,
        equality_checker: &mut equality_checker,
        warnings: &mut warnings,
        context: &mut context,
    };

    untaint_err(&mut state, file_item_list_id, type_check_file_items_dirty)?;
    Ok(warnings)
}

pub(super) fn type_check_file_items_dirty(
    state: &mut State,
    file_item_list_id: TypePositivityValidated<Option<NonEmptyListId<FileItemNodeId>>>,
) -> Result<(), Tainted<TypeCheckError>> {
    let file_item_list_id = file_item_list_id.raw();
    let items = state
        .registry
        .get_possibly_empty_list(file_item_list_id)
        .to_vec();
    for &item_id in &items {
        type_check_file_item_dirty(state, item_id)??;
    }
    state.context.pop_n(items.len());
    Ok(())
}
