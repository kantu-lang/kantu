use super::*;

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
        let name = registry.name_expression(self);
        let shifted_index = f.try_apply(name.db_index, cutoff)?;
        let shifted_with_dummy_id = NameExpression {
            db_index: shifted_index,
            ..*name
        };
        Ok(registry.add_name_expression_and_overwrite_its_id(shifted_with_dummy_id))
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
        let call = registry.call(self).clone();
        let shifted_callee_id = call.callee_id.try_shift_with_cutoff(f, cutoff, registry)?;
        let shifted_argument_id = call
            .arg_list_id
            .try_shift_with_cutoff(f, cutoff, registry)?;
        Ok(registry.add_call_and_overwrite_its_id(Call {
            id: dummy_id(),
            callee_id: shifted_callee_id,
            arg_list_id: shifted_argument_id,
        }))
    }
}

impl ShiftDbIndices for ListId<ExpressionId> {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        let list: Vec<ExpressionId> = registry
            .expression_list(self)
            .to_vec()
            .into_iter()
            .map(|id| id.try_shift_with_cutoff(f, cutoff, registry))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(registry.add_expression_list(list))
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
        let fun = registry.fun(self).clone();
        let param_arity = fun.param_list_id.len;
        let shifted_param_list_id = fun
            .param_list_id
            .try_shift_with_cutoff(f, cutoff, registry)?;
        let shifted_return_type_id =
            fun.return_type_id
                .try_shift_with_cutoff(f, cutoff + param_arity, registry)?;
        let shifted_body_id =
            fun.body_id
                .try_shift_with_cutoff(f, cutoff + param_arity + 1, registry)?;
        Ok(registry.add_fun_and_overwrite_its_id(Fun {
            id: dummy_id(),
            name_id: fun.name_id,
            param_list_id: shifted_param_list_id,
            return_type_id: shifted_return_type_id,
            body_id: shifted_body_id,
            skip_type_checking_body: fun.skip_type_checking_body,
        }))
    }
}

impl ShiftDbIndices for ListId<NodeId<Param>> {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        let list: Vec<NodeId<Param>> = registry
            .param_list(self)
            .to_vec()
            .into_iter()
            .enumerate()
            .map(|(index, id)| id.try_shift_with_cutoff(f, cutoff + index, registry))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(registry.add_param_list(list))
    }
}

impl ShiftDbIndices for NodeId<Param> {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        let param = registry.param(self).clone();
        let shifted_type_id = param.type_id.try_shift_with_cutoff(f, cutoff, registry)?;
        Ok(registry.add_param_and_overwrite_its_id(Param {
            id: dummy_id(),
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
        let match_ = registry.match_(self).clone();
        let shifted_matchee_id = match_
            .matchee_id
            .try_shift_with_cutoff(f, cutoff, registry)?;
        let shifted_case_list_id = match_
            .case_list_id
            .try_shift_with_cutoff(f, cutoff, registry)?;
        Ok(registry.add_match_and_overwrite_its_id(Match {
            id: dummy_id(),
            matchee_id: shifted_matchee_id,
            case_list_id: shifted_case_list_id,
        }))
    }
}

impl ShiftDbIndices for ListId<NodeId<MatchCase>> {
    type Output = Self;

    fn try_shift_with_cutoff<F: ShiftFn>(
        self,
        f: F,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, F::ShiftError> {
        let list: Vec<NodeId<MatchCase>> = registry
            .match_case_list(self)
            .to_vec()
            .into_iter()
            .map(|id| id.try_shift_with_cutoff(f, cutoff, registry))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(registry.add_match_case_list(list))
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
        let case = registry.match_case(self).clone();
        let arity = case.param_list_id.len;
        let shifted_output_id =
            case.output_id
                .try_shift_with_cutoff(f, cutoff + arity, registry)?;
        Ok(registry.add_match_case_and_overwrite_its_id(MatchCase {
            id: dummy_id(),
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
        let forall = registry.forall(self).clone();
        let arity = forall.param_list_id.len;
        let shifted_param_list_id = forall
            .param_list_id
            .try_shift_with_cutoff(f, cutoff, registry)?;
        let shifted_output_id =
            forall
                .output_id
                .try_shift_with_cutoff(f, cutoff + arity, registry)?;
        Ok(registry.add_forall_and_overwrite_its_id(Forall {
            id: dummy_id(),
            param_list_id: shifted_param_list_id,
            output_id: shifted_output_id,
        }))
    }
}
