use crate::data::{
    node_structural_identity_registry::{stripped_ast::*, NodeStructuralIdentityRegistry},
    x_light_ast as nonstripped,
    x_node_registry::{NodeId, NodeRegistry},
};

pub trait Strip {
    type Output: Eq + std::hash::Hash;

    fn strip(&self, nreg: &NodeRegistry, sreg: &mut NodeStructuralIdentityRegistry)
        -> Self::Output;
}

impl Strip for NodeId<nonstripped::NameExpression> {
    type Output = NameExpression;

    fn strip(
        &self,
        nreg: &NodeRegistry,
        _sreg: &mut NodeStructuralIdentityRegistry,
    ) -> Self::Output {
        let name = nreg.name_expression(*self);
        NameExpression {
            db_index: name.db_index,
        }
    }
}

impl Strip for NodeId<nonstripped::Call> {
    type Output = Call;

    fn strip(
        &self,
        nreg: &NodeRegistry,
        sreg: &mut NodeStructuralIdentityRegistry,
    ) -> Self::Output {
        let call = nreg.call(*self);
        // Call {
        //     callee_id: sreg.get_structural_id(call.callee_id, nreg),
        //     arg_list_id: sreg.get_structural_id(call.arg_list_id, nreg),
        // }
        unimplemented!()
    }
}
