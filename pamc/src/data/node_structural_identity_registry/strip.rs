use crate::data::{
    node_structural_identity_registry::{stripped_ast::*, NodeStructuralIdentityRegistry},
    x_light_ast as nonstripped,
    x_node_registry::NodeRegistry,
};

pub trait Strip {
    type Output: Eq + std::hash::Hash;

    fn strip(&self, nreg: &NodeRegistry, sreg: &mut NodeStructuralIdentityRegistry)
        -> Self::Output;
}

impl Strip for nonstripped::NameExpression {
    type Output = NameExpression;

    fn strip(
        &self,
        nreg: &NodeRegistry,
        sreg: &mut NodeStructuralIdentityRegistry,
    ) -> Self::Output {
        NameExpression {
            db_index: self.db_index,
        }
    }
}
