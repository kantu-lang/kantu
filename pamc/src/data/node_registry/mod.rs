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
    check_assertion_lists: ListSubregistry<NodeId<CheckAssertion>>,
    expression_lists: ListSubregistry<ExpressionId>,
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
            check_assertion_lists: ListSubregistry::new(),
            expression_lists: ListSubregistry::new(),
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

// TODO: Delete

// impl NodeRegistry {
//     pub fn add_file_and_overwrite_its_id(&mut self, file: File) -> NodeId<File> {
//         self.files.add_and_overwrite_id(file)
//     }

//     pub fn add_type_statement_and_overwrite_its_id(
//         &mut self,
//         type_statement: TypeStatement,
//     ) -> NodeId<TypeStatement> {
//         self.type_statements.add_and_overwrite_id(type_statement)
//     }

//     pub fn add_param_and_overwrite_its_id(&mut self, param: Param) -> NodeId<Param> {
//         self.params.add_and_overwrite_id(param)
//     }

//     pub fn add_variant_and_overwrite_its_id(&mut self, variant: Variant) -> NodeId<Variant> {
//         self.variants.add_and_overwrite_id(variant)
//     }

//     pub fn add_let_statement_and_overwrite_its_id(
//         &mut self,
//         let_statement: LetStatement,
//     ) -> NodeId<LetStatement> {
//         self.let_statements.add_and_overwrite_id(let_statement)
//     }

//     /// Panics if the provided name expression has zero components.
//     pub fn add_name_expression_and_overwrite_its_id(
//         &mut self,
//         name: NameExpression,
//     ) -> NodeId<NameExpression> {
//         if name.component_list_id.len == 0 {
//             panic!("NameExpression must have at least one component");
//         }

//         self.name_expressions.add_and_overwrite_id(name)
//     }

//     pub fn add_call_and_overwrite_its_id(&mut self, call: Call) -> NodeId<Call> {
//         self.calls.add_and_overwrite_id(call)
//     }

//     pub fn add_fun_and_overwrite_its_id(&mut self, fun: Fun) -> NodeId<Fun> {
//         self.funs.add_and_overwrite_id(fun)
//     }

//     pub fn add_match_and_overwrite_its_id(&mut self, match_: Match) -> NodeId<Match> {
//         self.matches.add_and_overwrite_id(match_)
//     }

//     pub fn add_match_case_and_overwrite_its_id(
//         &mut self,
//         match_case: MatchCase,
//     ) -> NodeId<MatchCase> {
//         self.match_cases.add_and_overwrite_id(match_case)
//     }

//     pub fn add_forall_and_overwrite_its_id(&mut self, forall: Forall) -> NodeId<Forall> {
//         self.foralls.add_and_overwrite_id(forall)
//     }

//     pub fn add_check_and_overwrite_its_id(&mut self, check: Check) -> NodeId<Check> {
//         self.checks.add_and_overwrite_id(check)
//     }

//     pub fn add_check_assertion_and_overwrite_its_id(
//         &mut self,
//         assertion: CheckAssertion,
//     ) -> NodeId<CheckAssertion> {
//         self.check_assertions.add_and_overwrite_id(assertion)
//     }

//     pub fn add_symbolically_invalid_expression_and_overwrite_its_id(
//         &mut self,
//         symbolically_invalid_expression: SymbolicallyInvalidExpression,
//     ) -> NodeId<SymbolicallyInvalidExpression> {
//         self.symbolically_invalid_expressions
//             .add_and_overwrite_id(symbolically_invalid_expression)
//     }

//     pub fn add_illegal_fun_recursion_expression_and_overwrite_its_id(
//         &mut self,
//         illegal_fun_recursion_expression: IllegalFunRecursionExpression,
//     ) -> NodeId<IllegalFunRecursionExpression> {
//         self.illegal_fun_recursion_expressions
//             .add_and_overwrite_id(illegal_fun_recursion_expression)
//     }

//     pub fn add_identifier_and_overwrite_its_id(
//         &mut self,
//         identifier: Identifier,
//     ) -> NodeId<Identifier> {
//         self.identifiers.add_and_overwrite_id(identifier)
//     }
// }

// impl NodeRegistry {
//     pub fn file(&self, id: NodeId<File>) -> &File {
//         self.files.get(id)
//     }

//     pub fn type_statement(&self, id: NodeId<TypeStatement>) -> &TypeStatement {
//         self.type_statements.get(id)
//     }

//     pub fn param(&self, id: NodeId<Param>) -> &Param {
//         self.params.get(id)
//     }

//     pub fn variant(&self, id: NodeId<Variant>) -> &Variant {
//         self.variants.get(id)
//     }

//     pub fn let_statement(&self, id: NodeId<LetStatement>) -> &LetStatement {
//         self.let_statements.get(id)
//     }

//     pub fn name_expression(&self, id: NodeId<NameExpression>) -> &NameExpression {
//         self.name_expressions.get(id)
//     }

