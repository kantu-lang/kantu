use super::*;

use std::iter::FromIterator;

impl GetIndexInSubregistry for ListId<ExpressionId> {
    type Stripped = Vec<ExpressionSemanticId>;

    fn subregistry_mut(sreg: &mut StrippedRegistry) -> &mut Subregistry<Self> {
        &mut sreg.expression_lists
    }

    fn strip(self, registry: &NodeRegistry, sreg: &mut StrippedRegistry) -> Self::Stripped {
        registry
            .expression_list(self)
            .iter()
            .copied()
            .map(|id| id.into_semantic_id(registry, sreg))
            .collect()
    }
}
impl IntoSemanticId for ListId<ExpressionId> {
    type Output = SemanticId<Vec<ExpressionSemanticId>>;

    fn into_semantic_id(
        self,
        registry: &NodeRegistry,
        sreg: &mut StrippedRegistry,
    ) -> Self::Output {
        let raw = self.get_index_in_subregistry(registry, sreg);
        SemanticId::new(raw)
    }
}

impl IntoSemanticId for ExpressionId {
    type Output = ExpressionSemanticId;

    fn into_semantic_id(
        self,
        registry: &NodeRegistry,
        sreg: &mut StrippedRegistry,
    ) -> Self::Output {
        match self {
            ExpressionId::Name(id) => {
                ExpressionSemanticId::Name(id.into_semantic_id(registry, sreg))
            }
            ExpressionId::Call(id) => {
                ExpressionSemanticId::Call(id.into_semantic_id(registry, sreg))
            }
            ExpressionId::Fun(id) => ExpressionSemanticId::Fun(id.into_semantic_id(registry, sreg)),
            ExpressionId::Match(id) => {
                ExpressionSemanticId::Match(id.into_semantic_id(registry, sreg))
            }
            ExpressionId::Forall(id) => {
                ExpressionSemanticId::Forall(id.into_semantic_id(registry, sreg))
            }
        }
    }
}

fn expression_ids_to_expression_vec_semantic_id(
    ids: impl IntoIterator<Item = ExpressionId>,
    registry: &NodeRegistry,
    sreg: &mut StrippedRegistry,
) -> SemanticId<Vec<ExpressionSemanticId>> {
    let ids = ids.into_iter().collect::<Vec<_>>();
    let stripped = ids
        .iter()
        .copied()
        .map(|id| id.into_semantic_id(registry, sreg))
        .collect();
    get_semantic_id_of_expression_vec(stripped, sreg)
}

fn get_semantic_id_of_expression_vec(
    stripped: Vec<ExpressionSemanticId>,
    sreg: &mut StrippedRegistry,
) -> SemanticId<Vec<ExpressionSemanticId>> {
    if let Some(sid) = sreg.expression_lists.injective.get(&stripped).copied() {
        return SemanticId::new(sid);
    }

    let new_raw = sreg.expression_lists.injective.len();
    sreg.expression_lists.injective.insert(stripped, new_raw);
    SemanticId::new(new_raw)
}

impl GetIndexInSubregistry for NodeId<NameExpression> {
    type Stripped = stripped::NameExpression;

    fn subregistry_mut(sreg: &mut StrippedRegistry) -> &mut Subregistry<Self> {
        &mut sreg.name_expressions
    }

    fn strip(self, registry: &NodeRegistry, _sreg: &mut StrippedRegistry) -> Self::Stripped {
        let name = registry.name_expression(self);
        stripped::NameExpression {
            db_index: name.db_index,
        }
    }
}
impl IntoSemanticId for NodeId<NameExpression> {
    type Output = SemanticId<stripped::NameExpression>;

    fn into_semantic_id(
        self,
        registry: &NodeRegistry,
        sreg: &mut StrippedRegistry,
    ) -> Self::Output {
        let raw = self.get_index_in_subregistry(registry, sreg);
        SemanticId::new(raw)
    }
}

impl GetIndexInSubregistry for NodeId<Call> {
    type Stripped = stripped::Call;

