use super::*;

pub fn get_db_index<'a, N>(context: &Context, name_components: N) -> Result<DbIndex, BindError>
where
    N: Clone + Iterator<Item = &'a ub::Identifier>,
{
    let lookup_result = context.get_db_index(name_components.clone().map(|c| &c.name));

    match lookup_result {
        Ok(db_index) => Ok(db_index),
        Err(Some(_)) => Err(ExpectedTermButNameRefersToModError {
            name_components: name_components.cloned().collect(),
        }
        .into()),
        Err(None) => Err(NameNotFoundError {
            name_components: name_components.cloned().collect(),
        }
        .into()),
    }
}

pub fn lookup_name<'a, N>(
    context: &Context,
    name_components: N,
) -> Result<(DotGraphNode, OwnedSymbolSource), NameNotFoundError>
where
    N: Clone + Iterator<Item = &'a ub::Identifier>,
{
    let lookup_result = context.lookup_name(name_components.clone().map(|c| &c.name));

    if let Some(entry) = lookup_result {
        Ok(entry)
    } else {
        Err(NameNotFoundError {
            name_components: name_components.cloned().collect(),
        })
    }
}

pub fn add_dot_edge(
    context: &mut Context,
    start: DotGraphNode,
    label: &IdentifierName,
    end: DotGraphNode,
    source: &ub::Identifier,
) -> Result<(), NameClashError> {
    let result = context.add_dot_edge(
        start,
        label,
        end,
        OwnedSymbolSource::Identifier(source.clone()),
    );
    if let Err((_, old_source)) = result {
        return Err(NameClashError {
            name: label.clone(),
            old: old_source,
            new: OwnedSymbolSource::Identifier(source.clone()),
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
pub fn add_new_dot_edge_or_ignore_duplicate(
    context: &mut Context,
    start: DotGraphNode,
    label: &IdentifierName,
    end: DotGraphNode,
    source: &ub::Identifier,
) -> Result<(), NameClashError> {
    let existing_entry = context
        .add_dot_edge(
            start,
            label,
            end,
            OwnedSymbolSource::Identifier(source.clone()),
        )
        .err();
    if let Some((old_end, old_source)) = existing_entry {
        if end == old_end {
            // Ignore duplicate
            return Ok(());
        }
        return Err(NameClashError {
            name: label.clone(),
            old: old_source,
            new: OwnedSymbolSource::Identifier(source.clone()),
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
    end: DotGraphNode,
    source: &OwnedSymbolSource,
) -> Result<(), NameClashError> {
    let existing_entry = context
        .add_dot_edge(start, label, end, source.clone())
        .err();
    if let Some((old_end, old_source)) = existing_entry {
        if end == old_end {
            // Ignore duplicate
            return Ok(());
        }
        return Err(NameClashError {
            name: label.clone(),
            old: old_source,
            new: source.clone(),
        });
    }
    Ok(())
}

pub fn create_name_and_add_to_mod(
    context: &mut Context,
    identifier: ub::Identifier,
) -> Result<Identifier, NameClashError> {
    let db_level = context.push_placeholder();
    add_dot_edge(
        context,
        DotGraphNode::Mod(context.current_file_id()),
        &identifier.name,
        DotGraphNode::LeafItem(db_level),
        &identifier,
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
