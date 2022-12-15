use super::*;

pub trait WithoutSpans {
    fn without_spans(self, registry: &mut NodeRegistry) -> Self;
}

impl WithoutSpans for ListId<NodeId<Param>> {
    fn without_spans(self, registry: &mut NodeRegistry) -> Self {
        let original = registry.get_list(self).to_vec();
        let new = original
            .into_iter()
            .map(|id| id.without_spans(registry))
            .collect();
        registry.add_list(new)
    }
}

impl WithoutSpans for NodeId<Param> {
    fn without_spans(self, registry: &mut NodeRegistry) -> Self {
        let original = registry.get(self).clone();
        let name_id = original.name_id.without_spans(registry);
        let type_id = original.type_id.without_spans(registry);
        registry.add(Param {
            id: dummy_id(),
            span: None,
            is_dashed: original.is_dashed,
            name_id,
            type_id,
        })
    }
}

impl WithoutSpans for NodeId<Identifier> {
    fn without_spans(self, registry: &mut NodeRegistry) -> Self {
        let original = registry.get(self).clone();
        registry.add(Identifier {
            id: dummy_id(),
            span: None,
            name: original.name.clone(),
        })
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
    fn without_spans(self, registry: &mut NodeRegistry) -> Self {
        let original = registry.get(self).clone();
        let component_list_id = original.component_list_id.without_spans(registry);
        registry.add(NameExpression {
            id: dummy_id(),
            span: None,
            component_list_id,
            db_index: original.db_index,
        })
    }
}

impl WithoutSpans for ListId<NodeId<Identifier>> {
    fn without_spans(self, registry: &mut NodeRegistry) -> Self {
        let original = registry.get_list(self).to_vec();
        let new = original
            .into_iter()
            .map(|id| id.without_spans(registry))
            .collect();
        registry.add_list(new)
    }
}

impl WithoutSpans for NodeId<Call> {
    fn without_spans(self, registry: &mut NodeRegistry) -> Self {
        let original = registry.get(self).clone();
        let callee_id = original.callee_id.without_spans(registry);
        let arg_list_id = original.arg_list_id.without_spans(registry);
        registry.add(Call {
            id: dummy_id(),
            span: None,
            callee_id,
            arg_list_id,
        })
    }
}

impl WithoutSpans for ListId<ExpressionId> {
    fn without_spans(self, registry: &mut NodeRegistry) -> Self {
        let original = registry.get_list(self).to_vec();
        let new = original
            .into_iter()
            .map(|id| id.without_spans(registry))
            .collect();
        registry.add_list(new)
    }
}

impl WithoutSpans for NodeId<Fun> {
    fn without_spans(self, registry: &mut NodeRegistry) -> Self {
        let original = registry.get(self).clone();
        let name_id = original.name_id.without_spans(registry);
        let param_list_id = original.param_list_id.without_spans(registry);
        let return_type_id = original.return_type_id.without_spans(registry);
        let body_id = original.body_id.without_spans(registry);
        registry.add(Fun {
            id: dummy_id(),
            span: None,
            name_id,
            param_list_id,
            return_type_id,
            body_id,
            skip_type_checking_body: original.skip_type_checking_body,
        })
    }
}

impl WithoutSpans for NodeId<Match> {
    fn without_spans(self, registry: &mut NodeRegistry) -> Self {
        let original = registry.get(self).clone();
        let matchee_id = original.matchee_id.without_spans(registry);
        let case_list_id = original.case_list_id.without_spans(registry);
        registry.add(Match {
            id: dummy_id(),
            span: None,
            matchee_id,
            case_list_id,
        })
    }
}

impl WithoutSpans for ListId<NodeId<MatchCase>> {
    fn without_spans(self, registry: &mut NodeRegistry) -> Self {
        let original = registry.get_list(self).to_vec();
        let new = original
            .into_iter()
            .map(|id| id.without_spans(registry))
            .collect();
        registry.add_list(new)
    }
}

impl WithoutSpans for NodeId<MatchCase> {
    fn without_spans(self, registry: &mut NodeRegistry) -> Self {
        let original = registry.get(self).clone();
        let variant_name_id = original.variant_name_id.without_spans(registry);
        let param_list_id = original.param_list_id.without_spans(registry);
        let output_id = original.output_id.without_spans(registry);
        registry.add(MatchCase {
            id: dummy_id(),
            span: None,
            variant_name_id,
            param_list_id,
            output_id,
        })
    }
}

impl WithoutSpans for NodeId<Forall> {
    fn without_spans(self, registry: &mut NodeRegistry) -> Self {
        let original = registry.get(self).clone();
        let param_list_id = original.param_list_id.without_spans(registry);
        let output_id = original.output_id.without_spans(registry);
        registry.add(Forall {
            id: dummy_id(),
            span: None,
            param_list_id,
            output_id,
        })
    }
}

impl WithoutSpans for NodeId<Check> {
    fn without_spans(self, check: &mut NodeRegistry) -> Self {
        let original = check.check(self).clone();
        let assertion_list_id = original.assertion_list_id.without_spans(check);
        let output_id = original.output_id.without_spans(check);
        check.add_check_and_overwrite_its_id(Check {
            id: dummy_id(),
            span: None,
            assertion_list_id,
            output_id,
        })
    }
}

impl WithoutSpans for ListId<NodeId<CheckAssertion>> {
    fn without_spans(self, registry: &mut NodeRegistry) -> Self {
        let original = registry.get_list(self).to_vec();
        let new = original
            .into_iter()
            .map(|id| id.without_spans(registry))
            .collect();
        registry.add_list(new)
    }
}

impl WithoutSpans for NodeId<CheckAssertion> {
    fn without_spans(self, registry: &mut NodeRegistry) -> Self {
        let original = registry.get(self).clone();
        let left_id = original.left_id.without_spans(registry);
        let right_id = original.right_id.without_spans(registry);
        registry.add(CheckAssertion {
            id: dummy_id(),
            span: None,
            kind: original.kind,
            left_id,
            right_id,
        })
    }
}

impl WithoutSpans for GoalKwOrPossiblyInvalidExpressionId {
    fn without_spans(self, registry: &mut NodeRegistry) -> Self {
        match self {
            GoalKwOrPossiblyInvalidExpressionId::GoalKw { .. } => {
                GoalKwOrPossiblyInvalidExpressionId::GoalKw { span: None }
            }
            GoalKwOrPossiblyInvalidExpressionId::Expression(id) => {
                GoalKwOrPossiblyInvalidExpressionId::Expression(id.without_spans(registry))
            }
        }
    }
}

impl WithoutSpans for QuestionMarkOrPossiblyInvalidExpressionId {
    fn without_spans(self, registry: &mut NodeRegistry) -> Self {
        match self {
            QuestionMarkOrPossiblyInvalidExpressionId::QuestionMark { .. } => {
                QuestionMarkOrPossiblyInvalidExpressionId::QuestionMark { span: None }
            }
            QuestionMarkOrPossiblyInvalidExpressionId::Expression(id) => {
                QuestionMarkOrPossiblyInvalidExpressionId::Expression(id.without_spans(registry))
            }
        }
    }
}

impl WithoutSpans for PossiblyInvalidExpressionId {
    fn without_spans(self, registry: &mut NodeRegistry) -> Self {
        match self {
            PossiblyInvalidExpressionId::Invalid(expression) => {
                PossiblyInvalidExpressionId::Invalid(expression)
            }
            PossiblyInvalidExpressionId::Valid(id) => {
                PossiblyInvalidExpressionId::Valid(id.without_spans(registry))
            }
        }
    }
}
