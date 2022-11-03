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
        _nreg: &NodeRegistry,
        _sreg: &mut NodeStructuralIdentityRegistry,
    ) -> Self::Output {
        NameExpression {
            db_index: self.db_index,
        }
    }
}

impl Strip for nonstripped::Call {
    type Output = Call;

    fn strip(
        &self,
        nreg: &NodeRegistry,
        sreg: &mut NodeStructuralIdentityRegistry,
    ) -> Self::Output {
        Call {
            callee_id: nreg.get_structural_id(self.callee_id, sreg),
            arg_list_id: nreg.get_structural_id(self.arg_list_id, sreg),
        }
    }
}