    fn subregistry_mut(sreg: &mut StrippedRegistry) -> &mut Subregistry<Self> {
        &mut sreg.calls
    }

    fn strip(self, registry: &NodeRegistry, sreg: &mut StrippedRegistry) -> Self::Stripped {
        let call = registry.call(self);
        stripped::Call {
            callee_id: call.callee_id.into_semantic_id(registry, sreg),
            arg_list_id: call.arg_list_id.into_semantic_id(registry, sreg),
        }
    }
}
impl IntoSemanticId for NodeId<Call> {
    type Output = SemanticId<stripped::Call>;

    fn into_semantic_id(
        self,
        registry: &NodeRegistry,
        sreg: &mut StrippedRegistry,
    ) -> Self::Output {
        let raw = self.get_index_in_subregistry(registry, sreg);
        SemanticId::new(raw)
    }
}

impl GetIndexInSubregistry for NodeId<Fun> {
    type Stripped = stripped::Fun;

    fn subregistry_mut(sreg: &mut StrippedRegistry) -> &mut Subregistry<Self> {
        &mut sreg.funs
    }

    fn strip(self, registry: &NodeRegistry, sreg: &mut StrippedRegistry) -> Self::Stripped {
        let fun = registry.fun(self);
        stripped::Fun {
            param_type_list_id: expression_ids_to_expression_vec_semantic_id(
                registry
                    .param_list(fun.param_list_id)
                    .iter()
                    .map(|param_id| {
                        let param = registry.param(*param_id);
                        param.type_id
                    }),
                registry,
                sreg,
            ),
            dash_index: registry
                .param_list(fun.param_list_id)
                .iter()
                .position(|param_id| {
                    let param = registry.param(*param_id);
                    param.is_dashed
                }),
            return_type_id: fun.return_type_id.into_semantic_id(registry, sreg),
            body_id: fun.body_id.into_semantic_id(registry, sreg),
        }
    }
}
impl IntoSemanticId for NodeId<Fun> {
    type Output = SemanticId<stripped::Fun>;

    fn into_semantic_id(
        self,
        registry: &NodeRegistry,
        sreg: &mut StrippedRegistry,
    ) -> Self::Output {
        let raw = self.get_index_in_subregistry(registry, sreg);
        SemanticId::new(raw)
    }
}

impl GetIndexInSubregistry for NodeId<Match> {
    type Stripped = stripped::Match;

    fn subregistry_mut(sreg: &mut StrippedRegistry) -> &mut Subregistry<Self> {
        &mut sreg.matches
    }

    fn strip(self, registry: &NodeRegistry, sreg: &mut StrippedRegistry) -> Self::Stripped {
        let match_ = registry.match_(self);
        stripped::Match {
            matchee_id: match_.matchee_id.into_semantic_id(registry, sreg),
            case_list_id: match_.case_list_id.into_semantic_id(registry, sreg),
        }
    }
}
impl IntoSemanticId for NodeId<Match> {
    type Output = SemanticId<stripped::Match>;

    fn into_semantic_id(
        self,
        registry: &NodeRegistry,
        sreg: &mut StrippedRegistry,
    ) -> Self::Output {
        let raw = self.get_index_in_subregistry(registry, sreg);
        SemanticId::new(raw)
    }
}

impl GetIndexInSubregistry for ListId<NodeId<MatchCase>> {
    type Stripped = MatchCaseSemanticIdSet;

    fn subregistry_mut(sreg: &mut StrippedRegistry) -> &mut Subregistry<Self> {
        &mut sreg.match_case_lists
    }

    fn strip(self, registry: &NodeRegistry, sreg: &mut StrippedRegistry) -> Self::Stripped {
        let case_ids = registry.match_case_list(self);
        case_ids
            .iter()
            .map(|case_id| case_id.into_semantic_id(registry, sreg))
            .collect()
    }
}
impl IntoSemanticId for ListId<NodeId<MatchCase>> {
    type Output = SemanticId<stripped::Set<SemanticId<stripped::MatchCase>>>;

