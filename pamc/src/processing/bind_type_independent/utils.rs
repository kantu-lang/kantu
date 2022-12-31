use super::*;

pub fn get_db_index<'a, N>(
    context: &Context,
    current_file_id: FileId,
    name_components: N,
) -> Result<DbIndex, BindError>
where
    N: Clone + Iterator<Item = &'a ub::Identifier>,
{
    let lookup_result =
        context.get_db_index(current_file_id, name_components.clone().map(|c| &c.name));

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
    current_file_id: FileId,
    name_components: N,
) -> Result<(DotGraphNode, OwnedSymbolSource), NameNotFoundError>
where
    N: Clone + Iterator<Item = &'a ub::Identifier>,
{
    let lookup_result =
        context.lookup_name(current_file_id, name_components.clone().map(|c| &c.name));

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
    if let Err(old_source) = result {
        return Err(NameClashError {
            old: old_source,
            new: OwnedSymbolSource::Identifier(source.clone()),
        });
    }
    Ok(())
}

pub fn add_dot_edge_with_source(
    context: &mut Context,
    start: DotGraphNode,
    label: &IdentifierName,
    end: DotGraphNode,
    source: &OwnedSymbolSource,
) -> Result<(), NameClashError> {
    let result = context.add_dot_edge(start, label, end, source.clone());
    if let Err(old_source) = result {
        return Err(NameClashError {
            old: old_source,
            new: source.clone(),
        });
    }
    Ok(())
}

pub fn create_name_and_add_to_mod(
    context: &mut Context,
    current_file_id: FileId,
    identifier: ub::Identifier,
) -> Result<Identifier, NameClashError> {
    let db_level = context.push_placeholder();
    add_dot_edge(
        context,
        DotGraphNode::Mod(current_file_id),
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

    let result = context.push_local(identifier.span.file_id, &identifier);
    if let Err(old_source) = result {
        return Err(NameClashError {
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
