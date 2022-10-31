use crate::data::{
    x_light_ast::*,
    x_node_registry::{ListId, NodeId, NodeRegistry},
};

#[derive(Clone, Debug)]
pub enum TypeCheckError {
    IllegalTypeExpression(ExpressionId),
    BadCallee(ExpressionId),
    WrongNumberOfArguments {
        call_id: NodeId<Call>,
        expected: usize,
        actual: usize,
    },
    TypeMismatch {
        expression_id: ExpressionId,
        expected_type_id: NormalFormId,
        actual_type_id: NormalFormId,
    },
}

pub fn type_check_files(
    registry: &mut NodeRegistry,
    file_ids: &[NodeId<File>],
) -> Result<(), TypeCheckError> {
    let mut context = Context::with_builtins(registry);
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
    let type_statement = registry.type_statement(type_statement_id).clone();
    let normalized_param_list_id =
        normalize_params(context, registry, type_statement.param_list_id)?;
    let type_constructor_type_id = NormalFormId::unchecked_new(
        Forall {
            id: dummy_id(),
            param_list_id: normalized_param_list_id,
            output_id: type0_expression(context, registry).raw(),
        }
        .collapse_if_nullary(registry),
    );
    let variant_name_list_id = {
        let variant_ids = registry.variant_list(type_statement.variant_list_id);
        let variant_name_ids = variant_ids
            .iter()
            .map(|&variant_id| registry.variant(variant_id).name_id)
            .collect();
        registry.add_identifier_list(variant_name_ids)
    };
    context.push(ContextEntry {
        type_id: type_constructor_type_id,
        definition: ContextEntryDefinition::Adt {
            variant_name_list_id,
        },
    });
    Ok(())
}

fn normalize_params(
    context: &mut Context,
    registry: &mut NodeRegistry,
    param_list_id: ListId<NodeId<Param>>,
) -> Result<ListId<NodeId<Param>>, TypeCheckError> {
    let normalized_list_id =
        normalize_params_and_leave_params_in_context(context, registry, param_list_id)?;
    context.pop_n(param_list_id.len);
    Ok(normalized_list_id)
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
            let type_id: ExpressionId = context.get_type(DbIndex(0), registry).raw();
            let old_param = registry.param(param_id);
            let normalized_param_with_dummy_id = Param {
                id: dummy_id(),
                is_dashed: old_param.is_dashed,
                name_id: old_param.name_id,
                type_id,
            };
            let normalized_id =
                registry.add_param_and_overwrite_its_id(normalized_param_with_dummy_id);
            Ok(normalized_id)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(registry.add_param_list(normalized_ids))
}

fn type_check_param(
    context: &mut Context,
    registry: &mut NodeRegistry,
    param_id: NodeId<Param>,
) -> Result<(), TypeCheckError> {
    let param = registry.param(param_id).clone();
    let param_type_type_id = get_type_of_expression(context, registry, param.type_id)?;
    if !is_term_equal_to_type0_or_type1(context, registry, param_type_type_id) {
        return Err(TypeCheckError::IllegalTypeExpression(param.type_id));
    }

    let normalized_type_id = evaluate_well_typed_expression(context, registry, param.type_id);
    context.push(ContextEntry {
        type_id: normalized_type_id,
        definition: ContextEntryDefinition::Uninterpreted,
    });
    Ok(())
}

fn type_check_type_variant(
    context: &mut Context,
    registry: &mut NodeRegistry,
    variant_id: NodeId<Variant>,
) -> Result<(), TypeCheckError> {
    let variant = registry.variant(variant_id).clone();
    let arity = variant.param_list_id.len;
    let normalized_param_list_id =
        normalize_params_and_leave_params_in_context(context, registry, variant.param_list_id)?;
    type_check_expression(context, registry, variant.return_type_id)?;
    let return_type_id = evaluate_well_typed_expression(context, registry, variant.return_type_id);
    let type_id = NormalFormId::unchecked_new(
        Forall {
            id: dummy_id(),
            param_list_id: normalized_param_list_id,
            output_id: return_type_id.raw(),
        }
        .collapse_if_nullary(registry),
    );
    context.pop_n(arity);
    context.push(ContextEntry {
        type_id,
        definition: ContextEntryDefinition::Variant {
            name_id: variant.name_id,
        },
    });
    Ok(())
}

