use crate::data::{
    x_light_ast::*,
    x_node_registry::{NodeId, NodeRegistry},
};

mod stripped_ast;
use stripped_ast::Strip;

// TODO: Implement Debug, PartialEq, Eq for StructuralId<T>,
// since #[derive] only works if T implements the respective traits.
#[derive(Debug, PartialEq, Eq)]
pub struct StructuralId<T> {
    pub raw: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> StructuralId<T> {
    pub fn new(raw: usize) -> Self {
        Self {
            raw,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Clone for StructuralId<T> {
    fn clone(&self) -> StructuralId<T> {
        StructuralId {
            raw: self.raw,
            _phantom: self._phantom,
        }
    }
}

impl<T> Copy for StructuralId<T> {}

impl<T> std::hash::Hash for StructuralId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct NodeStructuralIdentityRegistry {
    files: Subregistry<File>,
}

impl NodeStructuralIdentityRegistry {
    pub fn empty() -> Self {
        Self {}
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

impl ComputeStructuralIdentity for File {
    fn get_structural_id(
        id: NodeId<Self>,
        nreg: &NodeRegistry,
        sreg: &mut NodeStructuralIdentityRegistry,
    ) -> StructuralId<Self> {
        sreg.files.get_structural_id(id, nreg, sreg)
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
            node: &T,
            nreg: &NodeRegistry,
            sreg: &mut NodeStructuralIdentityRegistry,
        ) -> StructuralId<T> {
            let stripped = node.strip();

            if let Some(sid) = self.injective.get(&stripped) {
                return *sid;
            }

            let sid = StructuralId::<T>::new(self.injective.len());
            self.injective.insert(stripped, sid);

            sid
        }
    }
}
