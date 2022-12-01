use crate::data::{
    illegal_fun_recursion_error::*,
    light_ast::*,
    node_registry::{ListId, NodeId, NodeRegistry},
};

// TODO: Properly cleanup

// TODO: Test cleanup

pub fn validate_fun_recursion_in_file(
    registry: &mut NodeRegistry,
    file_id: NodeId<File>,
) -> Result<NodeId<File>, IllegalFunRecursionError> {
    let file = registry.file(file_id).clone();
    let mut context = Context::new();
    let item_ids = registry
        .file_item_list(file.item_list_id)
        .to_vec()
        .into_iter()
        .map(|item_id| validate_fun_recursion_in_file_item(&mut context, registry, item_id))
        .collect::<Result<Vec<_>, _>>()?;
    let item_list_id = registry.add_file_item_list(item_ids);
    Ok(registry.add_file_and_overwrite_its_id(File {
        id: dummy_id(),
        file_id: file.file_id,
        item_list_id,
    }))
}

fn validate_fun_recursion_in_file_item(
    context: &mut Context,
    registry: &mut NodeRegistry,
    item_id: FileItemNodeId,
) -> Result<FileItemNodeId, IllegalFunRecursionError> {
    Ok(match item_id {
        FileItemNodeId::Type(id) => FileItemNodeId::Type(validate_fun_recursion_in_type_statement(
            context, registry, id,
        )?),
        FileItemNodeId::Let(id) => FileItemNodeId::Let(validate_fun_recursion_in_let_statement(
            context, registry, id,
        )?),
    })
}

fn validate_fun_recursion_in_type_statement(
    context: &mut Context,
    registry: &mut NodeRegistry,
    type_statement_id: NodeId<TypeStatement>,
) -> Result<NodeId<TypeStatement>, IllegalFunRecursionError> {
    let type_statement = registry.type_statement(type_statement_id).clone();
    let param_list_id = validate_fun_recursion_in_params_and_leave_in_context(
        context,
        registry,
        type_statement.param_list_id,
    )?;
    context.pop_n(type_statement.param_list_id.len);

    context.push(ContextEntry::NoInformation);

    let variant_ids = registry
        .variant_list(type_statement.variant_list_id)
        .to_vec()
        .into_iter()
        .map(|variant_id| validate_fun_recursion_in_variant(context, registry, variant_id))
        .collect::<Result<Vec<_>, _>>()?;
    let variant_list_id = registry.add_variant_list(variant_ids);

    Ok(
        registry.add_type_statement_and_overwrite_its_id(TypeStatement {
            id: dummy_id(),
            name_id: type_statement.name_id,
            param_list_id,
            variant_list_id,
        }),
    )
}

fn validate_fun_recursion_in_variant(
    context: &mut Context,
    registry: &mut NodeRegistry,
    variant_id: NodeId<Variant>,
) -> Result<NodeId<Variant>, IllegalFunRecursionError> {
    let variant = registry.variant(variant_id).clone();
    let arity = variant.param_list_id.len;
    let param_list_id = validate_fun_recursion_in_params_and_leave_in_context(
        context,
        registry,
        variant.param_list_id,
    )?;
    let return_type_id =
        validate_fun_recursion_in_expression(context, registry, variant.return_type_id)?;
    context.pop_n(arity);

    context.push(ContextEntry::NoInformation);

    Ok(registry.add_variant_and_overwrite_its_id(Variant {
        id: dummy_id(),
        name_id: variant.name_id,
        param_list_id,
        return_type_id,
    }))
}

fn validate_fun_recursion_in_let_statement(
    context: &mut Context,
    registry: &mut NodeRegistry,
    let_statement_id: NodeId<LetStatement>,
) -> Result<NodeId<LetStatement>, IllegalFunRecursionError> {
    let let_statement = registry.let_statement(let_statement_id).clone();
    let value_id = validate_fun_recursion_in_expression(context, registry, let_statement.value_id)?;
    context.push(ContextEntry::NoInformation);
    Ok(
        registry.add_let_statement_and_overwrite_its_id(LetStatement {
            id: dummy_id(),
            name_id: let_statement.name_id,
            value_id,
        }),
    )
}

