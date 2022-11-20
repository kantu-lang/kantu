use crate::data::{
    bound_ast::*,
    // `ub` stands for "unbound".
    simplified_ast as ub,
    FileId,
};

use context::*;
mod context;

pub use error::*;
mod error;

/// The returned `Vec<File>` is not guaranteed to be in any particular order.
pub fn bind_files(files: Vec<ub::File>) -> Result<Vec<File>, BindError> {
    let files = sort_by_dependencies(files)?;
    let mut context = Context::with_builtins();

    let files = files
        .into_iter()
        .map(|file| bind_file(&mut context, file))
        .collect::<Result<Vec<_>, BindError>>()?;

    Ok(files)
}

fn sort_by_dependencies(
    files: Vec<ub::File>,
) -> Result<Vec<ub::File>, CircularFileDependencyError> {
    // TODO (distant): Actually sort, once we support `use` statements.
    Ok(files)
}

fn bind_file(context: &mut Context, file: ub::File) -> Result<File, BindError> {
    let number_of_file_items = file.items.len();
    let items = file
        .items
        .into_iter()
        .map(|item| bind_file_item(context, item))
        .collect::<Result<Vec<_>, BindError>>()?;
    context.pop_n(number_of_file_items);
    Ok(File { id: file.id, items })
}

fn bind_file_item(context: &mut Context, item: ub::FileItem) -> Result<FileItem, BindError> {
    match item {
        ub::FileItem::Type(type_statement) => Ok(FileItem::Type(bind_type_statement(
            context,
            type_statement,
        )?)),
        ub::FileItem::Let(let_statement) => {
            Ok(FileItem::Let(bind_let_statement(context, let_statement)?))
        }
    }
}

fn bind_type_statement(
    context: &mut Context,
    type_statement: ub::TypeStatement,
) -> Result<TypeStatement, BindError> {
    let params = {
        let arity = type_statement.params.len();
        let out = type_statement
            .params
            .into_iter()
            .map(|param| bind_param(context, param))
            .collect::<Result<Vec<_>, BindError>>()?;
        context.pop_n(arity);
        out
    };

    let type_name = create_name_and_add_to_scope(context, type_statement.name)?;

    let variants = type_statement
        .variants
        .into_iter()
        .map(|unbound| {
            bind_variant_and_add_restricted_dot_target(context, unbound, &type_name.name)
        })
        .collect::<Result<Vec<_>, BindError>>()?;

    for variant in &variants {
        let variant_name_components = [&type_name.name, &variant.name.name];
        context.lift_dot_target_restriction(&variant_name_components);
    }

    Ok(TypeStatement {
        name: type_name,
        params,
        variants,
    })
}

fn bind_param(context: &mut Context, param: ub::Param) -> Result<Param, BindError> {
    let type_ = bind_expression(context, param.type_)?;
    let name = create_name_and_add_to_scope(context, param.name)?;
    Ok(Param {
        is_dashed: param.is_dashed,
        name,
        type_,
    })
}

fn bind_variant_and_add_restricted_dot_target(
    context: &mut Context,
    variant: ub::Variant,
    type_name: &IdentifierName,
) -> Result<Variant, BindError> {
    let arity = variant.params.len();
    let params = variant
        .params
        .into_iter()
        .map(|param| bind_param(context, param))
        .collect::<Result<Vec<_>, BindError>>()?;
    let return_type = bind_expression(context, variant.return_type)?;
    context.pop_n(arity);

    let unbound_name = variant.name;
    let name = create_name_without_adding_to_scope(context, unbound_name.clone());

    context.add_restricted_name_to_scope(&[type_name, &unbound_name.name], &unbound_name)?;

    Ok(Variant {
        name,
        params,
        return_type,
    })
}

fn bind_let_statement(
    context: &mut Context,
    let_statement: ub::LetStatement,
) -> Result<LetStatement, BindError> {
    let value = bind_expression(context, let_statement.value)?;
    let name = create_name_and_add_to_scope(context, let_statement.name)?;
    Ok(LetStatement { name, value })
}

