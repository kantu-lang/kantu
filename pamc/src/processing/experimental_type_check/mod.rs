use crate::data::bound_ast::*;

#[derive(Clone, Debug)]
pub enum TypeCheckError {
    IllegalTypeExpression(Expression),
}

pub fn type_check_files(files: &[File]) -> Result<(), TypeCheckError> {
    let mut context = Context::with_builtins();
    for file in files {
        type_check_file(&mut context, file)?;
    }
    Ok(())
}

fn type_check_file(context: &mut Context, file: &File) -> Result<(), TypeCheckError> {
    for item in &file.items {
        type_check_file_item(context, item)?;
    }
    context.pop_n(file.items.len());
    Ok(())
}

fn type_check_file_item(context: &mut Context, item: &FileItem) -> Result<(), TypeCheckError> {
    match item {
        FileItem::Type(type_statement) => type_check_type_statement(context, type_statement),
        FileItem::Let(let_statement) => type_check_let_statement(context, let_statement),
    }
}

fn type_check_type_statement(
    context: &mut Context,
    type_statement: &TypeStatement,
) -> Result<(), TypeCheckError> {
    type_check_type_constructor(context, type_statement)?;
    for variant in &type_statement.variants {
        type_check_type_variant(context, variant)?;
    }
    Ok(())
}

fn type_check_type_constructor(
    context: &mut Context,
    type_statement: &TypeStatement,
) -> Result<(), TypeCheckError> {
    let params = normalize_params(context, &type_statement.params)?;
    let type_constructor_type = NormalForm::unchecked_new(
        Forall {
            params,
            output: type0_expression(context),
        }
        .collapse_if_nullary(),
    );
    context.push(type_constructor_type);
    Ok(())
}

fn normalize_params(context: &mut Context, params: &[Param]) -> Result<Vec<Param>, TypeCheckError> {
    let normalized = normalize_params_and_leave_params_in_context(context, params)?;
    context.pop_n(params.len());
    Ok(normalized)
}

fn normalize_params_and_leave_params_in_context(
    context: &mut Context,
    params: &[Param],
) -> Result<Vec<Param>, TypeCheckError> {
    let normalized = params
        .iter()
        .map(|param| {
            type_check_param(context, param)?;
            let type_ = context[0].clone();
            Ok(Param {
                is_dashed: param.is_dashed,
                name: param.name.clone(),
                type_,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(normalized)
}

fn type_check_param(context: &mut Context, param: &Param) -> Result<(), TypeCheckError> {
    type_check_expression(context, &param.type_)?;
    let type_ = evaluate_well_typed_expression(context, &param.type_);
    if !is_term_a_member_of_type0_or_type1(context, &type_) {
        return Err(TypeCheckError::IllegalTypeExpression(type_.into()));
    }
    context.push(type_);
    Ok(())
}

fn type_check_type_variant(context: &mut Context, variant: &Variant) -> Result<(), TypeCheckError> {
    let arity = variant.params.len();
    let params = normalize_params_and_leave_params_in_context(context, &variant.params)?;
    type_check_expression(context, &variant.return_type)?;
    let return_type = evaluate_well_typed_expression(context, &variant.return_type);
    let type_ = NormalForm::unchecked_new(
        Forall {
            params,
            output: return_type.into(),
        }
        .collapse_if_nullary(),
    );
    context.pop_n(arity);
    context.push(type_);
    Ok(())
}

fn type_check_let_statement(
    _context: &mut Context,
    _let_statement: &LetStatement,
) -> Result<(), TypeCheckError> {
    unimplemented!()
}

fn type_check_expression(
    _context: &mut Context,
    _expression: &Expression,
) -> Result<(), TypeCheckError> {
    unimplemented!()
}

fn evaluate_well_typed_expression(_context: &mut Context, _expression: &Expression) -> NormalForm {
    unimplemented!()
}

use context::*;
mod context {
    use super::*;

    use std::ops::Index;

    pub struct Context {
        stack: Vec<NormalForm>,
    }

    impl Context {
        pub fn with_builtins() -> Self {
            Self { stack: Vec::new() }
        }
    }

    impl Context {
        /// Panics if `n > self.len()`.
        pub fn pop_n(&mut self, _n: usize) {
            unimplemented!()
        }

        pub fn push(&mut self, expression: NormalForm) {
            self.stack.push(expression);
        }
    }

    impl Context {
        /// Returns the De Bruijn index of the `Type0` expression.
        pub fn type0_dbi(&self) -> usize {
            unimplemented!()
        }
    }

    impl Index<usize> for Context {
        type Output = Expression;

        fn index(&self, index: usize) -> &Self::Output {
            &self.stack[self.stack.len() - index - 1]
        }
    }
}

use misc::*;
mod misc {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct NormalForm(Expression);

    impl NormalForm {
        pub fn unchecked_new(expression: Expression) -> Self {
            Self(expression)
        }
    }

    impl std::ops::Deref for NormalForm {
        type Target = Expression;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl From<NormalForm> for Expression {
        fn from(normal_form: NormalForm) -> Self {
            normal_form.0
        }
    }

    pub fn type0_expression(context: &Context) -> Expression {
        Expression::Name(NameExpression {
            components: vec![Identifier {
                name: IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase),
                start: None,
            }],
            db_index: context.type0_dbi(),
        })
    }

    impl Forall {
        pub fn collapse_if_nullary(self) -> Expression {
            if self.params.is_empty() {
                self.output
            } else {
                Expression::Forall(Box::new(self))
            }
        }
    }

    pub fn is_term_a_member_of_type0_or_type1(_context: &Context, _term: &Expression) -> bool {
        unimplemented!()
    }
}
