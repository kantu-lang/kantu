use crate::data::{
    x_light_ast::*,
    x_node_registry::{ListId, NodeId, NodeRegistry},
};

use std::hash::Hash;

mod id;
pub use id::*;

mod strip;
use strip::Strip;

mod stripped_ast;

#[derive(Clone, Debug)]
pub struct NodeStructuralIdentityRegistry {
    name_expressions: Subregistry<NodeId<NameExpression>>,

    expression_lists: Subregistry<ListId<ExpressionId>>,
}

impl NodeStructuralIdentityRegistry {
    pub fn empty() -> Self {
        Self {
            name_expressions: Subregistry::new(),
            expression_lists: Subregistry::new(),
        }
    }
}

impl NodeStructuralIdentityRegistry {
    pub fn get_structural_id<T>(&mut self, input: T, registry: &NodeRegistry) -> T::Output
    where
        T: ComputeStructuralIdentity,
    {
        T::get_structural_id(input, registry, self)
    }
}

pub trait ComputeStructuralIdentity: Sized {
    type Output;

    fn get_structural_id(
        self,
        nreg: &NodeRegistry,
        sreg: &mut NodeStructuralIdentityRegistry,
    ) -> Self::Output;
}

impl ComputeStructuralIdentity for ListId<ExpressionId> {
    type Output = StructuralId<Vec<ExpressionStructuralId>>;

    fn get_structural_id(
        self,
        nreg: &NodeRegistry,
        sreg: &mut NodeStructuralIdentityRegistry,
    ) -> Self::Output {
        StructuralId::new(Subregistry::<ListId<ExpressionId>>::get_structural_id(
            self, nreg, sreg,
        ))
    }
}

impl ComputeStructuralIdentity for ExpressionId {
    type Output = ExpressionStructuralId;

    fn get_structural_id(
        self,
        nreg: &NodeRegistry,
        sreg: &mut NodeStructuralIdentityRegistry,
    ) -> Self::Output {
        match self {
            ExpressionId::Name(id) => ExpressionStructuralId::Name(StructuralId::new(
                Subregistry::<NodeId<NameExpression>>::get_structural_id(id, nreg, sreg),
            )),
            _ => unimplemented!(),
        }
    }
}

impl ComputeStructuralIdentity for NodeId<NameExpression> {
    type Output = StructuralId<NameExpression>;

    fn get_structural_id(
        self,
        nreg: &NodeRegistry,
        sreg: &mut NodeStructuralIdentityRegistry,
    ) -> Self::Output {
        let raw = Subregistry::<NodeId<NameExpression>>::get_structural_id(self, nreg, sreg);
        StructuralId::new(raw)
    }
}

use subregistry::*;
mod subregistry {
    use super::*;

    use std::fmt::Debug;

    use rustc_hash::FxHashMap;

    #[derive(Clone, Debug)]
    pub struct Subregistry<Input>
    where
        Input: Strip,
        Input::Output: Clone + Debug,
    {
        injective: FxHashMap<Input::Output, usize>,
        raw: FxHashMap<Input, usize>,
    }

    impl<Input> Subregistry<Input>
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
        Input: Strip + GetSubregistryMut + Eq + Hash,
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

    impl GetSubregistryMut for ListId<ExpressionId> {
        fn get_subreg_mut(sreg: &mut NodeStructuralIdentityRegistry) -> &mut Subregistry<Self> {
            &mut sreg.expression_lists
        }
    }

    impl GetSubregistryMut for NodeId<NameExpression> {
        fn get_subreg_mut(sreg: &mut NodeStructuralIdentityRegistry) -> &mut Subregistry<Self> {
            &mut sreg.name_expressions
        }
    }
}
