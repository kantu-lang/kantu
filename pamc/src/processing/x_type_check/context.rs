use super::*;

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
    },
    /// Algebraic data type
    Adt {
        variant_name_list_id: ListId<NodeId<Identifier>>,
    },
    Variant {
        name_id: NodeId<Identifier>,
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
                    vec![Identifier {
                        id: dummy_id(),
                        name: IdentifierName::Standard("Type2".to_owned()),
                        start: None,
                    }],
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
                    vec![Identifier {
                        id: dummy_id(),
                        name: IdentifierName::Standard("Type1".to_owned()),
                        start: None,
                    }],
                    DbIndex(0),
                ),
            ));
            ContextEntry {
                type_id: type0_type_id,
                definition: ContextEntryDefinition::Uninterpreted,
            }
        };
        Self {
            local_type_stack: vec![type1_entry, type0_entry],
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

    pub fn push(&mut self, entry: ContextEntry) {
        self.local_type_stack.push(entry);
    }

    pub fn len(&self) -> usize {
        self.local_type_stack.len()
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
    fn level_to_index(&self, level: DbLevel) -> DbIndex {
        DbIndex(self.len() - level.0 - 1)
    }

    fn index_to_level(&self, index: DbIndex) -> DbLevel {
        DbLevel(self.len() - index.0 - 1)
    }
}

impl Context {
    pub fn get_type(&self, index: DbIndex, registry: &mut NodeRegistry) -> NormalFormId {
        let level = self.index_to_level(index);
        if level == TYPE1_LEVEL {
            panic!("Type1 has no type. We may add support for infinite type hierarchies in the future. However, for now, Type1 is the \"limit\" type.");
        }
        self.local_type_stack[level.0]
            .type_id
            .upshift(index.0 + 1, registry)
    }

    pub fn get_definition(
        &self,
        index: DbIndex,
        registry: &mut NodeRegistry,
    ) -> ContextEntryDefinition {
        let level = self.index_to_level(index);
        if level == TYPE1_LEVEL {
            panic!("Type1 has no type. We may add support for infinite type hierarchies in the future. However, for now, Type1 is the \"limit\" type.");
        }
        self.local_type_stack[level.0]
            .definition
            .upshift(index.0 + 1, registry)
    }
}

impl Substitute for Context {
    type Output = Self;

    fn subst(self, _substitution: Substitution, _state: &mut ContextlessState) -> (Self, WasNoOp) {
        unimplemented!();
    }
}
