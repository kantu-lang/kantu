use crate::data::{unregistered_ast as ast, unregistered_sst::*};

#[derive(Clone, Debug)]
pub enum SimplifyAstError {
    IllegalDotLhs(ast::Expression),
}

pub fn simplify_file(unsimplified: ast::File) -> Result<File, SimplifyAstError> {
    Ok(File {
        id: unsimplified.id,
        items: unsimplified
            .items
            .into_iter()
            .map(simplify_file_item)
            .collect::<Result<Vec<_>, SimplifyAstError>>()?,
    })
}

fn simplify_file_item(unsimplified: ast::FileItem) -> Result<FileItem, SimplifyAstError> {
    Ok(match unsimplified {
        ast::FileItem::Type(unsimplified) => FileItem::Type(simplify_type_statement(unsimplified)?),
        ast::FileItem::Let(unsimplified) => FileItem::Let(simplify_let_statement(unsimplified)?),
    })
}

fn simplify_type_statement(
    unsimplified: ast::TypeStatement,
) -> Result<TypeStatement, SimplifyAstError> {
    let name = unsimplified.name;
    let params = unsimplified
        .params
        .into_iter()
        .map(simplify_param)
        .collect::<Result<Vec<_>, SimplifyAstError>>()?;
    let variants = unsimplified
        .variants
        .into_iter()
        .map(simplify_variant)
        .collect::<Result<Vec<_>, SimplifyAstError>>()?;
    Ok(TypeStatement {
        name,
        params,
        variants,
    })
}

fn simplify_param(unsimplified: ast::Param) -> Result<Param, SimplifyAstError> {
    Ok(Param {
        is_dashed: unsimplified.is_dashed,
        name: unsimplified.name,
        type_: simplify_expression(unsimplified.type_)?,
    })
}

fn simplify_variant(unsimplified: ast::Variant) -> Result<Variant, SimplifyAstError> {
    let params = unsimplified
        .params
        .into_iter()
        .map(simplify_param)
        .collect::<Result<Vec<_>, SimplifyAstError>>()?;
    Ok(Variant {
        name: unsimplified.name,
        params,
        return_type: simplify_expression(unsimplified.return_type)?,
    })
}

fn simplify_let_statement(
    unsimplified: ast::LetStatement,
) -> Result<LetStatement, SimplifyAstError> {
    Ok(LetStatement {
        name: unsimplified.name,
        value: simplify_expression(unsimplified.value)?,
    })
}

fn simplify_expression(unsimplified: ast::Expression) -> Result<Expression, SimplifyAstError> {
    Ok(match unsimplified {
        // identifier dot call fun match forall
        ast::Expression::Identifier(unsimplified) => simplify_identifier(unsimplified),
        ast::Expression::Dot(unsimplified) => simplify_dot(unsimplified)?,
        ast::Expression::Call(unsimplified) => simplify_call(*unsimplified)?,
        ast::Expression::Fun(unsimplified) => simplify_fun(*unsimplified)?,
        ast::Expression::Match(unsimplified) => simplify_match(*unsimplified)?,
        ast::Expression::Forall(unsimplified) => simplify_forall(*unsimplified)?,
    })
}

fn simplify_identifier(unsimplified: ast::Identifier) -> Expression {
    Expression::Name(NameExpression {
        components: vec![unsimplified],
    })
}

fn simplify_dot(unsimplified: Box<ast::Dot>) -> Result<Expression, SimplifyAstError> {
    #[derive(Clone, Debug)]
    struct NotANameExpressionError(ast::Expression);

    fn get_components(
        expr: ast::Expression,
    ) -> Result<Vec<ast::Identifier>, NotANameExpressionError> {
        match expr {
            ast::Expression::Identifier(identifier) => Ok(vec![identifier]),
            ast::Expression::Dot(dot) => {
                let mut components = get_components(dot.left)?;
                components.push(dot.right);
                Ok(components)
            }
            other => Err(NotANameExpressionError(other)),
        }
    }

    Ok(Expression::Name(NameExpression {
        components: get_components(ast::Expression::Dot(unsimplified))
            .map_err(|err| SimplifyAstError::IllegalDotLhs(err.0))?,
    }))
}

fn simplify_call(unsimplified: ast::Call) -> Result<Expression, SimplifyAstError> {
    Ok(Expression::Call(Box::new(Call {
        callee: simplify_expression(unsimplified.callee)?,
        args: unsimplified
            .args
            .into_iter()
            .map(simplify_expression)
            .collect::<Result<Vec<_>, SimplifyAstError>>()?,
    })))
}

fn simplify_fun(unsimplified: ast::Fun) -> Result<Expression, SimplifyAstError> {
    Ok(Expression::Fun(Box::new(Fun {
        name: unsimplified.name,
        params: unsimplified
            .params
            .into_iter()
            .map(simplify_param)
            .collect::<Result<Vec<_>, SimplifyAstError>>()?,
        return_type: simplify_expression(unsimplified.return_type)?,
        body: simplify_expression(unsimplified.body)?,
    })))
}

fn simplify_match(unsimplified: ast::Match) -> Result<Expression, SimplifyAstError> {
    Ok(Expression::Match(Box::new(Match {
        matchee: simplify_expression(unsimplified.matchee)?,
        cases: unsimplified
            .cases
            .into_iter()
            .map(simplify_match_case)
            .collect::<Result<Vec<_>, SimplifyAstError>>()?,
    })))
}

fn simplify_match_case(unsimplified: ast::MatchCase) -> Result<MatchCase, SimplifyAstError> {
    Ok(MatchCase {
        variant_name: unsimplified.variant_name,
        params: unsimplified.params,
        output: simplify_expression(unsimplified.output)?,
    })
}

fn simplify_forall(unsimplified: ast::Forall) -> Result<Expression, SimplifyAstError> {
    Ok(Expression::Forall(Box::new(Forall {
        params: unsimplified
            .params
            .into_iter()
            .map(simplify_param)
            .collect::<Result<Vec<_>, SimplifyAstError>>()?,
        output: simplify_expression(unsimplified.output)?,
    })))
}
