use super::*;

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

        /// Guaranteed to be consistent with `label_id` and `db_index`.
        value_id: NodeId<NameExpression>,
    },
    Explicit {
        label_id: NodeId<Identifier>,
        value_id: ExpressionId,
    },
}

impl LabeledCallArgId {
    pub fn implicit(
        label_id: NodeId<Identifier>,
        db_index: DbIndex,
        registry: &mut NodeRegistry,
    ) -> Self {
        fn dummy_id<T>() -> NodeId<T> {
            NodeId::new(0)
        }
        let span = registry.get(label_id).span;
        let component_list_id = registry.add_list(NonEmptyVec::singleton(label_id));
        let value_id = registry.add_and_overwrite_id(NameExpression {
            id: dummy_id(),
            span,
            component_list_id,
            db_index,
        });
        Self::Implicit {
            label_id,
            db_index,
            value_id,
        }
    }
}

impl LabeledCallArgId {
    pub fn label_id(&self) -> NodeId<Identifier> {
        match self {
            LabeledCallArgId::Implicit { label_id, .. } => *label_id,
            LabeledCallArgId::Explicit { label_id, .. } => *label_id,
        }
    }

    pub fn value_id(&self) -> ExpressionId {
        match *self {
            LabeledCallArgId::Implicit { value_id, .. } => ExpressionId::Name(value_id),
            LabeledCallArgId::Explicit { value_id, .. } => value_id,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NonEmptyMatchCaseParamListId {
    Unlabeled(NonEmptyListId<NodeId<Identifier>>),
    UniquelyLabeled {
        param_list_id: Option<NonEmptyListId<NodeId<LabeledMatchCaseParam>>>,
        triple_dot: Option<TextSpan>,
    },
}

impl OptionalNonEmptyVecLen for Option<NonEmptyMatchCaseParamListId> {
    fn len(&self) -> usize {
        self.as_ref().map(|v| v.explicit_len()).unwrap_or(0)
    }
}

impl NonEmptyMatchCaseParamListId {
    /// Note that this is not the _true_ length of the param list,
    /// but rather the number of params that are explicitly listed.
    /// For example, if we have
    ///
    /// ```pamlihu
    /// type Nat {
    ///    .O: Nat,
    ///    .S(~n: Nat): Nat,
    /// }
    ///
    /// let foo = match Nat.O {
    ///     .O => Nat.O,
    ///     .S(...) => Nat.O,
    /// };
    /// ```
    ///
    /// then the _true_ length of the `.S(...)` case is 1 (the `~n` param),
    /// but the _explicit_ length is 0 (since no params are explicitly listed).
    pub fn explicit_len(&self) -> usize {
        match self {
            NonEmptyMatchCaseParamListId::Unlabeled(vec) => vec.len.get(),
            NonEmptyMatchCaseParamListId::UniquelyLabeled { param_list_id, .. } => {
                param_list_id.map(|v| v.len.get()).unwrap_or(0)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExpressionId {
    Name(NodeId<NameExpression>),
    Todo(NodeId<TodoExpression>),
    Call(NodeId<Call>),
    Fun(NodeId<Fun>),
    Match(NodeId<Match>),
    Forall(NodeId<Forall>),
    Check(NodeId<Check>),
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
