use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Substitution {
    pub from: ExpressionId,
    pub to: ExpressionId,
}

pub(super) trait Substitute: Sized + SubstituteWithoutRemovingSpans
where
    Self::Output: WithoutSpans,
{
    fn subst(self, substitution: Substitution, state: &mut ContextlessState) -> Self::Output {
        self.subst_without_removing_spans(substitution, state)
            .without_spans(state.registry)
    }

    fn subst_all(self, substitutions: &[Substitution], state: &mut ContextlessState) -> Self::Output
    where
        Self: Sized + Substitute<Output = Self>,
    {
        let mut output = self;
        for &subst in substitutions {
            output = output.subst(subst, state);
        }
        output
    }
}

impl<T> Substitute for T
where
    T: SubstituteWithoutRemovingSpans,
    T::Output: WithoutSpans,
{
}

pub(super) trait SubstituteWithoutRemovingSpans {
    type Output;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output;
}

impl SubstituteWithoutRemovingSpans for ExpressionId {
    type Output = Self;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self {
        match self {
            ExpressionId::Name(name_id) => {
                name_id.subst_without_removing_spans(substitution, state)
            }
            ExpressionId::Call(call_id) => {
                call_id.subst_without_removing_spans(substitution, state)
            }
            ExpressionId::Fun(fun_id) => fun_id.subst_without_removing_spans(substitution, state),
            ExpressionId::Match(match_id) => {
                match_id.subst_without_removing_spans(substitution, state)
            }
            ExpressionId::Forall(forall_id) => {
                forall_id.subst_without_removing_spans(substitution, state)
            }
            ExpressionId::Check(check_id) => {
                check_id.subst_without_removing_spans(substitution, state)
            }
        }
    }
}

impl SubstituteWithoutRemovingSpans for NodeId<NameExpression> {
    type Output = ExpressionId;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        subst_if_equal_and_get_status(ExpressionId::Name(self), substitution, state).0
    }
}

/// This does **not** perform substitutions on
/// child nodes.
fn subst_if_equal_and_get_status(
    original: ExpressionId,
    substitution: Substitution,
    state: &mut ContextlessState,
) -> (ExpressionId, WasSyntacticNoOp) {
    let Substitution { from, to } = substitution;
    let is_equal = state.equality_checker.eq(original, from, state.registry);
    if is_equal {
        let to = to;
        let was_no_op = WasSyntacticNoOp(original == to);
        (to, was_no_op)
    } else {
        (original, WasSyntacticNoOp(true))
    }
}

impl SubstituteWithoutRemovingSpans for NodeId<Call> {
    type Output = ExpressionId;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let top_level =
            subst_if_equal_and_get_status(ExpressionId::Call(self), substitution, state);
        if let WasSyntacticNoOp(false) = top_level.1 {
            return top_level.0;
        }

        let call = state.registry.call(self).clone();
        let substituted_callee_id = call
            .callee_id
            .subst_without_removing_spans(substitution, state);
        let substituted_arg_list_id = call
            .arg_list_id
            .subst_without_removing_spans(substitution, state);
        ExpressionId::Call(state.registry.add_call_and_overwrite_its_id(Call {
            id: dummy_id(),
            span: None,
            callee_id: substituted_callee_id,
            arg_list_id: substituted_arg_list_id,
        }))
    }
}

impl SubstituteWithoutRemovingSpans for ListId<ExpressionId> {
    type Output = Self;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let new_ids = state
            .registry
            .expression_list(self)
            .to_vec()
            .into_iter()
            .map(|id| id.subst_without_removing_spans(substitution, state))
            .collect();
        state.registry.add_expression_list(new_ids)
    }
}

impl SubstituteWithoutRemovingSpans for NodeId<Fun> {
    type Output = ExpressionId;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let top_level = subst_if_equal_and_get_status(ExpressionId::Fun(self), substitution, state);
        if let WasSyntacticNoOp(false) = top_level.1 {
            return top_level.0;
        }

