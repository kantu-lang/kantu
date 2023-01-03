use super::*;

pub(super) fn verify_expression_is_visible_from(
    state: &State,
    expression_id: ExpressionId,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    let state = OffsetState {
        state,
        extra_entries_in_context: 0,
    };
    verify_expression(state, expression_id, perspective)
}

#[derive(Clone, Copy, Debug)]
struct OffsetState<'a, 'b> {
    state: &'a State<'b>,
    extra_entries_in_context: usize,
}

impl OffsetState<'_, '_> {
    fn file_tree(&self) -> &FileTree {
        self.state.file_tree
    }

    fn registry(&self) -> &NodeRegistry {
        self.state.registry
    }

    /// Returns Global if the provided index refers to
    /// one of the extra entries added to the context.
    fn get_visibility(&self, db_index: DbIndex) -> Visibility {
        let adjusted_db_index = db_index
            .0
            .checked_sub(self.extra_entries_in_context)
            .map(DbIndex);
        if let Some(adjusted_db_index) = adjusted_db_index {
            self.state.context.get_visibility(adjusted_db_index)
        } else {
            Visibility(ModScope::Global)
        }
    }

    fn extend(&self, n: usize) -> OffsetState {
        OffsetState {
            state: self.state,
            extra_entries_in_context: self.extra_entries_in_context + n,
        }
    }
}

fn verify_expression(
    state: OffsetState,
    expression_id: ExpressionId,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    match expression_id {
        ExpressionId::Name(id) => verify_name_expression(state, id, perspective),
        ExpressionId::Todo(_) => Ok(()),
        ExpressionId::Call(id) => verify_call(state, id, perspective),
        ExpressionId::Fun(id) => verify_fun(state, id, perspective),
        ExpressionId::Match(id) => verify_match(state, id, perspective),
        ExpressionId::Forall(id) => verify_forall(state, id, perspective),
        ExpressionId::Check(id) => verify_check_expression(state, id, perspective),
    }
}

fn verify_name_expression(
    state: OffsetState,
    id: NodeId<NameExpression>,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    let name = state.registry().get(id);
    let visibility = state.get_visibility(name.db_index);
    if !is_left_at_least_as_permissive_as_right(state.file_tree(), visibility.0, perspective.0) {
        return Err(id);
    }
    Ok(())
}

fn verify_call(
    state: OffsetState,
    id: NodeId<Call>,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    let call = state.registry().get(id);
    verify_expression(state, call.callee_id, perspective)?;
    verify_arg_list(state, call.arg_list_id, perspective)?;
    Ok(())
}

fn verify_arg_list(
    state: OffsetState,
    id: NonEmptyCallArgListId,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    match id {
        NonEmptyCallArgListId::Unlabeled(id) => verify_expression_list(state, id, perspective),
        NonEmptyCallArgListId::UniquelyLabeled(id) => {
            verify_labeled_call_arg_list(state, id, perspective)
        }
    }
}

fn verify_expression_list(
    state: OffsetState,
    list_id: NonEmptyListId<ExpressionId>,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    let list = state.registry().get_list(list_id);
    for &id in list.iter() {
        verify_expression(state, id, perspective)?;
    }
    Ok(())
}

fn verify_labeled_call_arg_list(
    state: OffsetState,
    list_id: NonEmptyListId<LabeledCallArgId>,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    let list = state.registry().get_list(list_id);
    for &id in list.iter() {
        verify_labeled_call_arg(state, id, perspective)?;
    }
    Ok(())
}

fn verify_labeled_call_arg(
    state: OffsetState,
    id: LabeledCallArgId,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    match id {
        LabeledCallArgId::Explicit {
            label_id: _,
            value_id,
        } => verify_expression(state, value_id, perspective),
        LabeledCallArgId::Implicit {
            label_id: _,
            db_index: _,
            value_id,
        } => verify_name_expression(state, value_id, perspective),
    }
}

fn verify_fun(
    state: OffsetState,
    id: NodeId<Fun>,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    let fun = state.registry().get(id);
    verify_param_list(state, fun.param_list_id, perspective)?;
    verify_expression(
        state.extend(fun.param_list_id.len()),
        fun.return_type_id,
        perspective,
    )?;
    verify_expression(
        state.extend(fun.param_list_id.len() + 1),
        fun.body_id,
        perspective,
    )?;
    Ok(())
}

fn verify_param_list(
    state: OffsetState,
    id: NonEmptyParamListId,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    match id {
        NonEmptyParamListId::Unlabeled(id) => verify_unlabeled_param_list(state, id, perspective),
        NonEmptyParamListId::UniquelyLabeled(id) => {
            verify_labeled_param_list(state, id, perspective)
        }
    }
}

fn verify_unlabeled_param_list(
    state: OffsetState,
    list_id: NonEmptyListId<NodeId<UnlabeledParam>>,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    let list = state.registry().get_list(list_id);
    for (i, param_id) in list.iter().copied().enumerate() {
        let param_state = state.extend(i);
        let param = param_state.registry().get(param_id);
        verify_expression(param_state, param.type_id, perspective)?;
    }
    Ok(())
}

fn verify_labeled_param_list(
    state: OffsetState,
    list_id: NonEmptyListId<NodeId<LabeledParam>>,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    let list = state.registry().get_list(list_id);
    for (i, param_id) in list.iter().copied().enumerate() {
        let param_state = state.extend(i);
        let param = param_state.registry().get(param_id);
        verify_expression(param_state, param.type_id, perspective)?;
    }
    Ok(())
}

fn verify_match(
    state: OffsetState,
    id: NodeId<Match>,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    let match_ = state.registry().get(id);
    verify_expression(state, match_.matchee_id, perspective)?;
    verify_optional_match_case_list(state, match_.case_list_id, perspective)?;
    Ok(())
}

fn verify_optional_match_case_list(
    state: OffsetState,
    list_id: Option<NonEmptyListId<NodeId<MatchCase>>>,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    let Some(list_id) = list_id else {
        return Ok(());
    };
    verify_match_case_list(state, list_id, perspective)
}

fn verify_match_case_list(
    state: OffsetState,
    list_id: NonEmptyListId<NodeId<MatchCase>>,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    let list = state.registry().get_list(list_id);
    for &id in list.iter() {
        verify_match_case(state, id, perspective)?;
    }
    Ok(())
}

fn verify_match_case(
    state: OffsetState,
    id: NodeId<MatchCase>,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    let case = state.registry().get(id);
    verify_match_case_output(
        state.extend(
            case.param_list_id
                .map(|list_id| list_id.explicit_len())
                .unwrap_or(0),
        ),
        case.output_id,
        perspective,
    )?;
    Ok(())
}

fn verify_match_case_output(
    state: OffsetState,
    id: MatchCaseOutputId,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    match id {
        MatchCaseOutputId::Some(id) => verify_expression(state, id, perspective),
        MatchCaseOutputId::ImpossibilityClaim(_) => Ok(()),
    }
}

fn verify_forall(
    state: OffsetState,
    id: NodeId<Forall>,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    let forall = state.registry().get(id);
    verify_param_list(state, forall.param_list_id, perspective)?;
    verify_expression(
        state.extend(forall.param_list_id.len()),
        forall.output_id,
        perspective,
    )?;
    Ok(())
}

fn verify_check_expression(
    state: OffsetState,
    id: NodeId<Check>,
    perspective: Visibility,
) -> Result<(), NodeId<NameExpression>> {
    let check = state.registry().get(id);
    verify_expression(state, check.output_id, perspective)?;
    Ok(())
}
