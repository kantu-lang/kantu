use super::*;

const NUMBER_OF_BUILTIN_ENTRIES: usize = 2;

#[derive(Debug, Clone)]
pub struct Context {
    /// Each type in the stack is expressed "locally" (i.e., relative
    /// to its position within the stack).
    ///
    /// For example, consider the scenario where `local_type_stack[1] == NameExpression { db_index: 0 }`.
    /// The local De Bruijn index `0` refers to the first symbol counting right-to-left _from position 1_.
    /// Thus, if `local_type_stack.len() == 3`, for example, then the global De Bruijn index for `local_type_stack[1]` is `2`.
    ///
    /// If an illustration would help, consider the following:
    /// ```text
    /// Type1: DNE
    /// Type0: Type1
    /// Nat: Type0
    ///
    /// ----------------------
    ///
    /// local_type_stack: [Type1, Type0, Nat] = [DNE, 0, 0]
    ///
    /// ----------------------
    ///
    /// local_type(Type0) = Type1 = 0
    /// // Why? - Count backwards from Type0 (not including Type0 itself):
    ///
    /// vvv
    /// (0)
    /// [Type1, Type0, Nat]
    ///
    /// ----------------------
    ///
    /// global_type(Type0) = Type1 = 2
    /// // Why? - Count backwards from the end of the stack (including the last item):
    ///
    /// vvv
    /// (2)     (1)    (0)
    /// [Type1, Type0, Nat]
    /// ```
    ///
    local_type_stack: Vec<ContextEntry>,
}

#[derive(Clone, Debug)]
pub struct ContextEntry {
    pub type_id: NormalFormId,
    pub definition: ContextEntryDefinition,
}

#[derive(Clone, Copy, Debug)]
pub enum ContextEntryDefinition {
    Alias {
        value_id: NormalFormId,
        visibility: Visibility,
        transparency: Transparency,
    },
    /// Algebraic data type
    Adt {
        variant_name_list_id: Option<NonEmptyListId<NodeId<Identifier>>>,
        visibility: Visibility,
    },
    Variant {
        name_id: NodeId<Identifier>,
        visibility: Visibility,
    },
    Uninterpreted,
}

const TYPE1_LEVEL: DbLevel = DbLevel(0);
const TYPE0_LEVEL: DbLevel = DbLevel(1);

impl Context {
    pub fn with_builtins(registry: &mut NodeRegistry) -> Self {
        // We should will never retrieve the type of `Type1`, since it is undefined.
        // However, we need to store _some_ object in the stack, so that the indices
        // of the other types are correct.
        let type1_entry = {
            let dummy_type1_type_id = NormalFormId::unchecked_new(ExpressionId::Name(
                add_name_expression_and_overwrite_component_ids(
                    registry,
                    NonEmptyVec::singleton(Identifier {
                        id: dummy_id(),
                        name: IdentifierName::Reserved(ReservedIdentifierName::Type2),
                        span: None,
                    }),
                    DbIndex(0),
                ),
            ));
            ContextEntry {
                type_id: dummy_type1_type_id,
                definition: ContextEntryDefinition::Uninterpreted,
            }
        };
        let type0_entry = {
            let type0_type_id = NormalFormId::unchecked_new(ExpressionId::Name(
                add_name_expression_and_overwrite_component_ids(
                    registry,
                    NonEmptyVec::singleton(Identifier {
                        id: dummy_id(),
                        name: IdentifierName::Reserved(ReservedIdentifierName::Type1),
                        span: None,
                    }),
                    DbIndex(0),
                ),
            ));
            ContextEntry {
                type_id: type0_type_id,
                definition: ContextEntryDefinition::Uninterpreted,
            }
        };
        let builtins: [ContextEntry; NUMBER_OF_BUILTIN_ENTRIES] = [type1_entry, type0_entry];
        Self {
            local_type_stack: builtins.to_vec(),
        }
    }
}

