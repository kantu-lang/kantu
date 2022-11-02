use crate::data::x_light_ast::*;

pub use crate::data::node_registry::ListId;
pub use crate::data::node_registry::NodeId;

use rustc_hash::FxHashMap;
use std::fmt::Debug;

use remove_id::RemoveId;
mod remove_id;

// TODO: We need to seriously redesign this because
// stripping is going to make debug messages inaccurate.

// For example, if the first NameExpression with a
// `db_index` of 0 will be the _only_ name expression to
// be registered.
// Consequently, all subsequent NameExpressions with a
// `db_index` of 0 will be assigned the same id as the first
// NameExpression, which will give them incorrect `name` and
// `start` values.
// For type checking, we don't care about `name` and `start`,
// which is the whole reason we're stripping in the first place.
// However, a developer obviously wants to know the name and location
// of the identifiers in their erroneous code.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FileItemNodeId {
    Type(NodeId<TypeStatement>),
    Let(NodeId<LetStatement>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExpressionId {
    Name(NodeId<NameExpression>),
    Call(NodeId<Call>),
    Fun(NodeId<Fun>),
    Match(NodeId<Match>),
    Forall(NodeId<Forall>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExpressionRef<'a> {
    Name(&'a NameExpression),
    Call(&'a Call),
    Fun(&'a Fun),
    Match(&'a Match),
    Forall(&'a Forall),
}

/// For any type `T`, if `T` implements `RemoveId`, then the
/// registry guarantees that for any two `T`s `x` and `y`
/// with respective `NodeId<T>`s `x_id` and `y_id`,
/// `x.remove_id() == y.remove_id()` implies `x_id == y_id`.
#[derive(Clone, Debug)]
pub struct NodeRegistry {
    files: Subregistry<File>,
    type_statements: Subregistry<TypeStatement>,
    params: Subregistry<Param>,
    variants: Subregistry<Variant>,
    let_statements: Subregistry<LetStatement>,
    name_expressions: Subregistry<NameExpression>,
    calls: Subregistry<Call>,
    funs: Subregistry<Fun>,
    matches: Subregistry<Match>,
    match_cases: Subregistry<MatchCase>,
    foralls: Subregistry<Forall>,

    // TODO: Make this a Subregistry.
    identifiers: Vec<Identifier>,

    file_item_lists: ListSubregistry<FileItemNodeId>,
    param_lists: ListSubregistry<NodeId<Param>>,
    variant_lists: ListSubregistry<NodeId<Variant>>,
    match_case_lists: ListSubregistry<NodeId<MatchCase>>,
    expression_lists: ListSubregistry<ExpressionId>,

    identifier_lists: Vec<NodeId<Identifier>>,
}

impl NodeRegistry {
    pub fn empty() -> Self {
        Self {
            files: Subregistry::new(),
            type_statements: Subregistry::new(),
            params: Subregistry::new(),
            variants: Subregistry::new(),
            let_statements: Subregistry::new(),
            name_expressions: Subregistry::new(),
            calls: Subregistry::new(),
            funs: Subregistry::new(),
            matches: Subregistry::new(),
            match_cases: Subregistry::new(),
            foralls: Subregistry::new(),

            identifiers: Vec::new(),

            file_item_lists: ListSubregistry::new(),
            param_lists: ListSubregistry::new(),
            variant_lists: ListSubregistry::new(),
            match_case_lists: ListSubregistry::new(),
            expression_lists: ListSubregistry::new(),

            identifier_lists: Vec::new(),
        }
    }
}

impl NodeRegistry {
    pub fn add_file_and_overwrite_its_id(&mut self, file: File) -> NodeId<File> {
        self.files.add_and_overwrite_id(file)
    }

    pub fn add_type_statement_and_overwrite_its_id(
        &mut self,
        type_statement: TypeStatement,
    ) -> NodeId<TypeStatement> {
        self.type_statements.add_and_overwrite_id(type_statement)
    }

    pub fn add_param_and_overwrite_its_id(&mut self, param: Param) -> NodeId<Param> {
        self.params.add_and_overwrite_id(param)
    }

    pub fn add_variant_and_overwrite_its_id(&mut self, variant: Variant) -> NodeId<Variant> {
        self.variants.add_and_overwrite_id(variant)
    }

    pub fn add_let_statement_and_overwrite_its_id(
        &mut self,
        let_statement: LetStatement,
    ) -> NodeId<LetStatement> {
        self.let_statements.add_and_overwrite_id(let_statement)
    }

    /// Panics if the provided name expression has zero components.
    pub fn add_name_expression_and_overwrite_its_id(
        &mut self,
        name: NameExpression,
    ) -> NodeId<NameExpression> {
        if name.component_list_id.len == 0 {
            panic!("NameExpression must have at least one component");
        }

        self.name_expressions.add_and_overwrite_id(name)
    }

    pub fn add_call_and_overwrite_its_id(&mut self, call: Call) -> NodeId<Call> {
        self.calls.add_and_overwrite_id(call)
    }

    pub fn add_fun_and_overwrite_its_id(&mut self, fun: Fun) -> NodeId<Fun> {
        self.funs.add_and_overwrite_id(fun)
    }

    pub fn add_match_and_overwrite_its_id(&mut self, match_: Match) -> NodeId<Match> {
        self.matches.add_and_overwrite_id(match_)
    }

    pub fn add_match_case_and_overwrite_its_id(
        &mut self,
        match_case: MatchCase,
    ) -> NodeId<MatchCase> {
        self.match_cases.add_and_overwrite_id(match_case)
    }

    pub fn add_forall_and_overwrite_its_id(&mut self, forall: Forall) -> NodeId<Forall> {
        self.foralls.add_and_overwrite_id(forall)
    }
}

impl NodeRegistry {
    pub fn file(&self, id: NodeId<File>) -> &File {
        self.files.get(id)
    }

    pub fn type_statement(&self, id: NodeId<TypeStatement>) -> &TypeStatement {
        self.type_statements.get(id)
    }

    pub fn param(&self, id: NodeId<Param>) -> &Param {
        self.params.get(id)
    }

    pub fn variant(&self, id: NodeId<Variant>) -> &Variant {
        self.variants.get(id)
    }

    pub fn let_statement(&self, id: NodeId<LetStatement>) -> &LetStatement {
        self.let_statements.get(id)
    }

    pub fn name_expression(&self, id: NodeId<NameExpression>) -> &NameExpression {
        self.name_expressions.get(id)
    }

    pub fn call(&self, id: NodeId<Call>) -> &Call {
        self.calls.get(id)
    }

    pub fn fun(&self, id: NodeId<Fun>) -> &Fun {
        self.funs.get(id)
    }

    pub fn match_(&self, id: NodeId<Match>) -> &Match {
        self.matches.get(id)
    }

    pub fn match_case(&self, id: NodeId<MatchCase>) -> &MatchCase {
        self.match_cases.get(id)
    }

    pub fn forall(&self, id: NodeId<Forall>) -> &Forall {
        self.foralls.get(id)
    }
}

impl NodeRegistry {
    pub fn add_identifier_and_overwrite_its_id(
        &mut self,
        identifier: Identifier,
    ) -> NodeId<Identifier> {
        self.identifiers.push(identifier.clone());
        NodeId::new(self.identifiers.len() - 1)
    }
}

impl NodeRegistry {
    pub fn identifier(&self, id: NodeId<Identifier>) -> &Identifier {
        &self.identifiers[id.raw]
    }
}

impl NodeRegistry {
    pub fn add_file_item_list(&mut self, list: Vec<FileItemNodeId>) -> ListId<FileItemNodeId> {
        self.file_item_lists.add(list)
    }

    pub fn add_param_list(&mut self, list: Vec<NodeId<Param>>) -> ListId<NodeId<Param>> {
        self.param_lists.add(list)
    }

    pub fn add_variant_list(&mut self, list: Vec<NodeId<Variant>>) -> ListId<NodeId<Variant>> {
        self.variant_lists.add(list)
    }

    pub fn add_match_case_list(
        &mut self,
        list: Vec<NodeId<MatchCase>>,
    ) -> ListId<NodeId<MatchCase>> {
        self.match_case_lists.add(list)
    }

    pub fn add_expression_list(&mut self, list: Vec<ExpressionId>) -> ListId<ExpressionId> {
        self.expression_lists.add(list)
    }
}

impl NodeRegistry {
    pub fn file_item_list(&self, id: ListId<FileItemNodeId>) -> &[FileItemNodeId] {
        self.file_item_lists.get(id)
    }

    pub fn param_list(&self, id: ListId<NodeId<Param>>) -> &[NodeId<Param>] {
        self.param_lists.get(id)
    }

    pub fn variant_list(&self, id: ListId<NodeId<Variant>>) -> &[NodeId<Variant>] {
        self.variant_lists.get(id)
    }

    pub fn match_case_list(&self, id: ListId<NodeId<MatchCase>>) -> &[NodeId<MatchCase>] {
        self.match_case_lists.get(id)
    }

    pub fn expression_list(&self, id: ListId<ExpressionId>) -> &[ExpressionId] {
        self.expression_lists.get(id)
    }
}

impl NodeRegistry {
    pub fn add_identifier_list(
        &mut self,
        mut list: Vec<NodeId<Identifier>>,
    ) -> ListId<NodeId<Identifier>> {
        let id = ListId::<NodeId<Identifier>>::new(self.identifier_lists.len(), list.len());
        self.identifier_lists.append(&mut list);
        id
    }
}

impl NodeRegistry {
    pub fn identifier_list(&self, id: ListId<NodeId<Identifier>>) -> &[NodeId<Identifier>] {
        let end = id.start + id.len;
        &self.identifier_lists[id.start..end]
    }
}

impl NodeRegistry {
    pub fn expression_ref(&self, id: ExpressionId) -> ExpressionRef<'_> {
        match id {
            ExpressionId::Name(id) => ExpressionRef::Name(self.name_expression(id)),
            ExpressionId::Call(id) => ExpressionRef::Call(self.call(id)),
            ExpressionId::Fun(id) => ExpressionRef::Fun(self.fun(id)),
            ExpressionId::Match(id) => ExpressionRef::Match(self.match_(id)),
            ExpressionId::Forall(id) => ExpressionRef::Forall(self.forall(id)),
        }
    }
}

impl NodeRegistry {
    pub fn rightmost_component(&self, id: NodeId<NameExpression>) -> &Identifier {
        let name = self.name_expression(id);
        let component_ids = self.identifier_list(name.component_list_id);
        let rightmost_component_id = *component_ids
            .last()
            .expect("A name expression should always have at least one component. This condition should have been checked by NodeRegistry::add_name_expression_and_overwrite_its_id. The fact that a zero-component name expression was successfully registered indicates a serious logic error.")
            ;
        self.identifier(rightmost_component_id)
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
        ids: FxHashMap<Vec<T>, ListId<T>>,
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
        pub fn get(&self, id: ListId<T>) -> &[T] {
            &self.flattened_items[id.start..(id.start + id.len)]
        }
    }

    impl<T> ListSubregistry<T>
    where
        T: Clone + Eq + std::hash::Hash,
    {
        pub fn add(&mut self, list: Vec<T>) -> ListId<T> {
            if let Some(existing_id) = self.ids.get(&list) {
                *existing_id
            } else {
                let new_id = ListId::<T>::new(self.flattened_items.len(), list.len());
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

    impl SetId for Param {
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
}
