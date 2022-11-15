use super::*;

pub trait ShiftDbIndices {
    type Output;

    fn try_shift_with_cutoff<A: ShiftAmount>(
        self,
        amount: A,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self::Output, A::ShiftError>;

    fn upshift(self, amount: usize, registry: &mut NodeRegistry) -> Self::Output
    where
        Self: Sized,
    {
        self.try_shift_with_cutoff(Upshift(amount), 0, registry)
            .safe_unwrap()
    }

    fn upshift_with_cutoff(
        self,
        amount: usize,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Self::Output
    where
        Self: Sized,
    {
        self.try_shift_with_cutoff(Upshift(amount), cutoff, registry)
            .safe_unwrap()
    }

    fn downshift(self, amount: usize, registry: &mut NodeRegistry) -> Self::Output
    where
        Self: Sized,
    {
        self.try_downshift(amount, registry)
            .unwrap_or_else(|err| panic!("Downshift failed: {:?}", err))
    }

    fn try_downshift(
        self,
        amount: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self::Output, DbIndexTooSmallForDownshiftError>
    where
        Self: Sized,
    {
        self.try_downshift_with_cutoff(amount, 0, registry)
    }

    fn downshift_with_cutoff(
        self,
        amount: usize,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Self::Output
    where
        Self: Sized,
    {
        self.try_downshift_with_cutoff(amount, cutoff, registry)
            .unwrap_or_else(|err| panic!("Downshift failed: {:?}", err))
    }

    fn try_downshift_with_cutoff(
        self,
        amount: usize,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self::Output, DbIndexTooSmallForDownshiftError>
    where
        Self: Sized,
    {
        self.try_shift_with_cutoff(Downshift(amount), cutoff, registry)
    }

    fn bishift(self, len: usize, pivot: DbIndex, registry: &mut NodeRegistry) -> Self::Output
    where
        Self: Sized,
    {
        self.try_bishift(len, pivot, registry)
            .unwrap_or_else(|err| panic!("Bishift failed: {:?}", err))
    }

    fn try_bishift(
        self,
        len: usize,
        pivot: DbIndex,
        registry: &mut NodeRegistry,
    ) -> Result<Self::Output, DbIndexTooSmallForDownshiftError>
    where
        Self: Sized,
    {
        self.try_shift_with_cutoff(Bishift { len, pivot }, 0, registry)
    }
}

pub trait ShiftAmount: Copy {
    // TODO: Remove trait bound after debug
    type ShiftError: std::fmt::Debug;
    fn try_apply(&self, i: DbIndex, cutoff: usize) -> Result<DbIndex, Self::ShiftError>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Upshift(usize);

impl ShiftAmount for Upshift {
    type ShiftError = Infallible;
    fn try_apply(&self, i: DbIndex, cutoff: usize) -> Result<DbIndex, Infallible> {
        if i.0 < cutoff {
            Ok(i)
        } else {
            Ok(DbIndex(i.0 + self.0))
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Downshift(usize);

impl ShiftAmount for Downshift {
    type ShiftError = DbIndexTooSmallForDownshiftError;
    fn try_apply(
        &self,
        i: DbIndex,
        cutoff: usize,
    ) -> Result<DbIndex, DbIndexTooSmallForDownshiftError> {
        if i.0 < cutoff {
            Ok(i)
        } else if i.0 < self.0 {
            Err(DbIndexTooSmallForDownshiftError {
                db_index: i,
                downshift_amount: self.0,
            })
        } else {
            Ok(DbIndex(i.0 - self.0))
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Bishift {
    len: usize,
    pivot: DbIndex,
}

impl Bishift {
    fn distance(self) -> usize {
        self.pivot.0 - self.len
    }
}

impl ShiftAmount for Bishift {
    type ShiftError = DbIndexTooSmallForDownshiftError;
    fn try_apply(
        &self,
        i: DbIndex,
        cutoff: usize,
    ) -> Result<DbIndex, DbIndexTooSmallForDownshiftError> {
        if (0..cutoff).contains(&i.0) {
            Ok(i)
        } else if (cutoff..cutoff + self.len).contains(&i.0) {
            Ok(Upshift(self.distance()).try_apply(i, cutoff).safe_unwrap())
        } else if (cutoff + self.len..cutoff + self.pivot.0).contains(&i.0) {
            Downshift(self.len).try_apply(i, cutoff)
        } else {
            // Indices equal to or greater than the pivot are left as-is.
            Ok(i)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DbIndexTooSmallForDownshiftError {
    db_index: DbIndex,
    downshift_amount: usize,
}

impl ShiftDbIndices for ContextEntry {
    type Output = Self;

    fn try_shift_with_cutoff<A: ShiftAmount>(
        self,
        amount: A,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self::Output, A::ShiftError> {
        Ok(ContextEntry {
            type_id: self
                .type_id
                .try_shift_with_cutoff(amount, cutoff, registry)?,
            definition: self
                .definition
                .try_shift_with_cutoff(amount, cutoff, registry)?,
        })
    }
}

impl ShiftDbIndices for ContextEntryDefinition {
    type Output = Self;

    fn try_shift_with_cutoff<A: ShiftAmount>(
        self,
        amount: A,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, A::ShiftError> {
        Ok(match self {
            ContextEntryDefinition::Alias { value_id } => ContextEntryDefinition::Alias {
                value_id: value_id.try_shift_with_cutoff(amount, cutoff, registry)?,
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

    fn try_shift_with_cutoff<A: ShiftAmount>(
        self,
        amount: A,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, A::ShiftError> {
        let from = self.from.try_shift_with_cutoff(amount, cutoff, registry)?;
        let to = self.to.try_shift_with_cutoff(amount, cutoff, registry)?;
        Ok(Substitution { from, to })
    }
}

impl ShiftDbIndices for NormalFormId {
    type Output = Self;

    fn try_shift_with_cutoff<A: ShiftAmount>(
        self,
        amount: A,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, A::ShiftError> {
        Ok(Self::unchecked_new(
            self.raw().try_shift_with_cutoff(amount, cutoff, registry)?,
        ))
    }
}

impl ShiftDbIndices for ExpressionId {
    type Output = Self;

    fn try_shift_with_cutoff<A: ShiftAmount>(
        self,
        amount: A,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, A::ShiftError> {
        Ok(match self {
            ExpressionId::Name(name_id) => {
                ExpressionId::Name(name_id.try_shift_with_cutoff(amount, cutoff, registry)?)
            }
            ExpressionId::Call(call_id) => {
                ExpressionId::Call(call_id.try_shift_with_cutoff(amount, cutoff, registry)?)
            }
            ExpressionId::Fun(fun_id) => {
                ExpressionId::Fun(fun_id.try_shift_with_cutoff(amount, cutoff, registry)?)
            }
            ExpressionId::Match(match_id) => {
                ExpressionId::Match(match_id.try_shift_with_cutoff(amount, cutoff, registry)?)
            }
            ExpressionId::Forall(forall_id) => {
                ExpressionId::Forall(forall_id.try_shift_with_cutoff(amount, cutoff, registry)?)
            }
        })
    }
}

impl ShiftDbIndices for NodeId<NameExpression> {
    type Output = Self;

    fn try_shift_with_cutoff<A: ShiftAmount>(
        self,
        amount: A,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, A::ShiftError> {
        let name = registry.name_expression(self);
        if let Err(err) = amount.try_apply(name.db_index, cutoff) {
            println!("DB_SHIFT_ERROR.error: {:?}", err);
            println!(
                "DB_SHIFT_ERROR.name: {:?}",
                crate::processing::x_expand_lightened::expand_expression(
                    registry,
                    ExpressionId::Name(self)
                )
            );
        }
        let shifted_index = amount.try_apply(name.db_index, cutoff)?;
        let shifted_with_dummy_id = NameExpression {
            db_index: shifted_index,
            ..*name
        };
        Ok(registry.add_name_expression_and_overwrite_its_id(shifted_with_dummy_id))
    }
}

impl ShiftDbIndices for NodeId<Call> {
    type Output = Self;

    fn try_shift_with_cutoff<A: ShiftAmount>(
        self,
        amount: A,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, A::ShiftError> {
        let call = registry.call(self).clone();
        let shifted_callee_id = call
            .callee_id
            .try_shift_with_cutoff(amount, cutoff, registry)?;
        let shifted_argument_id = call
            .arg_list_id
            .try_shift_with_cutoff(amount, cutoff, registry)?;
        Ok(registry.add_call_and_overwrite_its_id(Call {
            id: dummy_id(),
            callee_id: shifted_callee_id,
            arg_list_id: shifted_argument_id,
        }))
    }
}

impl ShiftDbIndices for ListId<ExpressionId> {
    type Output = Self;

    fn try_shift_with_cutoff<A: ShiftAmount>(
        self,
        amount: A,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, A::ShiftError> {
        let list: Vec<ExpressionId> = registry
            .expression_list(self)
            .to_vec()
            .into_iter()
            .map(|id| id.try_shift_with_cutoff(amount, cutoff, registry))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(registry.add_expression_list(list))
    }
}

impl ShiftDbIndices for NodeId<Fun> {
    type Output = Self;

    fn try_shift_with_cutoff<A: ShiftAmount>(
        self,
        amount: A,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, A::ShiftError> {
        let fun = registry.fun(self).clone();
        let param_arity = fun.param_list_id.len;
        let shifted_param_list_id = fun
            .param_list_id
            .try_shift_with_cutoff(amount, cutoff, registry)?;
        let shifted_return_type_id =
            fun.return_type_id
                .try_shift_with_cutoff(amount, cutoff + param_arity, registry)?;
        let shifted_body_id =
            fun.body_id
                .try_shift_with_cutoff(amount, cutoff + param_arity + 1, registry)?;
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

    fn try_shift_with_cutoff<A: ShiftAmount>(
        self,
        amount: A,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, A::ShiftError> {
        let list: Vec<NodeId<Param>> = registry
            .param_list(self)
            .to_vec()
            .into_iter()
            .enumerate()
            .map(|(index, id)| id.try_shift_with_cutoff(amount, cutoff + index, registry))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(registry.add_param_list(list))
    }
}

impl ShiftDbIndices for NodeId<Param> {
    type Output = Self;

    fn try_shift_with_cutoff<A: ShiftAmount>(
        self,
        amount: A,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, A::ShiftError> {
        let param = registry.param(self).clone();
        let shifted_type_id = param
            .type_id
            .try_shift_with_cutoff(amount, cutoff, registry)?;
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

    fn try_shift_with_cutoff<A: ShiftAmount>(
        self,
        amount: A,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, A::ShiftError> {
        let match_ = registry.match_(self).clone();
        let shifted_matchee_id = match_
            .matchee_id
            .try_shift_with_cutoff(amount, cutoff, registry)?;
        let shifted_case_list_id = match_
            .case_list_id
            .try_shift_with_cutoff(amount, cutoff, registry)?;
        Ok(registry.add_match_and_overwrite_its_id(Match {
            id: dummy_id(),
            matchee_id: shifted_matchee_id,
            case_list_id: shifted_case_list_id,
        }))
    }
}

impl ShiftDbIndices for ListId<NodeId<MatchCase>> {
    type Output = Self;

    fn try_shift_with_cutoff<A: ShiftAmount>(
        self,
        amount: A,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, A::ShiftError> {
        let list: Vec<NodeId<MatchCase>> = registry
            .match_case_list(self)
            .to_vec()
            .into_iter()
            .map(|id| id.try_shift_with_cutoff(amount, cutoff, registry))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(registry.add_match_case_list(list))
    }
}

impl ShiftDbIndices for NodeId<MatchCase> {
    type Output = Self;

    fn try_shift_with_cutoff<A: ShiftAmount>(
        self,
        amount: A,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, A::ShiftError> {
        let case = registry.match_case(self).clone();
        let arity = case.param_list_id.len;
        let shifted_output_id =
            case.output_id
                .try_shift_with_cutoff(amount, cutoff + arity, registry)?;
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

    fn try_shift_with_cutoff<A: ShiftAmount>(
        self,
        amount: A,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self, A::ShiftError> {
        let forall = registry.forall(self).clone();
        let arity = forall.param_list_id.len;
        let shifted_param_list_id = forall
            .param_list_id
            .try_shift_with_cutoff(amount, cutoff, registry)?;
        let shifted_output_id =
            forall
                .output_id
                .try_shift_with_cutoff(amount, cutoff + arity, registry)?;
        Ok(registry.add_forall_and_overwrite_its_id(Forall {
            id: dummy_id(),
            param_list_id: shifted_param_list_id,
            output_id: shifted_output_id,
        }))
    }
}