impl Context {
    /// Panics if `n > self.len()`.
    pub fn pop_n(&mut self, n: usize) {
        if n > self.len() {
            panic!(
                "Tried to pop {} elements from a context with only {} elements",
                n,
                self.len()
            );
        }
        self.local_type_stack.truncate(self.len() - n);
    }

    /// We effectively return `()`, but the reason we use the `Result` type we is to
    /// encourage the caller to only use `push` inside a function that returns `Result<_, Tainted<_>>`.
    pub fn push(&mut self, entry: ContextEntry) -> PushWarning {
        self.local_type_stack.push(entry);
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.local_type_stack.len()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Tainted<T>(T);

impl<T> Tainted<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }
}

pub fn tainted_err<T, E>(err: E) -> Result<T, Tainted<E>> {
    Err(Tainted::new(err))
}

impl<T> Tainted<T> {
    pub fn map<U, F>(self, f: F) -> Tainted<U>
    where
        F: FnOnce(T) -> U,
    {
        Tainted(f(self.0))
    }
}

impl From<Tainted<Infallible>> for Infallible {
    fn from(impossible: Tainted<Infallible>) -> Infallible {
        impossible.0
    }
}

impl From<Tainted<Infallible>> for Tainted<TypeCheckError> {
    fn from(impossible: Tainted<Infallible>) -> Self {
        #[allow(unreachable_code)]
        match Infallible::from(impossible) {}
    }
}

pub(super) fn untaint_err<In, Out, Err, F>(state: &mut State, input: In, f: F) -> Result<Out, Err>
where
    F: FnOnce(&mut State, In) -> Result<Out, Tainted<Err>>,
{
    let original_context_len = state.context.len();
    let original_scontext_len = state.substitution_context.len();
    let result = f(state, input);
    match result {
        Ok(ok) => Ok(ok),
        Err(err) => {
            state.context.truncate(original_context_len);
            state.substitution_context.truncate(original_scontext_len);
            Err(err.0)
        }
    }
}

pub type WithPushWarning<T> = Result<T, Tainted<Infallible>>;
pub type PushWarning = WithPushWarning<()>;

pub fn with_push_warning<T>(value: T) -> WithPushWarning<T> {
    Ok(value)
}

impl Context {
    fn truncate(&mut self, new_len: usize) {
        if new_len > self.len() {
            panic!(
                "Tried to truncate a context with {} elements to {} elements",
                self.len(),
                new_len
            );
        }
        self.local_type_stack.truncate(new_len);
    }
}

impl Context {
    /// Returns the De Bruijn index of the `Type0` expression.
    pub fn type0_dbi(&self) -> DbIndex {
        self.level_to_index(TYPE0_LEVEL)
    }

    /// Returns the De Bruijn index of the `Type1` expression.
    pub fn type1_dbi(&self) -> DbIndex {
        self.level_to_index(TYPE1_LEVEL)
    }
}

impl Context {
    pub fn level_to_index(&self, level: DbLevel) -> DbIndex {
        DbIndex(self.len() - level.0 - 1)
    }

    pub fn index_to_level(&self, index: DbIndex) -> DbLevel {
        DbLevel(self.len() - index.0 - 1)
    }
}

impl Context {
    pub fn get_type(&self, index: DbIndex, registry: &mut NodeRegistry) -> NormalFormId {
        let level = self.index_to_level(index);
        if level == TYPE1_LEVEL {
            panic!("Type1 has no type. We may add support for infinite type hierarchies in the future. However, for now, Type1 is the \"limit\" type.");
        }
        let out = self.local_type_stack[level.0]
            .type_id
            .upshift(index.0 + 1, registry);
        out
    }

    pub fn get_definition(
        &self,
        index: DbIndex,
        registry: &mut NodeRegistry,
    ) -> ContextEntryDefinition {
        let level = self.index_to_level(index);
        self.local_type_stack[level.0]
            .definition
            .upshift(index.0 + 1, registry)
    }

