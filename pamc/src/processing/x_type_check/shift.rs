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

    fn try_upshift(
        self,
        amount: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self::Output, Infallible>
    where
        Self: Sized,
    {
        self.try_shift_with_cutoff(Upshift(amount), 0, registry)
    }

    fn downshift(self, amount: usize, registry: &mut NodeRegistry) -> Self::Output
    where
        Self: Sized,
    {
        self.try_shift_with_cutoff(Downshift(amount), 0, registry)
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
        self.try_shift_with_cutoff(Downshift(amount), 0, registry)
    }
}

pub trait ShiftAmount {
    type ShiftError;
    fn try_apply(&self, i: DbIndex) -> Result<DbIndex, Self::ShiftError>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Upshift(usize);

impl ShiftAmount for Upshift {
    type ShiftError = Infallible;
    fn try_apply(&self, i: DbIndex) -> Result<DbIndex, Infallible> {
        Ok(DbIndex(i.0 + self.0))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Downshift(usize);

impl ShiftAmount for Downshift {
    type ShiftError = DbIndexTooSmallForDownshiftError;
    fn try_apply(&self, i: DbIndex) -> Result<DbIndex, DbIndexTooSmallForDownshiftError> {
        if i.0 < self.0 {
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
pub struct DbIndexTooSmallForDownshiftError {
    db_index: DbIndex,
    downshift_amount: usize,
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
        _amount: A,
        _cutoff: usize,
        _registry: &mut NodeRegistry,
    ) -> Result<Self, A::ShiftError> {
        unimplemented!()
    }
}

impl ShiftDbIndices for NodeId<Call> {
    type Output = Self;

    fn try_shift_with_cutoff<A: ShiftAmount>(
        self,
        _amount: A,
        _cutoff: usize,
        _registry: &mut NodeRegistry,
    ) -> Result<Self, A::ShiftError> {
        unimplemented!()
    }
}

impl ShiftDbIndices for NodeId<Fun> {
    type Output = Self;

    fn try_shift_with_cutoff<A: ShiftAmount>(
        self,
        _amount: A,
        _cutoff: usize,
        _registry: &mut NodeRegistry,
    ) -> Result<Self, A::ShiftError> {
        unimplemented!()
    }
}

impl ShiftDbIndices for NodeId<Match> {
    type Output = Self;

    fn try_shift_with_cutoff<A: ShiftAmount>(
        self,
        _amount: A,
        _cutoff: usize,
        _registry: &mut NodeRegistry,
    ) -> Result<Self, A::ShiftError> {
        unimplemented!()
    }
}

impl ShiftDbIndices for NodeId<Forall> {
    type Output = Self;

    fn try_shift_with_cutoff<A: ShiftAmount>(
        self,
        _amount: A,
        _cutoff: usize,
        _registry: &mut NodeRegistry,
    ) -> Result<Self, A::ShiftError> {
        unimplemented!()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PossibleArgListId {
    Nullary,
    Some(ListId<ExpressionId>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdtExpression {
    pub type_name_id: NodeId<NameExpression>,
    pub variant_name_list_id: ListId<NodeId<Identifier>>,
    pub arg_list_id: PossibleArgListId,
}

/// If the provided expression is has a variant at
/// the top level,this returns IDs for the variant name
/// and the variant's argument list.
/// Otherwise, returns `None`.
pub fn try_as_adt_expression(
    context: &mut Context,
    registry: &mut NodeRegistry,
    expression_id: NormalFormId,
) -> Option<AdtExpression> {
    match expression_id.raw() {
        ExpressionId::Name(name_id) => {
            let db_index = registry.name_expression(name_id).db_index;
            let definition = context.get_definition(db_index, registry);
            match definition {
                ContextEntryDefinition::Adt {
                    variant_name_list_id,
                } => Some(AdtExpression {
                    type_name_id: name_id,
                    variant_name_list_id,
                    arg_list_id: PossibleArgListId::Nullary,
                }),
                _ => None,
            }
        }
        ExpressionId::Call(call_id) => {
            let call = registry.call(call_id).clone();
            match call.callee_id {
                ExpressionId::Name(name_id) => {
                    let db_index = registry.name_expression(name_id).db_index;
                    let definition = context.get_definition(db_index, registry);
                    match definition {
                        ContextEntryDefinition::Adt {
                            variant_name_list_id,
                        } => Some(AdtExpression {
                            type_name_id: name_id,
                            variant_name_list_id: variant_name_list_id,
                            arg_list_id: PossibleArgListId::Some(call.arg_list_id),
                        }),
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }}
