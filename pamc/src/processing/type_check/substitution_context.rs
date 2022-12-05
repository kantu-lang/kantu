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

impl SubstitutionContext {
    pub fn get_adjusted_substitutions(
        &self,
        registry: &mut NodeRegistry,
        current_context_len: usize,
    ) -> Result<Vec<DynamicSubstitution>, ContextAndSubstitutionContextOutOfSyncError> {
        let mut out = vec![];
        for entry in self.stack.iter() {
            if entry.context_len > current_context_len {
                return Err(ContextAndSubstitutionContextOutOfSyncError {
                    current_context_len,
                    context_len_at_time_of_pushing_entry: entry.context_len,
                });
            }
            for substitution in entry.unadjusted_substitutions.iter() {
                out.push(substitution.upshift(current_context_len - entry.context_len, registry));
            }
        }
        Ok(out)
    }
}

#[derive(Debug, Clone)]
pub struct ContextAndSubstitutionContextOutOfSyncError {
    pub current_context_len: usize,
    pub context_len_at_time_of_pushing_entry: usize,
}
