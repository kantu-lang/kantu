use crate::data::registered_ast::*;

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

// TODO: Implement Debug, PartialEq, Eq for NodeId<T>,
// since #[derive] only works if T implements the respective traits.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeRegistry {
    files: Vec<File>,
    type_statements: Vec<TypeStatement>,
    identifiers: Vec<Identifier>,
    params: Vec<Param>,
    variants: Vec<Variant>,
    let_statements: Vec<LetStatement>,
    wrapped_expressions: Vec<WrappedExpression>,
    dots: Vec<Dot>,
    calls: Vec<Call>,
    funs: Vec<Fun>,
    matches: Vec<Match>,
    match_cases: Vec<MatchCase>,
    foralls: Vec<Forall>,
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
            wrapped_expressions: Vec::new(),
            dots: Vec::new(),
            calls: Vec::new(),
            funs: Vec::new(),
            matches: Vec::new(),
            match_cases: Vec::new(),
            foralls: Vec::new(),
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

    pub fn add_wrapped_expression_and_overwrite_its_id(
        &mut self,
        mut wrapped_expression: WrappedExpression,
    ) -> NodeId<WrappedExpression> {
        let id = NodeId::<WrappedExpression>::new(self.wrapped_expressions.len());
        wrapped_expression.id = id;
        self.wrapped_expressions.push(wrapped_expression);
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

    pub fn add_dot_and_overwrite_its_id(&mut self, mut dot: Dot) -> NodeId<Dot> {
        let id = NodeId::<Dot>::new(self.dots.len());
        dot.id = id;
        self.dots.push(dot);
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

    pub fn wrapped_expression(&self, id: NodeId<WrappedExpression>) -> &WrappedExpression {
        &self.wrapped_expressions[id.raw]
    }

    pub fn dot(&self, id: NodeId<Dot>) -> &Dot {
        &self.dots[id.raw]
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
    // TODO: Delete after we're done debugging.
    #[allow(non_snake_case)]
    pub fn TODO_identifiers(&self) -> &[Identifier] {
        &self.identifiers
    }
}
