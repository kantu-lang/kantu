use crate::data::{
    x_light_ast::*,
    x_node_registry::{ListId, NodeId, NodeRegistry},
};

#[derive(Clone, Debug)]
pub enum TypeCheckError {
    IllegalTypeExpression(ExpressionId),
    BadCallee(ExpressionId),
    WrongNumberOfArguments {
        call: NodeId<Call>,
        expected: usize,
        actual: usize,
    },
    TypeMismatch {
        expression: ExpressionId,
        expected_type: NormalForm,
        actual_type: NormalForm,
    },
}

pub fn type_check_files(
    registry: &mut NodeRegistry,
    file_ids: &[NodeId<File>],
) -> Result<(), TypeCheckError> {
    let mut context = Context::with_builtins();
    for &id in file_ids {
        type_check_file(&mut context, registry, id)?;
    }
    Ok(())
}

fn type_check_file(
    context: &mut Context,
    registry: &mut NodeRegistry,
    file_id: NodeId<File>,
) -> Result<(), TypeCheckError> {
    let file = registry.file(file_id);
    let items = registry.file_item_list(file.item_list_id).to_vec();
    for &item_id in &items {
        type_check_file_item(context, registry, item_id)?;
    }
    context.pop_n(items.len());
    Ok(())
}

fn type_check_file_item(
    context: &mut Context,
    registry: &mut NodeRegistry,
    item: FileItemNodeId,
) -> Result<(), TypeCheckError> {
    match item {
        FileItemNodeId::Type(type_statement) => {
            type_check_type_statement(context, registry, type_statement)
        }
        FileItemNodeId::Let(let_statement) => {
            type_check_let_statement(context, registry, let_statement)
        }
    }
}

fn type_check_type_statement(
    context: &mut Context,
    registry: &mut NodeRegistry,
    type_statement_id: NodeId<TypeStatement>,
) -> Result<(), TypeCheckError> {
    type_check_type_constructor(context, registry, type_statement_id)?;

    let type_statement = registry.type_statement(type_statement_id);
    let variant_ids = registry
        .variant_list(type_statement.variant_list_id)
        .to_vec();
    for variant_id in variant_ids {
        type_check_type_variant(context, registry, variant_id)?;
    }

    Ok(())
}

fn type_check_type_constructor(
    context: &mut Context,
    registry: &mut NodeRegistry,
    type_statement_id: NodeId<TypeStatement>,
) -> Result<(), TypeCheckError> {
    let type_statement = registry.type_statement(type_statement_id);
    let normalized_param_list_id =
        normalize_params(context, registry, type_statement.param_list_id)?;
    let type_constructor_type = NormalForm::unchecked_new(
        Forall {
            id: dummy_id(),
            param_list_id: normalized_param_list_id,
            output_id: type0_expression(context, registry).raw(),
        }
        .collapse_if_nullary(registry),
    );
    context.push(type_constructor_type);
    Ok(())
}

fn normalize_params(
    context: &mut Context,
    registry: &mut NodeRegistry,
    param_list_id: ListId<NodeId<Param>>,
) -> Result<ListId<NodeId<Param>>, TypeCheckError> {
    let normalized =
        normalize_params_and_leave_params_in_context(context, registry, param_list_id)?;
    context.pop_n(param_list_id.len);
    Ok(normalized)
}

fn normalize_params_and_leave_params_in_context(
    context: &mut Context,
    registry: &mut NodeRegistry,
    param_list_id: ListId<NodeId<Param>>,
) -> Result<ListId<NodeId<Param>>, TypeCheckError> {
    let param_ids = registry.param_list(param_list_id).to_vec();
    let normalized_ids = param_ids
        .iter()
        .copied()
        .map(|param_id| {
            type_check_param(context, registry, param_id)?;
            let type_id: ExpressionId = context.index(0).raw();
            let old_param = registry.param(param_id);
            let normalized_id = registry.add_param_and_overwrite_its_id(Param {
                id: dummy_id(),
                is_dashed: old_param.is_dashed,
                name_id: old_param.name_id,
                type_id,
            });
            Ok(normalized_id)
        })
        .collect::<Result<Vec<_>, _>>()?;
    registry.add_param_list(normalized_ids)
}

fn type_check_param(
    context: &mut Context,
    registry: &mut NodeRegistry,
    param_id: NodeId<Param>,
) -> Result<(), TypeCheckError> {
    let param = registry.param(param_id);
    type_check_expression(context, &param.type_)?;
    let type_ = evaluate_well_typed_expression(context, &param.type_);
    if !is_term_a_member_of_type0_or_type1(context, type_.as_nf_ref()) {
        return Err(TypeCheckError::IllegalTypeExpression(type_.into()));
    }
    context.push(type_);
    Ok(())
}

