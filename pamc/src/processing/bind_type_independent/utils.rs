use super::*;

pub fn get_db_index<'a, N>(context: &Context, name_components: N) -> Result<DbIndex, BindError>
where
    N: Clone + Iterator<Item = &'a ub::Identifier>,
{
    let lookup_result = context.get_db_index(name_components.clone().map(|c| &c.name));

    match lookup_result {
        Ok(db_index) => Ok(db_index),
        Err(Ok(_)) => Err(ExpectedTermButNameRefersToModError {
            name_components: name_components.cloned().collect(),
        }
        .into()),
        Err(Err(err)) => match err.kind {
            NameComponentNotFoundErrorKind::NotFound => Err(NameNotFoundError {
                name_components: name_components.cloned().collect(),
            }
            .into()),
            NameComponentNotFoundErrorKind::Private(actual_visibility) => Err(NameIsPrivateError {
                name_component: name_components
                    .clone()
                    .nth(err.index)
                    .expect("NameComponentNotFoundError index should be valid")
                    .clone(),
                required_visibility: Visibility::Mod(context.current_file_id()),
                actual_visibility,
            }
            .into()),
        },
    }
}

pub fn lookup_name<'a, N>(context: &Context, name_components: N) -> Result<DotGraphEntry, BindError>
where
    N: Clone + Iterator<Item = &'a ub::Identifier>,
{
    context
        .lookup_name(name_components.clone().map(|c| &c.name))
        .map_err(|err| match err.kind {
            NameComponentNotFoundErrorKind::NotFound => NameNotFoundError {
                name_components: name_components.cloned().collect(),
            }
            .into(),
            NameComponentNotFoundErrorKind::Private(actual_visibility) => NameIsPrivateError {
                name_component: name_components
                    .clone()
                    .nth(err.index)
                    .expect("NameComponentNotFoundError index should be valid")
                    .clone(),
                required_visibility: Visibility::Mod(context.current_file_id()),
                actual_visibility,
            }
            .into(),
        })
}

pub fn add_dot_edge(
    context: &mut Context,
    start: DotGraphNode,
    label: &IdentifierName,
    end_node: DotGraphNode,
    end_def: &ub::Identifier,
    end_visibility: Visibility,
) -> Result<(), NameClashError> {
    let result = context.add_dot_edge(
        start,
        label,
        DotGraphEntry {
            node: end_node,
            def: OwnedSymbolSource::Identifier(end_def.clone()),
            visibility: end_visibility,
        },
    );
    if let Err(existing_entry) = result {
        return Err(NameClashError {
            name: label.clone(),
            old: existing_entry.def,
            new: OwnedSymbolSource::Identifier(end_def.clone()),
        });
    }
    Ok(())
}

/// There are 3 cases:
/// 1. An edge with the given label is not present.
///    In this case, the edge is added and `Ok(())` is returned.
/// 2. An edge with the given label is present, and it points to the same `end`
///    AND that `end`'s definition is a wildcard.
///    In this case, this end's def is set to the new end def, and the
///    visibility is set to the more permissive of the two.
///    `Ok(())` is returned.
/// 3. An edge with the given label is present, and it points to a different `end`.
///    In this case, `Err(NameClashError)` is returned.
pub fn add_new_dot_edge_with_source_or_merge_with_wildcard_duplicate(
    context: &mut Context,
    start: DotGraphNode,
    label: &IdentifierName,
    end_node: DotGraphNode,
    end_def: &ub::Identifier,
    end_visibility: Visibility,
) -> Result<(), NameClashError> {
    let existing_entry = context
        .add_dot_edge(
            start,
            label,
            DotGraphEntry {
                node: end_node,
                def: OwnedSymbolSource::Identifier(end_def.clone()),
                visibility: end_visibility,
            },
        )
        .err();
    if let Some(existing_entry) = existing_entry {
        if end_node == existing_entry.node
            && matches!(&existing_entry.def, OwnedSymbolSource::WildcardImport(_))
        {
            let max_visibility = if context
                .is_left_more_permissive_than_right(end_visibility, existing_entry.visibility)
            {
                end_visibility
            } else {
                existing_entry.visibility
            };
            context.overwrite_dot_edge(
                start,
                label,
                DotGraphEntry {
                    node: end_node,
                    def: OwnedSymbolSource::Identifier(end_def.clone()),
                    visibility: max_visibility,
                },
            );
            return Ok(());
        }
        return Err(NameClashError {
            name: label.clone(),
            old: existing_entry.def,
            new: OwnedSymbolSource::Identifier(end_def.clone()),
        });
    }
    Ok(())
}

/// There are 3 cases:
/// 1. An edge with the given label is not present.
///    In this case, the edge is added and `Ok(())` is returned.
/// 2. An edge with the given label is present, and it points to the same `end`.
///    In this case, this is a no-op, and `Ok(())` is returned.
/// 3. An edge with the given label is present, and it points to a different `end`.
///    In this case, `Err(NameClashError)` is returned.
pub fn add_new_dot_edge_with_source_or_ignore_duplicate(
    context: &mut Context,
    start: DotGraphNode,
    label: &IdentifierName,
    end_node: DotGraphNode,
    end_def: &OwnedSymbolSource,
    end_visibility: Visibility,
) -> Result<(), NameClashError> {
    let existing_entry = context
        .add_dot_edge(
            start,
            label,
            DotGraphEntry {
                node: end_node,
                def: end_def.clone(),
                visibility: end_visibility,
            },
        )
        .err();
    if let Some(existing_entry) = existing_entry {
        if end_node == existing_entry.node {
            // Ignore duplicate
            return Ok(());
        }
        return Err(NameClashError {
            name: label.clone(),
            old: existing_entry.def,
            new: end_def.clone(),
        });
    }
    Ok(())
}

pub fn create_name_and_add_to_mod(
    context: &mut Context,
    identifier: ub::Identifier,
    visibility: Visibility,
) -> Result<Identifier, NameClashError> {
    let db_level = context.push_placeholder();
    add_dot_edge(
        context,
        DotGraphNode::Mod(context.current_file_id()),
        &identifier.name,
        DotGraphNode::LeafItem(db_level),
        &identifier,
        visibility,
    )?;
    Ok(identifier.into())
}

pub fn create_local_name_and_add_to_scope(
    context: &mut Context,
    identifier: ub::Identifier,
) -> Result<Identifier, NameClashError> {
    if let IdentifierName::Reserved(ReservedIdentifierName::Underscore) = &identifier.name {
        context.push_placeholder();
        return Ok(identifier.into());
    }

    let result = context.push_local(&identifier);
    if let Err(old_source) = result {
        return Err(NameClashError {
            name: identifier.name.clone(),
            old: old_source,
            new: OwnedSymbolSource::Identifier(identifier),
        });
    }

    Ok(identifier.into())
}

pub fn untaint_err<In, Out, Err, F>(context: &mut Context, input: In, f: F) -> Result<Out, Err>
where
    F: FnOnce(&mut Context, In) -> Result<Out, Err>,
{
    let original_len = context.len();
    let result = f(context, input);
    match result {
        Ok(ok) => Ok(ok),
        Err(err) => {
            context.truncate(original_len);
            Err(err)
        }
    }
}
