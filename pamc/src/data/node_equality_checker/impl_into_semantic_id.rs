use super::*;

use std::iter::FromIterator;

impl GetIndexInSubregistry for NonEmptyListId<ExpressionId> {
    type Stripped = Vec<ExpressionSemanticId>;

    fn subregistry_mut(sreg: &mut StrippedRegistry) -> &mut Subregistry<Self> {
        &mut sreg.expression_lists
    }

    fn strip(self, registry: &NodeRegistry, sreg: &mut StrippedRegistry) -> Self::Stripped {
        registry
            .get_list(self)
            .iter()
            .copied()
            .map(|id| id.into_semantic_id(registry, sreg))
            .collect()
    }
}
impl IntoSemanticId for NonEmptyListId<ExpressionId> {
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
            ExpressionId::Check(id) => {
                // Since check annotations carry no semantic meaning,
                // we can look exclusively at the `output`.
                let check = registry.get(id);
                check.output_id.into_semantic_id(registry, sreg)
            }
        }
    }
}

impl GetIndexInSubregistry for NodeId<NameExpression> {
    type Stripped = stripped::NameExpression;

    fn subregistry_mut(sreg: &mut StrippedRegistry) -> &mut Subregistry<Self> {
        &mut sreg.name_expressions
    }

    fn strip(self, registry: &NodeRegistry, _sreg: &mut StrippedRegistry) -> Self::Stripped {
        let name = registry.get(self);
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
        let call = registry.get(self);
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

impl IntoSemanticId for NonEmptyCallArgListId {
    type Output = NonEmptyCallArgListSemanticId;

    fn into_semantic_id(
        self,
        registry: &NodeRegistry,
        sreg: &mut StrippedRegistry,
    ) -> Self::Output {
        match self {
            NonEmptyCallArgListId::Unlabeled(id) => {
                NonEmptyCallArgListSemanticId::Unlabeled(id.into_semantic_id(registry, sreg))
            }
            NonEmptyCallArgListId::UniquelyLabeled(id) => {
                NonEmptyCallArgListSemanticId::UniquelyLabeled(id.into_semantic_id(registry, sreg))
            }
        }
    }
}

impl GetIndexInSubregistry for NonEmptyListId<LabeledCallArgId> {
    type Stripped = LabeledCallArgSemanticIdSet;

    fn subregistry_mut(sreg: &mut StrippedRegistry) -> &mut Subregistry<Self> {
        &mut sreg.labeled_call_arg_lists
    }

    fn strip(self, registry: &NodeRegistry, sreg: &mut StrippedRegistry) -> Self::Stripped {
        let arg_ids = registry.get_list(self);
        arg_ids
            .iter()
            .map(|arg_id| arg_id.into_semantic_id(registry, sreg))
            .collect()
    }
}
impl IntoSemanticId for NonEmptyListId<LabeledCallArgId> {
    type Output = SemanticId<stripped::Set<LabeledCallArgSemanticId>>;

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
pub struct LabeledCallArgSemanticIdSet(
    /// We internally use a sorted Vec instead of a hash set
    /// because Vecs should be faster to compare for equality
    /// than hash sets.
    Vec<LabeledCallArgSemanticId>,
);

impl FromIterator<LabeledCallArgSemanticId> for LabeledCallArgSemanticIdSet {
    fn from_iter<I: IntoIterator<Item = LabeledCallArgSemanticId>>(iter: I) -> Self {
        let mut v: Vec<LabeledCallArgSemanticId> = iter.into_iter().collect();
        v.sort_unstable_by(|a, b| a.cmp(&b));
        Self(v)
    }
}

impl GetIndexInSubregistry for LabeledCallArgId {
    type Stripped = LabeledCallArgSemanticId;

    fn subregistry_mut(sreg: &mut StrippedRegistry) -> &mut Subregistry<Self> {
        &mut sreg.labeled_call_args
    }

    fn strip(self, registry: &NodeRegistry, sreg: &mut StrippedRegistry) -> Self::Stripped {
        match self {
            LabeledCallArgId::Implicit {
                label_id: value_id,
                db_index,
            } => LabeledCallArgSemanticId::Implicit {
                label_id: value_id.into_semantic_id(registry, sreg),
                db_index,
            },
            LabeledCallArgId::Explicit { label_id, value_id } => {
                LabeledCallArgSemanticId::Explicit {
                    label_id: label_id.into_semantic_id(registry, sreg),
                    value_id: value_id.into_semantic_id(registry, sreg),
                }
            }
        }
    }
}
impl IntoSemanticId for LabeledCallArgId {
    type Output = LabeledCallArgSemanticId;

    fn into_semantic_id(
        self,
        registry: &NodeRegistry,
        sreg: &mut StrippedRegistry,
    ) -> Self::Output {
        self.strip(registry, sreg)
    }
}

impl GetIndexInSubregistry for NodeId<Fun> {
    type Stripped = stripped::Fun;

    fn subregistry_mut(sreg: &mut StrippedRegistry) -> &mut Subregistry<Self> {
        &mut sreg.funs
    }

    fn strip(self, registry: &NodeRegistry, sreg: &mut StrippedRegistry) -> Self::Stripped {
        let fun = registry.get(self);
        stripped::Fun {
            // TODO: Properly handle possibly labeled params
            param_list_id: fun.param_list_id.into_semantic_id(registry, sreg),
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

impl IntoSemanticId for NonEmptyParamListId {
    type Output = NonEmptyParamListSemanticId;

    fn into_semantic_id(
        self,
        registry: &NodeRegistry,
        sreg: &mut StrippedRegistry,
    ) -> Self::Output {
        match self {
            NonEmptyParamListId::Unlabeled(id) => {
                NonEmptyParamListSemanticId::Unlabeled(id.into_semantic_id(registry, sreg))
            }
            NonEmptyParamListId::UniquelyLabeled(id) => {
                NonEmptyParamListSemanticId::UniquelyLabeled(id.into_semantic_id(registry, sreg))
            }
        }
    }
}

impl GetIndexInSubregistry for NonEmptyListId<NodeId<UnlabeledParam>> {
    type Stripped = Vec<SemanticId<stripped::UnlabeledParam>>;

    fn subregistry_mut(sreg: &mut StrippedRegistry) -> &mut Subregistry<Self> {
        &mut sreg.unlabeled_param_lists
    }

    fn strip(self, registry: &NodeRegistry, sreg: &mut StrippedRegistry) -> Self::Stripped {
        let param_ids = registry.get_list(self);
        param_ids
            .iter()
            .map(|param_ids| param_ids.into_semantic_id(registry, sreg))
            .collect()
    }
}
impl IntoSemanticId for NonEmptyListId<NodeId<UnlabeledParam>> {
    type Output = SemanticId<Vec<SemanticId<stripped::UnlabeledParam>>>;

    fn into_semantic_id(
        self,
        registry: &NodeRegistry,
        sreg: &mut StrippedRegistry,
    ) -> Self::Output {
        let raw = self.get_index_in_subregistry(registry, sreg);
        SemanticId::new(raw)
    }
}

impl GetIndexInSubregistry for NodeId<UnlabeledParam> {
    type Stripped = stripped::UnlabeledParam;

    fn subregistry_mut(sreg: &mut StrippedRegistry) -> &mut Subregistry<Self> {
        &mut sreg.unlabeled_params
    }

    fn strip(self, registry: &NodeRegistry, sreg: &mut StrippedRegistry) -> Self::Stripped {
        let param = registry.get(self);
        stripped::UnlabeledParam {
            is_dashed: param.is_dashed,
            type_id: param.type_id.into_semantic_id(registry, sreg),
        }
    }
}
impl IntoSemanticId for NodeId<UnlabeledParam> {
    type Output = SemanticId<stripped::UnlabeledParam>;

    fn into_semantic_id(
        self,
        registry: &NodeRegistry,
        sreg: &mut StrippedRegistry,
    ) -> Self::Output {
        let raw = self.get_index_in_subregistry(registry, sreg);
        SemanticId::new(raw)
    }
}

impl GetIndexInSubregistry for NonEmptyListId<NodeId<LabeledParam>> {
    type Stripped = Vec<SemanticId<stripped::LabeledParam>>;

    fn subregistry_mut(sreg: &mut StrippedRegistry) -> &mut Subregistry<Self> {
        &mut sreg.labeled_param_lists
    }

    fn strip(self, registry: &NodeRegistry, sreg: &mut StrippedRegistry) -> Self::Stripped {
        let param_ids = registry.get_list(self);
        param_ids
            .iter()
            .map(|param_ids| param_ids.into_semantic_id(registry, sreg))
            .collect()
    }
}
impl IntoSemanticId for NonEmptyListId<NodeId<LabeledParam>> {
    type Output = SemanticId<Vec<SemanticId<stripped::LabeledParam>>>;

    fn into_semantic_id(
        self,
        registry: &NodeRegistry,
        sreg: &mut StrippedRegistry,
    ) -> Self::Output {
        let raw = self.get_index_in_subregistry(registry, sreg);
        SemanticId::new(raw)
    }
}

impl GetIndexInSubregistry for NodeId<LabeledParam> {
    type Stripped = stripped::LabeledParam;

    fn subregistry_mut(sreg: &mut StrippedRegistry) -> &mut Subregistry<Self> {
        &mut sreg.labeled_params
    }

    fn strip(self, registry: &NodeRegistry, sreg: &mut StrippedRegistry) -> Self::Stripped {
        let param = registry.get(self);
        stripped::LabeledParam {
            label_name_id: param.label_identifier_id().into_semantic_id(registry, sreg),
            is_dashed: param.is_dashed,
            type_id: param.type_id.into_semantic_id(registry, sreg),
        }
    }
}
impl IntoSemanticId for NodeId<LabeledParam> {
    type Output = SemanticId<stripped::LabeledParam>;

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
        let match_ = registry.get(self);
        stripped::Match {
            matchee_id: match_.matchee_id.into_semantic_id(registry, sreg),
            case_list_id: match_
                .case_list_id
                .map(|case_list_id| case_list_id.into_semantic_id(registry, sreg)),
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

impl GetIndexInSubregistry for NonEmptyListId<NodeId<MatchCase>> {
    type Stripped = MatchCaseSemanticIdSet;

    fn subregistry_mut(sreg: &mut StrippedRegistry) -> &mut Subregistry<Self> {
        &mut sreg.match_case_lists
    }

    fn strip(self, registry: &NodeRegistry, sreg: &mut StrippedRegistry) -> Self::Stripped {
        let case_ids = registry.get_list(self);
        case_ids
            .iter()
            .map(|case_id| case_id.into_semantic_id(registry, sreg))
            .collect()
    }
}
impl IntoSemanticId for NonEmptyListId<NodeId<MatchCase>> {
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
        let case = registry.get(self);
        // TODO: Track explicit param count
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
        registry.get(self).name.clone()
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
        let forall = registry.get(self);
        stripped::Forall {
            // TODO: Properly handle possibly labeled params
            param_list_id: forall.param_list_id.into_semantic_id(registry, sreg),
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
