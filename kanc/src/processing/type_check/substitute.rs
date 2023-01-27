use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Substitution {
    pub from: ExpressionRef<'a>,
    pub to: ExpressionRef<'a>,
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

impl<T> SubstituteWithoutRemovingSpans for Option<T>
where
    T: SubstituteWithoutRemovingSpans<Output = T>,
{
    type Output = Self;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self {
        self.map(|x| x.subst_without_removing_spans(substitution, state))
    }
}

impl SubstituteWithoutRemovingSpans for ExpressionRef<'a> {
    type Output = Self;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self {
        match self {
            ExpressionRef::Name(name_id) => {
                name_id.subst_without_removing_spans(substitution, state)
            }
            ExpressionRef::Todo(todo_id) => {
                todo_id.subst_without_removing_spans(substitution, state)
            }
            ExpressionRef::Call(call_id) => {
                call_id.subst_without_removing_spans(substitution, state)
            }
            ExpressionRef::Fun(fun_id) => fun_id.subst_without_removing_spans(substitution, state),
            ExpressionRef::Match(match_id) => {
                match_id.subst_without_removing_spans(substitution, state)
            }
            ExpressionRef::Forall(forall_id) => {
                forall_id.subst_without_removing_spans(substitution, state)
            }
            ExpressionRef::Check(check_id) => {
                check_id.subst_without_removing_spans(substitution, state)
            }
        }
    }
}

impl SubstituteWithoutRemovingSpans for &'a NameExpression<'a> {
    type Output = ExpressionRef<'a>;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        subst_if_equal_and_get_status(ExpressionRef::Name(self), substitution, state).0
    }
}

impl SubstituteWithoutRemovingSpans for &'a TodoExpression<'a> {
    type Output = ExpressionRef<'a>;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        subst_if_equal_and_get_status(ExpressionRef::Todo(self), substitution, state).0
    }
}

/// This does **not** perform substitutions on
/// child nodes.
fn subst_if_equal_and_get_status(
    original: ExpressionRef<'a>,
    substitution: Substitution,
    state: &mut ContextlessState,
) -> (ExpressionRef<'a>, WasSyntacticNoOp) {
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

impl SubstituteWithoutRemovingSpans for &'a Call<'a> {
    type Output = ExpressionRef<'a>;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let top_level =
            subst_if_equal_and_get_status(ExpressionRef::Call(self), substitution, state);
        if let WasSyntacticNoOp(false) = top_level.1 {
            return top_level.0;
        }

        let call = state.registry.get(self).clone();
        let substituted_callee_id = call
            .callee_id
            .subst_without_removing_spans(substitution, state);
        let substituted_arg_list_id = call
            .arg_list_id
            .subst_without_removing_spans(substitution, state);
        ExpressionRef::Call(state.registry.add_and_overwrite_id(Call {
            id: dummy_id(),
            span: None,
            callee_id: substituted_callee_id,
            arg_list_id: substituted_arg_list_id,
        }))
    }
}

impl SubstituteWithoutRemovingSpans for NonEmptyCallArgListId {
    type Output = Self;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        match self {
            NonEmptyCallArgListId::Unlabeled(id) => NonEmptyCallArgListId::Unlabeled(
                id.subst_without_removing_spans(substitution, state),
            ),
            NonEmptyCallArgListId::UniquelyLabeled(id) => NonEmptyCallArgListId::UniquelyLabeled(
                id.subst_without_removing_spans(substitution, state),
            ),
        }
    }
}

impl SubstituteWithoutRemovingSpans for NonEmptyListId<ExpressionRef<'a>> {
    type Output = Self;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let new_ids = state
            .registry
            .get_list(self)
            .to_non_empty_vec()
            .into_mapped(|id| id.subst_without_removing_spans(substitution, state));
        state.registry.add_list(new_ids)
    }
}

impl SubstituteWithoutRemovingSpans for NonEmptyListId<LabeledCallArgId> {
    type Output = Self;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let new_ids = state
            .registry
            .get_list(self)
            .to_non_empty_vec()
            .into_mapped(|id| id.subst_without_removing_spans(substitution, state));
        state.registry.add_list(new_ids)
    }
}

