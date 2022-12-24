use super::*;

pub fn get_possibly_empty_param_type_ids(
    registry: &NodeRegistry,
    id: Option<NonEmptyParamListId>,
) -> Vec<ExpressionId> {
    id.map(|id| get_param_type_ids(registry, id).into())
        .unwrap_or_else(|| vec![])
}

pub fn get_param_type_ids(
    registry: &NodeRegistry,
    id: NonEmptyParamListId,
) -> NonEmptyVec<ExpressionId> {
    match id {
        NonEmptyParamListId::Unlabeled(id) => get_unlabeled_param_ids(registry, id),
        NonEmptyParamListId::UniquelyLabeled(id) => get_labeled_param_ids(registry, id),
    }
}

pub fn get_unlabeled_param_ids(
    registry: &NodeRegistry,
    id: NonEmptyListId<NodeId<UnlabeledParam>>,
) -> NonEmptyVec<ExpressionId> {
    registry.get_list(id).to_mapped(|&param_id| {
        let param = registry.get(param_id);
        param.type_id
    })
}

pub fn get_labeled_param_ids(
    registry: &NodeRegistry,
    id: NonEmptyListId<NodeId<LabeledParam>>,
) -> NonEmptyVec<ExpressionId> {
    registry.get_list(id).to_mapped(|&param_id| {
        let param = registry.get(param_id);
        param.type_id
    })
}

pub fn verify_that_target_does_not_appear_in_any_param_type(
    registry: &NodeRegistry,
    list_id: NonEmptyParamListId,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    match list_id {
        NonEmptyParamListId::Unlabeled(id) => {
            verify_that_target_does_not_appear_in_any_unlabeled_param_type(registry, id, target)
        }
        NonEmptyParamListId::UniquelyLabeled(id) => {
            verify_that_target_does_not_appear_in_any_labeled_param_type(registry, id, target)
        }
    }
}

pub fn verify_that_target_does_not_appear_in_any_unlabeled_param_type(
    registry: &NodeRegistry,
    list_id: NonEmptyListId<NodeId<UnlabeledParam>>,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    let param_ids = registry.get_list(list_id);
    for (param_index, param_id) in param_ids.iter().copied().enumerate() {
        let param = registry.get(param_id);
        let shifted_target = DbIndex(target.0 + param_index);
        verify_that_target_does_not_appear_in_expression(registry, param.type_id, shifted_target)?;
    }
    Ok(())
}

pub fn verify_that_target_does_not_appear_in_any_labeled_param_type(
    registry: &NodeRegistry,
    list_id: NonEmptyListId<NodeId<LabeledParam>>,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    let param_ids = registry.get_list(list_id);
    for (param_index, param_id) in param_ids.iter().copied().enumerate() {
        let param = registry.get(param_id);
        let shifted_target = DbIndex(target.0 + param_index);
        verify_that_target_does_not_appear_in_expression(registry, param.type_id, shifted_target)?;
    }
    Ok(())
}

pub fn verify_that_target_does_not_appear_in_expression(
    registry: &NodeRegistry,
    id: ExpressionId,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    match id {
        ExpressionId::Name(id) => {
            verify_that_target_does_not_appear_in_name_expression(registry, id, target)
        }
        ExpressionId::Todo(_) => Ok(()),
        ExpressionId::Call(id) => verify_that_target_does_not_appear_in_call(registry, id, target),
        ExpressionId::Fun(id) => verify_that_target_does_not_appear_in_fun(registry, id, target),
        ExpressionId::Match(id) => {
            verify_that_target_does_not_appear_in_match(registry, id, target)
        }
        ExpressionId::Forall(id) => {
            verify_that_target_does_not_appear_in_forall(registry, id, target)
        }
        ExpressionId::Check(id) => {
            verify_that_target_does_not_appear_in_check_expression(registry, id, target)
        }
    }
}

pub fn verify_that_target_does_not_appear_in_name_expression(
    registry: &NodeRegistry,
    id: NodeId<NameExpression>,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    let name = registry.get(id);
    if name.db_index == target {
        Err(TypePositivityError::IllegalVariableAppearance(id))
    } else {
        Ok(())
    }
}

pub fn verify_that_target_does_not_appear_in_call(
    registry: &NodeRegistry,
    id: NodeId<Call>,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    let call = registry.get(id);
    verify_that_target_does_not_appear_in_expression(registry, call.callee_id, target)?;
    verify_that_target_does_not_appear_in_any_call_arg(registry, call.arg_list_id, target)?;
    Ok(())
}

pub fn verify_that_target_does_not_appear_in_any_call_arg(
    registry: &NodeRegistry,
    id: NonEmptyCallArgListId,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    match id {
        NonEmptyCallArgListId::Unlabeled(id) => {
            verify_that_target_does_not_appear_in_any_unlabeled_call_arg(registry, id, target)
        }
        NonEmptyCallArgListId::UniquelyLabeled(id) => {
            verify_that_target_does_not_appear_in_any_labeled_call_arg(registry, id, target)
        }
    }
}

pub fn verify_that_target_does_not_appear_in_any_unlabeled_call_arg(
    registry: &NodeRegistry,
    list_id: NonEmptyListId<ExpressionId>,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    let arg_ids = registry.get_list(list_id);
    for arg_id in arg_ids.iter().copied() {
        verify_that_target_does_not_appear_in_expression(registry, arg_id, target)?;
    }
    Ok(())
}

