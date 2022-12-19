use super::*;

impl<T> ShiftDbIndices for Option<T>
where
    T: ShiftDbIndices<Output = T>,
{
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self::Output, F::ShiftError> {
        self.map(|x| x.try_shift_with_cutoff(f, cutoff, registry))
            .transpose()
    }
}

impl ShiftDbIndices for ContextEntry {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self::Output, F::ShiftError> {
        Ok(ContextEntry {
            type_id: self.type_id.try_shift_with_cutoff(f, cutoff, registry)?,
            definition: self.definition.try_shift_with_cutoff(f, cutoff, registry)?,
        })
    }
}

impl ShiftDbIndices for ContextEntryDefinition {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        Ok(match self {
            ContextEntryDefinition::Alias { value_id } => ContextEntryDefinition::Alias {
                value_id: value_id.try_shift_with_cutoff(f, cutoff, registry)?,
            },

            ContextEntryDefinition::Adt {
                variant_name_list_id: _,
            }
            | ContextEntryDefinition::Variant { name_id: _ }
            | ContextEntryDefinition::Uninterpreted => self,
        })
    }
}

impl ShiftDbIndices for DynamicSubstitution {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        let _0 = self.0.try_shift_with_cutoff(f, cutoff, registry)?;
        let _1 = self.1.try_shift_with_cutoff(f, cutoff, registry)?;
        Ok(DynamicSubstitution(_0, _1))
    }
}

impl ShiftDbIndices for Substitution {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        let from = self.from.try_shift_with_cutoff(f, cutoff, registry)?;
        let to = self.to.try_shift_with_cutoff(f, cutoff, registry)?;
        Ok(Substitution { from, to })
    }
}

impl ShiftDbIndices for NormalFormId {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        Ok(Self::unchecked_new(
            self.raw().try_shift_with_cutoff(f, cutoff, registry)?,
        ))
    }
}

impl ShiftDbIndices for ExpressionId {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        Ok(match self {
            ExpressionId::Name(name_id) => {
                ExpressionId::Name(name_id.try_shift_with_cutoff(f, cutoff, registry)?)
            }
            ExpressionId::Call(call_id) => {
                ExpressionId::Call(call_id.try_shift_with_cutoff(f, cutoff, registry)?)
            }
            ExpressionId::Fun(fun_id) => {
                ExpressionId::Fun(fun_id.try_shift_with_cutoff(f, cutoff, registry)?)
            }
            ExpressionId::Match(match_id) => {
                ExpressionId::Match(match_id.try_shift_with_cutoff(f, cutoff, registry)?)
            }
            ExpressionId::Forall(forall_id) => {
                ExpressionId::Forall(forall_id.try_shift_with_cutoff(f, cutoff, registry)?)
            }
            ExpressionId::Check(check_id) => {
                ExpressionId::Check(check_id.try_shift_with_cutoff(f, cutoff, registry)?)
            }
        })
    }
}

impl ShiftDbIndices for NodeId<NameExpression> {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        let name = registry.get(self);
        let shifted_index = f.try_apply(name.db_index, cutoff)?;
        let shifted_with_dummy_id = NameExpression {
            db_index: shifted_index,
            ..*name
        };
        Ok(registry.add(shifted_with_dummy_id))
    }
}

impl ShiftDbIndices for NodeId<Call> {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        let call = registry.get(self).clone();
        let shifted_callee_id = call.callee_id.try_shift_with_cutoff(f, cutoff, registry)?;
        let shifted_argument_id = call
            .arg_list_id
            .try_shift_with_cutoff(f, cutoff, registry)?;
        Ok(registry.add(Call {
            id: dummy_id(),
            span: call.span,
            callee_id: shifted_callee_id,
            arg_list_id: shifted_argument_id,
        }))
    }
}

impl ShiftDbIndices for NonEmptyCallArgListId {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        match self {
            NonEmptyCallArgListId::Unlabeled(id) => {
                let shifted_id = id.try_shift_with_cutoff(f, cutoff, registry)?;
                Ok(NonEmptyCallArgListId::Unlabeled(shifted_id))
            }
            NonEmptyCallArgListId::UniquelyLabeled(id) => {
                let shifted_id = id.try_shift_with_cutoff(f, cutoff, registry)?;
                Ok(NonEmptyCallArgListId::UniquelyLabeled(shifted_id))
            }
        }
    }
}

impl ShiftDbIndices for NonEmptyListId<ExpressionId> {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        let list = registry
            .get_list(self)
            .to_non_empty_vec()
            .try_into_mapped(|id| id.try_shift_with_cutoff(f, cutoff, registry))?;
        Ok(registry.add_list(list))
    }
}

