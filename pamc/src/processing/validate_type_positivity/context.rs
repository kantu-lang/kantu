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
    pub definition: ContextEntryDefinition,
}

#[derive(Clone, Debug)]
pub enum ContextEntryDefinition {
    // TODO: Delete
    // /// Algebraic data type
    // Adt {
    //     variant_name_list_id: Option<NonEmptyListId<NodeId<Identifier>>>,
    // },
    // Variant {
    //     name_id: NodeId<Identifier>,
    // },
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
            ContextEntry {
                definition: ContextEntryDefinition::Uninterpreted,
            }
        };
        let type0_entry = {
            ContextEntry {
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

pub fn untaint_err<In, Out, Err, F>(
    context: &mut Context,
    registry: &NodeRegistry,
    input: In,
    f: F,
) -> Result<Out, Err>
where
    F: FnOnce(&mut Context, &NodeRegistry, In) -> Result<Out, Tainted<Err>>,
{
    let original_len = context.len();
    let result = f(context, registry, input);
    match result {
        Ok(ok) => Ok(ok),
        Err(err) => {
            context.truncate(original_len);
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
    // TODO: Delete
    // pub fn get_definition(
    //     &self,
    //     index: DbIndex,
    //     registry: &mut NodeRegistry,
    // ) -> ContextEntryDefinition {
    //     let level = self.index_to_level(index);
    //     self.local_type_stack[level.0]
    //         .definition
    //         .upshift(index.0 + 1, registry)
    // }
}
