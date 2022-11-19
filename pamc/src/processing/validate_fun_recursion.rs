use crate::data::{
    x_light_ast::*,
    x_node_registry::{NodeId, NodeRegistry},
};

#[derive(Clone, Debug)]
pub enum IllegalFunRecursionError {
    RecursiveReferenceWasNotDirectCall {
        reference: NodeId<NameExpression>,
    },
    NonSubstructPassedToDecreasingParam {
        callee: NodeId<NameExpression>,
        arg: ExpressionId,
    },
    RecursivelyCalledFunctionWithoutDecreasingParam {
        callee: NodeId<NameExpression>,
    },
}

pub fn validate_fun_recursion_in_file(
    registry: &NodeRegistry,
    file: &File,
) -> Result<(), IllegalFunRecursionError> {
    let mut context = Context::new(registry);
    let item_ids = registry.file_item_list(file.item_list_id);
    for item_id in item_ids {
        match item_id {
            FileItemNodeId::Type(type_id) => {
                let type_statement = registry.type_statement(*type_id);
                validate_fun_recursion_in_type_statement(&mut context, registry, type_statement)?;
            }
            FileItemNodeId::Let(let_id) => {
                let let_statement = registry.let_statement(*let_id);
                validate_fun_recursion_in_let_statement(&mut context, registry, let_statement)?;
            }
        }
    }
    Ok(())
}

fn validate_fun_recursion_in_type_statement(
    context: &mut Context,
    registry: &NodeRegistry,
    type_statement: &TypeStatement,
) -> Result<(), IllegalFunRecursionError> {
    let variant_ids = registry.variant_list(type_statement.variant_list_id);
    for variant_id in variant_ids {
        let variant = registry.variant(*variant_id);
        validate_fun_recursion_in_variant(context, registry, variant)?;
    }
    Ok(())
}

fn validate_fun_recursion_in_variant(
    context: &mut Context,
    registry: &NodeRegistry,
    variant: &Variant,
) -> Result<(), IllegalFunRecursionError> {
    let param_ids = registry.param_list(variant.param_list_id);
    for param_id in param_ids {
        let param = registry.param(*param_id);
        validate_fun_recursion_in_param(context, registry, param)?;
    }
    Ok(())
}

fn validate_fun_recursion_in_param(
    context: &mut Context,
    registry: &NodeRegistry,
    param: &Param,
) -> Result<(), IllegalFunRecursionError> {
    validate_fun_recursion_in_expression(context, registry, param.type_id)?;
    Ok(())
}

fn validate_fun_recursion_in_let_statement(
    context: &mut Context,
    registry: &NodeRegistry,
    let_statement: &LetStatement,
) -> Result<(), IllegalFunRecursionError> {
    validate_fun_recursion_in_expression(context, registry, let_statement.value_id)?;
    Ok(())
}

fn validate_fun_recursion_in_expression(
    context: &mut Context,
    registry: &NodeRegistry,
    expression_id: ExpressionId,
) -> Result<(), IllegalFunRecursionError> {
    match expression_id {
        ExpressionId::Name(id) => validate_fun_recursion_in_name(context, registry, id),
        ExpressionId::Call(id) => validate_fun_recursion_in_call(context, registry, id),
        ExpressionId::Fun(id) => validate_fun_recursion_in_fun(context, registry, id),
        ExpressionId::Match(id) => validate_fun_recursion_in_match(context, registry, id),
        ExpressionId::Forall(id) => validate_fun_recursion_in_forall(context, registry, id),
    }
}

fn validate_fun_recursion_in_name(
    context: &mut Context,
    registry: &NodeRegistry,
    name_id: NodeId<NameExpression>,
) -> Result<(), IllegalFunRecursionError> {
    if context.reference_restriction(name_id).is_some() {
        return Err(
            IllegalFunRecursionError::RecursiveReferenceWasNotDirectCall { reference: name_id },
        );
    }
    Ok(())
}

fn validate_fun_recursion_in_call(
    context: &mut Context,
    registry: &NodeRegistry,
    call_id: NodeId<Call>,
) -> Result<(), IllegalFunRecursionError> {
    let call = registry.call(call_id);
    let is_call_restricted = match call.callee_id {
        ExpressionId::Name(callee_name_id) => {
            let callee_name = registry.name_expression(callee_name_id);
            if let Some(restriction) = context.reference_restriction(callee_name_id) {
                match restriction {
                    ReferenceRestriction::MustCallWithSubstruct {
                        arg_position,
                        superstruct_db_level,
                        ..
                    } => {
                        let arg_ids = registry.expression_list(call.arg_list_id);
                        if arg_position < arg_ids.len() {
                            let expected_substruct_id = arg_ids[arg_position];
                            let err = Err(
                                IllegalFunRecursionError::NonSubstructPassedToDecreasingParam {
                                    callee: callee_name_id,
                                    arg: expected_substruct_id,
                                },
                            );
                            match expected_substruct_id {
                                ExpressionId::Name(expected_substruct_name_id) => {
                                    if !context.is_substruct_of_restricted_superstruct(
                                        expected_substruct_name_id,
                                        superstruct_db_level,
                                    ) {
                                        return err;
                                    }
                                }
                                _ => return err,
                            }
                        }
                    }
                    ReferenceRestriction::CannotCall { .. } => return Err(
                        IllegalFunRecursionError::RecursivelyCalledFunctionWithoutDecreasingParam {
                            callee: callee_name.id,
                        },
                    ),
                }
                true
            } else {
                false
            }
        }
        _ => false,
    };

    // If the call is restricted (i.e., in the form
    // `f(x, y, ...z)`, where `f` is a restricted recursive function,
    // then we need to skip the callee validation (otherwise `f` will trigger
    // an error, since it is a recursive reference).
    if !is_call_restricted {
        validate_fun_recursion_in_expression(context, registry, call.callee_id)?;
    }

    let arg_ids = registry.expression_list(call.arg_list_id);
    for arg_id in arg_ids {
        validate_fun_recursion_in_expression(context, registry, *arg_id)?;
    }
    Ok(())
}

