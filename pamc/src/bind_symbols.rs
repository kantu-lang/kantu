use crate::{bound_ast as bound, unbound_ast as unbound};

#[derive(Clone, Debug)]
pub enum BindError {}

#[derive(Clone, Debug)]
pub struct SymbolDatabase {}

/// The returned vector of files has an arbitrary order--it is **NOT**
/// guaranteed to have the same order as the input vector.
pub fn bind_symbols(
    files: Vec<unbound::File>,
) -> Result<(SymbolDatabase, Vec<bound::File>), BindError> {
    let files = sort_files_by_dependency(files);
    unimplemented!();
}

fn sort_files_by_dependency(files: Vec<unbound::File>) -> Vec<unbound::File> {
    // TODO: Sort by dependency once we support `use` statements.
    files
}
