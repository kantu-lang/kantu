use crate::data::{
    simplified_ast::*,
    // `ust` stands for "unsimplified syntax tree".
    unsimplified_ast as ust,
};

#[derive(Clone, Debug)]
pub enum SimplifyAstError {
    IllegalDotLhs(ust::Expression),
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
        params: vec_result_map(unsimplified.params, simplify_param)?,
        variants: vec_result_map(unsimplified.variants, simplify_variant)?,
    })
}

fn simplify_param(unsimplified: ust::Param) -> Result<Param, SimplifyAstError> {
    Ok(Param {
        span: unsimplified.span,
        is_dashed: unsimplified.is_dashed,
        name: unsimplified.name,
        type_: simplify_expression(unsimplified.type_)?,
    })
}

fn simplify_variant(unsimplified: ust::Variant) -> Result<Variant, SimplifyAstError> {
    Ok(Variant {
        span: unsimplified.span,
        name: unsimplified.name,
        params: vec_result_map(unsimplified.params, simplify_param)?,
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
        components: vec![unsimplified],
    })
}

fn simplify_dot(unsimplified: Box<ust::Dot>) -> Result<Expression, SimplifyAstError> {
    #[derive(Clone, Debug)]
    struct NotANameExpressionError(ust::Expression);

    fn get_components(
        expr: ust::Expression,
    ) -> Result<Vec<ust::Identifier>, NotANameExpressionError> {
        match expr {
            ust::Expression::Identifier(identifier) => Ok(vec![identifier]),
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
        args: vec_result_map(unsimplified.args, simplify_expression)?,
    })))
}

fn simplify_fun(unsimplified: ust::Fun) -> Result<Expression, SimplifyAstError> {
    Ok(Expression::Fun(Box::new(Fun {
        span: unsimplified.span,
        name: unsimplified.name,
        params: vec_result_map(unsimplified.params, simplify_param)?,
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
        params: vec_result_map(unsimplified.params, simplify_param)?,
        output: simplify_expression(unsimplified.output)?,
    })))
}

fn simplify_check(unsimplified: ust::Check) -> Result<Expression, SimplifyAstError> {
    Ok(Expression::Check(Box::new(Check {
        span: unsimplified.span,
        assertions: vec_result_map(unsimplified.assertions, simplify_check_assertion)?,
        output: simplify_expression(unsimplified.output)?,
    })))
}

fn simplify_check_assertion(
    unsimplified: ust::CheckAssertion,
) -> Result<CheckAssertion, SimplifyAstError> {
    Ok(match unsimplified {
        ust::CheckAssertion::Type(assertion) => {
            CheckAssertion::Type(simplify_type_assertion(assertion)?)
        }

        ust::CheckAssertion::NormalForm(assertion) => {
            CheckAssertion::NormalForm(simplify_normal_form_assertion(assertion)?)
        }
    })
}

fn simplify_type_assertion(
    unsimplified: ust::TypeAssertion,
) -> Result<TypeAssertion, SimplifyAstError> {
    Ok(TypeAssertion {
        span: unsimplified.span,
        left: simplify_expression(unsimplified.left)?,
        right: simplify_question_mark_or_expression(unsimplified.right)?,
    })
}

fn simplify_normal_form_assertion(
    unsimplified: ust::NormalFormAssertion,
) -> Result<NormalFormAssertion, SimplifyAstError> {
    Ok(NormalFormAssertion {
        span: unsimplified.span,
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