fn type_check_type_variant(
    context: &mut Context,
    registry: &mut NodeRegistry,
    variant_id: NodeId<Variant>,
) -> Result<(), TypeCheckError> {
    let variant = registry.variant(variant_id);
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
    registry: &mut NodeRegistry,
    let_statement_id: NodeId<LetStatement>,
) -> Result<(), TypeCheckError> {
    let let_statement = registry.let_statement(let_statement_id);
    let type_ = get_type_of_expression(context, registry, let_statement.value_id)?;
    context.push(type_);
    Ok(())
}

fn type_check_expression(
    context: &mut Context,
    registry: &mut NodeRegistry,
    expression: ExpressionId,
) -> Result<(), TypeCheckError> {
    // In the future, we could implement a version of this that skips the
    // allocations required by `get_type_of_expression`, since we don't
    // actually use the returned type.
    // But for now, we'll just reuse the existing code, for the sake of
    // simplicity.
    get_type_of_expression(context, registry, expression).map(std::mem::drop)
}

fn get_type_of_expression(
    context: &mut Context,
    registry: &mut NodeRegistry,
    id: ExpressionId,
) -> Result<NormalForm, TypeCheckError> {
    match id {
        ExpressionId::Name(name) => Ok(get_type_of_name(context, registry, name)),
        ExpressionId::Call(call) => get_type_of_call(context, registry, call),
        ExpressionId::Fun(fun) => get_type_of_fun(context, registry, fun),
        ExpressionId::Match(match_) => get_type_of_match(context, registry, match_),
        ExpressionId::Forall(forall) => get_type_of_forall(context, registry, forall),
    }
}

fn get_type_of_name(
    context: &mut Context,
    registry: &mut NodeRegistry,
    name_id: NodeId<NameExpression>,
) -> NormalForm {
    let name = registry.name_expression(name_id);
    context.index(name.db_index)
}

