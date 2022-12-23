use crate::data::{light_ast::*, node_registry::NodeId};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TypePositivityValidated<T>(T);

impl<T> TypePositivityValidated<T> {
    pub fn unchecked_new(value: T) -> Self {
        Self(value)
    }
}

impl<T> TypePositivityValidated<T> {
    pub fn raw(self) -> T {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TypePositivityError {
    ExpectedTypeGotFun(NodeId<Fun>),
    NonAdtCallee {
        call_id: NodeId<Call>,
        callee_id: ExpressionId,
    },
    IllegalVariableAppearance(NodeId<NameExpression>),
    VariantReturnTypeArityMismatch {
        actual: usize,
        expected: usize,
    },
    VariantReturnTypeHadNonNameElement {
        variant_id: NodeId<Variant>,
        type_arg_index: usize,
    },
}