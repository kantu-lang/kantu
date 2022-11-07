use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Substitution {
    pub from: NormalFormId,
    pub to: NormalFormId,
}

pub(super) trait Substitute {
    type Output;

    fn subst(self, substitution: Substitution, state: &mut ContextlessState) -> Self::Output;

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

impl Substitute for ExpressionId {
    type Output = Self;

    fn subst(self, substitution: Substitution, state: &mut ContextlessState) -> Self {
        match self {
            ExpressionId::Name(name_id) => ExpressionId::Name(name_id.subst(substitution, state)),
            ExpressionId::Call(call_id) => ExpressionId::Call(call_id.subst(substitution, state)),
            ExpressionId::Fun(fun_id) => ExpressionId::Fun(fun_id.subst(substitution, state)),
            ExpressionId::Match(match_id) => {
                ExpressionId::Match(match_id.subst(substitution, state))
            }
            ExpressionId::Forall(forall_id) => {
                ExpressionId::Forall(forall_id.subst(substitution, state))
            }
        }
    }
}

impl Substitute for NodeId<NameExpression> {
    type Output = Self;

    fn subst(self, _substitution: Substitution, _state: &mut ContextlessState) -> Self {
        unimplemented!()
    }
}

impl Substitute for NodeId<Call> {
    type Output = Self;

    fn subst(self, _substitution: Substitution, _state: &mut ContextlessState) -> Self {
        unimplemented!()
    }
}

impl Substitute for NodeId<Fun> {
    type Output = Self;

    fn subst(self, _substitution: Substitution, _state: &mut ContextlessState) -> Self {
        unimplemented!()
    }
}

impl Substitute for NodeId<Match> {
    type Output = Self;

    fn subst(self, _substitution: Substitution, _state: &mut ContextlessState) -> Self {
        unimplemented!()
    }
}

impl Substitute for NodeId<Forall> {
    type Output = Self;

    fn subst(self, _substitution: Substitution, _state: &mut ContextlessState) -> Self {
        unimplemented!()
    }
}