fn get_type_of_call(
    context: &mut Context,
    registry: &mut NodeRegistry,
    call_id: NodeId<Call>,
) -> Result<NormalForm, TypeCheckError> {
    let call = registry.call(call_id);
    let callee_type = get_type_of_expression(context, registry, &call.callee)?;
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

fn get_type_of_fun(
    context: &mut Context,
    registry: &mut NodeRegistry,
    fun: &Fun,
) -> Result<NormalForm, TypeCheckError> {
    let params = normalize_params_and_leave_params_in_context(context, &fun.params)?;
    {
        let return_type_type = get_type_of_expression(context, &fun.return_type)?;
        if !is_term_a_member_of_type0_or_type1(context, return_type_type.as_nf_ref()) {
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
    registry: &mut NodeRegistry,
    _match_: &Match,
) -> Result<NormalForm, TypeCheckError> {
    unimplemented!()
}

fn get_type_of_forall(
    context: &mut Context,
    registry: &mut NodeRegistry,
    forall: &Forall,
) -> Result<NormalForm, TypeCheckError> {
    normalize_params_and_leave_params_in_context(context, &forall.params)?;

    let output_type = get_type_of_expression(context, &forall.output)?;
    if !is_term_a_member_of_type0_or_type1(context, output_type.as_nf_ref()) {
        return Err(TypeCheckError::IllegalTypeExpression(forall.output.clone()));
    }

    context.pop_n(forall.params.len());

    Ok(type0_expression(context))
}

fn evaluate_well_typed_expression(
    _context: &mut Context,
    _registry: &mut NodeRegistry,
    _expression: &Expression,
) -> NormalForm {
    unimplemented!()
}

use context::*;
mod context {
    use super::*;

    pub struct Context {
        /// Each type in the stack is expressed "locally" (i.e., relative
        /// to its position within the stack).
        ///
        /// For example, consider the scenario where `local_type_stack[1] == NameExpression { db_index: 0 }`.
        /// The local De Bruijn index `0` refers to the first symbol counting right-to-left _from position 1_.
        /// Thus, if `local_type_stack.len() == 3`, for example, then the global De Bruijn index for `local_type_stack[1]` is `2`.
        ///
        /// If an illustration would help, consider the following:
        /// ```text
        /// Type1: DNE
        /// Type0: Type1
        /// Nat: Type0
        ///
        /// ----------------------
        ///
        /// local_type_stack: [Type1, Type0, Nat] = [DNE, 0, 0]
        ///
        /// ----------------------
        ///
        /// local_type(Type0) = Type1 = 0
        /// // Why? - Count backwards from Type0 (not including Type0 itself):
        ///
        /// vvv
        /// (0)
        /// [Type1, Type0, Nat]
        ///
        /// ----------------------
        ///
        /// global_type(Type0) = Type1 = 2
        /// // Why? - Count backwards from the end of the stack (including the last item):
        ///
        /// vvv
        /// (2)     (1)    (0)
        /// [Type1, Type0, Nat]
        /// ```
        ///
        local_type_stack: Vec<NormalForm>,
    }

    const TYPE1_LEVEL: usize = 0;
    const TYPE0_LEVEL: usize = 1;

    impl Context {
        pub fn with_builtins() -> Self {
            // We should will never retrieve the type of `Type1`, since it is undefined.
            // However, we need to store _some_ object in the stack, so that the indices
            // of the other types are correct.
            let dummy_type1_type = NormalForm::unchecked_new(Expression::Name(NameExpression {
                components: vec![Identifier {
                    name: IdentifierName::Standard("Type2".to_owned()),
                    start: None,
                }],
                db_index: 0,
            }));
            let type0_type = NormalForm::unchecked_new(Expression::Name(NameExpression {
                components: vec![Identifier {
                    name: IdentifierName::Standard("Type1".to_owned()),
                    start: None,
                }],
                db_index: 0,
            }));
            Self {
                local_type_stack: vec![dummy_type1_type, type0_type],
            }
        }
    }

    impl Context {
        /// Panics if `n > self.len()`.
        pub fn pop_n(&mut self, n: usize) {
            if n > self.len() {
                panic!(
                    "Tried to pop {} elements from a context with only {} elements",
                    n,
                    self.len()
                );
            }
            self.local_type_stack.truncate(self.len() - n);
        }

        pub fn push(&mut self, expression: NormalForm) {
            self.local_type_stack.push(expression);
        }

        pub fn len(&self) -> usize {
            self.local_type_stack.len()
        }
    }

    impl Context {
        /// Returns the De Bruijn index of the `Type0` expression.
        pub fn type0_dbi(&self) -> usize {
            self.level_to_index(TYPE0_LEVEL)
        }

        /// Returns the De Bruijn index of the `Type1` expression.
        pub fn type1_dbi(&self) -> usize {
            self.level_to_index(TYPE1_LEVEL)
        }
    }

    impl Context {
        fn level_to_index(&self, level: usize) -> usize {
            self.len() - level - 1
        }

        fn index_to_level(&self, index: usize) -> usize {
            self.len() - index - 1
        }
    }

    impl Context {
        pub fn index(&self, index: usize) -> NormalForm {
            let level = self.index_to_level(index);
            if level == TYPE1_LEVEL {
                panic!("Type1 has no type. We may add support for infinite type hierarchies in the future. However, for now, Type1 is the \"limit\" type.");
            }
            self.local_type_stack[level].clone().shift_up(index + 1)
        }
    }
}

use misc::*;
mod misc {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct NormalForm(ExpressionId);

    impl NormalForm {
        pub fn unchecked_new(expression: ExpressionId) -> Self {
            Self(expression)
        }
    }

    impl NormalForm {
        pub fn raw(self) -> ExpressionId {
            self.0
        }
    }

    pub fn type0_expression(context: &Context, registry: &mut NodeRegistry) -> NormalForm {
        let name_id = get_name_expression(
            registry,
            vec![Identifier {
                id: dummy_id(),
                name: IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase),
                start: None,
            }],
            context.type0_dbi(),
        );
        NormalForm::unchecked_new(ExpressionId::Name(name_id))
    }

    fn get_name_expression(
        registry: &mut NodeRegistry,
        components: Vec<Identifier>,
        db_index: usize,
    ) -> NodeId<NameExpression> {
        let component_ids = components
            .into_iter()
            .map(|component| registry.add_identifier_and_overwrite_its_id(component))
            .collect();
        let component_list_id = registry.add_identifier_list(component_ids);
        registry.add_name_expression_and_overwrite_its_id(NameExpression {
            id: dummy_id(),
            component_list_id,
            db_index,
        })
    }

    pub fn dummy_id<T>() -> NodeId<T> {
        NodeId::new(0)
    }

    impl Forall {
        pub fn collapse_if_nullary(self, registry: &mut NodeRegistry) -> ExpressionId {
            if self.param_list_id.len == 0 {
                self.output_id
            } else {
                let forall_id = registry.add_forall_and_overwrite_its_id(self);
                ExpressionId::Forall(forall_id)
            }
        }
    }

    pub fn is_term_a_member_of_type0_or_type1(context: &Context, term: NormalFormRef) -> bool {
        if let Expression::Name(name) = term.raw() {
            let i = name.db_index;
            i == context.type0_dbi() || i == context.type1_dbi()
        } else {
            false
        }
    }

    pub fn is_left_type_assignable_to_right_type(
        _context: &Context,
        _left: NormalFormRef,
        _right: NormalFormRef,
    ) -> bool {
        unimplemented!()
    }

    impl NormalForm {
        pub fn shift_up(self, amount: usize) -> Self {
            Self::unchecked_new(self.0.shift_up(amount))
        }
    }

    impl Expression {
        pub fn shift_up(self, amount: usize) -> Expression {
            self.shift_up_with_cutoff(amount, 0)
        }

        fn shift_up_with_cutoff(self, _amount: usize, _cutoff: usize) -> Expression {
            unimplemented!()
        }
    }
}
