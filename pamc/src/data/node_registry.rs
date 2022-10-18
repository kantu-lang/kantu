use crate::data::registered_sst::*;

// TODO: Implement Debug, PartialEq, Eq for NodeId<T>,
// since #[derive] only works if T implements the respective traits.
#[derive(Debug, PartialEq, Eq)]
pub struct NodeId<T> {
    pub raw: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> NodeId<T> {
    pub fn new(raw: usize) -> Self {
        Self {
            raw,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Clone for NodeId<T> {
    fn clone(&self) -> NodeId<T> {
        NodeId {
            raw: self.raw,
            _phantom: self._phantom,
        }
    }
}

impl<T> Copy for NodeId<T> {}

impl<T> std::hash::Hash for NodeId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

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

#[derive(Debug, PartialEq, Eq)]
pub struct ListId<T> {
    pub start: usize,
    pub len: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> ListId<T> {
    pub fn new(start: usize, len: usize) -> Self {
        Self {
            start,
            len,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Clone for ListId<T> {
    fn clone(&self) -> ListId<T> {
        Self::new(self.start, self.len)
    }
}

impl<T> Copy for ListId<T> {}

impl<T> std::hash::Hash for ListId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.start.hash(state);
        self.len.hash(state);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExpressionRef<'a> {
    Name(&'a NameExpression),
    Call(&'a Call),
    Fun(&'a Fun),
    Match(&'a Match),
    Forall(&'a Forall),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeRegistry {
    files: Vec<File>,
    type_statements: Vec<TypeStatement>,
    identifiers: Vec<Identifier>,
    params: Vec<Param>,
    variants: Vec<Variant>,
    let_statements: Vec<LetStatement>,
    name_expressions: Vec<NameExpression>,
    calls: Vec<Call>,
    funs: Vec<Fun>,
    matches: Vec<Match>,
    match_cases: Vec<MatchCase>,
    foralls: Vec<Forall>,

    file_item_lists: Vec<FileItemNodeId>,
    param_lists: Vec<NodeId<Param>>,
    variant_lists: Vec<NodeId<Variant>>,
    match_case_lists: Vec<NodeId<MatchCase>>,
    identifier_lists: Vec<NodeId<Identifier>>,
    expression_lists: Vec<ExpressionId>,
}

impl NodeRegistry {
    pub fn empty() -> Self {
        Self {
            files: Vec::new(),
            type_statements: Vec::new(),
            identifiers: Vec::new(),
            params: Vec::new(),
            variants: Vec::new(),
            let_statements: Vec::new(),
            name_expressions: Vec::new(),
            calls: Vec::new(),
            funs: Vec::new(),
            matches: Vec::new(),
            match_cases: Vec::new(),
            foralls: Vec::new(),

            file_item_lists: Vec::new(),
            param_lists: Vec::new(),
            variant_lists: Vec::new(),
            match_case_lists: Vec::new(),
            identifier_lists: Vec::new(),
            expression_lists: Vec::new(),
        }
    }
}

impl NodeRegistry {
    pub fn add_file_and_overwrite_its_id(&mut self, mut file: File) -> NodeId<File> {
        let id = NodeId::<File>::new(self.files.len());
        file.id = id;
        self.files.push(file);
        id
    }

    pub fn add_type_statement_and_overwrite_its_id(
        &mut self,
        mut type_statement: TypeStatement,
    ) -> NodeId<TypeStatement> {
        let id = NodeId::<TypeStatement>::new(self.type_statements.len());
        type_statement.id = id;
        self.type_statements.push(type_statement);
        id
    }

    pub fn add_standard_identifier_and_overwrite_its_id(
        &mut self,
        mut identifier: Identifier,
    ) -> NodeId<Identifier> {
        let id = NodeId::<Identifier>::new(self.identifiers.len());
        identifier.id = id;
        self.identifiers.push(identifier);
        id
    }

    pub fn add_param_and_overwrite_its_id(&mut self, mut param: Param) -> NodeId<Param> {
        let id = NodeId::<Param>::new(self.params.len());
        param.id = id;
        self.params.push(param);
        id
    }

    pub fn add_variant_and_overwrite_its_id(&mut self, mut variant: Variant) -> NodeId<Variant> {
        let id = NodeId::<Variant>::new(self.variants.len());
        variant.id = id;
        self.variants.push(variant);
        id
    }

    pub fn add_let_statement_and_overwrite_its_id(
        &mut self,
        mut let_statement: LetStatement,
    ) -> NodeId<LetStatement> {
        let id = NodeId::<LetStatement>::new(self.let_statements.len());
        let_statement.id = id;
        self.let_statements.push(let_statement);
        id
    }

    pub fn add_identifier_and_overwrite_its_id(
        &mut self,
        mut quasi_identifier: Identifier,
    ) -> NodeId<Identifier> {
        let id = NodeId::<Identifier>::new(self.identifiers.len());
        quasi_identifier.id = id;
        self.identifiers.push(quasi_identifier);
        id
    }

    /// Panics if the provided name expression has zero components.
    pub fn add_name_expression_and_overwrite_its_id(
        &mut self,
        mut name_expression: NameExpression,
    ) -> NodeId<NameExpression> {
        if name_expression.component_list_id.len == 0 {
            panic!("NameExpression must have at least one component");
        }

        let id = NodeId::<NameExpression>::new(self.name_expressions.len());
        name_expression.id = id;
        self.name_expressions.push(name_expression);
        id
    }

    pub fn add_call_and_overwrite_its_id(&mut self, mut call: Call) -> NodeId<Call> {
        let id = NodeId::<Call>::new(self.calls.len());
        call.id = id;
        self.calls.push(call);
        id
    }

    pub fn add_fun_and_overwrite_its_id(&mut self, mut fun: Fun) -> NodeId<Fun> {
        let id = NodeId::<Fun>::new(self.funs.len());
        fun.id = id;
        self.funs.push(fun);
        id
    }

    pub fn add_match_and_overwrite_its_id(&mut self, mut match_: Match) -> NodeId<Match> {
        let id = NodeId::<Match>::new(self.matches.len());
        match_.id = id;
        self.matches.push(match_);
        id
    }

    pub fn add_match_case_and_overwrite_its_id(
        &mut self,
        mut match_case: MatchCase,
    ) -> NodeId<MatchCase> {
        let id = NodeId::<MatchCase>::new(self.match_cases.len());
        match_case.id = id;
        self.match_cases.push(match_case);
        id
    }

    pub fn add_forall_and_overwrite_its_id(&mut self, mut forall: Forall) -> NodeId<Forall> {
        let id = NodeId::<Forall>::new(self.foralls.len());
        forall.id = id;
        self.foralls.push(forall);
        id
    }
}

impl NodeRegistry {
    pub fn file(&self, id: NodeId<File>) -> &File {
        &self.files[id.raw]
    }

    pub fn type_statement(&self, id: NodeId<TypeStatement>) -> &TypeStatement {
        &self.type_statements[id.raw]
    }

    pub fn identifier(&self, id: NodeId<Identifier>) -> &Identifier {
        &self.identifiers[id.raw]
    }

    pub fn param(&self, id: NodeId<Param>) -> &Param {
        &self.params[id.raw]
    }

    pub fn variant(&self, id: NodeId<Variant>) -> &Variant {
        &self.variants[id.raw]
    }

    pub fn let_statement(&self, id: NodeId<LetStatement>) -> &LetStatement {
        &self.let_statements[id.raw]
    }

    pub fn name_expression(&self, id: NodeId<NameExpression>) -> &NameExpression {
        &self.name_expressions[id.raw]
    }

    pub fn call(&self, id: NodeId<Call>) -> &Call {
        &self.calls[id.raw]
    }

    pub fn fun(&self, id: NodeId<Fun>) -> &Fun {
        &self.funs[id.raw]
    }

    pub fn match_(&self, id: NodeId<Match>) -> &Match {
        &self.matches[id.raw]
    }

    pub fn match_case(&self, id: NodeId<MatchCase>) -> &MatchCase {
        &self.match_cases[id.raw]
    }

    pub fn forall(&self, id: NodeId<Forall>) -> &Forall {
        &self.foralls[id.raw]
    }
}

impl NodeRegistry {
    pub fn add_file_item_list(&mut self, mut list: Vec<FileItemNodeId>) -> ListId<FileItemNodeId> {
        let id = ListId::<FileItemNodeId>::new(self.file_item_lists.len(), list.len());
        self.file_item_lists.append(&mut list);
        id
    }

    pub fn add_param_list(&mut self, mut list: Vec<NodeId<Param>>) -> ListId<NodeId<Param>> {
        let id = ListId::<NodeId<Param>>::new(self.param_lists.len(), list.len());
        self.param_lists.append(&mut list);
        id
    }

    pub fn add_variant_list(&mut self, mut list: Vec<NodeId<Variant>>) -> ListId<NodeId<Variant>> {
        let id = ListId::<NodeId<Variant>>::new(self.variant_lists.len(), list.len());
        self.variant_lists.append(&mut list);
        id
    }

    pub fn add_match_case_list(
        &mut self,
        mut list: Vec<NodeId<MatchCase>>,
    ) -> ListId<NodeId<MatchCase>> {
        let id = ListId::<NodeId<MatchCase>>::new(self.match_case_lists.len(), list.len());
        self.match_case_lists.append(&mut list);
        id
    }

    pub fn add_identifier_list(
        &mut self,
        mut list: Vec<NodeId<Identifier>>,
    ) -> ListId<NodeId<Identifier>> {
        let id = ListId::<NodeId<Identifier>>::new(self.identifier_lists.len(), list.len());
        self.identifier_lists.append(&mut list);
        id
    }

    pub fn add_expression_list(&mut self, mut list: Vec<ExpressionId>) -> ListId<ExpressionId> {
        let id = ListId::<ExpressionId>::new(self.expression_lists.len(), list.len());
        self.expression_lists.append(&mut list);
        id
    }
}

impl NodeRegistry {
    pub fn file_item_list(&self, id: ListId<FileItemNodeId>) -> &[FileItemNodeId] {
        let end = id.start + id.len;
        &self.file_item_lists[id.start..end]
    }

    pub fn param_list(&self, id: ListId<NodeId<Param>>) -> &[NodeId<Param>] {
        let end = id.start + id.len;
        &self.param_lists[id.start..end]
    }

    pub fn variant_list(&self, id: ListId<NodeId<Variant>>) -> &[NodeId<Variant>] {
        let end = id.start + id.len;
        &self.variant_lists[id.start..end]
    }

    pub fn match_case_list(&self, id: ListId<NodeId<MatchCase>>) -> &[NodeId<MatchCase>] {
        let end = id.start + id.len;
        &self.match_case_lists[id.start..end]
    }

    pub fn identifier_list(&self, id: ListId<NodeId<Identifier>>) -> &[NodeId<Identifier>] {
        let end = id.start + id.len;
        &self.identifier_lists[id.start..end]
    }

    pub fn expression_list(&self, id: ListId<ExpressionId>) -> &[ExpressionId] {
        let end = id.start + id.len;
        &self.expression_lists[id.start..end]
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
        let name_expression = self.name_expression(id);
        let component_ids = self.identifier_list(name_expression.component_list_id);
        let rightmost_component_id = *component_ids
            .last()
            .expect("A name expression should always have at least one component. This condition should have been checked by NodeRegistry::add_name_expression_and_overwrite_its_id. The fact that a zero-component name expression was successfully registered indicates a serious logic error.")
            ;
        self.identifier(rightmost_component_id)
    }
}

impl NodeRegistry {
    // TODO: Delete after we're done debugging.
    #[allow(non_snake_case)]
    pub fn TODO_identifiers(&self) -> &[Identifier] {
        &self.identifiers
    }
}