        let fun = state.registry.fun(self).clone();
        let substituted_param_list_id = fun
            .param_list_id
            .subst_without_removing_spans(substitution, state);
        let substituted_return_type_id = fun.return_type_id.subst_without_removing_spans(
            substitution.upshift(fun.param_list_id.len, state.registry),
            state,
        );
        let substituted_body_id = fun.body_id.subst_without_removing_spans(
            substitution.upshift(fun.param_list_id.len + 1, state.registry),
            state,
        );
        ExpressionId::Fun(state.registry.add_fun_and_overwrite_its_id(Fun {
            id: dummy_id(),
            span: None,
            name_id: fun.name_id,
            param_list_id: substituted_param_list_id,
            return_type_id: substituted_return_type_id,
            body_id: substituted_body_id,
            skip_type_checking_body: fun.skip_type_checking_body,
        }))
    }
}

impl SubstituteWithoutRemovingSpans for ListId<NodeId<Param>> {
    type Output = Self;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let new_ids = state
            .registry
            .param_list(self)
            .to_vec()
            .into_iter()
            .enumerate()
            .map(|(index, id)| {
                id.subst_without_removing_spans(substitution.upshift(index, state.registry), state)
            })
            .collect();
        state.registry.add_param_list(new_ids)
    }
}

impl SubstituteWithoutRemovingSpans for NodeId<Param> {
    type Output = NodeId<Param>;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let param = state.registry.param(self).clone();
        let substituted_type_id = param
            .type_id
            .subst_without_removing_spans(substitution, state);
        state.registry.add_param_and_overwrite_its_id(Param {
            id: dummy_id(),
            span: None,
            is_dashed: param.is_dashed,
            name_id: param.name_id,
            type_id: substituted_type_id,
        })
    }
}

impl SubstituteWithoutRemovingSpans for NodeId<Match> {
    type Output = ExpressionId;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let top_level =
            subst_if_equal_and_get_status(ExpressionId::Match(self), substitution, state);
        if let WasSyntacticNoOp(false) = top_level.1 {
            return top_level.0;
        }

        let match_ = state.registry.match_(self).clone();
        let substituted_matchee_id = match_
            .matchee_id
            .subst_without_removing_spans(substitution, state);
        let substituted_case_list_id = match_
            .case_list_id
            .subst_without_removing_spans(substitution, state);

        ExpressionId::Match(state.registry.add_match_and_overwrite_its_id(Match {
            id: dummy_id(),
            span: None,
            matchee_id: substituted_matchee_id,
            case_list_id: substituted_case_list_id,
        }))
    }
}

impl SubstituteWithoutRemovingSpans for ListId<NodeId<MatchCase>> {
    type Output = Self;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let new_ids = state
            .registry
            .match_case_list(self)
            .to_vec()
            .into_iter()
            .map(|id| id.subst_without_removing_spans(substitution, state))
            .collect();
        state.registry.add_match_case_list(new_ids)
    }
}

impl SubstituteWithoutRemovingSpans for NodeId<MatchCase> {
    type Output = NodeId<MatchCase>;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let case = state.registry.match_case(self).clone();
        let substituted_output_id = case.output_id.subst_without_removing_spans(
            substitution.upshift(case.param_list_id.len, state.registry),
            state,
        );
        state
            .registry
            .add_match_case_and_overwrite_its_id(MatchCase {
                id: dummy_id(),
                span: None,
                variant_name_id: case.variant_name_id,
                param_list_id: case.param_list_id,
                output_id: substituted_output_id,
            })
    }
}

impl SubstituteWithoutRemovingSpans for NodeId<Forall> {
    type Output = ExpressionId;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let top_level =
            subst_if_equal_and_get_status(ExpressionId::Forall(self), substitution, state);
        if let WasSyntacticNoOp(false) = top_level.1 {
            return top_level.0;
        }

