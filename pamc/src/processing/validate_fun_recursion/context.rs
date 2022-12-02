use super::*;

#[derive(Clone, Debug)]
pub struct Context {
    stack: Vec<ContextEntry>,
}

#[derive(Clone, Copy, Debug)]
pub enum ContextEntry {
    Substruct { superstruct_db_level: DbLevel },
    Fun(ReferenceRestriction),
    NoInformation,
}

#[derive(Clone, Copy, Debug)]
pub enum ReferenceRestriction {
    MustCallWithSubstruct {
        superstruct_db_level: DbLevel,
        arg_position: usize,
    },
    CannotCall,
}

impl Context {
    pub fn new() -> Self {
        Self {
            stack: vec![ContextEntry::NoInformation, ContextEntry::NoInformation],
        }
    }
}

impl Context {
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn index_to_level(&self, index: DbIndex) -> DbLevel {
        DbLevel(self.len() - index.0 - 1)
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

    /// Panics if `new_len > self.len()`.
    pub fn truncate(&mut self, new_len: usize) {
        if new_len > self.len() {
            panic!(
                "Tried to truncate a context with only {} elements to {} elements",
                self.len(),
                new_len
            );
        }
        self.stack.truncate(new_len);
    }

    /// We effectively return `()`, but the reason we use the `Result` type we is to
    /// encourage the caller to only use `push` inside a function that returns `Result<_, Tainted<_>>`.
    pub fn push(&mut self, entry: ContextEntry) -> Result<(), Tainted<Infallible>> {
        self.stack.push(entry);
        Ok(())
    }
}

impl Context {
    pub fn reference_restriction(&self, index: DbIndex) -> Option<ReferenceRestriction> {
        let level = self.index_to_level(index);
        let entry = self.stack[level.0];
        match entry {
            ContextEntry::Substruct { .. } => None,
            ContextEntry::Fun(restriction) => Some(restriction),
            ContextEntry::NoInformation => None,
        }
    }

    pub fn is_left_strict_substruct_of_right(&self, left: DbLevel, right: DbLevel) -> bool {
        left != right && self.is_left_inclusive_substruct_of_right(left, right)
    }

    fn is_left_inclusive_substruct_of_right(&self, left: DbLevel, right: DbLevel) -> bool {
        let mut current = left;
        loop {
            if current == right {
                return true;
            }
            let entry = self.stack[current.0];
            match entry {
                ContextEntry::Substruct {
                    superstruct_db_level,
                } => {
                    current = superstruct_db_level;
                    continue;
                }
                ContextEntry::Fun(_) | ContextEntry::NoInformation => {
                    return false;
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Tainted<T>(T);

impl<T> Tainted<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }
}

impl From<Tainted<Infallible>> for Infallible {
    fn from(impossible: Tainted<Infallible>) -> Infallible {
        impossible.0
    }
}

pub fn untaint_err<In, Out, Err, F>(
    context: &mut Context,
    registry: &mut NodeRegistry,
    input: In,
    f: F,
) -> Result<Out, Err>
where
    F: FnOnce(&mut Context, &mut NodeRegistry, In) -> Result<Out, Tainted<Err>>,
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
