use crate::data::{
    bind_error::*,
    bound_ast::*,
    file_tree::FileTree,
    non_empty_vec::*,
    // `ub` stands for "unbound".
    simplified_ast::{self as ub, ParenthesizedWeakAncestor},
    FileId,
    TextSpan,
};

pub use crate::data::bind_error::*;

use context::*;
mod context;

use dot_graph::*;
mod dot_graph;

use utils::*;
mod utils;

#[derive(Debug)]
struct State<'a> {
    out: Vec<FileItem>,
    context_data: ContextData<'a>,
    unchecked_files: Vec<ub::File>,
    file_tree: &'a FileTree,
}

pub fn bind_files(
    root_id: FileId,
    mut files: Vec<ub::File>,
    file_tree: &FileTree,
) -> Result<Vec<FileItem>, BindError> {
    let root_file = remove_file_with_id_or_panic(&mut files, root_id);
    let mut state = State {
        out: vec![],
        context_data: ContextData::with_builtins(file_tree),
        unchecked_files: files,
        file_tree,
    };

    add_items_from_file(&mut state, root_file)?;

    Ok(state.out)
}

fn remove_file_with_id_or_panic(files: &mut Vec<ub::File>, id: FileId) -> ub::File {
    let index = files
        .iter()
        .position(|file| file.id == id)
        .expect("File ID not found.");
    files.remove(index)
}

fn add_items_from_file(state: &mut State, file: ub::File) -> Result<(), BindError> {
    for item in file.items {
        add_items_from_file_item(state, item, file.id)?;
    }
    Ok(())
}

fn add_items_from_file_item(
    state: &mut State,
    item: ub::FileItem,
    item_file_id: FileId,
) -> Result<(), BindError> {
    match item {
        ub::FileItem::UseSingle(item) => add_single_import_to_context(
            &mut state.context_data.create_context_for_mod(item_file_id),
            item,
        ),
        ub::FileItem::UseWildcard(item) => add_wildcard_import_to_context(
            &mut state.context_data.create_context_for_mod(item_file_id),
            item,
        ),
        ub::FileItem::Mod(item) => add_mod_to_context(state, item, item_file_id),
        ub::FileItem::Type(item) => add_item_from_type_statement(state, item, item_file_id),
        ub::FileItem::Let(item) => add_item_from_let_statement(state, item, item_file_id),
    }
}

fn add_single_import_to_context(
    context: &mut Context,
    item: ub::UseSingleStatement,
) -> Result<(), BindError> {
    let start = DotGraphNode::Mod(context.current_file_id());
    let import_name = match &item.alternate_name {
        Some(name) => name,
        None => {
            let Some(last_component) = item
                .other_components
                .last()
            else {
                return Err(BindError::CannotUselesslyImportItemAsItself(CannotUselesslyImportItemAsItselfError {
                    use_statement: item
                }));
            };
            last_component
        }
    };
    let end = {
        let first_component_name =
            use_statement_first_component_into_identifier_name(item.first_component);
        let name_components =
            std::iter::once(&first_component_name).chain(item.other_components.iter());
        lookup_name(context, name_components)?.node
    };
    let visibility = get_visibility(context, item.visibility.as_ref())?;
    add_new_dot_edge_with_source_or_merge_with_wildcard_duplicate(
        context,
        start,
        &import_name.name,
        end,
        import_name,
        visibility,
    )?;
    Ok(())
}

fn get_visibility(
    context: &Context,
    pub_clause: Option<&ub::PubClause>,
) -> Result<Visibility, BindError> {
    let Some(pub_clause) = pub_clause else {
        return Ok(Visibility::Mod(context.current_file_id()));
    };
    let Some(ancestor) = &pub_clause.ancestor else {
        return Ok(Visibility::Global);
    };
    get_visibility_from_weak_ancestor_node(context, ancestor)
}