impl ShiftDbIndices for NonEmptyListId<LabeledCallArgId> {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        let list = registry
            .get_list(self)
            .to_non_empty_vec()
            .try_into_mapped(|id| id.try_shift_with_cutoff(f, cutoff, registry))?;
        Ok(registry.add_list(list))
    }
}

impl ShiftDbIndices for LabeledCallArgId {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        Ok(match self {
            LabeledCallArgId::Implicit { label_id, db_index } => LabeledCallArgId::Implicit {
                label_id,
                db_index: f.try_apply(db_index, cutoff)?,
            },
            LabeledCallArgId::Explicit { label_id, value_id } => LabeledCallArgId::Explicit {
                label_id,
                value_id: value_id.try_shift_with_cutoff(f, cutoff, registry)?,
            },
        })
    }
}

impl ShiftDbIndices for NodeId<Fun> {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        let fun = registry.get(self).clone();
        let param_arity = fun.param_list_id.len();
        let shifted_param_list_id = fun
            .param_list_id
            .try_shift_with_cutoff(f, cutoff, registry)?;
        let shifted_return_type_id =
            fun.return_type_id
                .try_shift_with_cutoff(f, cutoff + param_arity, registry)?;
        let shifted_body_id =
            fun.body_id
                .try_shift_with_cutoff(f, cutoff + param_arity + 1, registry)?;
        Ok(registry.add(Fun {
            id: dummy_id(),
            span: fun.span,
            name_id: fun.name_id,
            param_list_id: shifted_param_list_id,
            return_type_id: shifted_return_type_id,
            body_id: shifted_body_id,
            skip_type_checking_body: fun.skip_type_checking_body,
        }))
    }
}

impl ShiftDbIndices for NonEmptyParamListId {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        match self {
            NonEmptyParamListId::Unlabeled(id) => {
                let shifted_id = id.try_shift_with_cutoff(f, cutoff, registry)?;
                Ok(NonEmptyParamListId::Unlabeled(shifted_id))
            }
            NonEmptyParamListId::UniquelyLabeled(id) => {
                let shifted_id = id.try_shift_with_cutoff(f, cutoff, registry)?;
                Ok(NonEmptyParamListId::UniquelyLabeled(shifted_id))
            }
        }
    }
}

impl ShiftDbIndices for NonEmptyListId<NodeId<UnlabeledParam>> {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        let list = registry
            .get_list(self)
            .to_non_empty_vec()
            .try_enumerate_into_mapped(|(index, id)| {
                id.try_shift_with_cutoff(f, cutoff + index, registry)
            })?;
        Ok(registry.add_list(list))
    }
}

impl ShiftDbIndices for NodeId<UnlabeledParam> {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        let param = registry.get(self).clone();
        let shifted_type_id = param.type_id.try_shift_with_cutoff(f, cutoff, registry)?;
        Ok(registry.add(UnlabeledParam {
            id: dummy_id(),
            span: param.span,
            is_dashed: param.is_dashed,
            name_id: param.name_id,
            type_id: shifted_type_id,
        }))
    }
}

impl ShiftDbIndices for NonEmptyListId<NodeId<LabeledParam>> {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        let list = registry
            .get_list(self)
            .to_non_empty_vec()
            .try_enumerate_into_mapped(|(index, id)| {
                id.try_shift_with_cutoff(f, cutoff + index, registry)
            })?;
        Ok(registry.add_list(list))
    }
}

impl ShiftDbIndices for NodeId<LabeledParam> {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        let param = registry.get(self).clone();
        let shifted_type_id = param.type_id.try_shift_with_cutoff(f, cutoff, registry)?;
        Ok(registry.add(LabeledParam {
            id: dummy_id(),
            span: param.span,
            label_id: param.label_id,
            is_dashed: param.is_dashed,
            name_id: param.name_id,
            type_id: shifted_type_id,
        }))
    }
}

impl ShiftDbIndices for NodeId<Match> {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        let match_ = registry.get(self).clone();
        let shifted_matchee_id = match_
            .matchee_id
            .try_shift_with_cutoff(f, cutoff, registry)?;
        let shifted_case_list_id = match_
            .case_list_id
            .try_shift_with_cutoff(f, cutoff, registry)?;
        Ok(registry.add(Match {
            id: dummy_id(),
            span: match_.span,
            matchee_id: shifted_matchee_id,
            case_list_id: shifted_case_list_id,
        }))
    }
}

