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
    name_expressions: Subregistry<NodeId<NameExpression>, StructuralId<NameExpression>>,
}

impl NodeStructuralIdentityRegistry {
    pub fn empty() -> Self {
        Self {
            name_expressions: Subregistry::new(),
        }
    }
}

impl NodeStructuralIdentityRegistry {
    pub fn get_structural_id<T: ComputeStructuralIdentity<Output = StructuralId<T>>>(
        &mut self,
        id: NodeId<T>,
        registry: &NodeRegistry,
    ) -> StructuralId<T> {
        T::get_structural_id(id, registry, self)
    }
}

pub trait ComputeStructuralIdentity: Sized {
    type Output;

    fn get_structural_id(
        id: NodeId<Self>,
        nreg: &NodeRegistry,
        sreg: &mut NodeStructuralIdentityRegistry,
    ) -> Self::Output;
}

impl ComputeStructuralIdentity for ExpressionId {
    type Output = ExpressionStructuralId;

    fn get_structural_id(
        id: NodeId<Self>,
        nreg: &NodeRegistry,
        sreg: &mut NodeStructuralIdentityRegistry,
    ) -> ExpressionStructuralId {
        Subregistry::<ExpressionId, ExpressionStructuralId>::get_structural_id(
            id,
            nreg.name_expression(id),
            nreg,
            sreg,
        )
    }
}

impl ComputeStructuralIdentity for NameExpression {
    type Output = StructuralId<Self>;

    fn get_structural_id(
        id: NodeId<Self>,
        nreg: &NodeRegistry,
        sreg: &mut NodeStructuralIdentityRegistry,
    ) -> StructuralId<Self> {
        Subregistry::<NameExpression>::get_structural_id(id, nreg.name_expression(id), nreg, sreg)
    }
}

use subregistry::*;
mod subregistry {
    use super::*;

    use std::fmt::Debug;

    use rustc_hash::FxHashMap;

    #[derive(Clone, Debug)]
    pub struct Subregistry<Input, Output>
    where
        Input: Strip,
        Input::Output: Clone + Debug,
    {
        injective: FxHashMap<Input::Output, Output>,
        raw: FxHashMap<Input, Output>,
    }

    impl<Input, Output> Subregistry<Input, Output>
    where
        Input: Strip,
        Input::Output: Clone + Debug,
    {
        pub fn new() -> Self {
            Self {
                injective: FxHashMap::default(),
                raw: FxHashMap::default(),
            }
        }
    }

    impl<Input> Subregistry<Input>
    where
        Input: Strip + GetSubregistryMut,
        Input::Output: Clone + Debug,
    {
        pub fn get_structural_id(
            input: Input,
            nreg: &NodeRegistry,
            sreg: &mut NodeStructuralIdentityRegistry,
        ) -> usize {
            if let Some(sid) = Input::get_subreg_mut(sreg).raw.get(&input) {
                return *sid;
            }

            let stripped = input.strip(nreg, sreg);

            if let Some(sid) = Input::get_subreg_mut(sreg).injective.get(&stripped) {
                return *sid;
            }

            let sid = Input::get_subreg_mut(sreg).injective.len();
            Input::get_subreg_mut(sreg).injective.insert(stripped, sid);

            sid
        }
    }

    pub trait GetSubregistryMut
    where
        Self: Sized + Strip,
        Self::Output: Clone + Debug,
    {
        fn get_subreg_mut(sreg: &mut NodeStructuralIdentityRegistry) -> &mut Subregistry<Self>;
    }

    impl GetSubregistryMut for NameExpression {
        fn get_subreg_mut(sreg: &mut NodeStructuralIdentityRegistry) -> &mut Subregistry<Self> {
            &mut sreg.name_expressions
        }
    }
}
