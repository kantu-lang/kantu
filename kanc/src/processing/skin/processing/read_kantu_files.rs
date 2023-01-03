use crate::data::{file_tree::FileTree, unsimplified_ast as unsimplified};

use super::super::data::{error::ReadKantuFilesError, options::CompilerOptions};

pub fn read_kantu_files(
    options: &CompilerOptions,
) -> Result<(Vec<unsimplified::File>, FileTree), ReadKantuFilesError> {
    unimplemented!()
}