impl SubstituteWithoutRemovingSpans for LabeledCallArgId {
    type Output = Self;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        match self {
            LabeledCallArgId::Implicit {
                label_id,
                db_index,
                value_id,
            } => match substitution.from {
                ExpressionRef::Name(from_id) => {
                    let from_db_index = state.registry.get(from_id).db_index;
                    if from_db_index == db_index {
                        LabeledCallArgId::Explicit {
                            label_id,
                            value_id: substitution.to,
                        }
                    } else {
                        LabeledCallArgId::Implicit {
                            label_id,
                            db_index,
                            value_id,
                        }
                    }
                }
                _ => LabeledCallArgId::Implicit {
                    label_id,
                    db_index,
                    value_id,
                },
            },
            LabeledCallArgId::Explicit { label_id, value_id } => {
                let substituted_value_id =
                    value_id.subst_without_removing_spans(substitution, state);
                LabeledCallArgId::Explicit {
                    label_id,
                    value_id: substituted_value_id,
                }
            }
        }
    }
}

impl SubstituteWithoutRemovingSpans for &'a Fun<'a> {
    type Output = ExpressionRef<'a>;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let top_level =
            subst_if_equal_and_get_status(ExpressionRef::Fun(self), substitution, state);
        if let WasSyntacticNoOp(false) = top_level.1 {
            return top_level.0;
        }

        let fun = state.registry.get(self).clone();
        let substituted_param_list_id = fun
            .param_list_id
            .subst_without_removing_spans(substitution, state);
        let substituted_return_type_id = fun.return_type_id.subst_without_removing_spans(
            substitution.upshift(fun.param_list_id.len(), state.registry),
            state,
        );
        let substituted_body_id = fun.body_id.subst_without_removing_spans(
            substitution.upshift(fun.param_list_id.len() + 1, state.registry),
            state,
        );
        ExpressionRef::Fun(state.registry.add_and_overwrite_id(Fun {
            id: dummy_id(),
            span: None,
            name_id: fun.name_id,
            param_list_id: substituted_param_list_id,
            return_type_id: substituted_return_type_id,
            body_id: substituted_body_id,
        }))
    }
}

impl SubstituteWithoutRemovingSpans for NonEmptyParamListId {
    type Output = Self;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        match self {
            NonEmptyParamListId::Unlabeled(id) => {
                NonEmptyParamListId::Unlabeled(id.subst_without_removing_spans(substitution, state))
            }
            NonEmptyParamListId::UniquelyLabeled(id) => NonEmptyParamListId::UniquelyLabeled(
                id.subst_without_removing_spans(substitution, state),
            ),
        }
    }
}

impl SubstituteWithoutRemovingSpans for NonEmptyListId<&'a UnlabeledParam<'a>> {
    type Output = Self;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let new_ids = state
            .registry
            .get_list(self)
            .to_non_empty_vec()
            .enumerate_into_mapped(|(index, id)| {
                id.subst_without_removing_spans(substitution.upshift(index, state.registry), state)
            });
        state.registry.add_list(new_ids)
    }
}

impl SubstituteWithoutRemovingSpans for &'a UnlabeledParam<'a> {
    type Output = &'a UnlabeledParam<'a>;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let param = state.registry.get(self).clone();
        let substituted_type_id = param
            .type_id
            .subst_without_removing_spans(substitution, state);
        state.registry.add_and_overwrite_id(UnlabeledParam {
            id: dummy_id(),
            span: None,
            is_dashed: param.is_dashed,
            name_id: param.name_id,
            type_id: substituted_type_id,
        })
    }
}

impl SubstituteWithoutRemovingSpans for NonEmptyListId<&'a LabeledParam<'a>> {
    type Output = Self;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let new_ids = state
            .registry
            .get_list(self)
            .to_non_empty_vec()
            .enumerate_into_mapped(|(index, id)| {
                id.subst_without_removing_spans(substitution.upshift(index, state.registry), state)
            });
        state.registry.add_list(new_ids)
    }
}

impl SubstituteWithoutRemovingSpans for &'a LabeledParam<'a> {
    type Output = &'a LabeledParam<'a>;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let param = state.registry.get(self).clone();
        let substituted_type_id = param
            .type_id
            .subst_without_removing_spans(substitution, state);
        state.registry.add_and_overwrite_id(LabeledParam {
            id: dummy_id(),
            span: None,
            label_clause: param.label_clause,
            is_dashed: param.is_dashed,
            name_id: param.name_id,
            type_id: substituted_type_id,
        })
    }
}

impl SubstituteWithoutRemovingSpans for &'a Match<'a> {
    type Output = ExpressionRef<'a>;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let top_level =
            subst_if_equal_and_get_status(ExpressionRef::Match(self), substitution, state);
        if let WasSyntacticNoOp(false) = top_level.1 {
            return top_level.0;
        }