fn type_check_let_statement(
    context: &mut Context,
    registry: &mut NodeRegistry,
    let_statement_id: NodeId<LetStatement>,
) -> Result<(), TypeCheckError> {
    let let_statement = registry.let_statement(let_statement_id).clone();
    let type_id = get_type_of_expression(context, registry, let_statement.value_id)?;
    let normalized_value_id =
        evaluate_well_typed_expression(context, registry, let_statement.value_id);
    context.push(ContextEntry {
        type_id,
        definition: ContextEntryDefinition::Alias {
            value_id: normalized_value_id,
        },
    });
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
) -> Result<NormalFormId, TypeCheckError> {
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
) -> NormalFormId {
    let name = registry.name_expression(name_id);
    context.get_type(name.db_index, registry)
}

fn get_type_of_call(
    context: &mut Context,
    registry: &mut NodeRegistry,
    call_id: NodeId<Call>,
) -> Result<NormalFormId, TypeCheckError> {
    let call = registry.call(call_id).clone();
    let callee_type_id = get_type_of_expression(context, registry, call.callee_id)?;
    let callee_type_id = if let ExpressionId::Forall(id) = callee_type_id.raw() {
        id
    } else {
        return Err(TypeCheckError::BadCallee(call.callee_id));
    };
    let arg_ids = registry.expression_list(call.arg_list_id).to_vec();
    let arg_type_ids = arg_ids
        .iter()
        .copied()
        .map(|arg_id| get_type_of_expression(context, registry, arg_id))
        .collect::<Result<Vec<_>, _>>()?;
    let callee_type = registry.forall(callee_type_id);
    // We use the params of the callee _type_ rather than the params of the
    // callee itself, since the callee type is a normal form, which guarantees
    // that its params are normal forms.
    let callee_type_param_ids = registry.param_list(callee_type.param_list_id).to_vec();
    {
        let expected_arity = callee_type_param_ids.len();
        let actual_arity = arg_ids.len();
        if callee_type_param_ids.len() != arg_type_ids.len() {
            return Err(TypeCheckError::WrongNumberOfArguments {
                call_id: call_id,
                expected: expected_arity,
                actual: actual_arity,
            });
        }
    }
    for (i, (callee_type_param_id, arg_type_id)) in callee_type_param_ids
        .iter()
        .copied()
        .zip(arg_type_ids.iter().copied())
        .enumerate()
    {
        let callee_type_param = registry.param(callee_type_param_id);
        if !is_left_type_assignable_to_right_type(
            context,
            registry,
            arg_type_id,
            // This is safe because the param is the param of a normal
            // form Forall node, which guarantees that its type is a
            // normal form.
            NormalFormId::unchecked_new(callee_type_param.type_id),
        ) {
            return Err(TypeCheckError::TypeMismatch {
                expression_id: arg_ids[i],
                expected_type_id: NormalFormId::unchecked_new(callee_type_param.type_id),
                actual_type_id: arg_type_id,
            });
        }
    }
    Ok(NormalFormId::unchecked_new(callee_type.output_id))
}

fn get_type_of_fun(
    context: &mut Context,
    registry: &mut NodeRegistry,
    fun_id: NodeId<Fun>,
) -> Result<NormalFormId, TypeCheckError> {
    let original_context_len = context.len();

    let fun = registry.fun(fun_id).clone();
    let normalized_param_list_id =
        normalize_params_and_leave_params_in_context(context, registry, fun.param_list_id)?;
    {
        let return_type_type_id = get_type_of_expression(context, registry, fun.return_type_id)?;
        if !is_term_equal_to_type0_or_type1(context, registry, return_type_type_id) {
            return Err(TypeCheckError::IllegalTypeExpression(fun.return_type_id));
        }
    }
    let normalized_return_type_id =
        evaluate_well_typed_expression(context, registry, fun.return_type_id);

    let fun_type_id = NormalFormId::unchecked_new(ExpressionId::Forall(
        registry.add_forall_and_overwrite_its_id(Forall {
            id: dummy_id(),
            param_list_id: normalized_param_list_id,
            output_id: normalized_return_type_id.raw(),
        }),
    ));

    let shifted_fun_id = fun_id.upshift(context.len() - original_context_len, registry);
    let normalized_fun_id =
        evaluate_well_typed_expression(context, registry, ExpressionId::Fun(shifted_fun_id));
    context.push(ContextEntry {
        type_id: fun_type_id,
        definition: ContextEntryDefinition::Alias {
            value_id: normalized_fun_id,
        },
    });

    let normalized_body_type_id = get_type_of_expression(context, registry, fun.body_id)?;
    if !is_left_type_assignable_to_right_type(
        context,
        registry,
        normalized_body_type_id,
        normalized_return_type_id,
    ) {
        return Err(TypeCheckError::TypeMismatch {
            expression_id: fun.body_id,
            expected_type_id: normalized_return_type_id,
            actual_type_id: normalized_body_type_id,
        });
    }

    context.pop_n(fun.param_list_id.len + 1);
    Ok(fun_type_id)
}

