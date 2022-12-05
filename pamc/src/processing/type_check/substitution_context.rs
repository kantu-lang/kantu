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