fn get_visibility_from_weak_ancestor_node(
    context: &Context,
    ancestor: &ParenthesizedWeakAncestor,
) -> Result<Visibility, BindError> {
    match &ancestor.kind {
        ub::WeakAncestorKind::Global => Ok(Visibility::Global),
        ub::WeakAncestorKind::Mod => Ok(Visibility::Mod(context.current_file_id())),
        ub::WeakAncestorKind::Super(n) => {
            if let Some(DotGraphEntry {
                node: DotGraphNode::Mod(ancestor_id),
                def: _,
                visibility: _,
            }) = context.get_n_supers(n.get())
            {
                Ok(Visibility::Mod(ancestor_id))
            } else {
                Err(NameNotFoundError {
                    name_components: vec![ub::Identifier {
                        span: ancestor.span,
                        name: IdentifierName::new("super".to_string()),
                    }],
                }
                .into())
            }
        }
        ub::WeakAncestorKind::PackRelative { path_after_pack_kw } => {
            let name_components: Vec<ub::Identifier> = {
                let pack_kw_name = IdentifierName::Reserved(ReservedIdentifierName::Pack);
                let pack_kw = ub::Identifier {
                    span: TextSpan {
                        file_id: ancestor.span.file_id,
                        start: ancestor.span.start,
                        end: ancestor.span.start + pack_kw_name.src_str().len(),
                    },
                    name: pack_kw_name,
                };
                std::iter::once(pack_kw)
                    .chain(path_after_pack_kw.iter().cloned())
                    .collect()
            };
            let entry = lookup_name(context, name_components.iter())?;
            match entry.node {
                DotGraphNode::Mod(mod_id) => Ok(Visibility::Mod(mod_id)),
                DotGraphNode::LeafItem(_) => Err(BindError::ExpectedModButNameRefersToTerm(
                    ExpectedModButNameRefersToTermError { name_components },
                )),
            }
        }
    }
}

fn add_wildcard_import_to_context(
    context: &mut Context,
    item: ub::UseWildcardStatement,
) -> Result<(), BindError> {
    let source = OwnedSymbolSource::WildcardImport(item.clone());
    let start = {
        let first_component_name =
            use_statement_first_component_into_identifier_name(item.first_component);
        let name_components =
            std::iter::once(&first_component_name).chain(item.other_components.iter());
        lookup_name(context, name_components)?.node
    };
    let visibility = get_visibility(context, item.visibility.as_ref())?;

    let edges: Vec<(IdentifierName, DotGraphNode)> = context
        .get_edges(start)
        .into_iter()
        .map(|(label, entry)| (label.clone(), entry.node))
        .collect();
    for (label, end) in edges {
        add_new_dot_edge_with_source_or_ignore_duplicate(
            context,
            DotGraphNode::Mod(context.current_file_id()),
            &label,
            end,
            &source,
            visibility,
        )?;
    }
    Ok(())
}

fn use_statement_first_component_into_identifier_name(
    first_component: ub::UseStatementFirstComponent,
) -> ub::Identifier {
    match first_component.kind {
        ub::UseStatementFirstComponentKind::Mod => ub::Identifier {
            span: first_component.span,
            name: IdentifierName::Reserved(ReservedIdentifierName::Mod),
        },
        ub::UseStatementFirstComponentKind::Super(n) => match n.get() {
            1 => ub::Identifier {
                span: first_component.span,
                name: IdentifierName::Reserved(ReservedIdentifierName::Super),
            },
            2 => ub::Identifier {
                span: first_component.span,
                name: IdentifierName::Reserved(ReservedIdentifierName::Super2),
            },
            3 => ub::Identifier {
                span: first_component.span,
                name: IdentifierName::Reserved(ReservedIdentifierName::Super3),
            },
            4 => ub::Identifier {
                span: first_component.span,
                name: IdentifierName::Reserved(ReservedIdentifierName::Super4),
            },
            5 => ub::Identifier {
                span: first_component.span,
                name: IdentifierName::Reserved(ReservedIdentifierName::Super5),
            },
            6 => ub::Identifier {
                span: first_component.span,
                name: IdentifierName::Reserved(ReservedIdentifierName::Super6),
            },
            7 => ub::Identifier {
                span: first_component.span,
                name: IdentifierName::Reserved(ReservedIdentifierName::Super7),
            },
            8 => ub::Identifier {
                span: first_component.span,
                name: IdentifierName::Reserved(ReservedIdentifierName::Super8),
            },
            _ => panic!("n must be in the range [1, 8]"),
        },
        ub::UseStatementFirstComponentKind::Pack => ub::Identifier {
            span: first_component.span,
            name: IdentifierName::Reserved(ReservedIdentifierName::Pack),
        },
        ub::UseStatementFirstComponentKind::Identifier(name) => ub::Identifier {
            span: first_component.span,
            name,
        },
    }
}

