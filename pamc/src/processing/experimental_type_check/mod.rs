use crate::data::bound_ast::*;

#[derive(Clone, Debug)]
pub enum TypeCheckError {
    IllegalTypeExpression(Expression),
    BadCallee(Expression),
    WrongNumberOfArguments {
        call: Call,
        expected: usize,
        actual: usize,
    },
    TypeMismatch {
        expression: Expression,
        expected_type: NormalForm,
        actual_type: NormalForm,
    },
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
            output: type0_expression(context).into(),
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
            let type_: Expression = context[0].clone().into();
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
    if !is_term_a_member_of_type0_or_type1(context, type_.as_ref()) {
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
    context: &mut Context,
    let_statement: &LetStatement,
) -> Result<(), TypeCheckError> {
    let type_ = get_type_of_expression(context, &let_statement.value)?;
    context.push(type_);
    Ok(())
}

fn type_check_expression(
    context: &mut Context,
    expression: &Expression,
) -> Result<(), TypeCheckError> {
    // In the future, we could implement a version of this that skips the
    // allocations required by `get_type_of_expression`, since we don't
    // actually use the returned type.
    // But for now, we'll just reuse the existing code, for the sake of
    // simplicity.
    get_type_of_expression(context, expression).map(std::mem::drop)
}

fn get_type_of_expression(
    context: &mut Context,
    expression: &Expression,
) -> Result<NormalForm, TypeCheckError> {
    match expression {
        Expression::Name(name) => Ok(get_type_of_name(context, name)),
        Expression::Call(call) => get_type_of_call(context, call),
        Expression::Fun(fun) => get_type_of_fun(context, fun),
        Expression::Match(match_) => get_type_of_match(context, match_),
        Expression::Forall(forall) => get_type_of_forall(context, forall),
    }
}

fn get_type_of_name(context: &mut Context, name: &NameExpression) -> NormalForm {
    context[name.db_index].clone()
}

fn get_type_of_call(context: &mut Context, call: &Call) -> Result<NormalForm, TypeCheckError> {
    let callee_type = get_type_of_expression(context, &call.callee)?;
    let callee_type = if let Expression::Forall(forall) = callee_type.into() {
        forall
    } else {
        return Err(TypeCheckError::BadCallee(call.callee.clone()));
    };
    let arg_types = call
        .args
        .iter()
        .map(|arg| get_type_of_expression(context, arg))
        .collect::<Result<Vec<_>, _>>()?;
    if callee_type.params.len() != arg_types.len() {
        return Err(TypeCheckError::WrongNumberOfArguments {
            call: call.clone(),
            expected: callee_type.params.len(),
            actual: arg_types.len(),
        });
    }
    for (i, (param, arg_type)) in callee_type.params.iter().zip(arg_types.iter()).enumerate() {
        if !is_left_type_assignable_to_right_type(
            context,
            arg_type.as_nf_ref(),
            NormalFormRef::unchecked_new(&param.type_),
        ) {
            return Err(TypeCheckError::TypeMismatch {
                expression: call.args[i].clone(),
                expected_type: NormalForm::unchecked_new(param.type_.clone()),
                actual_type: arg_type.clone(),
            });
        }
    }
    Ok(NormalForm::unchecked_new(callee_type.output))
}

fn get_type_of_fun(context: &mut Context, fun: &Fun) -> Result<NormalForm, TypeCheckError> {
    let params = normalize_params_and_leave_params_in_context(context, &fun.params)?;
    {
        let return_type_type = get_type_of_expression(context, &fun.return_type)?;
        if !is_term_a_member_of_type0_or_type1(context, return_type_type.as_ref()) {
            return Err(TypeCheckError::IllegalTypeExpression(
                fun.return_type.clone(),
            ));
        }
    }
    let return_type = evaluate_well_typed_expression(context, &fun.return_type);

    let fun_type = NormalForm::unchecked_new(Expression::Forall(Box::new(Forall {
        params,
        output: return_type.clone().into(),
    })));

    context.push(fun_type.clone());

    let body_type = get_type_of_expression(context, &fun.body)?;
    if !is_left_type_assignable_to_right_type(
        context,
        body_type.as_nf_ref(),
        return_type.as_nf_ref(),
    ) {
        return Err(TypeCheckError::TypeMismatch {
            expression: fun.body.clone(),
            expected_type: return_type,
            actual_type: body_type,
        });
    }

    context.pop_n(fun.params.len() + 1);
    Ok(fun_type)
}

fn get_type_of_match(
    _context: &mut Context,
    _match_: &Match,
) -> Result<NormalForm, TypeCheckError> {
    unimplemented!()
}

fn get_type_of_forall(
    context: &mut Context,
    forall: &Forall,
) -> Result<NormalForm, TypeCheckError> {
    normalize_params_and_leave_params_in_context(context, &forall.params)?;

    let output_type = get_type_of_expression(context, &forall.output)?;
    if !is_term_a_member_of_type0_or_type1(context, output_type.as_ref()) {
        return Err(TypeCheckError::IllegalTypeExpression(forall.output.clone()));
    }

    context.pop_n(forall.params.len());

    Ok(type0_expression(context))
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
        type Output = NormalForm;

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

    impl std::convert::AsRef<Expression> for NormalForm {
        fn as_ref(&self) -> &Expression {
            &self.0
        }
    }

    impl NormalForm {
        pub fn as_nf_ref(&self) -> NormalFormRef<'_> {
            NormalFormRef::unchecked_new(&self.0)
        }
    }

    impl From<NormalForm> for Expression {
        fn from(normal_form: NormalForm) -> Self {
            normal_form.0
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct NormalFormRef<'a>(&'a Expression);

    impl<'a> NormalFormRef<'a> {
        pub fn unchecked_new(expression: &'a Expression) -> Self {
            Self(expression)
        }
    }

    impl NormalFormRef<'_> {
        pub fn raw(&self) -> &Expression {
            &self.0
        }
    }

    pub fn type0_expression(context: &Context) -> NormalForm {
        NormalForm::unchecked_new(Expression::Name(NameExpression {
            components: vec![Identifier {
                name: IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase),
                start: None,
            }],
            db_index: context.type0_dbi(),
        }))
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

    pub fn is_left_type_assignable_to_right_type(
        _context: &Context,
        _left: NormalFormRef,
        _right: NormalFormRef,
    ) -> bool {
        unimplemented!()
    }
}
