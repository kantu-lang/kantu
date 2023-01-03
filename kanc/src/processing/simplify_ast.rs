use crate::data::{
    non_empty_vec::NonEmptyVec,
    simplified_ast::*,
    // `ust` stands for "unsimplified syntax tree".
    unsimplified_ast as ust,
    TextSpan,
};

#[derive(Clone, Debug)]
pub enum SimplifyAstError {
    IllegalDotLhs(ust::Expression),

    HeterogeneousParams(NonEmptyVec<ust::Param>),
    UnderscoreParamLabel(ust::Param),
    DuplicateParamLabel(ust::Param, ust::Param),

    HeterogeneousCallArgs(NonEmptyVec<ust::CallArg>),
    UnderscoreCallArgLabel(ust::CallArg),
    DuplicateCallArgLabel(ust::CallArg, ust::CallArg),

    HeterogeneousMatchCaseParams(NonEmptyVec<ust::MatchCaseParam>),
    UnderscoreMatchCaseParamLabel(ust::MatchCaseParam),
    DuplicateMatchCaseParamLabel(ust::MatchCaseParam, ust::MatchCaseParam),
}

pub fn simplify_file(unsimplified: ust::File) -> Result<File, SimplifyAstError> {
    Ok(File {
        span: unsimplified.span,
        id: unsimplified.id,
        items: vec_result_map(unsimplified.items, simplify_file_item)?
            .into_iter()
            .collect(),
    })
}

/// Returns `Ok` if `f(x)` returns `Ok` for all `x` in `vec`.
/// Otherwise, returns `Err` with the first `Err` returned by `f`.
fn vec_result_map<T, U, E, F>(vec: Vec<T>, mut f: F) -> Result<Vec<U>, E>
where
    F: FnMut(T) -> Result<U, E>,
{
    let mut result = Vec::with_capacity(vec.len());
    for item in vec {
        result.push(f(item)?);
    }
    Ok(result)
}

fn simplify_file_item(unsimplified: ust::FileItem) -> Result<FileItem, SimplifyAstError> {
    Ok(match unsimplified {
        ust::FileItem::Use(unsimplified) => simplify_use_statement(unsimplified)?,
        ust::FileItem::Mod(unsimplified) => FileItem::Mod(simplify_mod_statement(unsimplified)?),
        ust::FileItem::Type(unsimplified) => FileItem::Type(simplify_type_statement(unsimplified)?),
        ust::FileItem::Let(unsimplified) => FileItem::Let(simplify_let_statement(unsimplified)?),
    })
}

fn simplify_use_statement(
    mut unsimplified: ust::UseStatement,
) -> Result<FileItem, SimplifyAstError> {
    let Some(import_modifier) = unsimplified.import_modifier.take() else {
        return Ok(FileItem::UseSingle(simplify_use_statement_with_no_import_modifier(unsimplified)?));
    };

    Ok(match import_modifier.kind {
        ust::WildcardOrAlternateNameKind::Wildcard => FileItem::UseWildcard(
            simplify_use_statement_with_wildcard(unsimplified, import_modifier.span)?,
        ),
        ust::WildcardOrAlternateNameKind::AlternateName(alternate_name) => {
            FileItem::UseSingle(simplify_use_statement_with_alternate_name(
                unsimplified,
                import_modifier.span,
                alternate_name,
            )?)
        }
    })
}

fn simplify_use_statement_with_no_import_modifier(
    unsimplified: ust::UseStatement,
) -> Result<UseSingleStatement, SimplifyAstError> {
    Ok(UseSingleStatement {
        span: unsimplified.span,
        visibility: unsimplified.visibility,
        first_component: unsimplified.first_component,
        other_components: unsimplified.other_components,
        alternate_name: None,
    })
}

fn simplify_use_statement_with_wildcard(
    unsimplified: ust::UseStatement,
    star_span: TextSpan,
) -> Result<UseWildcardStatement, SimplifyAstError> {
    Ok(UseWildcardStatement {
        span: unsimplified.span,
        visibility: unsimplified.visibility,
        first_component: unsimplified.first_component,
        other_components: unsimplified.other_components,
        star_span,
    })
}