fn add_mod_to_context(
    state: &mut State,
    item: ub::ModStatement,
    item_file_id: FileId,
) -> Result<(), BindError> {
    let Ok(mod_file_id) = state.file_tree.child(item_file_id, &item.name.name) else {
        return Err(BindError::ModFileNotFound(ModFileNotFoundError {
            mod_name: item.name,
        }));
    };
    let visibility = get_visibility(
        &state.context_data.create_context_for_mod(item_file_id),
        item.visibility.as_ref(),
    )?;
    add_dot_edge(
        &mut state.context_data.create_context_for_mod(item_file_id),
        DotGraphNode::Mod(item_file_id),
        &item.name.name,
        DotGraphNode::Mod(mod_file_id),
        &item.name,
        visibility,
    )?;

    let mod_file = remove_file_with_id_or_panic(&mut state.unchecked_files, mod_file_id);
    add_items_from_file(state, mod_file)?;

    Ok(())
}

fn add_item_from_type_statement(
    state: &mut State,
    item: ub::TypeStatement,
    item_file_id: FileId,
) -> Result<(), BindError> {
    let bound = bind_type_statement(
        &mut state.context_data.create_context_for_mod(item_file_id),
        item,
    )?;
    state.out.push(FileItem::Type(bound));
    Ok(())
}

fn add_item_from_let_statement(
    state: &mut State,
    item: ub::LetStatement,
    item_file_id: FileId,
) -> Result<(), BindError> {
    let bound = bind_let_statement(
        &mut state.context_data.create_context_for_mod(item_file_id),
        item,
    )?;
    state.out.push(FileItem::Let(bound));
    Ok(())
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
        let out = bind_optional_params(context, type_statement.params)?;
        context.pop_n(arity);
        out
    };

    let visibility = get_visibility(context, type_statement.visibility.as_ref())?;
    let type_name = create_name_and_add_to_mod(context, type_statement.name, visibility)?;

    let variants = type_statement
        .variants
        .into_iter()
        .map(|unbound| {
            bind_variant_and_add_dot_target(context, unbound, &type_name.name, visibility)
        })
        .collect::<Result<Vec<_>, BindError>>()?;

    Ok(TypeStatement {
        span: Some(type_statement.span),
        visibility,
        name: type_name,
        params,
        variants,
    })
}

fn bind_optional_params(
    context: &mut Context,
    params: Option<ub::NonEmptyParamVec>,
) -> Result<Option<NonEmptyParamVec>, BindError> {
    params
        .map(|params| bind_params(context, params))
        .transpose()
}

fn bind_params(
    context: &mut Context,
    params: ub::NonEmptyParamVec,
) -> Result<NonEmptyParamVec, BindError> {
    Ok(match params {
        ub::NonEmptyParamVec::Unlabeled(params) => NonEmptyParamVec::Unlabeled(
            params.try_into_mapped(|param| bind_unlabeled_param(context, param))?,
        ),
        ub::NonEmptyParamVec::UniquelyLabeled(params) => NonEmptyParamVec::UniquelyLabeled(
            params.try_into_mapped(|param| bind_labeled_param(context, param))?,
        ),
    })
}

