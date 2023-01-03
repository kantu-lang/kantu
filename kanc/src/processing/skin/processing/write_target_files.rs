use super::super::data::{error::WriteTargetFilesError, options::CompilerOptions};

use std::path::PathBuf;

pub fn write_target_files(
    options: &CompilerOptions,
    files: Vec<(PathBuf, String)>,
) -> Result<(), WriteTargetFilesError> {
    unimplemented!()
}
