use super::*;

pub trait WithoutSpans {
    fn without_spans(self, registry: &mut NodeRegistry) -> Self;
}

impl WithoutSpans for NodeId<Param> {
    fn without_spans(self, _: &mut NodeRegistry) -> Self {
        unimplemented!()
    }
}

impl WithoutSpans for NormalFormId {
    fn without_spans(self, registry: &mut NodeRegistry) -> Self {
        NormalFormId::unchecked_new(self.raw().without_spans(registry))
    }
}

impl WithoutSpans for ExpressionId {
    fn without_spans(self, registry: &mut NodeRegistry) -> Self {
        match self {
            ExpressionId::Name(id) => ExpressionId::Name(id.without_spans(registry)),
            ExpressionId::Call(id) => ExpressionId::Call(id.without_spans(registry)),
            ExpressionId::Fun(id) => ExpressionId::Fun(id.without_spans(registry)),
            ExpressionId::Match(id) => ExpressionId::Match(id.without_spans(registry)),
            ExpressionId::Forall(id) => ExpressionId::Forall(id.without_spans(registry)),
            ExpressionId::Check(id) => ExpressionId::Check(id.without_spans(registry)),
        }
    }
}

impl WithoutSpans for NodeId<NameExpression> {
    fn without_spans(self, _: &mut NodeRegistry) -> Self {
        unimplemented!()
    }
}

impl WithoutSpans for NodeId<Call> {
    fn without_spans(self, _: &mut NodeRegistry) -> Self {
        unimplemented!()
    }
}

impl WithoutSpans for NodeId<Fun> {
    fn without_spans(self, _: &mut NodeRegistry) -> Self {
        unimplemented!()
    }
}

impl WithoutSpans for NodeId<Match> {
    fn without_spans(self, _: &mut NodeRegistry) -> Self {
        unimplemented!()
    }
}

impl WithoutSpans for NodeId<Forall> {
    fn without_spans(self, _: &mut NodeRegistry) -> Self {
        unimplemented!()
    }
}

impl WithoutSpans for NodeId<Check> {
    fn without_spans(self, _: &mut NodeRegistry) -> Self {
        unimplemented!()
    }
}