        let match_ = state.registry.get(self).clone();
        let substituted_matchee_id = match_
            .matchee_id
            .subst_without_removing_spans(substitution, state);
        let substituted_case_list_id = match_
            .case_list_id
            .subst_without_removing_spans(substitution, state);

        ExpressionRef::Match(state.registry.add_and_overwrite_id(Match {
            id: dummy_id(),
            span: None,
            matchee_id: substituted_matchee_id,
            case_list_id: substituted_case_list_id,
        }))
    }
}

impl SubstituteWithoutRemovingSpans for NonEmptyListId<&'a MatchCase<'a>> {
    type Output = Self;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let new_ids = state
            .registry
            .get_list(self)
            .to_non_empty_vec()
            .into_mapped(|id| id.subst_without_removing_spans(substitution, state));
        state.registry.add_list(new_ids)
    }
}

impl SubstituteWithoutRemovingSpans for &'a MatchCase<'a> {
    type Output = &'a MatchCase<'a>;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let case = state.registry.get(self).clone();
        let substituted_output_id = case.output_id.subst_without_removing_spans(
            substitution.upshift(case.param_list_id.len(), state.registry),
            state,
        );
        state.registry.add_and_overwrite_id(MatchCase {
            id: dummy_id(),
            span: None,
            variant_name_id: case.variant_name_id,
            param_list_id: case.param_list_id,
            output_id: substituted_output_id,
        })
    }
}

impl SubstituteWithoutRemovingSpans for MatchCaseOutputId {
    type Output = MatchCaseOutputId;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        match self {
            MatchCaseOutputId::Some(id) => {
                let substituted_id = id.subst_without_removing_spans(substitution, state);
                MatchCaseOutputId::Some(substituted_id)
            }
            MatchCaseOutputId::ImpossibilityClaim(kw_span) => {
                MatchCaseOutputId::ImpossibilityClaim(kw_span)
            }
        }
    }
}

impl SubstituteWithoutRemovingSpans for &'a Forall<'a> {
    type Output = ExpressionRef<'a>;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let top_level =
            subst_if_equal_and_get_status(ExpressionRef::Forall(self), substitution, state);
        if let WasSyntacticNoOp(false) = top_level.1 {
            return top_level.0;
        }

        let forall = state.registry.get(self).clone();
        let substituted_param_list_id = forall
            .param_list_id
            .subst_without_removing_spans(substitution, state);
        let substituted_output_id = forall.output_id.subst_without_removing_spans(
            substitution.upshift(forall.param_list_id.len(), state.registry),
            state,
        );

        ExpressionRef::Forall(state.registry.add_and_overwrite_id(Forall {
            id: dummy_id(),
            span: None,
            param_list_id: substituted_param_list_id,
            output_id: substituted_output_id,
        }))
    }
}

impl SubstituteWithoutRemovingSpans for &'a Check<'a> {
    type Output = ExpressionRef<'a>;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let top_level =
            subst_if_equal_and_get_status(ExpressionRef::Check(self), substitution, state);
        if let WasSyntacticNoOp(false) = top_level.1 {
            return top_level.0;
        }

        let check = state.registry.get(self).clone();
        let substituted_assertion_list_id = check
            .assertion_list_id
            .subst_without_removing_spans(substitution, state);
        let substituted_output_id = check
            .output_id
            .subst_without_removing_spans(substitution, state);

        ExpressionRef::Check(state.registry.add_and_overwrite_id(Check {
            id: dummy_id(),
            span: None,
            assertion_list_id: substituted_assertion_list_id,
            output_id: substituted_output_id,
        }))
    }
}

impl SubstituteWithoutRemovingSpans for NonEmptyListId<&'a CheckAssertion<'a>> {
    type Output = Self;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let new_ids = state
            .registry
            .get_list(self)
            .to_non_empty_vec()
            .into_mapped(|id| id.subst_without_removing_spans(substitution, state));
        state.registry.add_list(new_ids)
    }
}

impl SubstituteWithoutRemovingSpans for &'a CheckAssertion<'a> {
    type Output = Self;

    fn subst_without_removing_spans(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> Self::Output {
        let original = state.registry.get(self).clone();
        let substituted_left_id = original
            .left_id
            .subst_without_removing_spans(substitution, state);
        let substituted_right_id = original
            .right_id
            .subst_without_removing_spans(substitution, state);
        state.registry.add_and_overwrite_id(CheckAssertion {
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
