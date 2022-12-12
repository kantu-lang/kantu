use crate::data::{
    bind_error::*,
    bound_ast::*,
    // `ub` stands for "unbound".
    simplified_ast as ub,
};

pub use crate::data::bind_error::*;

use context::*;
mod context;

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
    untaint_err(context, file, bind_file_dirty)
}

fn bind_file_dirty(context: &mut Context, file: ub::File) -> Result<File, BindError> {
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
    untaint_err(context, item, bind_file_item_dirty)
}

fn bind_file_item_dirty(context: &mut Context, item: ub::FileItem) -> Result<FileItem, BindError> {
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
    untaint_err(context, type_statement, bind_type_statement_dirty)
}

fn bind_type_statement_dirty(
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
    untaint_err(context, param, bind_param_dirty)
}

fn bind_param_dirty(context: &mut Context, param: ub::Param) -> Result<Param, BindError> {
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
    untaint_err(
        context,
        (variant, type_name),
        bind_variant_and_add_restricted_dot_target_dirty,
    )
}

fn bind_variant_and_add_restricted_dot_target_dirty(
    context: &mut Context,
    (variant, type_name): (ub::Variant, &IdentifierName),
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

    context.add_temporarily_restricted_name_to_scope_unless_singleton_underscore(
        [type_name, &unbound_name.name].iter().copied(),
        &unbound_name,
    )?;

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
    untaint_err(context, let_statement, bind_let_statement_dirty)
}

fn bind_let_statement_dirty(
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
    untaint_err(context, expression, bind_expression_dirty)
}

fn bind_expression_dirty(
    context: &mut Context,
    expression: ub::Expression,
) -> Result<Expression, BindError> {
    match expression {
        ub::Expression::Name(name) => bind_name_expression_dirty(context, name),
        ub::Expression::Call(call) => bind_call_expression_dirty(context, *call),
        ub::Expression::Fun(fun) => bind_fun_dirty(context, *fun),
        ub::Expression::Match(match_) => bind_match_dirty(context, *match_),
        ub::Expression::Forall(forall) => bind_forall_dirty(context, *forall),
        ub::Expression::Check(check) => bind_check_dirty(context, *check),
    }
}

fn bind_name_expression_dirty(
    context: &mut Context,
    name: ub::NameExpression,
) -> Result<Expression, BindError> {
    let db_index = context.get_db_index(&name.components)?;
    Ok(Expression::Name(NameExpression {
        components: name.components.into_iter().map(Into::into).collect(),
        db_index,
    }))
}

fn bind_call_expression_dirty(
    context: &mut Context,
    call: ub::Call,
) -> Result<Expression, BindError> {
    let callee = bind_expression_dirty(context, call.callee)?;
    let args = call
        .args
        .into_iter()
        .map(|arg| bind_expression_dirty(context, arg))
        .collect::<Result<Vec<_>, BindError>>()?;
    Ok(Expression::Call(Box::new(Call { callee, args })))
}

fn bind_fun_dirty(context: &mut Context, fun: ub::Fun) -> Result<Expression, BindError> {
    let param_arity = fun.params.len();
    let params = fun
        .params
        .into_iter()
        .map(|param| bind_param(context, param))
        .collect::<Result<Vec<_>, BindError>>()?;
    let return_type = bind_expression_dirty(context, fun.return_type)?;

    let name = create_name_and_add_to_scope(context, fun.name)?;

    let body = bind_expression_dirty(context, fun.body)?;
    let fun = Expression::Fun(Box::new(Fun {
        name,
        params,
        return_type,
        body,
        skip_type_checking_body: false,
    }));

    context.pop_n(param_arity + 1);
    Ok(fun)
}

fn bind_match_dirty(context: &mut Context, match_: ub::Match) -> Result<Expression, BindError> {
    let matchee = bind_expression_dirty(context, match_.matchee)?;
    let cases = match_
        .cases
        .into_iter()
        .map(|case| bind_match_case(context, case))
        .collect::<Result<Vec<_>, BindError>>()?;
    Ok(Expression::Match(Box::new(Match { matchee, cases })))
}

fn bind_match_case(context: &mut Context, case: ub::MatchCase) -> Result<MatchCase, BindError> {
    let arity = case.params.len();
    let variant_name = case.variant_name.into();
    let params = case
        .params
        .into_iter()
        .map(|param| -> Result<_, BindError> { Ok(create_name_and_add_to_scope(context, param)?) })
        .collect::<Result<Vec<_>, _>>()?;
    let output = bind_expression_dirty(context, case.output)?;

    context.pop_n(arity);
    Ok(MatchCase {
        variant_name,
        params,
        output,
    })
}

