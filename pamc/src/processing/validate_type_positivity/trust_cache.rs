use super::*;

use rustc_hash::FxHashSet;

#[derive(Debug, Clone)]
pub struct TrustCache {
    trusted_type_params: FxHashSet<TypeParam>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TypeParam {
    pub type_id: NodeId<TypeStatement>,
    pub param_index: usize,
}

impl TrustCache {
    pub fn empty() -> Self {
        Self {
            trusted_type_params: FxHashSet::default(),
        }
    }
}

impl TrustCache {
    pub fn trust(&mut self, param: TypeParam) {
        self.trusted_type_params.insert(param);
    }

    pub fn is_trusted(&self, param: TypeParam) -> bool {
        self.trusted_type_params.contains(&param)
    }
}
