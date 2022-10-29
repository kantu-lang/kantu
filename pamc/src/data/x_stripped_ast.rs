use crate::data::{
    x_light_ast as decorated,
    x_node_registry::{ListId, NodeId},
    FileId,
};

pub trait Strip {
    type Output: Eq + std::hash::Hash;
    fn strip(&self) -> Self::Output;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct File {
    pub file_id: FileId,
}
impl Strip for decorated::File {
    type Output = File;
    fn strip(&self) -> Self::Output {
        File {
            file_id: self.file_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeStatement {
    pub param_list_id: ListId<NodeId<decorated::Param>>,
    pub variant_list_id: ListId<NodeId<decorated::Variant>>,
}
impl Strip for decorated::TypeStatement {
    type Output = TypeStatement;
    fn strip(&self) -> Self::Output {
        TypeStatement {
            param_list_id: self.param_list_id,
            variant_list_id: self.variant_list_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Param {
    pub is_dashed: bool,
    pub type_id: ExpressionId,
}
impl Strip for decorated::Param {
    type Output = Param;
    fn strip(&self) -> Self::Output {
        Param {
            is_dashed: self.is_dashed,
            type_id: self.type_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variant {
    pub param_list_id: ListId<NodeId<decorated::Param>>,
    pub return_type_id: ExpressionId,
}
impl Strip for decorated::Variant {
    type Output = Variant;
    fn strip(&self) -> Self::Output {
        Variant {
            param_list_id: self.param_list_id,
            return_type_id: self.return_type_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LetStatement {
    pub value_id: ExpressionId,
}
impl Strip for decorated::LetStatement {
    type Output = LetStatement;
    fn strip(&self) -> Self::Output {
        LetStatement {
            value_id: self.value_id,
        }
    }
}

pub type ExpressionId = crate::data::x_node_registry::ExpressionId;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NameExpression {
    /// De Bruijn index (zero-based).
    pub db_index: DbIndex,
}
impl Strip for decorated::NameExpression {
    type Output = NameExpression;
    fn strip(&self) -> Self::Output {
        NameExpression {
            db_index: self.db_index,
        }
    }
}

pub use crate::data::bound_ast::{DbIndex, DbLevel};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Call {
    pub callee_id: ExpressionId,
    pub arg_list_id: ListId<ExpressionId>,
}
impl Strip for decorated::Call {
    type Output = Call;
    fn strip(&self) -> Self::Output {
        Call {
            callee_id: self.callee_id,
            arg_list_id: self.arg_list_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Fun {
    pub param_list_id: ListId<NodeId<decorated::Param>>,
    pub return_type_id: ExpressionId,
    pub body_id: ExpressionId,
}
impl Strip for decorated::Fun {
    type Output = Fun;
    fn strip(&self) -> Self::Output {
        Fun {
            param_list_id: self.param_list_id,
            return_type_id: self.return_type_id,
            body_id: self.body_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Match {
    pub matchee_id: ExpressionId,
    pub case_list_id: ListId<NodeId<decorated::MatchCase>>,
}
impl Strip for decorated::Match {
    type Output = Match;
    fn strip(&self) -> Self::Output {
        Match {
            matchee_id: self.matchee_id,
            case_list_id: self.case_list_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MatchCase {
    pub param_list_len: usize,
    pub output_id: ExpressionId,
}
impl Strip for decorated::MatchCase {
    type Output = MatchCase;
    fn strip(&self) -> Self::Output {
        MatchCase {
            param_list_len: self.param_list_id.len,
            output_id: self.output_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Forall {
    pub param_list_id: ListId<NodeId<decorated::Param>>,
    pub output_id: ExpressionId,
}
impl Strip for decorated::Forall {
    type Output = Forall;
    fn strip(&self) -> Self::Output {
        Forall {
            param_list_id: self.param_list_id,
            output_id: self.output_id,
        }
    }
}
