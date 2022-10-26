use crate::data::{
    bound_ast::*,
    // `ub` stands for "unbound".
    simplified_ast as ub,
    symbol_database::{Symbol, SymbolProvider, SymbolToDotTargetsMap},
    FileId,
};

use state::*;
mod state;

pub use error::*;
mod error;

// TODO: Forbid fun return type from using the fun it declares.

/// The returned `Vec<File>` is not guaranteed to be in any particular order.
pub fn bind_symbols_to_identifiers(
    files: Vec<ub::File>,
) -> Result<(Vec<File>, SymbolProvider, SymbolToDotTargetsMap), BindError> {
    let file_node_ids = sort_by_dependencies(files)?;
    let mut state = State::with_builtins();

    let files = file_node_ids
        .into_iter()
        .map(|file| bind_file(&mut state, file))
        .collect::<Result<Vec<_>, BindError>>()?;

    let (provider, dot_targets) = state.into_provider_and_dot_targets();
    Ok((files, provider, dot_targets))
}

fn sort_by_dependencies(
    files: Vec<ub::File>,
) -> Result<Vec<ub::File>, CircularFileDependencyError> {
    // TODO (distant): Actually sort, once we support `use` statements.
    Ok(files)
}

fn bind_file(state: &mut State, file: ub::File) -> Result<File, BindError> {
    state.push_scope();
    let items = file
        .items
        .into_iter()
        .map(|item| bind_file_item(state, item))
        .collect::<Result<Vec<_>, BindError>>()?;
    state.pop_scope_or_panic();
    Ok(File { id: file.id, items })
}

fn bind_file_item(state: &mut State, item: ub::FileItem) -> Result<FileItem, BindError> {
    match item {
        ub::FileItem::Type(type_statement) => {
            Ok(FileItem::Type(bind_type_statement(state, type_statement)?))
        }
        ub::FileItem::Let(let_statement) => {
            Ok(FileItem::Let(bind_let_statement(state, let_statement)?))
        }
    }
}

fn bind_type_statement(
    state: &mut State,
    type_statement: ub::TypeStatement,
) -> Result<TypeStatement, BindError> {
    let params = {
        state.push_scope();
        let out = type_statement
            .params
            .into_iter()
            .map(|param| bind_param(state, param))
            .collect::<Result<Vec<_>, BindError>>()?;
        state.pop_scope_or_panic();
        out
    };

    let name = create_name_and_add_to_scope(state, type_statement.name)?;

    let (variants, original_names): (Vec<Variant>, Vec<ub::Identifier>) = {
        let variants_with_original_names: Vec<(Variant, ub::Identifier)> = type_statement
            .variants
            .into_iter()
            .map(|unbound| {
                let original_name = unbound.name.clone();
                let variant = bind_variant_without_declaring_dot_target(state, unbound)?;
                Ok((variant, original_name))
            })
            .collect::<Result<Vec<_>, BindError>>()?;
        variants_with_original_names.into_iter().unzip()
    };

    for (variant, original_name) in variants.iter().zip(original_names.into_iter()) {
        state.add_dot_target_to_scope(
            (name.symbol, variant.name.component.name.clone()),
            (
                variant.name.symbol,
                OwnedSymbolSource::Identifier(original_name.clone()),
            ),
        )?;
    }

    Ok(TypeStatement {
        name,
        params,
        variants,
    })
}

fn bind_param(state: &mut State, param: ub::Param) -> Result<Param, BindError> {
    let type_ = bind_expression(state, param.type_)?;
    let name = create_name_and_add_to_scope(state, param.name)?;
    Ok(Param {
        is_dashed: param.is_dashed,
        name,
        type_,
    })
}

fn bind_variant_without_declaring_dot_target(
    state: &mut State,
    variant: ub::Variant,
) -> Result<Variant, BindError> {
    state.push_scope();
    let params = variant
        .params
        .into_iter()
        .map(|param| bind_param(state, param))
        .collect::<Result<Vec<_>, BindError>>()?;
    let return_type = bind_expression(state, variant.return_type)?;
    state.pop_scope_or_panic();

    Ok(Variant {
        name: create_name_without_adding_to_scope(state, variant.name),
        params,
        return_type,
    })
}

fn bind_let_statement(
    state: &mut State,
    let_statement: ub::LetStatement,
) -> Result<LetStatement, BindError> {
    let value = bind_expression(state, let_statement.value)?;
    let name = create_name_and_add_to_scope(state, let_statement.name)?;
    Ok(LetStatement { name, value })
}