        let forall = state.registry.forall(self).clone();
        let substituted_param_list_id = forall
            .param_list_id
            .subst_without_removing_spans(substitution, state);
        let substituted_output_id = forall.output_id.subst_without_removing_spans(
            substitution.upshift(forall.param_list_id.len, state.registry),
            state,
        );

        ExpressionId::Forall(state.registry.add_forall_and_overwrite_its_id(Forall {
            id: dummy_id(),
            span: None,
            param_list_id: substituted_param_list_id,
            output_id: substituted_output_id,
        }))
    }
}

impl SubstituteWithoutRemovingSpans for NodeId<Check> {
    type Output = ExpressionId;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let top_level =
            subst_if_equal_and_get_status(ExpressionId::Check(self), substitution, state);
        if let WasSyntacticNoOp(false) = top_level.1 {
            return top_level.0;
        }

        let check = state.registry.check(self).clone();
        let substituted_assertion_list_id = check
            .assertion_list_id
            .subst_without_removing_spans(substitution, state);
        let substituted_output_id = check
            .output_id
            .subst_without_removing_spans(substitution, state);

        ExpressionId::Check(state.registry.add_check_and_overwrite_its_id(Check {
            id: dummy_id(),
            span: None,
            assertion_list_id: substituted_assertion_list_id,
            output_id: substituted_output_id,
        }))
    }
}

impl SubstituteWithoutRemovingSpans for ListId<NodeId<CheckAssertion>> {
    type Output = Self;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let new_ids = state
            .registry
            .check_assertion_list(self)
            .to_vec()
            .into_iter()
            .map(|id| id.subst_without_removing_spans(substitution, state))
            .collect();
        state.registry.add_check_assertion_list(new_ids)
    }
}

impl SubstituteWithoutRemovingSpans for NodeId<CheckAssertion> {
    type Output = Self;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let original = state.registry.check_assertion(self).clone();
        let substituted_left_id = original
            .left_id
            .subst_without_removing_spans(substitution, state);
        let substituted_right_id = original
            .right_id
            .subst_without_removing_spans(substitution, state);
        state
            .registry
            .add_check_assertion_and_overwrite_its_id(CheckAssertion {
                id: dummy_id(),
                span: None,
                kind: original.kind,
                left_id: substituted_left_id,
                right_id: substituted_right_id,
            })
    }
}

impl SubstituteWithoutRemovingSpans for GoalKwOrPossiblyInvalidExpressionId {
    type Output = Self;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        match self {
            GoalKwOrPossiblyInvalidExpressionId::GoalKw { span: start } => {
                GoalKwOrPossiblyInvalidExpressionId::GoalKw { span: start }
            }
            GoalKwOrPossiblyInvalidExpressionId::Expression(id) => {
                GoalKwOrPossiblyInvalidExpressionId::Expression(
                    id.subst_without_removing_spans(substitution, state),
                )
            }
        }
    }
}

impl SubstituteWithoutRemovingSpans for QuestionMarkOrPossiblyInvalidExpressionId {
    type Output = Self;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        match self {
            QuestionMarkOrPossiblyInvalidExpressionId::QuestionMark { span: start } => {
                QuestionMarkOrPossiblyInvalidExpressionId::QuestionMark { span: start }
            }
            QuestionMarkOrPossiblyInvalidExpressionId::Expression(id) => {
                QuestionMarkOrPossiblyInvalidExpressionId::Expression(
                    id.subst_without_removing_spans(substitution, state),
                )
            }
        }
    }
}

impl SubstituteWithoutRemovingSpans for PossiblyInvalidExpressionId {
    type Output = Self;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        match self {
            PossiblyInvalidExpressionId::Valid(id) => PossiblyInvalidExpressionId::Valid(
                id.subst_without_removing_spans(substitution, state),
            ),
            PossiblyInvalidExpressionId::Invalid(id) => PossiblyInvalidExpressionId::Invalid(id),
        }
    }
}
