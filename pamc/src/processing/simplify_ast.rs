use crate::data::{
    non_empty_vec::NonEmptyVec,
    simplified_ast::*,
    // `ust` stands for "unsimplified syntax tree".
    unsimplified_ast as ust,
};

#[derive(Clone, Debug)]
pub enum SimplifyAstError {
    IllegalDotLhs(ust::Expression),
    HeterogeneousParams(NonEmptyVec<ust::Param>),
}

pub fn simplify_file(unsimplified: ust::File) -> Result<File, SimplifyAstError> {
    Ok(File {
        span: unsimplified.span,
        id: unsimplified.id,
        items: vec_result_map(unsimplified.items, simplify_file_item)?,
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
        ust::FileItem::Type(unsimplified) => FileItem::Type(simplify_type_statement(unsimplified)?),
        ust::FileItem::Let(unsimplified) => FileItem::Let(simplify_let_statement(unsimplified)?),
    })
}

fn simplify_type_statement(
    unsimplified: ust::TypeStatement,
) -> Result<TypeStatement, SimplifyAstError> {
    Ok(TypeStatement {
        span: unsimplified.span,
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
    let hetero_err = SimplifyAstError::HeterogeneousParams(unsimplified.clone());
    let (last, remaining) = unsimplified.into_popped();
    if let Some(label) = last.label {
        let last = LabeledParam {
            span: last.span,
            label,
            is_dashed: last.is_dashed,
            name: last.name,
            type_: simplify_expression(last.type_)?,
        };
        let remaining = simplify_params_but_require_labels(remaining, &hetero_err)?;
        Ok(NonEmptyParamVec::Labeled(NonEmptyVec::from_pushed(
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
        name: unsimplified.name,
        value: simplify_expression(unsimplified.value)?,
    })
}

fn simplify_expression(unsimplified: ust::Expression) -> Result<Expression, SimplifyAstError> {
    Ok(match unsimplified {
        // identifier dot call fun match forall
        ust::Expression::Identifier(unsimplified) => simplify_identifier(unsimplified),
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
        args: unsimplified.args.try_into_mapped(simplify_expression)?,
    })))
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
        params: unsimplified.params,
        output: simplify_expression(unsimplified.output)?,
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
