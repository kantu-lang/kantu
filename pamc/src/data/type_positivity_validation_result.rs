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
    VariantReturnTypeTypeArgArityMismatch {
        return_type_id: ExpressionId,
        actual: usize,
        expected: usize,
    },
    VariantReturnTypeHadNonNameTypeArg {
        variant_id: NodeId<Variant>,
        type_arg_index: usize,
    },
}