fn bind_unlabeled_param(
    context: &mut Context,
    param: ub::UnlabeledParam,
) -> Result<UnlabeledParam, BindError> {
    untaint_err(context, param, bind_unlabeled_param_dirty)
}

fn bind_unlabeled_param_dirty(
    context: &mut Context,
    param: ub::UnlabeledParam,
) -> Result<UnlabeledParam, BindError> {
    let type_ = bind_expression(context, param.type_)?;
    let name = create_local_name_and_add_to_scope(context, param.name)?;
    Ok(UnlabeledParam {
        span: Some(param.span),
        is_dashed: param.is_dashed,
        name,
        type_,
    })
}

fn bind_labeled_param(
    context: &mut Context,
    param: ub::LabeledParam,
) -> Result<LabeledParam, BindError> {
    untaint_err(context, param, bind_labeled_param_dirty)
}

fn bind_labeled_param_dirty(
    context: &mut Context,
    param: ub::LabeledParam,
) -> Result<LabeledParam, BindError> {
    let type_ = bind_expression(context, param.type_)?;
    let name = create_local_name_and_add_to_scope(context, param.name)?;
    Ok(LabeledParam {
        span: Some(param.span),
        label: param.label.into(),
        is_dashed: param.is_dashed,
        name,
        type_,
    })
}

fn bind_variant_and_add_dot_target(
    context: &mut Context,
    variant: ub::Variant,
    type_name: &IdentifierName,
    type_visibility: Visibility,
) -> Result<Variant, BindError> {
    untaint_err(
        context,
        (variant, type_name, type_visibility),
        bind_variant_and_add_restricted_dot_target_dirty,
    )
}

