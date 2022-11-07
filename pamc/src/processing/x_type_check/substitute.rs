use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Substitution {
    pub from: NormalFormId,
    pub to: NormalFormId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WasNoOp(pub bool);

impl std::ops::BitAndAssign for WasNoOp {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl std::ops::BitAnd for WasNoOp {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self {
        self &= rhs;
        self
    }
}

pub(super) trait Substitute {
    type Output;

    fn subst(
        self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> (Self::Output, WasNoOp);

    fn subst_all(
        self,
        substitutions: &[Substitution],
        state: &mut ContextlessState,
    ) -> (Self::Output, WasNoOp)
    where
        Self: Sized + Substitute<Output = Self>,
    {
        let mut output = self;
        let mut was_no_op = WasNoOp(true);
        for &subst in substitutions {
            let result = output.subst(subst, state);
            output = result.0;
            was_no_op &= result.1;
        }
        (output, was_no_op)
    }
}

impl Substitute for ExpressionId {
    type Output = Self;

    fn subst(self, substitution: Substitution, state: &mut ContextlessState) -> (Self, WasNoOp) {
        match self {
            ExpressionId::Name(name_id) => {
                name_id.subst(substitution, state).map0(ExpressionId::Name)
            }
            ExpressionId::Call(call_id) => {
                call_id.subst(substitution, state).map0(ExpressionId::Call)
            }
            ExpressionId::Fun(fun_id) => fun_id.subst(substitution, state).map0(ExpressionId::Fun),
            ExpressionId::Match(match_id) => match_id
                .subst(substitution, state)
                .map0(ExpressionId::Match),
            ExpressionId::Forall(forall_id) => forall_id
                .subst(substitution, state)
                .map0(ExpressionId::Forall),
        }
    }
}

impl Substitute for NodeId<NameExpression> {
    type Output = Self;

    fn subst(self, _substitution: Substitution, _state: &mut ContextlessState) -> (Self, WasNoOp) {
        unimplemented!()
    }
}

impl Substitute for NodeId<Call> {
    type Output = Self;

    fn subst(self, _substitution: Substitution, _state: &mut ContextlessState) -> (Self, WasNoOp) {
        unimplemented!()
    }
}

impl Substitute for NodeId<Fun> {
    type Output = Self;

    fn subst(self, _substitution: Substitution, _state: &mut ContextlessState) -> (Self, WasNoOp) {
        unimplemented!()
    }
}

impl Substitute for NodeId<Match> {
    type Output = Self;

    fn subst(self, _substitution: Substitution, _state: &mut ContextlessState) -> (Self, WasNoOp) {
        unimplemented!()
    }
}

impl Substitute for NodeId<Forall> {
    type Output = Self;

    fn subst(self, _substitution: Substitution, _state: &mut ContextlessState) -> (Self, WasNoOp) {
        unimplemented!()
    }
}
