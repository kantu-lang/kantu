use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Arena {
    files: Vec<File>,
    type_statements: Vec<TypeStatement>,
    identifiers: Vec<Identifier>,
    params: Vec<Param>,
    constructors: Vec<Constructor>,
    let_statements: Vec<LetStatement>,
    expressions: Vec<Expression>,
    quasi_identifiers: Vec<QuasiIdentifier>,
    dots: Vec<Dot>,
    calls: Vec<Call>,
    funs: Vec<Fun>,
    matches: Vec<Match>,
    match_cases: Vec<MatchCase>,
    foralls: Vec<Forall>,
}

impl Arena {
    pub fn empty() -> Self {
        Self {
            files: Vec::new(),
            type_statements: Vec::new(),
            identifiers: Vec::new(),
            params: Vec::new(),
            constructors: Vec::new(),
            let_statements: Vec::new(),
            expressions: Vec::new(),
            quasi_identifiers: Vec::new(),
            dots: Vec::new(),
            calls: Vec::new(),
            funs: Vec::new(),
            matches: Vec::new(),
            match_cases: Vec::new(),
            foralls: Vec::new(),
        }
    }
}

impl Arena {
    pub fn add_file(&mut self, file: File) -> NodeId<File> {
        let id = NodeId::<File>::new(self.files.len());
        self.files.push(file);
        id
    }

    pub fn add_type_statement(&mut self, type_statement: TypeStatement) -> NodeId<TypeStatement> {
        let id = NodeId::<TypeStatement>::new(self.type_statements.len());
        self.type_statements.push(type_statement);
        id
    }

    pub fn add_identifier(&mut self, identifier: Identifier) -> NodeId<Identifier> {
        let id = NodeId::<Identifier>::new(self.identifiers.len());
        self.identifiers.push(identifier);
        id
    }

    pub fn add_param(&mut self, param: Param) -> NodeId<Param> {
        let id = NodeId::<Param>::new(self.params.len());
        self.params.push(param);
        id
    }

    pub fn add_constructor(&mut self, constructor: Constructor) -> NodeId<Constructor> {
        let id = NodeId::<Constructor>::new(self.constructors.len());
        self.constructors.push(constructor);
        id
    }

    pub fn add_let_statement(&mut self, let_statement: LetStatement) -> NodeId<LetStatement> {
        let id = NodeId::<LetStatement>::new(self.let_statements.len());
        self.let_statements.push(let_statement);
        id
    }

    pub fn add_expression(&mut self, expression: Expression) -> NodeId<Expression> {
        let id = NodeId::<Expression>::new(self.expressions.len());
        self.expressions.push(expression);
        id
    }

    pub fn add_quasi_identifier(
        &mut self,
        quasi_identifier: QuasiIdentifier,
    ) -> NodeId<QuasiIdentifier> {
        let id = NodeId::<QuasiIdentifier>::new(self.quasi_identifiers.len());
        self.quasi_identifiers.push(quasi_identifier);
        id
    }

    pub fn add_dot(&mut self, dot: Dot) -> NodeId<Dot> {
        let id = NodeId::<Dot>::new(self.dots.len());
        self.dots.push(dot);
        id
    }

    pub fn add_call(&mut self, call: Call) -> NodeId<Call> {
        let id = NodeId::<Call>::new(self.calls.len());
        self.calls.push(call);
        id
    }

    pub fn add_fun(&mut self, fun: Fun) -> NodeId<Fun> {
        let id = NodeId::<Fun>::new(self.funs.len());
        self.funs.push(fun);
        id
    }

    pub fn add_match(&mut self, match_: Match) -> NodeId<Match> {
        let id = NodeId::<Match>::new(self.matches.len());
        self.matches.push(match_);
        id
    }

    pub fn add_match_case(&mut self, match_case: MatchCase) -> NodeId<MatchCase> {
        let id = NodeId::<MatchCase>::new(self.match_cases.len());
        self.match_cases.push(match_case);
        id
    }

    pub fn add_forall(&mut self, forall: Forall) -> NodeId<Forall> {
        let id = NodeId::<Forall>::new(self.foralls.len());
        self.foralls.push(forall);
        id
    }
}

impl Arena {
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

    pub fn constructor(&self, id: NodeId<Constructor>) -> &Constructor {
        &self.constructors[id.raw]
    }

    pub fn let_statement(&self, id: NodeId<LetStatement>) -> &LetStatement {
        &self.let_statements[id.raw]
    }

    pub fn expression(&self, id: NodeId<Expression>) -> &Expression {
        &self.expressions[id.raw]
    }

    pub fn quasi_identifier(&self, id: NodeId<QuasiIdentifier>) -> &QuasiIdentifier {
        &self.quasi_identifiers[id.raw]
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