fn bind_variant_and_add_restricted_dot_target_dirty(
    context: &mut Context,
    (variant, type_name, type_visibility): (ub::Variant, &IdentifierName, Visibility),
) -> Result<Variant, BindError> {
    let arity = variant.params.len();
    let params = bind_optional_params(context, variant.params)?;
    let return_type = bind_expression(context, variant.return_type)?;
    context.pop_n(arity);

    let unbound_variant_name = variant.name;
    let name = unbound_variant_name.clone().into();

    let type_db_index = context
        .get_db_index(std::iter::once(type_name))
        .expect("type_name should already be in the context.");
    let type_db_level = context.index_to_level(type_db_index);
    let variant_db_level = context.push_placeholder();

    add_dot_edge(
        context,
        DotGraphNode::LeafItem(type_db_level),
        &unbound_variant_name.name,
        DotGraphNode::LeafItem(variant_db_level),
        &unbound_variant_name,
        type_visibility,
    )?;

    Ok(Variant {
        span: Some(variant.span),
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
    let visibility = get_visibility(context, let_statement.visibility.as_ref())?;
    let transparency = get_transparency(context, let_statement.transparency.as_ref())?;
    let name = create_name_and_add_to_mod(context, let_statement.name, visibility)?;
    Ok(LetStatement {
        span: Some(let_statement.span),
        visibility,
        transparency,
        name,
        value,
    })
}

fn get_transparency(
    context: &Context,
    transparency: Option<&ub::ParenthesizedWeakAncestor>,
) -> Result<Transparency, BindError> {
    let Some(ancestor) = transparency else {
        return Ok(Transparency(Visibility::Mod(context.current_file_id())));
    };
    get_visibility_from_weak_ancestor_node(context, ancestor).map(Transparency)
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
        ub::Expression::Todo(span) => Ok(Expression::Todo(Some(span))),
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
    let db_index = get_db_index(context, name.components.as_ref().iter())?;
    Ok(Expression::Name(NameExpression {
        span: Some(name.span),
        components: name.components.into_mapped(Into::into),
        db_index,
    }))
}

fn bind_call_expression_dirty(
    context: &mut Context,
    call: ub::Call,
) -> Result<Expression, BindError> {
    let callee = bind_expression_dirty(context, call.callee)?;
    let args = bind_call_args(context, call.args)?;
    Ok(Expression::Call(Box::new(Call {
        span: Some(call.span),
        callee,
        args,
    })))
}

fn bind_call_args(
    context: &mut Context,
    args: ub::NonEmptyCallArgVec,
) -> Result<NonEmptyCallArgVec, BindError> {
    Ok(match args {
        ub::NonEmptyCallArgVec::Unlabeled(args) => NonEmptyCallArgVec::Unlabeled(
            args.try_into_mapped(|arg| bind_expression_dirty(context, arg))?,
        ),
        ub::NonEmptyCallArgVec::UniquelyLabeled(args) => NonEmptyCallArgVec::UniquelyLabeled(
            args.try_into_mapped(|arg| bind_labeled_call_arg_dirty(context, arg))?,
        ),
    })
}

fn bind_labeled_call_arg_dirty(
    context: &mut Context,
    arg: ub::LabeledCallArg,
) -> Result<LabeledCallArg, BindError> {
    match arg {
        ub::LabeledCallArg::Implicit(value) => Ok(LabeledCallArg::Implicit {
            db_index: get_db_index(context, std::iter::once(&value))?,
            label: value.into(),
        }),
        ub::LabeledCallArg::Explicit(label, value) => Ok(LabeledCallArg::Explicit {
            label: label.into(),
            value: bind_expression_dirty(context, value)?,
        }),
    }
}

fn bind_fun_dirty(context: &mut Context, fun: ub::Fun) -> Result<Expression, BindError> {
    let param_arity = fun.params.len();
    let params = bind_params(context, fun.params)?;
    let return_type = bind_expression_dirty(context, fun.return_type)?;

    let name = create_local_name_and_add_to_scope(context, fun.name)?;

    let body = bind_expression_dirty(context, fun.body)?;
    let fun = Expression::Fun(Box::new(Fun {
        span: Some(fun.span),
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
    Ok(Expression::Match(Box::new(Match {
        span: Some(match_.span),
        matchee,
        cases,
    })))
}

fn bind_match_case(context: &mut Context, case: ub::MatchCase) -> Result<MatchCase, BindError> {
    let arity = case.params.len();
    let variant_name = case.variant_name.into();
    let params = bind_optional_match_case_params(context, case.params)?;
    let output = bind_match_case_output_dirty(context, case.output)?;

    context.pop_n(arity);
    Ok(MatchCase {
        span: Some(case.span),
        variant_name,
        params,
        output,
    })
}

fn bind_optional_match_case_params(
    context: &mut Context,
    params: Option<ub::NonEmptyMatchCaseParamVec>,
) -> Result<Option<NonEmptyMatchCaseParamVec>, BindError> {
    params
        .map(|params| bind_match_case_params(context, params))
        .transpose()
}

fn bind_match_case_params(
    context: &mut Context,
    params: ub::NonEmptyMatchCaseParamVec,
) -> Result<NonEmptyMatchCaseParamVec, BindError> {
    Ok(match params {
        ub::NonEmptyMatchCaseParamVec::Unlabeled(params) => NonEmptyMatchCaseParamVec::Unlabeled(
            params.try_into_mapped(|param| -> Result<_, BindError> {
                Ok(create_local_name_and_add_to_scope(context, param)?)
            })?,
        ),

        ub::NonEmptyMatchCaseParamVec::UniquelyLabeled { params, triple_dot } => {
            NonEmptyMatchCaseParamVec::UniquelyLabeled {
                params: params
                    .map(|params| {
                        params.try_into_mapped(
                            |param| -> Result<LabeledMatchCaseParam, BindError> {
                                let name = create_local_name_and_add_to_scope(context, param.name)?;
                                Ok(LabeledMatchCaseParam {
                                    span: Some(param.span),
                                    label: param.label.into(),
                                    name,
                                })
                            },
                        )
                    })
                    .transpose()?,
                triple_dot,
            }
        }
    })
}

fn bind_match_case_output_dirty(
    context: &mut Context,
    output: ub::MatchCaseOutput,
) -> Result<MatchCaseOutput, BindError> {
    Ok(match output {
        ub::MatchCaseOutput::Some(expression) => {
            MatchCaseOutput::Some(bind_expression_dirty(context, expression)?)
        }
        ub::MatchCaseOutput::ImpossibilityClaim(kw_span) => {
            MatchCaseOutput::ImpossibilityClaim(Some(kw_span))
        }
    })
}

fn bind_forall_dirty(context: &mut Context, forall: ub::Forall) -> Result<Expression, BindError> {
    let arity = forall.params.len();
    let params = bind_params(context, forall.params)?;
    let output = bind_expression_dirty(context, forall.output)?;
    let forall = Expression::Forall(Box::new(Forall {
        span: Some(forall.span),
        params,
        output,
    }));

    context.pop_n(arity);
    Ok(forall)
}

fn bind_check_dirty(context: &mut Context, check: ub::Check) -> Result<Expression, BindError> {
    let assertions = check
        .assertions
        .try_into_mapped(|param| bind_check_assertion_dirty(context, param))?;
    let output = bind_expression_dirty(context, check.output)?;
    Ok(Expression::Check(Box::new(Check {
        span: Some(check.span),
        assertions,
        output,
    })))
}

fn bind_check_assertion_dirty(
    context: &mut Context,
    check: ub::CheckAssertion,
) -> Result<CheckAssertion, BindError> {
    let left = bind_goal_kw_or_possibly_invalid_expression(context, check.left);
    let right = bind_question_mark_or_possibly_invalid_expression(context, check.right);
    Ok(CheckAssertion {
        span: Some(check.span),
        kind: check.kind,
        left,
        right,
    })
}

fn bind_goal_kw_or_possibly_invalid_expression(
    context: &mut Context,
    expression: ub::GoalKwOrExpression,
) -> GoalKwOrPossiblyInvalidExpression {
    match expression {
        ub::GoalKwOrExpression::GoalKw { span: start } => {
            GoalKwOrPossiblyInvalidExpression::GoalKw { span: Some(start) }
        }
        ub::GoalKwOrExpression::Expression(expression) => {
            GoalKwOrPossiblyInvalidExpression::Expression(bind_possibly_invalid_expression(
                context, expression,
            ))
        }
    }
}

fn bind_question_mark_or_possibly_invalid_expression(
    context: &mut Context,
    expression: ub::QuestionMarkOrExpression,
) -> QuestionMarkOrPossiblyInvalidExpression {
    match expression {
        ub::QuestionMarkOrExpression::QuestionMark { span: start } => {
            QuestionMarkOrPossiblyInvalidExpression::QuestionMark { span: Some(start) }
        }
        ub::QuestionMarkOrExpression::Expression(expression) => {
            QuestionMarkOrPossiblyInvalidExpression::Expression(bind_possibly_invalid_expression(
                context, expression,
            ))
        }
    }
}

fn bind_possibly_invalid_expression(
    context: &mut Context,
    expression: ub::Expression,
) -> PossiblyInvalidExpression {
    // Since we're not using `?` to terminate early (like we normally do),
    // we need to use `bind_expression` (instead of `bind_expression_dirty`),
    // since we need the context to be clean even if `bind_result`
    // is an `Err(_)` (since we'll still ultimately return `Ok` in that case).
    let bind_result = bind_expression(context, expression.clone());
    match bind_result {
        Ok(bound) => PossiblyInvalidExpression::Valid(bound),
        Err(error) => PossiblyInvalidExpression::Invalid(InvalidExpression::SymbolicallyInvalid(
            SymbolicallyInvalidExpression {
                expression,
                error,
                span_invalidated: false,
            },
        )),
    }
}