fn simplify_use_statement_with_alternate_name(
    unsimplified: ust::UseStatement,
    alternate_name_span: TextSpan,
    alternate_name: IdentifierName,
) -> Result<UseSingleStatement, SimplifyAstError> {
    let alternate_name = Some(Identifier {
        span: alternate_name_span,
        name: alternate_name,
    });
    Ok(UseSingleStatement {
        span: unsimplified.span,
        visibility: unsimplified.visibility,
        first_component: unsimplified.first_component,
        other_components: unsimplified.other_components,
        alternate_name,
    })
}

fn simplify_mod_statement(
    unsimplified: ust::ModStatement,
) -> Result<ModStatement, SimplifyAstError> {
    Ok(unsimplified)
}

fn simplify_type_statement(
    unsimplified: ust::TypeStatement,
) -> Result<TypeStatement, SimplifyAstError> {
    Ok(TypeStatement {
        span: unsimplified.span,
        visibility: unsimplified.visibility,
        name: unsimplified.name,
        params: simplify_optional_params(unsimplified.params)?,
        variants: vec_result_map(unsimplified.variants, simplify_variant)?,
    })
}

fn simplify_optional_params(
    unsimplified: Option<NonEmptyVec<ust::Param>>,
) -> Result<Option<NonEmptyParamVec>, SimplifyAstError> {
    Ok(unsimplified.map(simplify_params).transpose()?)
}

fn simplify_params(
    unsimplified: NonEmptyVec<ust::Param>,
) -> Result<NonEmptyParamVec, SimplifyAstError> {
    validate_there_are_no_duplicate_param_labels(&unsimplified)?;

    let hetero_err = SimplifyAstError::HeterogeneousParams(unsimplified.clone());
    let (remaining, last) = unsimplified.into_popped();

    validate_param_label_is_not_underscore(&last)?;

    if let Some(label) = last.label {
        let last = LabeledParam {
            span: last.span,
            label,
            is_dashed: last.is_dashed,
            name: last.name,
            type_: simplify_expression(last.type_)?,
        };
        let remaining = simplify_params_but_require_labels(remaining, &hetero_err)?;
        Ok(NonEmptyParamVec::UniquelyLabeled(NonEmptyVec::from_pushed(
            remaining, last,
        )))
    } else {
        let last = UnlabeledParam {
            span: last.span,
            is_dashed: last.is_dashed,
            name: last.name,
            type_: simplify_expression(last.type_)?,
        };
        let remaining = simplify_params_but_forbid_labels(remaining, &hetero_err)?;
        Ok(NonEmptyParamVec::Unlabeled(NonEmptyVec::from_pushed(
            remaining, last,
        )))
    }
}

fn validate_there_are_no_duplicate_param_labels(
    unsimplified: &[ust::Param],
) -> Result<(), SimplifyAstError> {
    use std::collections::HashMap;
    let mut seen: HashMap<&IdentifierName, &ust::Param> = HashMap::new();
    for param in unsimplified {
        let Some(label_name) = param.label_name() else {
            continue;
        };
        if let Some(existing_param_with_same_name) = seen.get(&label_name).copied() {
            return Err(SimplifyAstError::DuplicateParamLabel(
                param.clone(),
                existing_param_with_same_name.clone(),
            ));
        }
        seen.insert(label_name, param);
    }
    Ok(())
}

fn simplify_params_but_require_labels(
    unsimplified: Vec<ust::Param>,
    hetero_err: &SimplifyAstError,
) -> Result<Vec<LabeledParam>, SimplifyAstError> {
    unsimplified
        .into_iter()
        .map(|param| simplify_param_but_require_label(param, hetero_err))
        .collect()
}

fn simplify_params_but_forbid_labels(
    unsimplified: Vec<ust::Param>,
    hetero_err: &SimplifyAstError,
) -> Result<Vec<UnlabeledParam>, SimplifyAstError> {
    unsimplified
        .into_iter()
        .map(|param| simplify_param_but_forbid_label(param, hetero_err))
        .collect()
}

fn simplify_param_but_require_label(
    unsimplified: ust::Param,
    hetero_err: &SimplifyAstError,
) -> Result<LabeledParam, SimplifyAstError> {
    validate_param_label_is_not_underscore(&unsimplified)?;

    if let Some(label) = unsimplified.label {
        Ok(LabeledParam {
            span: unsimplified.span,
            label,
            is_dashed: unsimplified.is_dashed,
            name: unsimplified.name,
            type_: simplify_expression(unsimplified.type_)?,
        })
    } else {
        Err(hetero_err.clone())
    }
}

