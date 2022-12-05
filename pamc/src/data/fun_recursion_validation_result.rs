use crate::data::{light_ast::*, node_registry::NodeId};

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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum IllegalFunRecursionError {
    RecursiveReferenceWasNotDirectCall {
        reference: NodeId<NameExpression>,
    },
    NonSubstructPassedToDecreasingParam {
        callee: NodeId<NameExpression>,
        arg: ExpressionId,
    },
    RecursivelyCalledFunctionWithoutDecreasingParam {
        callee: NodeId<NameExpression>,
    },
}