fn bind_forall_dirty(context: &mut Context, forall: ub::Forall) -> Result<Expression, BindError> {
    let arity = forall.params.len();
    let params = forall
        .params
        .into_iter()
        .map(|param| bind_param(context, param))
        .collect::<Result<Vec<_>, BindError>>()?;
    let output = bind_expression_dirty(context, forall.output)?;
    let forall = Expression::Forall(Box::new(Forall { params, output }));

    context.pop_n(arity);
    Ok(forall)
}

fn bind_check_dirty(context: &mut Context, check: ub::Check) -> Result<Expression, BindError> {
    // TODO: Properly bind
    bind_expression_dirty(context, check.output)
}

// fn bind_check_dirty(context: &mut Context, check: ub::Check) -> Result<Expression, BindError> {
//     let checkee_annotation = bind_checkee_annotation_dirty(context, check.checkee_annotation)?;
//     let output = bind_expression_dirty(context, check.output)?;
//     let check = Expression::Check(Box::new(Check {
//         checkee_annotation,
//         output,
//     }));
//     Ok(check)
// }

// fn bind_checkee_annotation_dirty(
//     context: &mut Context,
//     checkee_annotation: ub::CheckeeAnnotation,
// ) -> Result<CheckeeAnnotation, BindError> {
//     match checkee_annotation {
//         ub::CheckeeAnnotation::Goal(annotation) => Ok(CheckeeAnnotation::Goal(
//             bind_goal_checkee_annotation_dirty(context, annotation)?,
//         )),
//         ub::CheckeeAnnotation::Expression(annotation) => Ok(CheckeeAnnotation::Expression(
//             bind_expression_checkee_annotation_dirty(context, annotation)?,
//         )),
//     }
// }

// fn bind_goal_checkee_annotation_dirty(
//     context: &mut Context,
//     annotation: ub::GoalCheckeeAnnotation,
// ) -> Result<GoalCheckeeAnnotation, BindError> {
//     let checkee_type =
//         bind_question_mark_or_possibly_invalid_expression(context, annotation.checkee_type);
//     Ok(GoalCheckeeAnnotation {
//         goal_kw_span: annotation.goal_kw_span,
//         checkee_type,
//     })
// }

// fn bind_expression_checkee_annotation_dirty(
//     context: &mut Context,
//     annotation: ub::ExpressionCheckeeAnnotation,
// ) -> Result<ExpressionCheckeeAnnotation, BindError> {
//     Ok(ExpressionCheckeeAnnotation {
//         checkee: bind_expression_dirty(context, annotation.checkee)?,
//         checkee_type: bind_question_mark_or_possibly_invalid_expression(
//             context,
//             annotation.checkee_type,
//         ),
//         checkee_value: annotation.checkee_value.map(|checkee_value| {
//             bind_question_mark_or_possibly_invalid_expression(context, checkee_value)
//         }),
//     })
// }

// fn bind_question_mark_or_possibly_invalid_expression(
//     context: &mut Context,
//     expression: ub::QuestionMarkOrExpression,
// ) -> QuestionMarkOrPossiblyInvalidExpression {
//     match expression {
//         ub::QuestionMarkOrExpression::QuestionMark { span: start } => {
//             QuestionMarkOrPossiblyInvalidExpression::QuestionMark { span: start }
//         }
//         ub::QuestionMarkOrExpression::Expression(expression) => {
//             // Since we're not using `?` to terminate early (like we normally do),
//             // we need to use `bind_expression` (instead of `bind_expression_dirty`),
//             // since we need the context to be clean even if `bind_result`
//             // is an `Err(_)` (since we'll still ultimately return `Ok` in that case).
//             let bind_result = bind_expression(context, expression.clone());
//             let possibly_invalid_expression = match bind_result {
//                 Ok(bound) => PossiblyInvalidExpression::Valid(bound),
//                 Err(error) => {
//                     PossiblyInvalidExpression::Invalid(InvalidExpression::SymbolicallyInvalid(
//                         SymbolicallyInvalidExpression { expression, error },
//                     ))
//                 }
//             };
//             QuestionMarkOrPossiblyInvalidExpression::Expression(possibly_invalid_expression)
//         }
//     }
// }

fn create_name_without_adding_to_scope(
    _context: &mut Context,
    identifier: ub::Identifier,
) -> Identifier {
    identifier.into()
}

fn create_name_and_add_to_scope(
    context: &mut Context,
    identifier: ub::Identifier,
) -> Result<Identifier, NameClashError> {
    context.add_unrestricted_unqualified_name_to_scope_unless_underscore(&identifier)?;
    Ok(identifier.into())
}

fn untaint_err<In, Out, Err, F>(context: &mut Context, input: In, f: F) -> Result<Out, Err>
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