fn simplify_param_but_forbid_label(
    unsimplified: ust::Param,
    hetero_err: &SimplifyAstError,
) -> Result<UnlabeledParam, SimplifyAstError> {
    validate_param_label_is_not_underscore(&unsimplified)?;

    if let Some(_) = unsimplified.label {
        Err(hetero_err.clone())
    } else {
        Ok(UnlabeledParam {
            span: unsimplified.span,
            is_dashed: unsimplified.is_dashed,
            name: unsimplified.name,
            type_: simplify_expression(unsimplified.type_)?,
        })
    }
}

fn validate_param_label_is_not_underscore(param: &ust::Param) -> Result<(), SimplifyAstError> {
    let Some(label_name) = param.label_name() else {
        return Ok(());
    };
    match label_name {
        IdentifierName::Reserved(ReservedIdentifierName::Underscore) => {
            Err(SimplifyAstError::UnderscoreParamLabel(param.clone()))
        }
        _ => Ok(()),
    }
}

fn simplify_variant(unsimplified: ust::Variant) -> Result<Variant, SimplifyAstError> {
    Ok(Variant {
        span: unsimplified.span,
        name: unsimplified.name,
        params: simplify_optional_params(unsimplified.params)?,
        return_type: simplify_expression(unsimplified.return_type)?,
    })
}

fn simplify_let_statement(
    unsimplified: ust::LetStatement,
) -> Result<LetStatement, SimplifyAstError> {
    Ok(LetStatement {
        span: unsimplified.span,
        visibility: unsimplified.visibility,
        transparency: unsimplified.transparency,
        name: unsimplified.name,
        value: simplify_expression(unsimplified.value)?,
    })
}

fn simplify_expression(unsimplified: ust::Expression) -> Result<Expression, SimplifyAstError> {
    Ok(match unsimplified {
        ust::Expression::Identifier(unsimplified) => simplify_identifier(unsimplified),
        ust::Expression::Todo(span) => Expression::Todo(span),
        ust::Expression::Dot(unsimplified) => simplify_dot(unsimplified)?,
        ust::Expression::Call(unsimplified) => simplify_call(*unsimplified)?,
        ust::Expression::Fun(unsimplified) => simplify_fun(*unsimplified)?,
        ust::Expression::Match(unsimplified) => simplify_match(*unsimplified)?,
        ust::Expression::Forall(unsimplified) => simplify_forall(*unsimplified)?,
        ust::Expression::Check(unsimplified) => simplify_check(*unsimplified)?,
    })
}

fn simplify_identifier(unsimplified: ust::Identifier) -> Expression {
    Expression::Name(NameExpression {
        span: unsimplified.span,
        components: NonEmptyVec::singleton(unsimplified),
    })
}

fn simplify_dot(unsimplified: Box<ust::Dot>) -> Result<Expression, SimplifyAstError> {
    #[derive(Clone, Debug)]
    struct NotANameExpressionError(ust::Expression);

    fn get_components(
        expr: ust::Expression,
    ) -> Result<NonEmptyVec<ust::Identifier>, NotANameExpressionError> {
        match expr {
            ust::Expression::Identifier(identifier) => Ok(NonEmptyVec::singleton(identifier)),
            ust::Expression::Dot(dot) => {
                let mut components = get_components(dot.left)?;
                components.push(dot.right);
                Ok(components)
            }
            other => Err(NotANameExpressionError(other)),
        }
    }

    Ok(Expression::Name(NameExpression {
        span: unsimplified.span,
        components: get_components(ust::Expression::Dot(unsimplified))
            .map_err(|err| SimplifyAstError::IllegalDotLhs(err.0))?,
    }))
}

fn simplify_call(unsimplified: ust::Call) -> Result<Expression, SimplifyAstError> {
    Ok(Expression::Call(Box::new(Call {
        span: unsimplified.span,
        callee: simplify_expression(unsimplified.callee)?,
        args: simplify_call_args(unsimplified.args)?,
    })))
}