fn get_type_of_match(
    _context: &mut Context,
    _registry: &mut NodeRegistry,
    _match_id: NodeId<Match>,
) -> Result<NormalFormId, TypeCheckError> {
    unimplemented!()
}

fn get_type_of_forall(
    context: &mut Context,
    registry: &mut NodeRegistry,
    forall_id: NodeId<Forall>,
) -> Result<NormalFormId, TypeCheckError> {
    let forall = registry.forall(forall_id).clone();
    normalize_params_and_leave_params_in_context(context, registry, forall.param_list_id)?;

    let output_type_id = get_type_of_expression(context, registry, forall.output_id)?;
    if !is_term_equal_to_type0_or_type1(context, registry, output_type_id) {
        return Err(TypeCheckError::IllegalTypeExpression(forall.output_id));
    }

    context.pop_n(forall.param_list_id.len);

    Ok(type0_expression(context, registry))
}

fn evaluate_well_typed_expression(
    _context: &mut Context,
    _registry: &mut NodeRegistry,
    _id: ExpressionId,
) -> NormalFormId {
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
        local_type_stack: Vec<ContextEntry>,
    }

    #[derive(Clone, Debug)]
    pub struct ContextEntry {
        pub type_id: NormalFormId,
        pub definition: ContextEntryDefinition,
    }

    #[derive(Clone, Debug)]
    pub enum ContextEntryDefinition {
        Alias {
            value_id: NormalFormId,
        },
        /// Algebraic data type
        Adt {
            variant_name_list_id: ListId<NodeId<Identifier>>,
        },
        Variant {
            name_id: NodeId<Identifier>,
        },
        Uninterpreted,
    }

    const TYPE1_LEVEL: DbLevel = DbLevel(0);
    const TYPE0_LEVEL: DbLevel = DbLevel(1);

    impl Context {
        pub fn with_builtins(registry: &mut NodeRegistry) -> Self {
            // We should will never retrieve the type of `Type1`, since it is undefined.
            // However, we need to store _some_ object in the stack, so that the indices
            // of the other types are correct.
            let type1_entry = {
                let dummy_type1_type_id = NormalFormId::unchecked_new(ExpressionId::Name(
                    add_name_expression_and_overwrite_component_ids(
                        registry,
                        vec![Identifier {
                            id: dummy_id(),
                            name: IdentifierName::Standard("Type2".to_owned()),
                            start: None,
                        }],
                        DbIndex(0),
                    ),
                ));
                ContextEntry {
                    type_id: dummy_type1_type_id,
                    definition: ContextEntryDefinition::Uninterpreted,
                }
            };
            let type0_entry = {
                let type0_type_id = NormalFormId::unchecked_new(ExpressionId::Name(
                    add_name_expression_and_overwrite_component_ids(
                        registry,
                        vec![Identifier {
                            id: dummy_id(),
                            name: IdentifierName::Standard("Type1".to_owned()),
                            start: None,
                        }],
                        DbIndex(0),
                    ),
                ));
                ContextEntry {
                    type_id: type0_type_id,
                    definition: ContextEntryDefinition::Uninterpreted,
                }
            };
            Self {
                local_type_stack: vec![type1_entry, type0_entry],
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

        pub fn push(&mut self, entry: ContextEntry) {
            self.local_type_stack.push(entry);
        }

        pub fn len(&self) -> usize {
            self.local_type_stack.len()
        }
    }

    impl Context {
        /// Returns the De Bruijn index of the `Type0` expression.
        pub fn type0_dbi(&self) -> DbIndex {
            self.level_to_index(TYPE0_LEVEL)
        }

        /// Returns the De Bruijn index of the `Type1` expression.
        pub fn type1_dbi(&self) -> DbIndex {
            self.level_to_index(TYPE1_LEVEL)
        }
    }

    impl Context {
        fn level_to_index(&self, level: DbLevel) -> DbIndex {
            DbIndex(self.len() - level.0 - 1)
        }

        fn index_to_level(&self, index: DbIndex) -> DbLevel {
            DbLevel(self.len() - index.0 - 1)
        }
    }

    impl Context {
        pub fn get_type(&self, index: DbIndex, registry: &mut NodeRegistry) -> NormalFormId {
            let level = self.index_to_level(index);
            if level == TYPE1_LEVEL {
                panic!("Type1 has no type. We may add support for infinite type hierarchies in the future. However, for now, Type1 is the \"limit\" type.");
            }
            self.local_type_stack[level.0]
                .type_id
                .clone()
                .upshift(index.0 + 1, registry)
        }
    }
}

use misc::*;
mod misc {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct NormalFormId(ExpressionId);

    impl NormalFormId {
        pub fn unchecked_new(expression: ExpressionId) -> Self {
            Self(expression)
        }
    }

    impl NormalFormId {
        pub fn raw(self) -> ExpressionId {
            self.0
        }
    }

    pub fn type0_expression(context: &Context, registry: &mut NodeRegistry) -> NormalFormId {
        let name_id = add_name_expression_and_overwrite_component_ids(
            registry,
            vec![Identifier {
                id: dummy_id(),
                name: IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase),
                start: None,
            }],
            context.type0_dbi(),
        );
        NormalFormId::unchecked_new(ExpressionId::Name(name_id))
    }

    pub fn add_name_expression_and_overwrite_component_ids(
        registry: &mut NodeRegistry,
        components: Vec<Identifier>,
        db_index: DbIndex,
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

    pub fn is_term_equal_to_type0_or_type1(
        context: &Context,
        registry: &NodeRegistry,
        term: NormalFormId,
    ) -> bool {
        if let ExpressionId::Name(name_id) = term.raw() {
            let name = registry.name_expression(name_id);
            let i = name.db_index;
            i == context.type0_dbi() || i == context.type1_dbi()
        } else {
            false
        }
    }

    pub fn is_left_type_assignable_to_right_type(
        _context: &Context,
        _registry: &NodeRegistry,
        _left: NormalFormId,
        _right: NormalFormId,
    ) -> bool {
        unimplemented!()
    }
}

use shift::*;
mod shift {
    use super::*;

    pub trait ShiftDbIndices {
        type Output;

        fn upshift_with_cutoff(
            self,
            amount: usize,
            cutoff: usize,
            registry: &mut NodeRegistry,
        ) -> Self::Output;

        fn upshift(self, amount: usize, registry: &mut NodeRegistry) -> Self::Output
        where
            Self: Sized,
        {
            self.upshift_with_cutoff(amount, 0, registry)
        }
    }

    impl ShiftDbIndices for NormalFormId {
        type Output = Self;

        fn upshift_with_cutoff(
            self,
            amount: usize,
            cutoff: usize,
            registry: &mut NodeRegistry,
        ) -> Self {
            Self::unchecked_new(self.raw().upshift_with_cutoff(amount, cutoff, registry))
        }
    }

    impl ShiftDbIndices for ExpressionId {
        type Output = Self;

        fn upshift_with_cutoff(
            self,
            amount: usize,
            cutoff: usize,
            registry: &mut NodeRegistry,
        ) -> Self {
            match self {
                ExpressionId::Name(name_id) => {
                    ExpressionId::Name(name_id.upshift_with_cutoff(amount, cutoff, registry))
                }
                ExpressionId::Call(call_id) => {
                    ExpressionId::Call(call_id.upshift_with_cutoff(amount, cutoff, registry))
                }
                ExpressionId::Fun(fun_id) => {
                    ExpressionId::Fun(fun_id.upshift_with_cutoff(amount, cutoff, registry))
                }
                ExpressionId::Match(match_id) => {
                    ExpressionId::Match(match_id.upshift_with_cutoff(amount, cutoff, registry))
                }
                ExpressionId::Forall(forall_id) => {
                    ExpressionId::Forall(forall_id.upshift_with_cutoff(amount, cutoff, registry))
                }
            }
        }
    }

    impl ShiftDbIndices for NodeId<NameExpression> {
        type Output = Self;

        fn upshift_with_cutoff(
            self,
            _amount: usize,
            _cutoff: usize,
            _registry: &mut NodeRegistry,
        ) -> Self {
            unimplemented!()
        }
    }

    impl ShiftDbIndices for NodeId<Call> {
        type Output = Self;

        fn upshift_with_cutoff(
            self,
            _amount: usize,
            _cutoff: usize,
            _registry: &mut NodeRegistry,
        ) -> Self {
            unimplemented!()
        }
    }

    impl ShiftDbIndices for NodeId<Fun> {
        type Output = Self;

        fn upshift_with_cutoff(
            self,
            _amount: usize,
            _cutoff: usize,
            _registry: &mut NodeRegistry,
        ) -> Self {
            unimplemented!()
        }
    }

    impl ShiftDbIndices for NodeId<Match> {
        type Output = Self;

        fn upshift_with_cutoff(
            self,
            _amount: usize,
            _cutoff: usize,
            _registry: &mut NodeRegistry,
        ) -> Self {
            unimplemented!()
        }
    }

    impl ShiftDbIndices for NodeId<Forall> {
        type Output = Self;

        fn upshift_with_cutoff(
            self,
            _amount: usize,
            _cutoff: usize,
            _registry: &mut NodeRegistry,
        ) -> Self {
            unimplemented!()
        }
    }
}