    pub fn get_visibility(&self, index: DbIndex) -> Visibility {
        let level = self.index_to_level(index);
        let definition = self.local_type_stack[level.0].definition;
        match definition {
            ContextEntryDefinition::Uninterpreted => Visibility(ModScope::Global),
            ContextEntryDefinition::Alias { visibility, .. } => visibility,
            ContextEntryDefinition::Adt { visibility, .. } => visibility,
            ContextEntryDefinition::Variant { visibility, .. } => visibility,
        }
    }
}

impl SubstituteInPlaceAndGetNoOpStatus for Context {
    fn subst_in_place_and_get_status(
        &mut self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> WasSyntacticNoOp {
        let mut was_no_op = WasSyntacticNoOp(true);
        for i in NUMBER_OF_BUILTIN_ENTRIES..self.len() {
            let level = DbLevel(i);
            was_no_op &= self.subst_entry_in_place(level, substitution, state);
        }
        was_no_op
    }
}

impl Context {
    fn subst_entry_in_place(
        &mut self,
        level: DbLevel,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> WasSyntacticNoOp {
        self.subst_entry_type_id_in_place(level, substitution, state)
            & self.subst_entry_definition_in_place(level, substitution, state)
    }

    fn subst_entry_type_id_in_place(
        &mut self,
        level: DbLevel,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> WasSyntacticNoOp {
        let shift_amount = self.level_to_index(level).0 + 1;

        let (substituted_type_id, was_no_op) = {
            let original_type_id = self.local_type_stack[level.0].type_id.raw();
            let substituted_type_id = original_type_id
                .upshift(shift_amount, state.registry)
                .subst(substitution, state)
                .downshift(shift_amount, state.registry);
            let was_no_op = WasSyntacticNoOp(substituted_type_id == original_type_id);
            (substituted_type_id, was_no_op)
        };
        self.local_type_stack[level.0].type_id = {
            let mut context = self.clone_slice(level);
            evaluate_well_typed_expression(
                &mut State {
                    file_tree: state.file_tree,
                    substitution_context: state.substitution_context,
                    registry: state.registry,
                    equality_checker: state.equality_checker,
                    warnings: state.warnings,
                    required_transparency_for_substitution: state
                        .required_transparency_for_substitution,
                    context: &mut context,
                },
                substituted_type_id,
            )
        };

        was_no_op
    }

    fn subst_entry_definition_in_place(
        &mut self,
        level: DbLevel,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> WasSyntacticNoOp {
        let shift_amount = self.level_to_index(level).0 + 1;

        let original_definition = self.local_type_stack[level.0].definition;
        let (new_definition, was_no_op) = match original_definition {
            ContextEntryDefinition::Alias {
                value_id,
                visibility,
                transparency,
            } => {
                let substituted = value_id
                    .raw()
                    .upshift(shift_amount, state.registry)
                    .subst(substitution, state)
                    .downshift(shift_amount, state.registry);
                let was_no_op = WasSyntacticNoOp(substituted == value_id.raw());
                let new_definition = {
                    let mut context = self.clone_slice(level);
                    ContextEntryDefinition::Alias {
                        value_id: evaluate_well_typed_expression(
                            &mut State {
                                file_tree: state.file_tree,
                                substitution_context: state.substitution_context,
                                registry: state.registry,
                                equality_checker: state.equality_checker,
                                warnings: state.warnings,
                                required_transparency_for_substitution: state
                                    .required_transparency_for_substitution,
                                context: &mut context,
                            },
                            substituted,
                        ),
                        visibility,
                        transparency,
                    }
                };
                (new_definition, was_no_op)
            }
            ContextEntryDefinition::Adt { .. }
            | ContextEntryDefinition::Variant { .. }
            | ContextEntryDefinition::Uninterpreted => {
                (original_definition, WasSyntacticNoOp(true))
            }
        };
        self.local_type_stack[level.0].definition = new_definition;

        was_no_op
    }
}

impl Context {
    pub(crate) fn clone_slice(&self, excl_upper_bound: DbLevel) -> Context {
        Context {
            local_type_stack: self.local_type_stack[0..excl_upper_bound.0].to_vec(),
        }
    }
}
