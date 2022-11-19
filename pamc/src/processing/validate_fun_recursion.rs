use crate::data::{
    x_light_ast::*,
    x_node_registry::{ListId, NodeId, NodeRegistry},
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
    let mut context = Context::new();
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
    validate_fun_recursion_in_params_and_leave_in_context(
        context,
        registry,
        type_statement.param_list_id,
    )?;
    context.pop_n(type_statement.param_list_id.len);

    context.push(ContextEntry::NoInformation);

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
    let arity = variant.param_list_id.len;
    validate_fun_recursion_in_params_and_leave_in_context(
        context,
        registry,
        variant.param_list_id,
    )?;
    validate_fun_recursion_in_expression(context, registry, variant.return_type_id)?;
    context.pop_n(arity);

    context.push(ContextEntry::NoInformation);

    Ok(())
}

fn validate_fun_recursion_in_let_statement(
    context: &mut Context,
    registry: &NodeRegistry,
    let_statement: &LetStatement,
) -> Result<(), IllegalFunRecursionError> {
    validate_fun_recursion_in_expression(context, registry, let_statement.value_id)?;
    context.push(ContextEntry::NoInformation);
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
    let name = registry.name_expression(name_id);
    if context.reference_restriction(name.db_index).is_some() {
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
            if let Some(restriction) = context.reference_restriction(callee_name.db_index) {
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
                                    let expected_substruct =
                                        registry.name_expression(expected_substruct_name_id);
                                    let expected_substruct_db_level =
                                        context.index_to_level(expected_substruct.db_index);
                                    if !context.is_left_strict_substruct_of_right(
                                        expected_substruct_db_level,
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
    validate_fun_recursion_in_params_and_leave_in_context(context, registry, fun.param_list_id)?;
    validate_fun_recursion_in_expression(context, registry, fun.return_type_id)?;

    let param_ids = registry.param_list(fun.param_list_id);
    let decreasing_param_position = param_ids.iter().position(|param_id| {
        let param = registry.param(*param_id);
        param.is_dashed
    });
    let reference_restriction = match decreasing_param_position {
        Some(param_position) => {
            let superstruct_db_index = DbIndex(param_ids.len() - param_position - 1);
            let superstruct_db_level = context.index_to_level(superstruct_db_index);
            ReferenceRestriction::MustCallWithSubstruct {
                superstruct_db_level,
                arg_position: param_position,
            }
        }
        None => ReferenceRestriction::CannotCall,
    };

    context.push(ContextEntry::Fun(reference_restriction));
    validate_fun_recursion_in_expression(context, registry, fun.body_id)?;
    context.pop_n(param_ids.len() + 1);

    Ok(())
}

fn validate_fun_recursion_in_params_and_leave_in_context(
    context: &mut Context,
    registry: &NodeRegistry,
    param_list_id: ListId<NodeId<Param>>,
) -> Result<(), IllegalFunRecursionError> {
    let param_ids = registry.param_list(param_list_id);
    for param_id in param_ids {
        let param = registry.param(*param_id);
        validate_fun_recursion_in_expression(context, registry, param.type_id)?;
        context.push(ContextEntry::NoInformation);
    }
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
    let matchee_db_index = match match_.matchee_id {
        ExpressionId::Name(matchee_name_id) => {
            let matchee_name = registry.name_expression(matchee_name_id);
            Some(matchee_name.db_index)
        }
        _ => None,
    };
    for case_id in case_ids {
        validate_fun_recursion_in_match_case(context, registry, *case_id, matchee_db_index)?;
    }
    Ok(())
}

fn validate_fun_recursion_in_match_case(
    context: &mut Context,
    registry: &NodeRegistry,
    case_id: NodeId<MatchCase>,
    matchee_db_index: Option<DbIndex>,
) -> Result<(), IllegalFunRecursionError> {
    let case = registry.match_case(case_id);
    let case_arity = case.param_list_id.len;

    if let Some(matchee_db_index) = matchee_db_index {
        let matchee_db_level = context.index_to_level(matchee_db_index);
        for _ in 0..case_arity {
            context.push(ContextEntry::Substruct {
                superstruct_db_level: matchee_db_level,
            });
        }
    } else {
        for _ in 0..case_arity {
            context.push(ContextEntry::NoInformation);
        }
    }

    validate_fun_recursion_in_expression(context, registry, case.output_id)?;
    context.pop_n(case_arity);
    Ok(())
}

fn validate_fun_recursion_in_forall(
    context: &mut Context,
    registry: &NodeRegistry,
    forall_id: NodeId<Forall>,
) -> Result<(), IllegalFunRecursionError> {
    let forall = registry.forall(forall_id);
    let arity = forall.param_list_id.len;

    validate_fun_recursion_in_params_and_leave_in_context(context, registry, forall.param_list_id)?;
    validate_fun_recursion_in_expression(context, registry, forall.output_id)?;
    context.pop_n(arity);

    Ok(())
}

#[derive(Clone, Debug)]
struct Context {
    stack: Vec<ContextEntry>,
}

#[derive(Clone, Copy, Debug)]
enum ContextEntry {
    Substruct { superstruct_db_level: DbLevel },
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

impl Context {
    fn new() -> Self {
        Self {
            stack: vec![ContextEntry::NoInformation, ContextEntry::NoInformation],
        }
    }
}

impl Context {
    fn len(&self) -> usize {
        self.stack.len()
    }

    fn index_to_level(&self, index: DbIndex) -> DbLevel {
        DbLevel(self.len() - index.0 - 1)
    }
}

impl Context {
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

impl Context {
    fn reference_restriction(&self, index: DbIndex) -> Option<ReferenceRestriction> {
        let level = self.index_to_level(index);
        let entry = self.stack[level.0];
        match entry {
            ContextEntry::Substruct { .. } => None,
            ContextEntry::Fun(restriction) => Some(restriction),
            ContextEntry::NoInformation => None,
        }
    }

    fn is_left_strict_substruct_of_right(&self, left: DbLevel, right: DbLevel) -> bool {
        left != right && self.is_left_inclusive_substruct_of_right(left, right)
    }

    fn is_left_inclusive_substruct_of_right(&self, left: DbLevel, right: DbLevel) -> bool {
        let mut current = left;
        loop {
            if current == right {
                return true;
            }
            let entry = self.stack[current.0];
            match entry {
                ContextEntry::Substruct {
                    superstruct_db_level,
                } => {
                    current = superstruct_db_level;
                    continue;
                }
                ContextEntry::Fun(_) | ContextEntry::NoInformation => {
                    return false;
                }
            }
        }
    }
}
