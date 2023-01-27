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
pub enum TypePositivityError<'a> {
    ExpectedTypeGotFun(&'a Fun<'a>),
    NonAdtCallee {
        call_id: &'a Call<'a>,
        callee_id: ExpressionRef<'a>,
    },
    IllegalVariableAppearance(&'a NameExpression<'a>),
    VariantReturnTypeTypeArgArityMismatch {
        return_type_id: ExpressionRef<'a>,
        actual: usize,
        expected: usize,
    },
    VariantReturnTypeHadNonNameTypeArg {
        variant_id: &'a Variant<'a>,
        type_arg_index: usize,
    },
}
