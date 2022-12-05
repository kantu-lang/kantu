use super::*;

#[derive(Debug, Clone)]
pub struct SubstitutionContext {
    stack: Vec<SubstitutionContextEntry>,
}

#[derive(Clone, Debug)]
pub struct SubstitutionContextEntry {
    pub context_len: usize,
    pub unadjusted_substitutions: Vec<DynamicSubstitution>,
}

impl SubstitutionContext {
    pub fn empty() -> Self {
        Self { stack: vec![] }
    }
}

impl SubstitutionContext {
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn push(&mut self, entry: SubstitutionContextEntry) {
        self.stack.push(entry);
    }

    pub fn pop(&mut self) {
        self.stack
            .pop()
            .expect("Tried to pop an empty substitution context");
    }

    pub fn truncate(&mut self, new_len: usize) {
        if new_len > self.len() {
            panic!(
                "Tried to truncate a substitution context with only {} elements to a new length of {} elements",
                self.len(),
                new_len
            );
        }
        self.stack.truncate(new_len);
    }
}