impl ShiftDbIndices for NonEmptyListId<NodeId<MatchCase>> {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        let list = registry
            .get_list(self)
            .to_non_empty_vec()
            .try_into_mapped(|id| id.try_shift_with_cutoff(f, cutoff, registry))?;
        Ok(registry.add_list(list))
    }
}

impl ShiftDbIndices for NodeId<MatchCase> {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        let case = registry.get(self).clone();
        let arity = case.param_list_id.len();
        let shifted_output_id =
            case.output_id
                .try_shift_with_cutoff(f, cutoff + arity, registry)?;
        Ok(registry.add(MatchCase {
            id: dummy_id(),
            span: case.span,
            variant_name_id: case.variant_name_id,
            param_list_id: case.param_list_id,
            output_id: shifted_output_id,
        }))
    }
}

impl ShiftDbIndices for NodeId<Forall> {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        let forall = registry.get(self).clone();
        let arity = forall.param_list_id.len();
        let shifted_param_list_id = forall
            .param_list_id
            .try_shift_with_cutoff(f, cutoff, registry)?;
        let shifted_output_id =
            forall
                .output_id
                .try_shift_with_cutoff(f, cutoff + arity, registry)?;
        Ok(registry.add(Forall {
            id: dummy_id(),
            span: forall.span,
            param_list_id: shifted_param_list_id,
            output_id: shifted_output_id,
        }))
    }
}

impl ShiftDbIndices for NodeId<Check> {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        let check = registry.get(self).clone();
        let shifted_assertion_list_id = check
            .assertion_list_id
            .try_shift_with_cutoff(f, cutoff, registry)?;
        let shifted_output_id = check.output_id.try_shift_with_cutoff(f, cutoff, registry)?;
        Ok(registry.add(Check {
            id: dummy_id(),
            span: check.span,
            assertion_list_id: shifted_assertion_list_id,
            output_id: shifted_output_id,
        }))
    }
}

impl ShiftDbIndices for NonEmptyListId<NodeId<CheckAssertion>> {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        let list = registry
            .get_list(self)
            .to_non_empty_vec()
            .try_into_mapped(|id| id.try_shift_with_cutoff(f, cutoff, registry))?;
        Ok(registry.add_list(list))
    }
}

impl ShiftDbIndices for NodeId<CheckAssertion> {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        let assertion = registry.get(self).clone();
        let shifted_left_id = assertion
            .left_id
            .try_shift_with_cutoff(f, cutoff, registry)?;
        let shifted_right_id = assertion
            .right_id
            .try_shift_with_cutoff(f, cutoff, registry)?;
        Ok(registry.add(CheckAssertion {
            id: dummy_id(),
            span: assertion.span,
            kind: assertion.kind,
            left_id: shifted_left_id,
            right_id: shifted_right_id,
        }))
    }
}

impl ShiftDbIndices for GoalKwOrPossiblyInvalidExpressionId {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        Ok(match self {
            GoalKwOrPossiblyInvalidExpressionId::GoalKw { span: start } => {
                GoalKwOrPossiblyInvalidExpressionId::GoalKw { span: start }
            }
            GoalKwOrPossiblyInvalidExpressionId::Expression(id) => {
                GoalKwOrPossiblyInvalidExpressionId::Expression(
                    id.try_shift_with_cutoff(f, cutoff, registry)?.into(),
                )
            }
        })
    }
}

impl ShiftDbIndices for QuestionMarkOrPossiblyInvalidExpressionId {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        Ok(match self {
            QuestionMarkOrPossiblyInvalidExpressionId::QuestionMark { span: start } => {
                QuestionMarkOrPossiblyInvalidExpressionId::QuestionMark { span: start }
            }
            QuestionMarkOrPossiblyInvalidExpressionId::Expression(id) => {
                QuestionMarkOrPossiblyInvalidExpressionId::Expression(
                    id.try_shift_with_cutoff(f, cutoff, registry)?.into(),
                )
            }
        })
    }
}

impl ShiftDbIndices for PossiblyInvalidExpressionId {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        Ok(match self {
            PossiblyInvalidExpressionId::Valid(id) => {
                PossiblyInvalidExpressionId::Valid(id.try_shift_with_cutoff(f, cutoff, registry)?)
            }
            PossiblyInvalidExpressionId::Invalid(original_invalid) => {
                // We can return the original as-is since it's not bound in
                // the first place (so there's nothing to shift).
                PossiblyInvalidExpressionId::Invalid(original_invalid)
            }
        })
    }
}