fn bind_expression(state: &mut State, expression: ub::Expression) -> Result<Expression, BindError> {
    match expression {
        ub::Expression::Name(name) => bind_name_expression(state, name),
        ub::Expression::Call(call) => bind_call_expression(state, *call),
        ub::Expression::Fun(fun) => bind_fun(state, *fun),
        ub::Expression::Match(match_) => bind_match(state, *match_),
        ub::Expression::Forall(forall) => bind_forall(state, *forall),
    }
}

fn bind_name_expression(
    state: &mut State,
    name: ub::NameExpression,
) -> Result<Expression, BindError> {
    let (first, rest) = split_first_and_rest(&name.components)
        .expect("NameExpression must have at least one component.");
    let symbol = {
        let mut current = state.get_symbol(&first)?;
        for component in rest {
            current = state.get_dot_target_symbol((current, &component))?;
        }
        current
    };
    let db_index = state
        .get_db_index(symbol)
        .expect("Symbol should be within scope.");
    Ok(Expression::Name(NameExpression {
        components: name.components,
        symbol,
        db_index,
    }))
}

fn split_first_and_rest<T>(components: &[T]) -> Option<(&T, &[T])> {
    if components.is_empty() {
        return None;
    }
    Some((&components[0], &components[1..]))
}

fn bind_call_expression(state: &mut State, call: ub::Call) -> Result<Expression, BindError> {
    let callee = bind_expression(state, call.callee)?;
    let args = call
        .args
        .into_iter()
        .map(|arg| bind_expression(state, arg))
        .collect::<Result<Vec<_>, BindError>>()?;
    Ok(Expression::Call(Box::new(Call { callee, args })))
}

fn bind_fun(state: &mut State, fun: ub::Fun) -> Result<Expression, BindError> {
    state.push_scope();

    let params = fun
        .params
        .into_iter()
        .map(|param| bind_param(state, param))
        .collect::<Result<Vec<_>, BindError>>()?;
    let return_type = bind_expression(state, fun.return_type)?;

    let name = create_name_and_add_to_scope(state, fun.name)?;

    let body = bind_expression(state, fun.body)?;
    let fun = Expression::Fun(Box::new(Fun {
        name,
        params,
        return_type,
        body,
    }));

    state.pop_scope_or_panic();
    Ok(fun)
}

fn bind_match(state: &mut State, match_: ub::Match) -> Result<Expression, BindError> {
    let matchee = bind_expression(state, match_.matchee)?;
    let cases = match_
        .cases
        .into_iter()
        .map(|case| bind_match_case(state, case))
        .collect::<Result<Vec<_>, BindError>>()?;
    Ok(Expression::Match(Box::new(Match { matchee, cases })))
}

fn bind_match_case(state: &mut State, case: ub::MatchCase) -> Result<MatchCase, BindError> {
    state.push_scope();
    let variant_name = UnresolvedSingletonName {
        component: case.variant_name,
    };
    let params = case
        .params
        .into_iter()
        .map(|param| create_name_and_add_to_scope(state, param))
        .collect::<Result<Vec<_>, _>>()?;
    let output = bind_expression(state, case.output)?;
    state.pop_scope_or_panic();
    Ok(MatchCase {
        variant_name,
        params,
        output,
    })
}

fn bind_forall(state: &mut State, forall: ub::Forall) -> Result<Expression, BindError> {
    state.push_scope();

    let params = forall
        .params
        .into_iter()
        .map(|param| bind_param(state, param))
        .collect::<Result<Vec<_>, BindError>>()?;
    let output = bind_expression(state, forall.output)?;
    let forall = Expression::Forall(Box::new(Forall { params, output }));

    state.pop_scope_or_panic();
    Ok(forall)
}

fn create_name_without_adding_to_scope(
    state: &mut State,
    identifier: ub::Identifier,
) -> SingletonName {
    SingletonName {
        component: identifier,
        symbol: state.new_symbol(),
    }
}

fn create_name_and_add_to_scope(
    state: &mut State,
    identifier: ub::Identifier,
) -> Result<SingletonName, NameClashError> {
    let symbol = state.add_name_to_scope(&identifier)?;
    Ok(SingletonName {
        component: identifier,
        symbol,
    })
}
