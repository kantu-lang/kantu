use crate::data::light_ast::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FunRecursionValidated<T>(T);

impl<T> FunRecursionValidated<T> {
    pub fn unchecked_new(value: T) -> Self {
        Self(value)
    }
}

impl<T> FunRecursionValidated<T> {
    pub fn raw(self) -> T {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IllegalFunRecursionError<'a> {
    RecursiveReferenceWasNotDirectCall {
        reference_id: &'a NameExpression<'a>,
    },
    NonSubstructPassedToDecreasingParam {
        callee_id: &'a NameExpression<'a>,
        arg_id: &'a ExpressionRef<'a>,
    },
    RecursivelyCalledFunctionWithoutDecreasingParam {
        callee_id: &'a NameExpression<'a>,
    },
    LabelednessMismatch(&'a Call<'a>),
}
