use crate::data::light_ast::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VariantReturnTypesValidated<T>(T);

impl<T> VariantReturnTypesValidated<T> {
    pub fn unchecked_new(value: T) -> Self {
        Self(value)
    }
}

impl<T> VariantReturnTypesValidated<T> {
    pub fn raw(self) -> T {
        self.0
    }
}

#[derive(Clone, Debug)]
pub struct IllegalVariantReturnTypeError(pub ExpressionRef<'a>);