fn simplify_call_args(
    unsimplified: NonEmptyVec<ust::CallArg>,
) -> Result<NonEmptyCallArgVec, SimplifyAstError> {
    validate_there_are_no_duplicate_call_arg_labels(&unsimplified)?;

    let hetero_err = SimplifyAstError::HeterogeneousCallArgs(unsimplified.clone());
    let (remaining, last) = unsimplified.into_popped();

    validate_call_arg_label_is_not_underscore(&last)?;

    if last.label.is_some() {
        let last = simplify_call_arg_but_require_label(last, &hetero_err)?;
        let remaining = simplify_call_args_but_require_labels(remaining, &hetero_err)?;
        Ok(NonEmptyCallArgVec::UniquelyLabeled(
            NonEmptyVec::from_pushed(remaining, last),
        ))
    } else {
        let last = simplify_call_arg_but_forbid_label(last, &hetero_err)?;
        let remaining = simplify_call_args_but_forbid_labels(remaining, &hetero_err)?;
        Ok(NonEmptyCallArgVec::Unlabeled(NonEmptyVec::from_pushed(
            remaining, last,
        )))
    }
}

fn validate_there_are_no_duplicate_call_arg_labels(
    unsimplified: &[ust::CallArg],
) -> Result<(), SimplifyAstError> {
    use std::collections::HashMap;
    let mut seen: HashMap<&IdentifierName, &ust::CallArg> = HashMap::new();
    for param in unsimplified {
        let Some(label_name) = param.label_name() else {
            continue;
        };
        if let Some(existing_param_with_same_name) = seen.get(&label_name).copied() {
            return Err(SimplifyAstError::DuplicateCallArgLabel(
                param.clone(),
                existing_param_with_same_name.clone(),
            ));
        }
        seen.insert(label_name, param);
    }
    Ok(())
}

fn simplify_call_args_but_require_labels(
    unsimplified: Vec<ust::CallArg>,
    hetero_err: &SimplifyAstError,
) -> Result<Vec<LabeledCallArg>, SimplifyAstError> {
    unsimplified
        .into_iter()
        .map(|param| simplify_call_arg_but_require_label(param, hetero_err))
        .collect()
}

fn simplify_call_args_but_forbid_labels(
    unsimplified: Vec<ust::CallArg>,
    hetero_err: &SimplifyAstError,
) -> Result<Vec<Expression>, SimplifyAstError> {
    unsimplified
        .into_iter()
        .map(|param| simplify_call_arg_but_forbid_label(param, hetero_err))
        .collect()
}

fn simplify_call_arg_but_require_label(
    unsimplified: ust::CallArg,
    hetero_err: &SimplifyAstError,
) -> Result<LabeledCallArg, SimplifyAstError> {
    validate_call_arg_label_is_not_underscore(&unsimplified)?;

    if let Some(label) = unsimplified.label {
        Ok(match label {
            ParamLabel::Implicit => {
                let ust::Expression::Identifier(label) = unsimplified.value else {
                    panic!("Impossible: Implicitly labeled call arg value must be an identifier.");
                };
                LabeledCallArg::Implicit(label)
            }
            ParamLabel::Explicit(label) => {
                LabeledCallArg::Explicit(label, simplify_expression(unsimplified.value)?)
            }
        })
    } else {
        Err(hetero_err.clone())
    }
}

fn simplify_call_arg_but_forbid_label(
    unsimplified: ust::CallArg,
    hetero_err: &SimplifyAstError,
) -> Result<Expression, SimplifyAstError> {
    validate_call_arg_label_is_not_underscore(&unsimplified)?;

    if let Some(_) = unsimplified.label {
        Err(hetero_err.clone())
    } else {
        simplify_expression(unsimplified.value)
    }
}

fn validate_call_arg_label_is_not_underscore(arg: &ust::CallArg) -> Result<(), SimplifyAstError> {
    let Some(label_name) = arg.label_name() else {
        return Ok(());
    };
    match label_name {
        IdentifierName::Reserved(ReservedIdentifierName::Underscore) => {
            Err(SimplifyAstError::UnderscoreCallArgLabel(arg.clone()))
        }
        _ => Ok(()),
    }
}