    fn into_semantic_id(
        self,
        registry: &NodeRegistry,
        sreg: &mut StrippedRegistry,
    ) -> Self::Output {
        let raw = self.get_index_in_subregistry(registry, sreg);
        SemanticId::new(raw)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchCaseSemanticIdSet(
    /// We internally use a sorted Vec instead of a hash set
    /// because Vecs should be faster to compare for equality
    /// than hash sets.
    Vec<SemanticId<stripped::MatchCase>>,
);

impl FromIterator<SemanticId<stripped::MatchCase>> for MatchCaseSemanticIdSet {
    fn from_iter<I: IntoIterator<Item = SemanticId<stripped::MatchCase>>>(iter: I) -> Self {
        let mut v: Vec<SemanticId<stripped_ast::MatchCase>> = iter.into_iter().collect();
        v.sort_unstable_by(|a, b| a.raw.cmp(&b.raw));
        Self(v)
    }
}

impl GetIndexInSubregistry for NodeId<MatchCase> {
    type Stripped = stripped::MatchCase;

    fn subregistry_mut(sreg: &mut StrippedRegistry) -> &mut Subregistry<Self> {
        &mut sreg.match_cases
    }

    fn strip(self, registry: &NodeRegistry, sreg: &mut StrippedRegistry) -> Self::Stripped {
        let case = registry.match_case(self);
        stripped::MatchCase {
            variant_name_id: case.variant_name_id.into_semantic_id(registry, sreg),
            output_id: case.output_id.into_semantic_id(registry, sreg),
        }
    }
}
impl IntoSemanticId for NodeId<MatchCase> {
    type Output = SemanticId<stripped::MatchCase>;

    fn into_semantic_id(
        self,
        registry: &NodeRegistry,
        sreg: &mut StrippedRegistry,
    ) -> Self::Output {
        let raw = self.get_index_in_subregistry(registry, sreg);
        SemanticId::new(raw)
    }
}

impl GetIndexInSubregistry for NodeId<Identifier> {
    type Stripped = stripped::IdentifierName;

    fn subregistry_mut(sreg: &mut StrippedRegistry) -> &mut Subregistry<Self> {
        &mut sreg.identifier_names
    }

    fn strip(self, registry: &NodeRegistry, _sreg: &mut StrippedRegistry) -> Self::Stripped {
        registry.identifier(self).name.clone()
    }
}
impl IntoSemanticId for NodeId<Identifier> {
    type Output = SemanticId<stripped::IdentifierName>;

    fn into_semantic_id(
        self,
        registry: &NodeRegistry,
        sreg: &mut StrippedRegistry,
    ) -> Self::Output {
        let raw = self.get_index_in_subregistry(registry, sreg);
        SemanticId::new(raw)
    }
}

impl GetIndexInSubregistry for NodeId<Forall> {
    type Stripped = stripped::Forall;

    fn subregistry_mut(sreg: &mut StrippedRegistry) -> &mut Subregistry<Self> {
        &mut sreg.foralls
    }

    fn strip(self, registry: &NodeRegistry, sreg: &mut StrippedRegistry) -> Self::Stripped {
        let forall = registry.forall(self);
        stripped::Forall {
            param_type_list_id: expression_ids_to_expression_vec_semantic_id(
                registry
                    .param_list(forall.param_list_id)
                    .iter()
                    .map(|param_id| {
                        let param = registry.param(*param_id);
                        param.type_id
                    }),
                registry,
                sreg,
            ),
            output_id: forall.output_id.into_semantic_id(registry, sreg),
        }
    }
}
impl IntoSemanticId for NodeId<Forall> {
    type Output = SemanticId<stripped::Forall>;

    fn into_semantic_id(
        self,
        registry: &NodeRegistry,
        sreg: &mut StrippedRegistry,
    ) -> Self::Output {
        let raw = self.get_index_in_subregistry(registry, sreg);
        SemanticId::new(raw)
    }
}
