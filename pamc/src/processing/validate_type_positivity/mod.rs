use crate::data::{
    fun_recursion_validation_result::FunRecursionValidated,
    light_ast::*,
    node_registry::{NodeId, NodeRegistry},
    type_positivity_validation_result::*,
};

use std::convert::Infallible;

use context::*;
mod context;

type TaintedTypePositivityError = Tainted<TypePositivityError>;

impl From<Tainted<Infallible>> for TaintedTypePositivityError {
    fn from(impossible: Tainted<Infallible>) -> Self {
        #[allow(unreachable_code)]
        match Infallible::from(impossible) {}
    }
}

pub fn validate_type_positivity_in_file(
    registry: &NodeRegistry,
    file_id: FunRecursionValidated<NodeId<File>>,
) -> Result<TypePositivityValidated<NodeId<File>>, TypePositivityError> {
    unimplemented!()
}

fn dummy_id<T>() -> NodeId<T> {
    NodeId::new(0)
}
