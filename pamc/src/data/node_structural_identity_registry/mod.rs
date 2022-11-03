use crate::data::{
    x_light_ast::*,
    x_node_registry::{NodeId, NodeRegistry},
};

mod id;
pub use id::*;

mod strip;
use strip::Strip;

mod stripped_ast;

#[derive(Clone, Debug)]
pub struct NodeStructuralIdentityRegistry {
    name_expressions: Subregistry<NameExpression>,
}

impl NodeStructuralIdentityRegistry {
    pub fn empty() -> Self {
        Self {
            name_expressions: Subregistry::new(),
        }
    }
}

impl NodeStructuralIdentityRegistry {
    pub fn get_structural_id<T: ComputeStructuralIdentity>(
        &mut self,
        id: NodeId<T>,
        registry: &NodeRegistry,
    ) -> StructuralId<T> {
        T::get_structural_id(id, registry, self)
    }
}

pub trait ComputeStructuralIdentity: Sized {
    fn get_structural_id(
        id: NodeId<Self>,
        nreg: &NodeRegistry,
        sreg: &mut NodeStructuralIdentityRegistry,
    ) -> StructuralId<Self>;
}

impl ComputeStructuralIdentity for NameExpression {
    fn get_structural_id(
        id: NodeId<Self>,
        nreg: &NodeRegistry,
        sreg: &mut NodeStructuralIdentityRegistry,
    ) -> StructuralId<Self> {
        sreg.name_expressions
            .get_structural_id(id, nreg.name_expression(id), nreg, sreg)
    }
}

use subregistry::*;
mod subregistry {
    use super::*;

    use std::fmt::Debug;

    use rustc_hash::FxHashMap;

    #[derive(Clone, Debug)]
    pub struct Subregistry<T>
    where
        T: Strip,
        T::Output: Clone + Debug,
    {
        injective: FxHashMap<T::Output, StructuralId<T>>,
        raw: FxHashMap<NodeId<T>, StructuralId<T>>,
    }

    impl<T> Subregistry<T>
    where
        T: Strip,
        T::Output: Clone + Debug,
    {
        pub fn new() -> Self {
            Self {
                injective: FxHashMap::default(),
                raw: FxHashMap::default(),
            }
        }
    }

    impl<T> Subregistry<T>
    where
        T: Strip,
        T::Output: Clone + Debug,
        NodeId<T>: Eq,
    {
        pub fn get_structural_id(
            &mut self,
            id: NodeId<T>,
            node: &T,
            nreg: &NodeRegistry,
            sreg: &mut NodeStructuralIdentityRegistry,
        ) -> StructuralId<T> {
            if let Some(sid) = self.raw.get(&id) {
                return *sid;
            }

            let stripped = node.strip(nreg, sreg);

            if let Some(sid) = self.injective.get(&stripped) {
                return *sid;
            }

            let sid = StructuralId::<T>::new(self.injective.len());
            self.injective.insert(stripped, sid);

            sid
        }
    }
}
