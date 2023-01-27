use super::*;

const NUMBER_OF_BUILTIN_ENTRIES: usize = 2;

#[derive(Debug, Clone)]
pub struct Context {
    stack: Vec<ContextEntryDefinition>,
}

#[derive(Clone, Debug)]
pub enum ContextEntryDefinition {
    /// Algebraic data type
    Adt(&'a TypeStatement<'a>),
    Variant(&'a Variant<'a>),
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
            stack: builtins.to_vec(),
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
        self.stack.truncate(self.len() - n);
    }

    pub fn push(&mut self, entry: ContextEntryDefinition) {
        self.stack.push(entry);
    }

    pub fn push_n_uninterpreted(&mut self, n: usize) {
        for _ in 0..n {
            self.push(ContextEntryDefinition::Uninterpreted);
        }
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }
}

impl Context {
    pub fn index_to_level(&self, index: DbIndex) -> DbLevel {
        DbLevel(self.len() - index.0 - 1)
    }
}

impl Context {
    pub fn get_definition(&self, index: DbIndex) -> &ContextEntryDefinition {
        let level = self.index_to_level(index);
        &self.stack[level.0]
    }
}

impl Context {
    pub fn clone_up_to_excl(&self, index: DbIndex) -> Self {
        let level = self.index_to_level(index);
        Self {
            stack: self.stack[..level.0].to_vec(),
        }
    }
}