pub fn verify_that_target_does_not_appear_in_any_labeled_call_arg(
    registry: &NodeRegistry,
    list_id: NonEmptyListId<LabeledCallArgId>,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    let arg_ids = registry.get_list(list_id);
    for arg_id in arg_ids.iter().copied() {
        verify_that_target_does_not_appear_in_expression(registry, arg_id.value_id(), target)?;
    }
    Ok(())
}

pub fn verify_that_target_does_not_appear_in_fun(
    registry: &NodeRegistry,
    id: NodeId<Fun>,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    let fun = registry.get(id);

    verify_that_target_does_not_appear_in_any_param_type(registry, fun.param_list_id, target)?;

    let return_type_target = DbIndex(target.0 + fun.param_list_id.len());
    verify_that_target_does_not_appear_in_expression(
        registry,
        fun.return_type_id,
        return_type_target,
    )?;

    let body_target = DbIndex(target.0 + fun.param_list_id.len() + 1);
    verify_that_target_does_not_appear_in_expression(registry, fun.body_id, body_target)?;

    Ok(())
}

pub fn verify_that_target_does_not_appear_in_match(
    registry: &NodeRegistry,
    id: NodeId<Match>,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    let match_ = registry.get(id);
    verify_that_target_does_not_appear_in_expression(registry, match_.matchee_id, target)?;
    verify_that_target_does_not_appear_in_any_match_case_output(
        registry,
        match_.case_list_id,
        target,
    )?;
    Ok(())
}

pub fn verify_that_target_does_not_appear_in_any_match_case_output(
    registry: &NodeRegistry,
    id: Option<NonEmptyListId<NodeId<MatchCase>>>,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    let case_ids = registry.get_possibly_empty_list(id);
    for &case_ids in case_ids {
        verify_that_target_does_not_appear_in_match_case_output(registry, case_ids, target)?;
    }
    Ok(())
}

pub fn verify_that_target_does_not_appear_in_match_case_output(
    registry: &NodeRegistry,
    id: NodeId<MatchCase>,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    let case = registry.get(id);
    let explicit_arity = case
        .param_list_id
        .map(|param_list_id| param_list_id.explicit_len())
        .unwrap_or(0);
    let output_target = DbIndex(target.0 + explicit_arity);
    verify_that_target_does_not_appear_in_expression(registry, case.output_id, output_target)
}

pub fn verify_that_target_does_not_appear_in_forall(
    registry: &NodeRegistry,
    id: NodeId<Forall>,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    let forall = registry.get(id);

    verify_that_target_does_not_appear_in_any_param_type(registry, forall.param_list_id, target)?;

    let output_target = DbIndex(target.0 + forall.param_list_id.len());
    verify_that_target_does_not_appear_in_expression(registry, forall.output_id, output_target)?;

    Ok(())
}

pub fn verify_that_target_does_not_appear_in_check_expression(
    registry: &NodeRegistry,
    id: NodeId<Check>,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    let check = registry.get(id);
    verify_that_target_does_not_appear_in_expression(registry, check.output_id, target)
}

pub fn does_target_appear_in_expression(
    registry: &NodeRegistry,
    id: ExpressionId,
    target: DbIndex,
) -> bool {
    verify_that_target_does_not_appear_in_expression(registry, id, target).is_err()
}

pub fn get_arg_values(
    registry: &NodeRegistry,
    arg_list_id: NonEmptyCallArgListId,
) -> NonEmptyVec<ExpressionId> {
    match arg_list_id {
        NonEmptyCallArgListId::Unlabeled(id) => get_unlabeled_arg_values(registry, id),
        NonEmptyCallArgListId::UniquelyLabeled(id) => get_labeled_arg_values(registry, id),
    }
}

pub fn get_unlabeled_arg_values(
    registry: &NodeRegistry,
    id: NonEmptyListId<ExpressionId>,
) -> NonEmptyVec<ExpressionId> {
    registry.get_list(id).to_non_empty_vec()
}

pub fn get_labeled_arg_values(
    registry: &NodeRegistry,
    id: NonEmptyListId<LabeledCallArgId>,
) -> NonEmptyVec<ExpressionId> {
    registry.get_list(id).to_mapped(|&arg_id| arg_id.value_id())
}

pub fn get_ith_call_arg_value(
    registry: &NodeRegistry,
    i: usize,
    id: NonEmptyCallArgListId,
) -> Option<ExpressionId> {
    match id {
        NonEmptyCallArgListId::Unlabeled(id) => get_ith_unlabeled_call_arg_value(registry, i, id),
        NonEmptyCallArgListId::UniquelyLabeled(id) => {
            get_ith_labeled_call_arg_value(registry, i, id)
        }
    }
}

pub fn get_ith_unlabeled_call_arg_value(
    registry: &NodeRegistry,
    i: usize,
    id: NonEmptyListId<ExpressionId>,
) -> Option<ExpressionId> {
    let value_ids = registry.get_list(id);
    value_ids.get(i).copied()
}

pub fn get_ith_labeled_call_arg_value(
    registry: &NodeRegistry,
    i: usize,
    id: NonEmptyListId<LabeledCallArgId>,
) -> Option<ExpressionId> {
    let value_ids = registry.get_list(id);
    value_ids.get(i).map(|arg_id| arg_id.value_id())
}
