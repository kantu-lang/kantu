use crate::data::{
    bind_error::BindError,
    fun_recursion_validation_result::IllegalFunRecursionError,
    light_ast as with_id,
    node_registry::{
        CheckAssertionId, GoalKwOrExpressionId, ListId, NodeId,
        QuestionMarkOrPossiblyInvalidExpressionId,
    },
    simplified_ast as unbound, FileId, TextSpan,
};

// TODO: We could probably greatly simplify this by just making a
// generic `WithNormalizedId<T: SetId>` struct, and then
// set the id to zero (or some other constant) in `WithNormalizedId::new()`.

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
    pub span: TextSpan,
    pub file_id: FileId,
    pub item_list_id: ListId<FileItemNodeId>,
}
impl RemoveId for with_id::File {
    type Output = File;
    fn remove_id(&self) -> Self::Output {
        File {
            span: self.span,
            file_id: self.file_id,
            item_list_id: self.item_list_id,
        }
    }
}
impl AddId for File {
    type Output = with_id::File;
    fn add_id(&self, id: NodeId<Self::Output>) -> Self::Output {
        with_id::File {
            span: self.span,
            file_id: self.file_id,
            id,
            item_list_id: self.item_list_id,
        }
    }
}

pub type FileItemNodeId = crate::data::node_registry::FileItemNodeId;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeStatement {
    pub span: TextSpan,
    pub name_id: NodeId<with_id::Identifier>,
    pub param_list_id: ListId<NodeId<with_id::Param>>,
    pub variant_list_id: ListId<NodeId<with_id::Variant>>,
}
impl RemoveId for with_id::TypeStatement {
    type Output = TypeStatement;
    fn remove_id(&self) -> Self::Output {
        TypeStatement {
            span: self.span,
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
            span: self.span,
            name_id: self.name_id,
            param_list_id: self.param_list_id,
            variant_list_id: self.variant_list_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Param {
    pub span: TextSpan,
    pub is_dashed: bool,
    pub name_id: NodeId<with_id::Identifier>,
    pub type_id: ExpressionId,
}
impl RemoveId for with_id::Param {
    type Output = Param;
    fn remove_id(&self) -> Self::Output {
        Param {
            span: self.span,
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
            span: self.span,
            is_dashed: self.is_dashed,
            name_id: self.name_id,
            type_id: self.type_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variant {
    pub span: TextSpan,
    pub name_id: NodeId<with_id::Identifier>,
    pub param_list_id: ListId<NodeId<with_id::Param>>,
    pub return_type_id: ExpressionId,
}
impl RemoveId for with_id::Variant {
    type Output = Variant;
    fn remove_id(&self) -> Self::Output {
        Variant {
            span: self.span,
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
            span: self.span,
            name_id: self.name_id,
            param_list_id: self.param_list_id,
            return_type_id: self.return_type_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LetStatement {
    pub span: TextSpan,
    pub name_id: NodeId<with_id::Identifier>,
    pub value_id: ExpressionId,
}
impl RemoveId for with_id::LetStatement {
    type Output = LetStatement;
    fn remove_id(&self) -> Self::Output {
        LetStatement {
            span: self.span,
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
            span: self.span,
            name_id: self.name_id,
            value_id: self.value_id,
        }
    }
}

pub type ExpressionId = crate::data::node_registry::ExpressionId;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NameExpression {
    pub span: TextSpan,
    pub component_list_id: ListId<NodeId<with_id::Identifier>>,
    /// De Bruijn index (zero-based).
    pub db_index: DbIndex,
}
impl RemoveId for with_id::NameExpression {
    type Output = NameExpression;
    fn remove_id(&self) -> Self::Output {
        NameExpression {
            span: self.span,
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
            span: self.span,
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
    pub span: Option<TextSpan>,
    pub name: IdentifierName,
}
impl RemoveId for with_id::Identifier {
    type Output = Identifier;
    fn remove_id(&self) -> Self::Output {
        Identifier {
            span: self.span,
            name: self.name.clone(),
        }
    }
}
impl AddId for Identifier {
    type Output = with_id::Identifier;
    fn add_id(&self, id: NodeId<Self::Output>) -> Self::Output {
        with_id::Identifier {
            id,
            span: self.span,
            name: self.name.clone(),
        }
    }
}

pub use crate::data::simplified_ast::IdentifierName;

pub use crate::data::simplified_ast::ReservedIdentifierName;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Call {
    pub span: TextSpan,
    pub callee_id: ExpressionId,
    pub arg_list_id: ListId<ExpressionId>,
}
impl RemoveId for with_id::Call {
    type Output = Call;
    fn remove_id(&self) -> Self::Output {
        Call {
            span: self.span,
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
            span: self.span,
            callee_id: self.callee_id,
            arg_list_id: self.arg_list_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Fun {
    pub span: TextSpan,
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
            span: self.span,
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
            span: self.span,
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
    pub span: TextSpan,
    pub matchee_id: ExpressionId,
    pub case_list_id: ListId<NodeId<with_id::MatchCase>>,
}
impl RemoveId for with_id::Match {
    type Output = Match;
    fn remove_id(&self) -> Self::Output {
        Match {
            span: self.span,
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
            span: self.span,
            matchee_id: self.matchee_id,
            case_list_id: self.case_list_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MatchCase {
    pub span: TextSpan,
    pub variant_name_id: NodeId<with_id::Identifier>,
    pub param_list_id: ListId<NodeId<with_id::Identifier>>,
    pub output_id: ExpressionId,
}
impl RemoveId for with_id::MatchCase {
    type Output = MatchCase;
    fn remove_id(&self) -> Self::Output {
        MatchCase {
            span: self.span,
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
            span: self.span,
            variant_name_id: self.variant_name_id,
            param_list_id: self.param_list_id,
            output_id: self.output_id,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Forall {
    pub span: TextSpan,
    pub param_list_id: ListId<NodeId<with_id::Param>>,
    pub output_id: ExpressionId,
}
impl RemoveId for with_id::Forall {
    type Output = Forall;
    fn remove_id(&self) -> Self::Output {
        Forall {
            span: self.span,
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
            span: self.span,
            param_list_id: self.param_list_id,
            output_id: self.output_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Check {
    pub span: TextSpan,
    pub assertion_list_id: ListId<CheckAssertionId>,
    pub output_id: ExpressionId,
}
impl RemoveId for with_id::Check {
    type Output = Check;
    fn remove_id(&self) -> Self::Output {
        Check {
            span: self.span,
            assertion_list_id: self.assertion_list_id,
            output_id: self.output_id,
        }
    }
}
impl AddId for Check {
    type Output = with_id::Check;
    fn add_id(&self, id: NodeId<Self::Output>) -> Self::Output {
        with_id::Check {
            id,
            span: self.span,
            assertion_list_id: self.assertion_list_id,
            output_id: self.output_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeAssertion {
    pub span: TextSpan,
    pub left_id: ExpressionId,
    pub right_id: QuestionMarkOrPossiblyInvalidExpressionId,
}
impl RemoveId for with_id::TypeAssertion {
    type Output = TypeAssertion;
    fn remove_id(&self) -> Self::Output {
        TypeAssertion {
            span: self.span,
            left_id: self.left_id,
            right_id: self.right_id,
        }
    }
}
impl AddId for TypeAssertion {
    type Output = with_id::TypeAssertion;
    fn add_id(&self, id: NodeId<Self::Output>) -> Self::Output {
        with_id::TypeAssertion {
            id,
            span: self.span,
            left_id: self.left_id,
            right_id: self.right_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NormalFormAssertion {
    pub span: TextSpan,
    pub left_id: GoalKwOrExpressionId,
    pub right_id: QuestionMarkOrPossiblyInvalidExpressionId,
}
impl RemoveId for with_id::NormalFormAssertion {
    type Output = NormalFormAssertion;
    fn remove_id(&self) -> Self::Output {
        NormalFormAssertion {
            span: self.span,
            left_id: self.left_id,
            right_id: self.right_id,
        }
    }
}
impl AddId for NormalFormAssertion {
    type Output = with_id::NormalFormAssertion;
    fn add_id(&self, id: NodeId<Self::Output>) -> Self::Output {
        with_id::NormalFormAssertion {
            id,
            span: self.span,
            left_id: self.left_id,
            right_id: self.right_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SymbolicallyInvalidExpression {
    pub span: TextSpan,
    pub expression: unbound::Expression,
    pub error: BindError,
}
impl RemoveId for with_id::SymbolicallyInvalidExpression {
    type Output = SymbolicallyInvalidExpression;
    fn remove_id(&self) -> Self::Output {
        SymbolicallyInvalidExpression {
            span: self.span,
            expression: self.expression.clone(),
            error: self.error.clone(),
        }
    }
}
impl AddId for SymbolicallyInvalidExpression {
    type Output = with_id::SymbolicallyInvalidExpression;
    fn add_id(&self, id: NodeId<Self::Output>) -> Self::Output {
        with_id::SymbolicallyInvalidExpression {
            id,
            span: self.span,
            expression: self.expression.clone(),
            error: self.error.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IllegalFunRecursionExpression {
    pub span: TextSpan,
    pub expression_id: with_id::ExpressionId,
    pub error: IllegalFunRecursionError,
}
impl RemoveId for with_id::IllegalFunRecursionExpression {
    type Output = IllegalFunRecursionExpression;
    fn remove_id(&self) -> Self::Output {
        IllegalFunRecursionExpression {
            span: self.span,
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
            span: self.span,
            expression_id: self.expression_id.clone(),
            error: self.error.clone(),
        }
    }
}