//     pub fn call(&self, id: NodeId<Call>) -> &Call {
//         self.calls.get(id)
//     }

//     pub fn fun(&self, id: NodeId<Fun>) -> &Fun {
//         self.funs.get(id)
//     }

//     pub fn match_(&self, id: NodeId<Match>) -> &Match {
//         self.matches.get(id)
//     }

//     pub fn match_case(&self, id: NodeId<MatchCase>) -> &MatchCase {
//         self.match_cases.get(id)
//     }

//     pub fn forall(&self, id: NodeId<Forall>) -> &Forall {
//         self.foralls.get(id)
//     }

//     pub fn check(&self, id: NodeId<Check>) -> &Check {
//         self.checks.get(id)
//     }

//     pub fn check_assertion(&self, id: NodeId<CheckAssertion>) -> &CheckAssertion {
//         self.check_assertions.get(id)
//     }

//     pub fn symbolically_invalid_expression(
//         &self,
//         id: NodeId<SymbolicallyInvalidExpression>,
//     ) -> &SymbolicallyInvalidExpression {
//         self.symbolically_invalid_expressions.get(id)
//     }

//     pub fn illegal_fun_recursion_expression(
//         &self,
//         id: NodeId<IllegalFunRecursionExpression>,
//     ) -> &IllegalFunRecursionExpression {
//         self.illegal_fun_recursion_expressions.get(id)
//     }

//     pub fn identifier(&self, id: NodeId<Identifier>) -> &Identifier {
//         &self.identifiers.get(id)
//     }
// }

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

    impl RegisterableList for NodeId<Identifier> {
        fn subregistry(registry: &NodeRegistry) -> &ListSubregistry<Self> {
            &registry.identifier_lists
        }

        fn subregistry_mut(registry: &mut NodeRegistry) -> &mut ListSubregistry<Self> {
            &mut registry.identifier_lists
        }
    }
}

// TODO: Delete

// impl NodeRegistry {
//     pub fn add_file_item_list(
//         &mut self,
//         list: Vec<FileItemNodeId>,
//     ) -> Option<NonEmptyListId<FileItemNodeId>> {
//         self.file_item_lists.add_non_empty(list)
//     }

//     pub fn add_param_list(
//         &mut self,
//         list: NonEmptyVec<NodeId<Param>>,
//     ) -> NonEmptyListId<NodeId<Param>> {
//         self.param_lists.add_non_empty(list)
//     }

//     pub fn add_variant_list(&mut self, list: Vec<NodeId<Variant>>) -> ListId<NodeId<Variant>> {
//         self.variant_lists.add_non_empty(list)
//     }

//     pub fn add_match_case_list(
//         &mut self,
//         list: Vec<NodeId<MatchCase>>,
//     ) -> ListId<NodeId<MatchCase>> {
//         self.match_case_lists.add_non_empty(list)
//     }

//     pub fn add_check_assertion_list(
//         &mut self,
//         list: Vec<NodeId<CheckAssertion>>,
//     ) -> ListId<NodeId<CheckAssertion>> {
//         self.check_assertion_lists.add_non_empty(list)
//     }

//     pub fn add_expression_list(&mut self, list: Vec<ExpressionId>) -> ListId<ExpressionId> {
//         self.expression_lists.add_non_empty(list)
//     }

//     pub fn add_identifier_list(
//         &mut self,
//         list: Vec<NodeId<Identifier>>,
//     ) -> ListId<NodeId<Identifier>> {
//         self.identifier_lists.add_non_empty(list)
//     }
// }

// TODO: Delete

// impl NodeRegistry {
//     pub fn file_item_list(&self, id: ListId<FileItemNodeId>) -> &[FileItemNodeId] {
//         self.file_item_lists.get(id)
//     }

//     pub fn param_list(&self, id: ListId<NodeId<Param>>) -> &[NodeId<Param>] {
//         self.param_lists.get(id)
//     }

//     pub fn variant_list(&self, id: ListId<NodeId<Variant>>) -> &[NodeId<Variant>] {
//         self.variant_lists.get(id)
//     }

//     pub fn match_case_list(&self, id: ListId<NodeId<MatchCase>>) -> &[NodeId<MatchCase>] {
//         self.match_case_lists.get(id)
//     }

//     pub fn check_assertion_list(
//         &self,
//         id: ListId<NodeId<CheckAssertion>>,
//     ) -> &[NodeId<CheckAssertion>] {
//         self.check_assertion_lists.get(id)
//     }

//     pub fn expression_list(&self, id: ListId<ExpressionId>) -> &[ExpressionId] {
//         self.expression_lists.get(id)
//     }

//     pub fn identifier_list(&self, id: ListId<NodeId<Identifier>>) -> &[NodeId<Identifier>] {
//         self.identifier_lists.get(id)
//     }
// }

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
