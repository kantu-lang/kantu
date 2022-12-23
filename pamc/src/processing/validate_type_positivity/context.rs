use super::*;

const NUMBER_OF_BUILTIN_ENTRIES: usize = 2;

#[derive(Debug, Clone)]
pub struct Context {
    local_stack: Vec<ContextEntryDefinition>,
}

#[derive(Clone, Debug)]
pub enum ContextEntryDefinition {
    /// Algebraic data type
    Adt(NodeId<TypeStatement>),
    Variant(NodeId<Variant>),
    Uninterpreted,
}

impl Context {
    pub fn with_builtins() -> Self {
        // We should will never retrieve the type of `Type1`, since it is undefined.
        // However, we need to store _some_ object in the stack, so that the indices
        // of the other types are correct.
        let type1_def = ContextEntryDefinition::Uninterpreted;
        let type0_def = ContextEntryDefinition::Uninterpreted;
        let builtins: [ContextEntryDefinition; NUMBER_OF_BUILTIN_ENTRIES] = [type1_def, type0_def];
        Self {
            local_stack: builtins.to_vec(),
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
        self.local_stack.truncate(self.len() - n);
    }

    /// We effectively return `()`, but the reason we use the `Result` type we is to
    /// encourage the caller to only use `push` inside a function that returns `Result<_, Tainted<_>>`.
    pub fn push(&mut self, entry: ContextEntryDefinition) {
        self.local_stack.push(entry);
    }

    pub fn len(&self) -> usize {
        self.local_stack.len()
    }
}

// TODO: Delete
// #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
// pub struct Tainted<T>(T);

// impl<T> Tainted<T> {
//     pub fn new(value: T) -> Self {
//         Self(value)
//     }
// }

// pub fn tainted_err<T, E>(err: E) -> Result<T, Tainted<E>> {
//     Err(Tainted::new(err))
// }

// impl<T> Tainted<T> {
//     pub fn map<U, F>(self, f: F) -> Tainted<U>
//     where
//         F: FnOnce(T) -> U,
//     {
//         Tainted(f(self.0))
//     }
// }

// impl From<Tainted<Infallible>> for Infallible {
//     fn from(impossible: Tainted<Infallible>) -> Infallible {
//         impossible.0
//     }
// }

// pub fn untaint_err<In, Out, Err, F>(
//     context: &mut Context,
//     registry: &NodeRegistry,
//     input: In,
//     f: F,
// ) -> Result<Out, Err>
// where
//     F: FnOnce(&mut Context, &NodeRegistry, In) -> Result<Out, Tainted<Err>>,
// {
//     let original_len = context.len();
//     let result = f(context, registry, input);
//     match result {
//         Ok(ok) => Ok(ok),
//         Err(err) => {
//             context.truncate(original_len);
//             Err(err.0)
//         }
//     }
// }

// pub type WithPushWarning<T> = Result<T, Tainted<Infallible>>;
// pub type PushWarning = WithPushWarning<()>;

// pub fn with_push_warning<T>(value: T) -> WithPushWarning<T> {
//     Ok(value)
// }

// TODO: Delete
// impl Context {
//     fn truncate(&mut self, new_len: usize) {
//         if new_len > self.len() {
//             panic!(
//                 "Tried to truncate a context with {} elements to {} elements",
//                 self.len(),
//                 new_len
//             );
//         }
//         self.local_stack.truncate(new_len);
//     }
// }

// impl Context {
//     pub fn level_to_index(&self, level: DbLevel) -> DbIndex {
//         DbIndex(self.len() - level.0 - 1)
//     }

//     pub fn index_to_level(&self, index: DbIndex) -> DbLevel {
//         DbLevel(self.len() - index.0 - 1)
//     }
// }

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