fn validate_fun_recursion_in_fun(
    context: &mut Context,
    registry: &NodeRegistry,
    fun_id: NodeId<Fun>,
) -> Result<(), IllegalFunRecursionError> {
    let fun = registry.fun(fun_id);
    let param_ids = registry.param_list(fun.param_list_id);
    for param_id in param_ids {
        let param = registry.param(*param_id);
        validate_fun_recursion_in_expression(context, registry, param.type_id)?;
    }
    validate_fun_recursion_in_expression(context, registry, fun.return_type_id)?;

    let decreasing_param_position_and_decreasing_param =
        param_ids.iter().enumerate().find(|(_i, param_id)| {
            let param = registry.param(**param_id);
            param.is_dashed
        });
    let reference_restriction = match decreasing_param_position_and_decreasing_param {
        Some((param_position, decreasing_param_id)) => {
            let decreasing_param = registry.param(*decreasing_param_id);
            context.create_must_call_with_substruct_restriction(
                fun.name_id,
                param_position,
                decreasing_param.name_id,
            )
        }
        None => context.create_cannot_call_restriction(fun.name_id),
    };

    context.push_reference_restriction(reference_restriction);
    validate_fun_recursion_in_expression(context, registry, fun.body_id)?;
    context.pop_reference_restriction();

    Ok(())
}

fn validate_fun_recursion_in_match(
    context: &mut Context,
    registry: &NodeRegistry,
    match_id: NodeId<Match>,
) -> Result<(), IllegalFunRecursionError> {
    let match_ = registry.match_(match_id);
    validate_fun_recursion_in_expression(context, registry, match_.matchee_id)?;
    let case_ids = registry.match_case_list(match_.case_list_id);
    match match_.matchee_id {
        ExpressionId::Name(matchee_name_id) => {
            if let Some(mut substructs) = context.matchee_substructs_mut(matchee_name_id) {
                for case_id in case_ids {
                    let case = registry.match_case(*case_id);
                    let param_ids = registry.identifier_list(case.param_list_id);
                    for case_param_id in param_ids {
                        let case_param = registry.identifier(*case_param_id);
                        substructs.push(case_param.id);
                    }
                }
            }
        }
        _ => {}
    }
    for case_id in case_ids {
        let case = registry.match_case(*case_id);
        validate_fun_recursion_in_expression(context, registry, case.output_id)?;
    }
    Ok(())
}

fn validate_fun_recursion_in_forall(
    context: &mut Context,
    registry: &NodeRegistry,
    forall_id: NodeId<Forall>,
) -> Result<(), IllegalFunRecursionError> {
    let forall = registry.forall(forall_id);
    let param_ids = registry.param_list(forall.param_list_id);
    for param_id in param_ids {
        let param = registry.param(*param_id);
        validate_fun_recursion_in_expression(context, registry, param.type_id)?;
    }
    validate_fun_recursion_in_expression(context, registry, forall.output_id)?;

    Ok(())
}

#[derive(Clone, Debug)]
struct Context<'a> {
    registry: &'a NodeRegistry,
    raw: RawContext,
}

impl<'a> Context<'a> {
    fn new(registry: &'a NodeRegistry) -> Self {
        Self {
            registry,
            raw: RawContext::new(),
        }
    }
}

impl Context<'_> {
    fn reference_restriction(
        &self,
        name_id: NodeId<NameExpression>,
    ) -> Option<ReferenceRestriction> {
        let index = self.registry.name_expression(name_id).db_index;
        let level = self.raw.index_to_level(index);
        self.raw.reference_restriction(level)
    }

    fn is_substruct_of_restricted_superstruct(
        &self,
        possible_substruct_id: NodeId<NameExpression>,
        possible_superstruct_level: DbLevel,
    ) -> bool {
        unimplemented!()
    }
}

#[derive(Clone, Debug)]
struct RawContext {
    stack: Vec<ContextEntry>,
}

#[derive(Clone, Copy, Debug)]
enum ContextEntry {
    Substructure { superstruct_db_level: DbLevel },
    Fun(ReferenceRestriction),
    NoInformation,
}

#[derive(Clone, Copy, Debug)]
enum ReferenceRestriction {
    MustCallWithSubstruct {
        superstruct_db_level: DbLevel,
        arg_position: usize,
    },
    CannotCall,
}

impl RawContext {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }
}

impl RawContext {
    fn len(&self) -> usize {
        self.stack.len()
    }

    fn level_to_index(&self, level: DbLevel) -> DbIndex {
        DbIndex(self.len() - level.0 - 1)
    }

    fn index_to_level(&self, index: DbIndex) -> DbLevel {
        DbLevel(self.len() - index.0 - 1)
    }
}

impl RawContext {
    /// Panics if `n > self.len()`.
    fn pop_n(&mut self, n: usize) {
        if n > self.len() {
            panic!(
                "Tried to pop {} elements from a context with only {} elements",
                n,
                self.len()
            );
        }
        self.stack.truncate(self.len() - n);
    }

    fn push(&mut self, entry: ContextEntry) {
        self.stack.push(entry);
    }
}

impl RawContext {
    fn reference_restriction(&self, level: DbLevel) -> Option<ReferenceRestriction> {
        let entry = self.stack[level.0];
        match entry {
            ContextEntry::Substructure { .. } => None,
            ContextEntry::Fun(restriction) => Some(restriction),
            ContextEntry::NoInformation => None,
        }
    }
}
