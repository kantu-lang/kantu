use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Substitution {
    Single {
        from: NormalFormId,
        to: NormalFormId,
    },
    Repeated {
        from: NormalFormId,
        to: NormalFormId,
    },
}

pub trait Substitute {
    type Output;

    fn subst(self, substitution: Substitution, registry: &mut NodeRegistry) -> Self::Output;

    fn subst_all(self, substitutions: &[Substitution], registry: &mut NodeRegistry) -> Self::Output
    where
        Self: Sized + Substitute<Output = Self>,
    {
        let mut result = self;
        for &subst in substitutions {
            result = result.subst(subst, registry);
        }
        result
    }
}

impl Substitute for ExpressionId {
    type Output = Self;

    fn subst(self, substitution: Substitution, registry: &mut NodeRegistry) -> Self {
        match self {
            ExpressionId::Name(name_id) => {
                ExpressionId::Name(name_id.subst(substitution, registry))
            }
            ExpressionId::Call(call_id) => {
                ExpressionId::Call(call_id.subst(substitution, registry))
            }
            ExpressionId::Fun(fun_id) => ExpressionId::Fun(fun_id.subst(substitution, registry)),
            ExpressionId::Match(match_id) => {
                ExpressionId::Match(match_id.subst(substitution, registry))
            }
            ExpressionId::Forall(forall_id) => {
                ExpressionId::Forall(forall_id.subst(substitution, registry))
            }
        }
    }
}

impl Substitute for NodeId<NameExpression> {
    type Output = Self;

    fn subst(self, _substitution: Substitution, _registry: &mut NodeRegistry) -> Self {
        unimplemented!()
    }
}

impl Substitute for NodeId<Call> {
    type Output = Self;

    fn subst(self, _substitution: Substitution, _registry: &mut NodeRegistry) -> Self {
        unimplemented!()
    }
}

impl Substitute for NodeId<Fun> {
    type Output = Self;

    fn subst(self, _substitution: Substitution, _registry: &mut NodeRegistry) -> Self {
        unimplemented!()
    }
}

impl Substitute for NodeId<Match> {
    type Output = Self;

    fn subst(self, _substitution: Substitution, _registry: &mut NodeRegistry) -> Self {
        unimplemented!()
    }
}

impl Substitute for NodeId<Forall> {
    type Output = Self;

    fn subst(self, _substitution: Substitution, _registry: &mut NodeRegistry) -> Self {
        unimplemented!()
    }
}
