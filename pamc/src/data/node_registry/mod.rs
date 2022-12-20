use crate::data::{
    light_ast::*,
    non_empty_vec::{NonEmptySlice, NonEmptyVec, OptionalNonEmptyVecLen},
    TextSpan,
};

use std::{fmt::Debug, hash::Hash, num::NonZeroUsize};

use rustc_hash::FxHashMap;

use remove_id::RemoveId;
mod remove_id;

pub use node_id::*;
mod node_id;

pub use non_empty_list_id::*;
mod non_empty_list_id;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FileItemNodeId {
    Type(NodeId<TypeStatement>),
    Let(NodeId<LetStatement>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NonEmptyParamListId {
    Unlabeled(NonEmptyListId<NodeId<UnlabeledParam>>),
    UniquelyLabeled(NonEmptyListId<NodeId<LabeledParam>>),
}

impl OptionalNonEmptyVecLen for Option<NonEmptyParamListId> {
    fn len(&self) -> usize {
        self.as_ref().map(|v| v.non_zero_len().get()).unwrap_or(0)
    }
}

impl NonEmptyParamListId {
    pub fn len(&self) -> usize {
        self.non_zero_len().get()
    }

    pub fn non_zero_len(&self) -> NonZeroUsize {
        match self {
            NonEmptyParamListId::Unlabeled(vec) => vec.len,
            NonEmptyParamListId::UniquelyLabeled(vec) => vec.len,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ParamLabelId {
    Implicit,
    Explicit(NodeId<Identifier>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NonEmptyCallArgListId {
    Unlabeled(NonEmptyListId<ExpressionId>),
    UniquelyLabeled(NonEmptyListId<LabeledCallArgId>),
}

impl OptionalNonEmptyVecLen for Option<NonEmptyCallArgListId> {
    fn len(&self) -> usize {
        self.as_ref().map(|v| v.non_zero_len().get()).unwrap_or(0)
    }
}

impl NonEmptyCallArgListId {
    pub fn len(&self) -> usize {
        self.non_zero_len().get()
    }

    pub fn non_zero_len(&self) -> NonZeroUsize {
        match self {
            NonEmptyCallArgListId::Unlabeled(vec) => vec.len,
            NonEmptyCallArgListId::UniquelyLabeled(vec) => vec.len,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LabeledCallArgId {
    Implicit {
        label_id: NodeId<Identifier>,
        db_index: DbIndex,
    },
    Explicit {
        label_id: NodeId<Identifier>,
        value_id: ExpressionId,
    },
}

impl LabeledCallArgId {
    pub fn label_id(&self) -> NodeId<Identifier> {
        match self {
            LabeledCallArgId::Implicit { label_id, .. } => *label_id,
            LabeledCallArgId::Explicit { label_id, .. } => *label_id,
        }
    }

    pub fn value_id(&self, registry: &mut NodeRegistry) -> ExpressionId {
        fn dummy_id<T>() -> NodeId<T> {
            NodeId::new(0)
        }

        match *self {
            LabeledCallArgId::Implicit { label_id, db_index } => {
                let span = registry.get(label_id).span;
                let component_list_id = registry.add_list(NonEmptyVec::singleton(label_id));
                ExpressionId::Name(registry.add(NameExpression {
                    id: dummy_id(),
                    span,
                    component_list_id,
                    db_index,
                }))
            }
            LabeledCallArgId::Explicit { value_id, .. } => value_id,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NonEmptyMatchCaseParamListId {
    Unlabeled(NonEmptyListId<NodeId<Identifier>>),
    UniquelyLabeled {
        param_list_id: NonEmptyListId<NodeId<LabeledMatchCaseParam>>,
        triple_dot: Option<TextSpan>,
    },
}

impl OptionalNonEmptyVecLen for Option<NonEmptyMatchCaseParamListId> {
    fn len(&self) -> usize {
        self.as_ref().map(|v| v.non_zero_len().get()).unwrap_or(0)
    }
}

impl NonEmptyMatchCaseParamListId {
    pub fn len(&self) -> usize {
        self.non_zero_len().get()
    }

    pub fn non_zero_len(&self) -> NonZeroUsize {
        match self {
            NonEmptyMatchCaseParamListId::Unlabeled(vec) => vec.len,
            NonEmptyMatchCaseParamListId::UniquelyLabeled { param_list_id, .. } => {
                param_list_id.len
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExpressionId {
    Name(NodeId<NameExpression>),
    Call(NodeId<Call>),
    Fun(NodeId<Fun>),
    Match(NodeId<Match>),
    Forall(NodeId<Forall>),
    Check(NodeId<Check>),
}

#[derive(Clone, Copy, Debug)]
pub enum ExpressionRef<'a> {
    Name(&'a NameExpression),
    Call(&'a Call),
    Fun(&'a Fun),
    Match(&'a Match),
    Forall(&'a Forall),
    Check(&'a Check),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QuestionMarkOrPossiblyInvalidExpressionId {
    QuestionMark { span: Option<TextSpan> },
    Expression(PossiblyInvalidExpressionId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GoalKwOrPossiblyInvalidExpressionId {
    GoalKw { span: Option<TextSpan> },
    Expression(PossiblyInvalidExpressionId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PossiblyInvalidExpressionId {
    Valid(ExpressionId),
    Invalid(InvalidExpressionId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InvalidExpressionId {
    SymbolicallyInvalid(NodeId<SymbolicallyInvalidExpression>),
    IllegalFunRecursion(NodeId<IllegalFunRecursionExpression>),
}

/// For any type `T`, if `T` implements `RemoveId`, then the
/// registry guarantees that for any two `T`s `x` and `y`
/// with respective `NodeId<T>`s `x_id` and `y_id`,
/// `x.remove_id() == y.remove_id()` implies `x_id == y_id`.
#[derive(Clone, Debug)]
pub struct NodeRegistry {
    files: Subregistry<File>,
    type_statements: Subregistry<TypeStatement>,
    unlabeled_params: Subregistry<UnlabeledParam>,
    labeled_params: Subregistry<LabeledParam>,
    variants: Subregistry<Variant>,
    let_statements: Subregistry<LetStatement>,
    name_expressions: Subregistry<NameExpression>,
    calls: Subregistry<Call>,
    funs: Subregistry<Fun>,
    matches: Subregistry<Match>,
    match_cases: Subregistry<MatchCase>,
    labeled_match_case_params: Subregistry<LabeledMatchCaseParam>,
    foralls: Subregistry<Forall>,
    checks: Subregistry<Check>,
    check_assertions: Subregistry<CheckAssertion>,
    symbolically_invalid_expressions: Subregistry<SymbolicallyInvalidExpression>,
    illegal_fun_recursion_expressions: Subregistry<IllegalFunRecursionExpression>,
    identifiers: Subregistry<Identifier>,

    file_item_lists: ListSubregistry<FileItemNodeId>,
    unlabeled_param_lists: ListSubregistry<NodeId<UnlabeledParam>>,
    labeled_param_lists: ListSubregistry<NodeId<LabeledParam>>,
    variant_lists: ListSubregistry<NodeId<Variant>>,
    match_case_lists: ListSubregistry<NodeId<MatchCase>>,
    labeled_match_case_param_lists: ListSubregistry<NodeId<LabeledMatchCaseParam>>,
    check_assertion_lists: ListSubregistry<NodeId<CheckAssertion>>,
    expression_lists: ListSubregistry<ExpressionId>,
    labeled_call_arg_lists: ListSubregistry<LabeledCallArgId>,
    identifier_lists: ListSubregistry<NodeId<Identifier>>,
}

impl NodeRegistry {
    pub fn empty() -> Self {
        Self {
            files: Subregistry::new(),
            type_statements: Subregistry::new(),
            unlabeled_params: Subregistry::new(),
            labeled_params: Subregistry::new(),
            variants: Subregistry::new(),
            let_statements: Subregistry::new(),
            name_expressions: Subregistry::new(),
            calls: Subregistry::new(),
            funs: Subregistry::new(),
            matches: Subregistry::new(),
            match_cases: Subregistry::new(),
            labeled_match_case_params: Subregistry::new(),
            foralls: Subregistry::new(),
            checks: Subregistry::new(),
            check_assertions: Subregistry::new(),
            symbolically_invalid_expressions: Subregistry::new(),
            illegal_fun_recursion_expressions: Subregistry::new(),
            identifiers: Subregistry::new(),

            file_item_lists: ListSubregistry::new(),
            unlabeled_param_lists: ListSubregistry::new(),
            labeled_param_lists: ListSubregistry::new(),
            variant_lists: ListSubregistry::new(),
            match_case_lists: ListSubregistry::new(),
            labeled_match_case_param_lists: ListSubregistry::new(),
            check_assertion_lists: ListSubregistry::new(),
            expression_lists: ListSubregistry::new(),
            labeled_call_arg_lists: ListSubregistry::new(),
            identifier_lists: ListSubregistry::new(),
        }
    }
}

impl NodeRegistry {
    // TODO: Rename to `add_and_overwrite_id`
    pub fn add<T>(&mut self, item: T) -> NodeId<T>
    where
        T: RegisterableNode + SetId,
        T::Output: Clone + Debug,
    {
        T::subregistry_mut(self).add_and_overwrite_id(item)
    }

    pub fn get<T>(&self, id: NodeId<T>) -> &T
    where
        T: RegisterableNode,
        T::Output: Clone + Debug,
    {
        T::subregistry(self).get(id)
    }
}

// TODO: Move

pub use registerable_node::RegisterableNode;
mod registerable_node {
    use super::*;

    pub trait RegisterableNode
    where
        Self: Sized + RemoveId,
        Self::Output: Clone + Debug,
    {
        fn subregistry(registry: &NodeRegistry) -> &Subregistry<Self>;
        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut Subregistry<Self>;
    }

    impl RegisterableNode for File {
        fn subregistry(registry: &NodeRegistry) -> &Subregistry<Self> {
            &registry.files
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut Subregistry<Self> {
            &mut registry.files
        }
    }

    impl RegisterableNode for TypeStatement {
        fn subregistry(registry: &NodeRegistry) -> &Subregistry<Self> {
            &registry.type_statements
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut Subregistry<Self> {
            &mut registry.type_statements
        }
    }

    impl RegisterableNode for UnlabeledParam {
        fn subregistry(registry: &NodeRegistry) -> &Subregistry<Self> {
            &registry.unlabeled_params
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut Subregistry<Self> {
            &mut registry.unlabeled_params
        }
    }

    impl RegisterableNode for LabeledParam {
        fn subregistry(registry: &NodeRegistry) -> &Subregistry<Self> {
            &registry.labeled_params
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut Subregistry<Self> {
            &mut registry.labeled_params
        }
    }

    impl RegisterableNode for Variant {
        fn subregistry(registry: &NodeRegistry) -> &Subregistry<Self> {
            &registry.variants
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut Subregistry<Self> {
            &mut registry.variants
        }
    }

    impl RegisterableNode for LetStatement {
        fn subregistry(registry: &NodeRegistry) -> &Subregistry<Self> {
            &registry.let_statements
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut Subregistry<Self> {
            &mut registry.let_statements
        }
    }

    impl RegisterableNode for NameExpression {
        fn subregistry(registry: &NodeRegistry) -> &Subregistry<Self> {
            &registry.name_expressions
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut Subregistry<Self> {
            &mut registry.name_expressions
        }
    }

    impl RegisterableNode for Call {
        fn subregistry(registry: &NodeRegistry) -> &Subregistry<Self> {
            &registry.calls
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut Subregistry<Self> {
            &mut registry.calls
        }
    }

    impl RegisterableNode for Fun {
        fn subregistry(registry: &NodeRegistry) -> &Subregistry<Self> {
            &registry.funs
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut Subregistry<Self> {
            &mut registry.funs
        }
    }

    impl RegisterableNode for Match {
        fn subregistry(registry: &NodeRegistry) -> &Subregistry<Self> {
            &registry.matches
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut Subregistry<Self> {
            &mut registry.matches
        }
    }

    impl RegisterableNode for MatchCase {
        fn subregistry(registry: &NodeRegistry) -> &Subregistry<Self> {
            &registry.match_cases
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut Subregistry<Self> {
            &mut registry.match_cases
        }
    }

    impl RegisterableNode for LabeledMatchCaseParam {
        fn subregistry(registry: &NodeRegistry) -> &Subregistry<Self> {
            &registry.labeled_match_case_params
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut Subregistry<Self> {
            &mut registry.labeled_match_case_params
        }
    }

    impl RegisterableNode for Forall {
        fn subregistry(registry: &NodeRegistry) -> &Subregistry<Self> {
            &registry.foralls
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut Subregistry<Self> {
            &mut registry.foralls
        }
    }

    impl RegisterableNode for Check {
        fn subregistry(registry: &NodeRegistry) -> &Subregistry<Self> {
            &registry.checks
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut Subregistry<Self> {
            &mut registry.checks
        }
    }

    impl RegisterableNode for CheckAssertion {
        fn subregistry(registry: &NodeRegistry) -> &Subregistry<Self> {
            &registry.check_assertions
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut Subregistry<Self> {
            &mut registry.check_assertions
        }
    }

    impl RegisterableNode for SymbolicallyInvalidExpression {
        fn subregistry(registry: &NodeRegistry) -> &Subregistry<Self> {
            &registry.symbolically_invalid_expressions
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut Subregistry<Self> {
            &mut registry.symbolically_invalid_expressions
        }
    }

    impl RegisterableNode for IllegalFunRecursionExpression {
        fn subregistry(registry: &NodeRegistry) -> &Subregistry<Self> {
            &registry.illegal_fun_recursion_expressions
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut Subregistry<Self> {
            &mut registry.illegal_fun_recursion_expressions
        }
    }

    impl RegisterableNode for Identifier {
        fn subregistry(registry: &NodeRegistry) -> &Subregistry<Self> {
            &registry.identifiers
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut Subregistry<Self> {
            &mut registry.identifiers
        }
    }
}

impl NodeRegistry {
    pub fn add_list<T>(&mut self, list: NonEmptyVec<T>) -> NonEmptyListId<T>
    where
        T: RegisterableList + Clone + Eq + Hash,
    {
        T::subregistry_mut(self).add(list)
    }

    pub fn add_possibly_empty_list<T, L>(&mut self, list: L) -> Option<NonEmptyListId<T>>
    where
        T: RegisterableList + Clone + Eq + Hash,
        L: IntoOptionalNonEmptyVec<T>,
    {
        list.into_optional_non_empty_vec()
            .map(|list| T::subregistry_mut(self).add(list))
    }

    pub fn get_list<T>(&self, id: NonEmptyListId<T>) -> NonEmptySlice<'_, T>
    where
        T: RegisterableList + Clone + Eq + Hash,
    {
        T::subregistry(self).get(id)
    }

    pub fn get_possibly_empty_list<T>(&self, id: Option<NonEmptyListId<T>>) -> &[T]
    where
        T: RegisterableList + Clone + Eq + Hash,
    {
        if let Some(id) = id {
            self.get_list(id).into()
        } else {
            &[]
        }
    }
}

pub trait IntoOptionalNonEmptyVec<T> {
    fn into_optional_non_empty_vec(self) -> Option<NonEmptyVec<T>>;
}

impl<T> IntoOptionalNonEmptyVec<T> for Option<NonEmptyVec<T>> {
    fn into_optional_non_empty_vec(self) -> Option<NonEmptyVec<T>> {
        self
    }
}

impl<T> IntoOptionalNonEmptyVec<T> for Vec<T> {
    fn into_optional_non_empty_vec(self) -> Option<NonEmptyVec<T>> {
        NonEmptyVec::try_from(self).ok()
    }
}

// TODO: Move
pub use registerable_list::RegisterableList;
mod registerable_list {
    use super::*;

    pub trait RegisterableList: Sized {
        fn subregistry(registry: &NodeRegistry) -> &ListSubregistry<Self>;
        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut ListSubregistry<Self>;
    }

    impl RegisterableList for FileItemNodeId {
        fn subregistry(registry: &NodeRegistry) -> &ListSubregistry<Self> {
            &registry.file_item_lists
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut ListSubregistry<Self> {
            &mut registry.file_item_lists
        }
    }

    impl RegisterableList for NodeId<UnlabeledParam> {
        fn subregistry(registry: &NodeRegistry) -> &ListSubregistry<Self> {
            &registry.unlabeled_param_lists
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut ListSubregistry<Self> {
            &mut registry.unlabeled_param_lists
        }
    }

    impl RegisterableList for NodeId<LabeledParam> {
        fn subregistry(registry: &NodeRegistry) -> &ListSubregistry<Self> {
            &registry.labeled_param_lists
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut ListSubregistry<Self> {
            &mut registry.labeled_param_lists
        }
    }

    impl RegisterableList for NodeId<Variant> {
        fn subregistry(registry: &NodeRegistry) -> &ListSubregistry<Self> {
            &registry.variant_lists
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut ListSubregistry<Self> {
            &mut registry.variant_lists
        }
    }

    impl RegisterableList for NodeId<MatchCase> {
        fn subregistry(registry: &NodeRegistry) -> &ListSubregistry<Self> {
            &registry.match_case_lists
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut ListSubregistry<Self> {
            &mut registry.match_case_lists
        }
    }

    impl RegisterableList for NodeId<LabeledMatchCaseParam> {
        fn subregistry(registry: &NodeRegistry) -> &ListSubregistry<Self> {
            &registry.labeled_match_case_param_lists
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut ListSubregistry<Self> {
            &mut registry.labeled_match_case_param_lists
        }
    }

    impl RegisterableList for NodeId<CheckAssertion> {
        fn subregistry(registry: &NodeRegistry) -> &ListSubregistry<Self> {
            &registry.check_assertion_lists
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut ListSubregistry<Self> {
            &mut registry.check_assertion_lists
        }
    }

    impl RegisterableList for ExpressionId {
        fn subregistry(registry: &NodeRegistry) -> &ListSubregistry<Self> {
            &registry.expression_lists
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut ListSubregistry<Self> {
            &mut registry.expression_lists
        }
    }

    impl RegisterableList for LabeledCallArgId {
        fn subregistry(registry: &NodeRegistry) -> &ListSubregistry<Self> {
            &registry.labeled_call_arg_lists
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut ListSubregistry<Self> {
            &mut registry.labeled_call_arg_lists
        }
    }

    impl RegisterableList for NodeId<Identifier> {
        fn subregistry(registry: &NodeRegistry) -> &ListSubregistry<Self> {
            &registry.identifier_lists
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut ListSubregistry<Self> {
            &mut registry.identifier_lists
        }
    }
}

impl NodeRegistry {
    pub fn expression_ref(&self, id: ExpressionId) -> ExpressionRef<'_> {
        match id {
            ExpressionId::Name(id) => ExpressionRef::Name(self.get(id)),
            ExpressionId::Call(id) => ExpressionRef::Call(self.get(id)),
            ExpressionId::Fun(id) => ExpressionRef::Fun(self.get(id)),
            ExpressionId::Match(id) => ExpressionRef::Match(self.get(id)),
            ExpressionId::Forall(id) => ExpressionRef::Forall(self.get(id)),
            ExpressionId::Check(id) => ExpressionRef::Check(self.get(id)),
        }
    }
}

use subregistry::*;
mod subregistry {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct Subregistry<T>
    where
        T: RemoveId,
        T::Output: Clone + Debug,
    {
        items: Vec<T>,
        ids: FxHashMap<T::Output, NodeId<T>>,
    }

    impl<T> Subregistry<T>
    where
        T: RemoveId,
        T::Output: Clone + Debug,
    {
        pub fn new() -> Self {
            Self {
                items: Vec::new(),
                ids: FxHashMap::default(),
            }
        }
    }

    impl<T> Subregistry<T>
    where
        T: RemoveId,
        T::Output: Clone + Debug,
    {
        pub fn get(&self, id: NodeId<T>) -> &T {
            &self.items[id.raw]
        }
    }

    impl<T> Subregistry<T>
    where
        T: RemoveId + SetId,
        T::Output: Clone + Debug,
    {
        pub fn add_and_overwrite_id(&mut self, mut item: T) -> NodeId<T> {
            if let Some(existing_id) = self.ids.get(&item.remove_id()) {
                *existing_id
            } else {
                let new_id = NodeId::<T>::new(self.items.len());
                item.set_id(new_id);
                self.ids.insert(item.remove_id(), new_id);
                self.items.push(item);
                new_id
            }
        }
    }
}

use list_subregistry::*;
mod list_subregistry {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct ListSubregistry<T> {
        flattened_items: Vec<T>,
        ids: FxHashMap<NonEmptyVec<T>, NonEmptyListId<T>>,
    }

    impl<T> ListSubregistry<T> {
        pub fn new() -> Self {
            Self {
                flattened_items: Vec::new(),
                ids: FxHashMap::default(),
            }
        }
    }

    impl<T> ListSubregistry<T> {
        pub fn get(&self, id: NonEmptyListId<T>) -> NonEmptySlice<'_, T> {
            NonEmptySlice::new(&self.flattened_items, id.start, id.len)
        }
    }

    impl<T> ListSubregistry<T>
    where
        T: Clone + Eq + std::hash::Hash,
    {
        pub fn add(&mut self, list: NonEmptyVec<T>) -> NonEmptyListId<T> {
            if let Some(existing_id) = self.ids.get(&list) {
                *existing_id
            } else {
                let new_id =
                    NonEmptyListId::<T>::new(self.flattened_items.len(), list.non_zero_len());
                self.flattened_items.extend(list.iter().cloned());
                self.ids.insert(list, new_id);
                new_id
            }
        }
    }
}

use set_id::*;
mod set_id {
    use super::*;

    pub trait SetId: Sized {
        fn set_id(&mut self, id: NodeId<Self>);
    }

    impl SetId for File {
        fn set_id(&mut self, id: NodeId<Self>) {
            self.id = id;
        }
    }

    impl SetId for TypeStatement {
        fn set_id(&mut self, id: NodeId<Self>) {
            self.id = id;
        }
    }

    impl SetId for UnlabeledParam {
        fn set_id(&mut self, id: NodeId<Self>) {
            self.id = id;
        }
    }

    impl SetId for LabeledParam {
        fn set_id(&mut self, id: NodeId<Self>) {
            self.id = id;
        }
    }

    impl SetId for Variant {
        fn set_id(&mut self, id: NodeId<Self>) {
            self.id = id;
        }
    }

    impl SetId for LetStatement {
        fn set_id(&mut self, id: NodeId<Self>) {
            self.id = id;
        }
    }

    impl SetId for NameExpression {
        fn set_id(&mut self, id: NodeId<Self>) {
            self.id = id;
        }
    }

    impl SetId for Identifier {
        fn set_id(&mut self, id: NodeId<Self>) {
            self.id = id;
        }
    }

    impl SetId for Call {
        fn set_id(&mut self, id: NodeId<Self>) {
            self.id = id;
        }
    }

    impl SetId for Fun {
        fn set_id(&mut self, id: NodeId<Self>) {
            self.id = id;
        }
    }

    impl SetId for Match {
        fn set_id(&mut self, id: NodeId<Self>) {
            self.id = id;
        }
    }

    impl SetId for MatchCase {
        fn set_id(&mut self, id: NodeId<Self>) {
            self.id = id;
        }
    }

    impl SetId for LabeledMatchCaseParam {
        fn set_id(&mut self, id: NodeId<Self>) {
            self.id = id;
        }
    }

    impl SetId for Forall {
        fn set_id(&mut self, id: NodeId<Self>) {
            self.id = id;
        }
    }

    impl SetId for Check {
        fn set_id(&mut self, id: NodeId<Self>) {
            self.id = id;
        }
    }

    impl SetId for CheckAssertion {
        fn set_id(&mut self, id: NodeId<Self>) {
            self.id = id;
        }
    }

    impl SetId for SymbolicallyInvalidExpression {
        fn set_id(&mut self, id: NodeId<Self>) {
            self.id = id;
        }
    }

    impl SetId for IllegalFunRecursionExpression {
        fn set_id(&mut self, id: NodeId<Self>) {
            self.id = id;
        }
    }
}
