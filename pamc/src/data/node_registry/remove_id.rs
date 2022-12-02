use crate::data::{
    bind_error::BindError,
    illegal_fun_recursion_error::IllegalFunRecursionError,
    light_ast as with_id,
    node_registry::{ListId, NodeId, QuestionMarkOrPossiblyInvalidExpressionId},
    simplified_ast as unbound, FileId, TextPosition,
};

pub trait RemoveId {
    type Output: Eq + std::hash::Hash + AddId<Output = Self>;
    fn remove_id(&self) -> Self::Output;
}

/// This trait isn't actually meant to be used.
/// We just require it as a trait bound on `RemoveId::Output`
/// to ensure that the only information lost is the id.
/// In the past, bugs were caused when other fields were omitted
/// when removing the ids, which caused nodes that were not equal
/// (even modulo id) to be incorrectly considered equal (modulo id).
pub trait AddId {
    type Output;
    fn add_id(&self, id: NodeId<Self::Output>) -> Self::Output;
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
impl AddId for File {
    type Output = with_id::File;
    fn add_id(&self, id: NodeId<Self::Output>) -> Self::Output {
        with_id::File {
            file_id: self.file_id,
            id,
            item_list_id: self.item_list_id,
        }
    }
}

pub type FileItemNodeId = crate::data::node_registry::FileItemNodeId;

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
impl AddId for TypeStatement {
    type Output = with_id::TypeStatement;
    fn add_id(&self, id: NodeId<Self::Output>) -> Self::Output {
        with_id::TypeStatement {
            id,
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
impl AddId for Param {
    type Output = with_id::Param;
    fn add_id(&self, id: NodeId<Self::Output>) -> Self::Output {
        with_id::Param {
            id,
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
impl AddId for Variant {
    type Output = with_id::Variant;
    fn add_id(&self, id: NodeId<Self::Output>) -> Self::Output {
        with_id::Variant {
            id,
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
impl AddId for LetStatement {
    type Output = with_id::LetStatement;
    fn add_id(&self, id: NodeId<Self::Output>) -> Self::Output {
        with_id::LetStatement {
            id,
            name_id: self.name_id,
            value_id: self.value_id,
        }
    }
}

pub type ExpressionId = crate::data::node_registry::ExpressionId;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NameExpression {
    pub component_list_id: ListId<NodeId<with_id::Identifier>>,
    /// De Bruijn index (zero-based).
    pub db_index: DbIndex,
}
impl RemoveId for with_id::NameExpression {
    type Output = NameExpression;
    fn remove_id(&self) -> Self::Output {
        NameExpression {
            component_list_id: self.component_list_id,
            db_index: self.db_index,
        }
    }
}
impl AddId for NameExpression {
    type Output = with_id::NameExpression;
    fn add_id(&self, id: NodeId<Self::Output>) -> Self::Output {
        with_id::NameExpression {
            id,
            component_list_id: self.component_list_id,
            db_index: self.db_index,
        }
    }
}

pub use crate::data::bound_ast::{DbIndex, DbLevel};

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
impl AddId for Identifier {
    type Output = with_id::Identifier;
    fn add_id(&self, id: NodeId<Self::Output>) -> Self::Output {
        with_id::Identifier {
            id,
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
impl AddId for Call {
    type Output = with_id::Call;
    fn add_id(&self, id: NodeId<Self::Output>) -> Self::Output {
        with_id::Call {
            id,
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
    pub skip_type_checking_body: bool,
}
impl RemoveId for with_id::Fun {
    type Output = Fun;
    fn remove_id(&self) -> Self::Output {
        Fun {
            name_id: self.name_id,
            param_list_id: self.param_list_id,
            return_type_id: self.return_type_id,
            body_id: self.body_id,
            skip_type_checking_body: self.skip_type_checking_body,
        }
    }
}
impl AddId for Fun {
    type Output = with_id::Fun;
    fn add_id(&self, id: NodeId<Self::Output>) -> Self::Output {
        with_id::Fun {
            id,
            name_id: self.name_id,
            param_list_id: self.param_list_id,
            return_type_id: self.return_type_id,
            body_id: self.body_id,
            skip_type_checking_body: self.skip_type_checking_body,
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
impl AddId for Match {
    type Output = with_id::Match;
    fn add_id(&self, id: NodeId<Self::Output>) -> Self::Output {
        with_id::Match {
            id,
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
impl AddId for MatchCase {
    type Output = with_id::MatchCase;
    fn add_id(&self, id: NodeId<Self::Output>) -> Self::Output {
        with_id::MatchCase {
            id,
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
impl AddId for Forall {
    type Output = with_id::Forall;
    fn add_id(&self, id: NodeId<Self::Output>) -> Self::Output {
        with_id::Forall {
            id,
            param_list_id: self.param_list_id,
            output_id: self.output_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Check {
    pub checkee_annotation_id: with_id::CheckeeAnnotationId,
    pub output_id: ExpressionId,
}
impl RemoveId for with_id::Check {
    type Output = Check;
    fn remove_id(&self) -> Self::Output {
        Check {
            checkee_annotation_id: self.checkee_annotation_id,
            output_id: self.output_id,
        }
    }
}
impl AddId for Check {
    type Output = with_id::Check;
    fn add_id(&self, id: NodeId<Self::Output>) -> Self::Output {
        with_id::Check {
            id,
            checkee_annotation_id: self.checkee_annotation_id,
            output_id: self.output_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GoalCheckeeAnnotation {
    pub goal_kw_position: TextPosition,
    pub checkee_type_id: QuestionMarkOrPossiblyInvalidExpressionId,
}
impl RemoveId for with_id::GoalCheckeeAnnotation {
    type Output = GoalCheckeeAnnotation;
    fn remove_id(&self) -> Self::Output {
        GoalCheckeeAnnotation {
            goal_kw_position: self.goal_kw_position,
            checkee_type_id: self.checkee_type_id,
        }
    }
}
impl AddId for GoalCheckeeAnnotation {
    type Output = with_id::GoalCheckeeAnnotation;
    fn add_id(&self, id: NodeId<Self::Output>) -> Self::Output {
        with_id::GoalCheckeeAnnotation {
            id,
            goal_kw_position: self.goal_kw_position,
            checkee_type_id: self.checkee_type_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExpressionCheckeeAnnotation {
    pub checkee_id: ExpressionId,
    pub checkee_type_id: QuestionMarkOrPossiblyInvalidExpressionId,
    pub checkee_value_id: Option<QuestionMarkOrPossiblyInvalidExpressionId>,
}
impl RemoveId for with_id::ExpressionCheckeeAnnotation {
    type Output = ExpressionCheckeeAnnotation;
    fn remove_id(&self) -> Self::Output {
        ExpressionCheckeeAnnotation {
            checkee_id: self.checkee_id,
            checkee_type_id: self.checkee_type_id,
            checkee_value_id: self.checkee_value_id,
        }
    }
}
impl AddId for ExpressionCheckeeAnnotation {
    type Output = with_id::ExpressionCheckeeAnnotation;
    fn add_id(&self, id: NodeId<Self::Output>) -> Self::Output {
        with_id::ExpressionCheckeeAnnotation {
            id,
            checkee_id: self.checkee_id,
            checkee_type_id: self.checkee_type_id,
            checkee_value_id: self.checkee_value_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnbindableExpression {
    pub expression: unbound::Expression,
    pub error: BindError,
}
impl RemoveId for with_id::UnbindableExpression {
    type Output = UnbindableExpression;
    fn remove_id(&self) -> Self::Output {
        UnbindableExpression {
            expression: self.expression.clone(),
            error: self.error.clone(),
        }
    }
}
impl AddId for UnbindableExpression {
    type Output = with_id::UnbindableExpression;
    fn add_id(&self, id: NodeId<Self::Output>) -> Self::Output {
        with_id::UnbindableExpression {
            id,
            expression: self.expression.clone(),
            error: self.error.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IllegalFunRecursionExpression {
    pub expression_id: with_id::ExpressionId,
    pub error: IllegalFunRecursionError,
}
impl RemoveId for with_id::IllegalFunRecursionExpression {
    type Output = IllegalFunRecursionExpression;
    fn remove_id(&self) -> Self::Output {
        IllegalFunRecursionExpression {
            expression_id: self.expression_id.clone(),
            error: self.error.clone(),
        }
    }
}
impl AddId for IllegalFunRecursionExpression {
    type Output = with_id::IllegalFunRecursionExpression;
    fn add_id(&self, id: NodeId<Self::Output>) -> Self::Output {
        with_id::IllegalFunRecursionExpression {
            id,
            expression_id: self.expression_id.clone(),
            error: self.error.clone(),
        }
    }
}
