use crate::data::{
    x_light_ast::*,
    x_node_registry::{ListId, NodeId, NodeRegistry},
};

use std::{fmt::Debug, hash::Hash};

use rustc_hash::FxHashMap;

mod id;
pub use id::*;

mod impl_into_semantic_id;

mod stripped_ast;
use stripped_ast as stripped;

#[derive(Clone, Debug)]
pub struct NodeEqualityChecker(StrippedRegistry);

impl NodeEqualityChecker {
    pub fn new() -> Self {
        Self(StrippedRegistry::empty())
    }
}

impl NodeEqualityChecker {
    pub fn eq<T>(&mut self, a: T, b: T, registry: &NodeRegistry) -> bool
    where
        T: IntoSemanticId,
    {
        a.into_semantic_id(registry, &mut self.0) == b.into_semantic_id(registry, &mut self.0)
    }
}

pub trait IntoSemanticId: Copy + Eq + Hash {
    type Output: Eq + Hash;
    type Stripped: Eq + Hash;

    fn subregistry_mut(sreg: &mut StrippedRegistry) -> &mut Subregistry<Self>;

    fn strip(self, registry: &NodeRegistry, sreg: &mut StrippedRegistry) -> Self::Stripped;

    fn into_semantic_id(self, registry: &NodeRegistry, sreg: &mut StrippedRegistry)
        -> Self::Output;

    fn get_index_in_subregistry(
        self,
        registry: &NodeRegistry,
        sreg: &mut StrippedRegistry,
    ) -> usize {
        if let Some(sid) = Self::subregistry_mut(sreg).raw.get(&self) {
            return *sid;
        }

        let stripped = self.strip(registry, sreg);

        if let Some(sid) = Self::subregistry_mut(sreg)
            .injective
            .get(&stripped)
            .copied()
        {
            Self::subregistry_mut(sreg).raw.insert(self, sid);
            return sid;
        }

        let sid = Self::subregistry_mut(sreg).injective.len();
        Self::subregistry_mut(sreg).raw.insert(self, sid);
        Self::subregistry_mut(sreg).injective.insert(stripped, sid);

        sid
    }
}

#[derive(Clone, Debug)]
pub struct StrippedRegistry {
    expression_lists: Subregistry<ListId<ExpressionId>>,
    match_case_lists: Subregistry<ListId<NodeId<MatchCase>>>,
    match_cases: Subregistry<NodeId<MatchCase>>,
    identifier_names: Subregistry<NodeId<Identifier>>,

    name_expressions: Subregistry<NodeId<NameExpression>>,
    calls: Subregistry<NodeId<Call>>,
    funs: Subregistry<NodeId<Fun>>,
    matches: Subregistry<NodeId<Match>>,
    foralls: Subregistry<NodeId<Forall>>,
}

impl StrippedRegistry {
    fn empty() -> Self {
        Self {
            expression_lists: Subregistry::empty(),
            match_case_lists: Subregistry::empty(),
            match_cases: Subregistry::empty(),
            identifier_names: Subregistry::empty(),

            name_expressions: Subregistry::empty(),
            calls: Subregistry::empty(),
            funs: Subregistry::empty(),
            matches: Subregistry::empty(),
            foralls: Subregistry::empty(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Subregistry<Input>
where
    Input: IntoSemanticId,
{
    injective: FxHashMap<Input::Stripped, usize>,
    raw: FxHashMap<Input, usize>,
}

impl<Input> Subregistry<Input>
where
    Input: IntoSemanticId,
{
    fn empty() -> Self {
        Self {
            injective: FxHashMap::default(),
            raw: FxHashMap::default(),
        }
    }
}
