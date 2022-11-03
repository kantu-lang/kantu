use crate::data::{
    x_light_ast as with_id,
    x_node_registry::{ListId, NodeId},
    FileId, TextPosition,
};

pub trait RemoveId {
    type Output: Eq + std::hash::Hash;
    fn remove_id(&self) -> Self::Output;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct File {
    pub file_id: FileId,
    pub item_list_id: ListId<FileItemNodeId>,
}
impl RemoveId for with_id::File {
    type Output = File;
    fn remove_id(&self) -> Self::Output {
        File {
            file_id: self.file_id,
            item_list_id: self.item_list_id,
        }
    }
}

pub type FileItemNodeId = crate::data::x_node_registry::FileItemNodeId;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeStatement {
    pub name_id: NodeId<with_id::Identifier>,
    pub param_list_id: ListId<NodeId<with_id::Param>>,
    pub variant_list_id: ListId<NodeId<with_id::Variant>>,
}
impl RemoveId for with_id::TypeStatement {
    type Output = TypeStatement;
    fn remove_id(&self) -> Self::Output {
        TypeStatement {
            name_id: self.name_id,
            param_list_id: self.param_list_id,
            variant_list_id: self.variant_list_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Param {
    pub is_dashed: bool,
    pub name_id: NodeId<with_id::Identifier>,
    pub type_id: ExpressionId,
}
impl RemoveId for with_id::Param {
    type Output = Param;
    fn remove_id(&self) -> Self::Output {
        Param {
            is_dashed: self.is_dashed,
            name_id: self.name_id,
            type_id: self.type_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variant {
    pub name_id: NodeId<with_id::Identifier>,
    pub param_list_id: ListId<NodeId<with_id::Param>>,
    pub return_type_id: ExpressionId,
}
impl RemoveId for with_id::Variant {
    type Output = Variant;
    fn remove_id(&self) -> Self::Output {
        Variant {
            name_id: self.name_id,
            param_list_id: self.param_list_id,
            return_type_id: self.return_type_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LetStatement {
    pub name_id: NodeId<with_id::Identifier>,
    pub value_id: ExpressionId,
}
impl RemoveId for with_id::LetStatement {
    type Output = LetStatement;
    fn remove_id(&self) -> Self::Output {
        LetStatement {
            name_id: self.name_id,
            value_id: self.value_id,
        }
    }
}

pub type ExpressionId = crate::data::x_node_registry::ExpressionId;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NameExpression {
    pub component_list_id: ListId<NodeId<with_id::Identifier>>,
}
impl RemoveId for with_id::NameExpression {
    type Output = NameExpression;
    fn remove_id(&self) -> Self::Output {
        NameExpression {
            component_list_id: self.component_list_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Identifier {
    /// This is `None` if the identifier is either
    /// 1. a built-in identifier (e.g., `Type`)
    /// 2. an identifier that appears in compiler-generated expressions
    pub start: Option<TextPosition>,
    pub name: IdentifierName,
}
impl RemoveId for with_id::Identifier {
    type Output = Identifier;
    fn remove_id(&self) -> Self::Output {
        Identifier {
            start: self.start,
            name: self.name.clone(),
        }
    }
}

pub use crate::data::simplified_ast::IdentifierName;

pub use crate::data::simplified_ast::ReservedIdentifierName;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Call {
    pub callee_id: ExpressionId,
    pub arg_list_id: ListId<ExpressionId>,
}
impl RemoveId for with_id::Call {
    type Output = Call;
    fn remove_id(&self) -> Self::Output {
        Call {
            callee_id: self.callee_id,
            arg_list_id: self.arg_list_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Fun {
    pub name_id: NodeId<with_id::Identifier>,
    pub param_list_id: ListId<NodeId<with_id::Param>>,
    pub return_type_id: ExpressionId,
    pub body_id: ExpressionId,
}
impl RemoveId for with_id::Fun {
    type Output = Fun;
    fn remove_id(&self) -> Self::Output {
        Fun {
            name_id: self.name_id,
            param_list_id: self.param_list_id,
            return_type_id: self.return_type_id,
            body_id: self.body_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Match {
    pub matchee_id: ExpressionId,
    pub case_list_id: ListId<NodeId<with_id::MatchCase>>,
}
impl RemoveId for with_id::Match {
    type Output = Match;
    fn remove_id(&self) -> Self::Output {
        Match {
            matchee_id: self.matchee_id,
            case_list_id: self.case_list_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MatchCase {
    pub variant_name_id: NodeId<with_id::Identifier>,
    pub param_list_id: ListId<NodeId<with_id::Identifier>>,
    pub output_id: ExpressionId,
}
impl RemoveId for with_id::MatchCase {
    type Output = MatchCase;
    fn remove_id(&self) -> Self::Output {
        MatchCase {
            variant_name_id: self.variant_name_id,
            param_list_id: self.param_list_id,
            output_id: self.output_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Forall {
    pub param_list_id: ListId<NodeId<with_id::Param>>,
    pub output_id: ExpressionId,
}
impl RemoveId for with_id::Forall {
    type Output = Forall;
    fn remove_id(&self) -> Self::Output {
        Forall {
            param_list_id: self.param_list_id,
            output_id: self.output_id,
        }
    }
}