fn simplify_fun(unsimplified: ust::Fun) -> Result<Expression, SimplifyAstError> {
    Ok(Expression::Fun(Box::new(Fun {
        span: unsimplified.span,
        name: unsimplified.name,
        params: simplify_params(unsimplified.params)?,
        return_type: simplify_expression(unsimplified.return_type)?,
        body: simplify_expression(unsimplified.body)?,
    })))
}

fn simplify_match(unsimplified: ust::Match) -> Result<Expression, SimplifyAstError> {
    Ok(Expression::Match(Box::new(Match {
        span: unsimplified.span,
        matchee: simplify_expression(unsimplified.matchee)?,
        cases: vec_result_map(unsimplified.cases, simplify_match_case)?,
    })))
}

fn simplify_match_case(unsimplified: ust::MatchCase) -> Result<MatchCase, SimplifyAstError> {
    Ok(MatchCase {
        span: unsimplified.span,
        variant_name: unsimplified.variant_name,
        params: simplify_optional_match_case_params(unsimplified.params, unsimplified.triple_dot)?,
        output: simplify_match_case_output(unsimplified.output)?,
    })
}

fn simplify_optional_match_case_params(
    unsimplified: Option<NonEmptyVec<ust::MatchCaseParam>>,
    optional_triple_dot: Option<TextSpan>,
) -> Result<Option<NonEmptyMatchCaseParamVec>, SimplifyAstError> {
    Ok(match (unsimplified, optional_triple_dot) {
        (None, None) => None,
        (None, Some(triple_dot)) => Some(NonEmptyMatchCaseParamVec::UniquelyLabeled {
            params: None,
            triple_dot: Some(triple_dot),
        }),
        (Some(params), optional_triple_dot) => {
            Some(simplify_match_case_params(params, optional_triple_dot)?)
        }
    })
}

fn simplify_match_case_params(
    unsimplified: NonEmptyVec<ust::MatchCaseParam>,
    triple_dot: Option<TextSpan>,
) -> Result<NonEmptyMatchCaseParamVec, SimplifyAstError> {
    validate_there_are_no_duplicate_match_case_param_labels(unsimplified.as_ref())?;

    let hetero_err = SimplifyAstError::HeterogeneousMatchCaseParams(unsimplified.clone());
    let (remaining, last) = unsimplified.into_popped();

    validate_match_case_param_label_is_not_underscore(&last)?;

    if let Some(label) = last.label {
        let last = LabeledMatchCaseParam {
            span: last.span,
            label,
            name: last.name,
        };
        let remaining = simplify_match_case_params_but_require_labels(remaining, &hetero_err)?;
        Ok(NonEmptyMatchCaseParamVec::UniquelyLabeled {
            params: Some(NonEmptyVec::from_pushed(remaining, last)),
            triple_dot,
        })
    } else {
        let last = last.name;
        let remaining = simplify_match_case_params_but_forbid_labels(remaining, &hetero_err)?;
        Ok(NonEmptyMatchCaseParamVec::Unlabeled(
            NonEmptyVec::from_pushed(remaining, last),
        ))
    }
}

fn validate_there_are_no_duplicate_match_case_param_labels(
    unsimplified: &[ust::MatchCaseParam],
) -> Result<(), SimplifyAstError> {
    use std::collections::HashMap;
    let mut seen: HashMap<&IdentifierName, &ust::MatchCaseParam> = HashMap::new();
    for param in unsimplified {
        let Some(label_name) = param.label_name() else {
            continue;
        };
        if let Some(existing_param_with_same_name) = seen.get(&label_name).copied() {
            return Err(SimplifyAstError::DuplicateMatchCaseParamLabel(
                param.clone(),
                existing_param_with_same_name.clone(),
            ));
        }
        seen.insert(label_name, param);
    }
    Ok(())
}

fn validate_match_case_param_label_is_not_underscore(
    param: &ust::MatchCaseParam,
) -> Result<(), SimplifyAstError> {
    let Some(label_name) = param.label_name() else {
        return Ok(());
    };
    match label_name {
        IdentifierName::Reserved(ReservedIdentifierName::Underscore) => Err(
            SimplifyAstError::UnderscoreMatchCaseParamLabel(param.clone()),
        ),
        _ => Ok(()),
    }
}