fn validate_fun_recursion_in_expression(
    context: &mut Context,
    registry: &mut NodeRegistry,
    expression_id: ExpressionId,
) -> Result<ExpressionId, IllegalFunRecursionError> {
    Ok(match expression_id {
        ExpressionId::Name(id) => {
            validate_fun_recursion_in_name(context, registry, id).map(ExpressionId::Name)?
        }
        ExpressionId::Call(id) => {
            validate_fun_recursion_in_call(context, registry, id).map(ExpressionId::Call)?
        }
        ExpressionId::Fun(id) => {
            validate_fun_recursion_in_fun(context, registry, id).map(ExpressionId::Fun)?
        }
        ExpressionId::Match(id) => {
            validate_fun_recursion_in_match(context, registry, id).map(ExpressionId::Match)?
        }
        ExpressionId::Forall(id) => {
            validate_fun_recursion_in_forall(context, registry, id).map(ExpressionId::Forall)?
        }
        ExpressionId::Check(id) => {
            validate_fun_recursion_in_check(context, registry, id).map(ExpressionId::Check)?
        }
    })
}

fn validate_fun_recursion_in_name(
    context: &mut Context,
    registry: &mut NodeRegistry,
    name_id: NodeId<NameExpression>,
) -> Result<NodeId<NameExpression>, IllegalFunRecursionError> {
    let name = registry.name_expression(name_id);
    if context.reference_restriction(name.db_index).is_some() {
        return Err(
            IllegalFunRecursionError::RecursiveReferenceWasNotDirectCall { reference: name_id },
        );
    }
    Ok(name_id)
}

fn validate_fun_recursion_in_call(
    context: &mut Context,
    registry: &mut NodeRegistry,
    call_id: NodeId<Call>,
) -> Result<NodeId<Call>, IllegalFunRecursionError> {
    let call = registry.call(call_id).clone();

    let is_restricted = is_call_restricted(context, registry, call_id)?;

    // If the call is restricted (i.e., in the form
    // `f(x, y, ...z)`, where `f` is a Name referring to a restricted recursive function,
    // then we need to skip the callee validation (otherwise `f` will trigger
    // an error, since it is a recursive reference).
    let callee_id = if is_restricted {
        // We don't need to do any additional processing
        // (e.g., validate Check expressions)
        // since the callee is a Name expression.
        call.callee_id
    } else {
        validate_fun_recursion_in_expression(context, registry, call.callee_id)?
    };

    let arg_ids = registry
        .expression_list(call.arg_list_id)
        .to_vec()
        .into_iter()
        .map(|arg_id| validate_fun_recursion_in_expression(context, registry, arg_id))
        .collect::<Result<Vec<_>, _>>()?;
    let arg_list_id = registry.add_expression_list(arg_ids);

    Ok(registry.add_call_and_overwrite_its_id(Call {
        id: dummy_id(),
        callee_id,
        arg_list_id,
    }))
}

fn is_call_restricted(
    context: &Context,
    registry: &NodeRegistry,
    call_id: NodeId<Call>,
) -> Result<bool, IllegalFunRecursionError> {
    let call = registry.call(call_id).clone();
    match call.callee_id {
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
                Ok(true)
            } else {
                Ok(false)
            }
        }
        _ => Ok(false),
    }
}

fn validate_fun_recursion_in_fun(
    context: &mut Context,
    registry: &mut NodeRegistry,
    fun_id: NodeId<Fun>,
) -> Result<NodeId<Fun>, IllegalFunRecursionError> {
    let fun = registry.fun(fun_id).clone();
    let param_list_id = validate_fun_recursion_in_params_and_leave_in_context(
        context,
        registry,
        fun.param_list_id,
    )?;
    let return_type_id =
        validate_fun_recursion_in_expression(context, registry, fun.return_type_id)?;

    let reference_restriction = {
        let param_ids = registry.param_list(fun.param_list_id);
        let decreasing_param_position = param_ids.iter().position(|param_id| {
            let param = registry.param(*param_id);
            param.is_dashed
        });
        match decreasing_param_position {
            Some(param_position) => {
                let superstruct_db_index = DbIndex(param_ids.len() - param_position - 1);
                let superstruct_db_level = context.index_to_level(superstruct_db_index);
                ReferenceRestriction::MustCallWithSubstruct {
                    superstruct_db_level,
                    arg_position: param_position,
                }
            }
            None => ReferenceRestriction::CannotCall,
        }
    };

    context.push(ContextEntry::Fun(reference_restriction));
    let body_id = validate_fun_recursion_in_expression(context, registry, fun.body_id)?;
    context.pop_n(param_list_id.len + 1);

    Ok(registry.add_fun_and_overwrite_its_id(Fun {
        id: dummy_id(),
        name_id: fun.name_id,
        param_list_id,
        return_type_id,
        body_id,
        skip_type_checking_body: fun.skip_type_checking_body,
    }))
}