fn bind_expression(
    context: &mut Context,
    expression: ub::Expression,
) -> Result<Expression, BindError> {
    match expression {
        ub::Expression::Name(name) => bind_name_expression(context, name),
        ub::Expression::Call(call) => bind_call_expression(context, *call),
        ub::Expression::Fun(fun) => bind_fun(context, *fun),
        ub::Expression::Match(match_) => bind_match(context, *match_),
        ub::Expression::Forall(forall) => bind_forall(context, *forall),
    }
}

fn bind_name_expression(
    context: &mut Context,
    name: ub::NameExpression,
) -> Result<Expression, BindError> {
    let name_components = name.components.iter().map(|identifier| &identifier.name);
    let db_index = context
        .get_db_index(name_components)
        .expect("Symbol should be within scope.");
    Ok(Expression::Name(NameExpression {
        components: name.components.into_iter().map(Into::into).collect(),
        db_index,
    }))
}

fn split_first_and_rest<T>(components: &[T]) -> Option<(&T, &[T])> {
    if components.is_empty() {
        return None;
    }
    Some((&components[0], &components[1..]))
}

fn bind_call_expression(context: &mut Context, call: ub::Call) -> Result<Expression, BindError> {
    let callee = bind_expression(context, call.callee)?;
    let args = call
        .args
        .into_iter()
        .map(|arg| bind_expression(context, arg))
        .collect::<Result<Vec<_>, BindError>>()?;
    Ok(Expression::Call(Box::new(Call { callee, args })))
}

fn bind_fun(context: &mut Context, fun: ub::Fun) -> Result<Expression, BindError> {
    context.push_scope();

    let params = fun
        .params
        .into_iter()
        .map(|param| bind_param(context, param))
        .collect::<Result<Vec<_>, BindError>>()?;
    let return_type = bind_expression(context, fun.return_type)?;

    let (name, _) = create_name_and_add_to_scope(context, fun.name)?;

    let body = bind_expression(context, fun.body)?;
    let fun = Expression::Fun(Box::new(Fun {
        name,
        params,
        return_type,
        body,
        skip_type_checking_body: false,
    }));

    context.pop_scope_or_panic();
    Ok(fun)
}

fn bind_match(context: &mut Context, match_: ub::Match) -> Result<Expression, BindError> {
    let matchee = bind_expression(context, match_.matchee)?;
    let cases = match_
        .cases
        .into_iter()
        .map(|case| bind_match_case(context, case))
        .collect::<Result<Vec<_>, BindError>>()?;
    Ok(Expression::Match(Box::new(Match { matchee, cases })))
}

fn bind_match_case(context: &mut Context, case: ub::MatchCase) -> Result<MatchCase, BindError> {
    context.push_scope();
    let variant_name = case.variant_name.into();
    let params = case
        .params
        .into_iter()
        .map(|param| -> Result<_, BindError> {
            Ok(create_name_and_add_to_scope(context, param)?.0)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let output = bind_expression(context, case.output)?;
    context.pop_scope_or_panic();
    Ok(MatchCase {
        variant_name,
        params,
        output,
    })
}

fn bind_forall(context: &mut Context, forall: ub::Forall) -> Result<Expression, BindError> {
    context.push_scope();

    let params = forall
        .params
        .into_iter()
        .map(|param| bind_param(context, param))
        .collect::<Result<Vec<_>, BindError>>()?;
    let output = bind_expression(context, forall.output)?;
    let forall = Expression::Forall(Box::new(Forall { params, output }));

    context.pop_scope_or_panic();
    Ok(forall)
}

fn create_name_without_adding_to_scope(
    context: &mut Context,
    identifier: ub::Identifier,
) -> Identifier {
    identifier.into()
}

fn create_name_and_add_to_scope(
    context: &mut Context,
    identifier: ub::Identifier,
) -> Result<Identifier, NameClashError> {
    context.add_unrestricted_unqualified_name_to_scope(&identifier)?;
    Ok(identifier.into())
}