fn simplify_match_case_params_but_require_labels(
    unsimplified: Vec<ust::MatchCaseParam>,
    hetero_err: &SimplifyAstError,
) -> Result<Vec<LabeledMatchCaseParam>, SimplifyAstError> {
    unsimplified
        .into_iter()
        .map(|param| simplify_match_case_param_but_require_label(param, hetero_err))
        .collect()
}

fn simplify_match_case_params_but_forbid_labels(
    unsimplified: Vec<ust::MatchCaseParam>,
    hetero_err: &SimplifyAstError,
) -> Result<Vec<Identifier>, SimplifyAstError> {
    unsimplified
        .into_iter()
        .map(|param| simplify_match_case_param_but_forbid_label(param, hetero_err))
        .collect()
}

fn simplify_match_case_param_but_require_label(
    unsimplified: ust::MatchCaseParam,
    hetero_err: &SimplifyAstError,
) -> Result<LabeledMatchCaseParam, SimplifyAstError> {
    validate_match_case_param_label_is_not_underscore(&unsimplified)?;

    if let Some(label) = unsimplified.label {
        Ok(LabeledMatchCaseParam {
            span: unsimplified.span,
            label,
            name: unsimplified.name,
        })
    } else {
        Err(hetero_err.clone())
    }
}

fn simplify_match_case_param_but_forbid_label(
    unsimplified: ust::MatchCaseParam,
    hetero_err: &SimplifyAstError,
) -> Result<Identifier, SimplifyAstError> {
    validate_match_case_param_label_is_not_underscore(&unsimplified)?;

    if let Some(_) = unsimplified.label {
        Err(hetero_err.clone())
    } else {
        Ok(unsimplified.name)
    }
}

fn simplify_match_case_output(
    unsimplified: ust::MatchCaseOutput,
) -> Result<MatchCaseOutput, SimplifyAstError> {
    Ok(match unsimplified {
        ust::MatchCaseOutput::Some(expression) => {
            MatchCaseOutput::Some(simplify_expression(expression)?)
        }
        ust::MatchCaseOutput::ImpossibilityClaim(kw_span) => {
            MatchCaseOutput::ImpossibilityClaim(kw_span)
        }
    })
}

fn simplify_forall(unsimplified: ust::Forall) -> Result<Expression, SimplifyAstError> {
    Ok(Expression::Forall(Box::new(Forall {
        span: unsimplified.span,
        params: simplify_params(unsimplified.params)?,
        output: simplify_expression(unsimplified.output)?,
    })))
}

fn simplify_check(unsimplified: ust::Check) -> Result<Expression, SimplifyAstError> {
    Ok(Expression::Check(Box::new(Check {
        span: unsimplified.span,
        assertions: unsimplified
            .assertions
            .try_into_mapped(simplify_check_assertion)?,
        output: simplify_expression(unsimplified.output)?,
    })))
}

fn simplify_check_assertion(
    unsimplified: ust::CheckAssertion,
) -> Result<CheckAssertion, SimplifyAstError> {
    Ok(CheckAssertion {
        span: unsimplified.span,
        kind: unsimplified.kind,
        left: simplify_goal_kw_or_expression(unsimplified.left)?,
        right: simplify_question_mark_or_expression(unsimplified.right)?,
    })
}

fn simplify_question_mark_or_expression(
    unsimplified: ust::QuestionMarkOrExpression,
) -> Result<QuestionMarkOrExpression, SimplifyAstError> {
    Ok(match unsimplified {
        ust::QuestionMarkOrExpression::QuestionMark { span } => {
            QuestionMarkOrExpression::QuestionMark { span }
        }
        ust::QuestionMarkOrExpression::Expression(expr) => {
            QuestionMarkOrExpression::Expression(simplify_expression(expr)?)
        }
    })
}

fn simplify_goal_kw_or_expression(
    unsimplified: ust::GoalKwOrExpression,
) -> Result<GoalKwOrExpression, SimplifyAstError> {
    Ok(match unsimplified {
        ust::GoalKwOrExpression::GoalKw { span } => GoalKwOrExpression::GoalKw { span },
        ust::GoalKwOrExpression::Expression(expr) => {
            GoalKwOrExpression::Expression(simplify_expression(expr)?)
        }
    })
}