fn validate_fun_recursion_in_params_and_leave_in_context(
    context: &mut Context,
    registry: &mut NodeRegistry,
    param_list_id: ListId<NodeId<Param>>,
) -> Result<ListId<NodeId<Param>>, IllegalFunRecursionError> {
    let param_ids = registry
        .param_list(param_list_id)
        .to_vec()
        .into_iter()
        .map(|param_id| {
            let param = registry.param(param_id).clone();
            let type_id = validate_fun_recursion_in_expression(context, registry, param.type_id)?;
            context.push(ContextEntry::NoInformation);
            Ok(registry.add_param_and_overwrite_its_id(Param {
                id: dummy_id(),
                name_id: param.name_id,
                type_id,
                is_dashed: param.is_dashed,
            }))
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(registry.add_param_list(param_ids))
}

fn validate_fun_recursion_in_match(
    context: &mut Context,
    registry: &mut NodeRegistry,
    match_id: NodeId<Match>,
) -> Result<NodeId<Match>, IllegalFunRecursionError> {
    let match_ = registry.match_(match_id).clone();
    let matchee_id = validate_fun_recursion_in_expression(context, registry, match_.matchee_id)?;
    let matchee_db_index = match match_.matchee_id {
        ExpressionId::Name(matchee_name_id) => {
            let matchee_name = registry.name_expression(matchee_name_id);
            Some(matchee_name.db_index)
        }
        _ => None,
    };

    let case_ids = registry
        .match_case_list(match_.case_list_id)
        .to_vec()
        .into_iter()
        .map(|case_id| {
            validate_fun_recursion_in_match_case(context, registry, case_id, matchee_db_index)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let case_list_id = registry.add_match_case_list(case_ids);

    Ok(registry.add_match_and_overwrite_its_id(Match {
        id: dummy_id(),
        matchee_id,
        case_list_id,
    }))
}

fn validate_fun_recursion_in_match_case(
    context: &mut Context,
    registry: &mut NodeRegistry,
    case_id: NodeId<MatchCase>,
    matchee_db_index: Option<DbIndex>,
) -> Result<NodeId<MatchCase>, IllegalFunRecursionError> {
    let case = registry.match_case(case_id).clone();
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

    // We don't need to do any validation since it's just a
    // list of identifiers.
    let param_list_id = case.param_list_id;

    let output_id = validate_fun_recursion_in_expression(context, registry, case.output_id)?;
    context.pop_n(case_arity);

    Ok(registry.add_match_case_and_overwrite_its_id(MatchCase {
        id: dummy_id(),
        variant_name_id: case.variant_name_id,
        param_list_id,
        output_id,
    }))
}

fn validate_fun_recursion_in_forall(
    context: &mut Context,
    registry: &mut NodeRegistry,
    forall_id: NodeId<Forall>,
) -> Result<NodeId<Forall>, IllegalFunRecursionError> {
    let forall = registry.forall(forall_id).clone();
    let arity = forall.param_list_id.len;

    let param_list_id = validate_fun_recursion_in_params_and_leave_in_context(
        context,
        registry,
        forall.param_list_id,
    )?;
    let output_id = validate_fun_recursion_in_expression(context, registry, forall.output_id)?;
    context.pop_n(arity);

    Ok(registry.add_forall_and_overwrite_its_id(Forall {
        id: dummy_id(),
        param_list_id,
        output_id,
    }))
}

fn validate_fun_recursion_in_check(
    context: &mut Context,
    registry: &mut NodeRegistry,
    check_id: NodeId<Check>,
) -> Result<NodeId<Check>, IllegalFunRecursionError> {
    let check = registry.check(check_id).clone();
    let checkee_annotation_id = validate_fun_recursion_in_checkee_annotation(
        context,
        registry,
        check.checkee_annotation_id,
    )?;
    let output_id = validate_fun_recursion_in_expression(context, registry, check.output_id)?;
    Ok(registry.add_check_and_overwrite_its_id(Check {
        id: dummy_id(),
        checkee_annotation_id,
        output_id,
    }))
}

fn validate_fun_recursion_in_checkee_annotation(
    context: &mut Context,
    registry: &mut NodeRegistry,
    id: CheckeeAnnotationId,
) -> Result<CheckeeAnnotationId, IllegalFunRecursionError> {
    Ok(match id {
        CheckeeAnnotationId::Goal(id) => CheckeeAnnotationId::Goal(
            validate_fun_recursion_in_goal_checkee_annotation(context, registry, id)?,
        ),
        CheckeeAnnotationId::Expression(id) => CheckeeAnnotationId::Expression(
            validate_fun_recursion_in_expression_checkee_annotation(context, registry, id)?,
        ),
    })
}

fn validate_fun_recursion_in_goal_checkee_annotation(
    context: &mut Context,
    registry: &mut NodeRegistry,
    id: NodeId<GoalCheckeeAnnotation>,
) -> Result<NodeId<GoalCheckeeAnnotation>, IllegalFunRecursionError> {
    let annotation = registry.goal_checkee_annotation(id).clone();
    let checkee_type_id = validate_fun_recursion_in_question_mark_or_possibly_invalid_expression(
        context,
        registry,
        annotation.checkee_type_id,
    )?;
    Ok(
        registry.add_goal_checkee_annotation_and_overwrite_its_id(GoalCheckeeAnnotation {
            id: dummy_id(),
            goal_kw_position: annotation.goal_kw_position,
            checkee_type_id,
        }),
    )
}

fn validate_fun_recursion_in_expression_checkee_annotation(
    context: &mut Context,
    registry: &mut NodeRegistry,
    id: NodeId<ExpressionCheckeeAnnotation>,
) -> Result<NodeId<ExpressionCheckeeAnnotation>, IllegalFunRecursionError> {
    let annotation = registry.expression_checkee_annotation(id).clone();
    let checkee_id =
        validate_fun_recursion_in_expression(context, registry, annotation.checkee_id)?;
    let checkee_type_id = validate_fun_recursion_in_question_mark_or_possibly_invalid_expression(
        context,
        registry,
        annotation.checkee_type_id,
    )?;
    let checkee_value_id = annotation
        .checkee_value_id
        .map(|id| {
            validate_fun_recursion_in_question_mark_or_possibly_invalid_expression(
                context, registry, id,
            )
        })
        .transpose()?;

    Ok(
        registry.add_expression_checkee_annotation_and_overwrite_its_id(
            ExpressionCheckeeAnnotation {
                id: dummy_id(),
                checkee_id,
                checkee_type_id,
                checkee_value_id,
            },
        ),
    )
}

fn validate_fun_recursion_in_question_mark_or_possibly_invalid_expression(
    context: &mut Context,
    registry: &mut NodeRegistry,
    id: QuestionMarkOrPossiblyInvalidExpressionId,
) -> Result<QuestionMarkOrPossiblyInvalidExpressionId, IllegalFunRecursionError> {
    Ok(match id {
        QuestionMarkOrPossiblyInvalidExpressionId::QuestionMark { start } => {
            QuestionMarkOrPossiblyInvalidExpressionId::QuestionMark { start }
        }
        QuestionMarkOrPossiblyInvalidExpressionId::Expression(id) => {
            QuestionMarkOrPossiblyInvalidExpressionId::Expression(
                validate_fun_recursion_in_possibly_invalid_expression(context, registry, id)?,
            )
        }
    })
}

fn validate_fun_recursion_in_possibly_invalid_expression(
    context: &mut Context,
    registry: &mut NodeRegistry,
    original_id: PossiblyInvalidExpressionId,
) -> Result<PossiblyInvalidExpressionId, IllegalFunRecursionError> {
    match original_id {
        PossiblyInvalidExpressionId::Invalid(original_id) => {
            Ok(PossiblyInvalidExpressionId::Invalid(original_id))
        }
        PossiblyInvalidExpressionId::Valid(original_id) => {
            // TODO: Handle cleanup
            let validation_result =
                validate_fun_recursion_in_expression(context, registry, original_id);
            match validation_result {
                Ok(validated_id) => Ok(PossiblyInvalidExpressionId::Valid(validated_id)),
                Err(err) => Ok(PossiblyInvalidExpressionId::Invalid(
                    InvalidExpressionId::IllegalFunRecursion(
                        registry.add_illegal_fun_recursion_expression_and_overwrite_its_id(
                            IllegalFunRecursionExpression {
                                id: dummy_id(),
                                expression_id: original_id,
                                error: err,
                            },
                        ),
                    ),
                )),
            }
        }
    }
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

fn dummy_id<T>() -> NodeId<T> {
    NodeId::new(0)
}
