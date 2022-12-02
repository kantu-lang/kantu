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

    pub fn push(&mut self, entry: ContextEntry) {
        self.stack.push(entry);
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
