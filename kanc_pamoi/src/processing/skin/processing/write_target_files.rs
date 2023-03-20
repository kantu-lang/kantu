use super::super::data::{error::WriteTargetFilesError, options::CompilerOptions};

use std::{fs, path::PathBuf};

pub fn write_target_files(
    options: &CompilerOptions,
    files: Vec<(PathBuf, String)>,
) -> Result<(), WriteTargetFilesError> {
    if options.target_dir.exists() {
        if options.target_dir.is_dir() {
            fs::remove_dir_all(&options.target_dir).map_err(|raw_err| {
                WriteTargetFilesError::CannotRemoveTargetDir(options.target_dir.clone(), raw_err)
            })?;
        } else {
            return Err(WriteTargetFilesError::TargetDirExistsButIsNotDir(
                options.target_dir.clone(),
            ));
        }
    }

    fs::create_dir(&options.target_dir).map_err(|raw_err| {
        WriteTargetFilesError::CannotCreateTargetDir(options.target_dir.clone(), raw_err)
    })?;

    for (rel_path, content) in files {
        let abs_path = options.target_dir.join(rel_path);
        fs::write(&abs_path, content)
            .map_err(|raw_err| WriteTargetFilesError::CannotWriteFile(abs_path, raw_err))?;
    }

    Ok(())
}
