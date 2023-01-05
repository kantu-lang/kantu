use super::super::data::{error::ReadKantuFilesError, options::CompilerOptions};

use crate::{
    data::{file_tree::FileTree, unsimplified_ast as unsimplified, FileId},
    processing::{lex::lex, parse::parse_file},
};

use std::{
    fs,
    path::{Path, PathBuf},
};

// TODO: DRY (slighlty altered but mostly copied from `crate::tests::integration::utils`).
// We could probably make the utils version depend on this one.
pub fn read_kantu_files(
    options: &CompilerOptions,
) -> Result<(Vec<unsimplified::File>, FileTree), ReadKantuFilesError> {
    let (root_file, root_file_path) = {
        let pack_yscl_dir = options
            .pack_yscl_abs_path
            .parent()
            .expect("pack.yscl path should have parent");
        let root_file_path = pack_yscl_dir.join("src/mod.k");
        let root_file = lex_and_parse_file(&root_file_path, FileId(0))?;
        (root_file, root_file_path)
    };

    let root_file_id = root_file.id;
    let mut files_and_paths = vec![(root_file, root_file_path)];
    let mut file_tree = FileTree::from_root(root_file_id);
    parse_children_then_add(&mut files_and_paths, &mut file_tree, root_file_id)?;
    let files = files_and_paths
        .into_iter()
        .map(|(file, _)| file)
        .collect::<Vec<_>>();
    Ok((files, file_tree))
}

fn lex_and_parse_file(path: &Path, id: FileId) -> Result<unsimplified::File, ReadKantuFilesError> {
    let src = fs::read_to_string(path)
        .map_err(|raw_err| ReadKantuFilesError::CannotReadFile(path.to_path_buf(), raw_err))?;
    let tokens =
        lex(&src).map_err(|raw_err| ReadKantuFilesError::LexError(path.to_path_buf(), raw_err))?;
    parse_file(tokens, id)
        .map_err(|raw_err| ReadKantuFilesError::ParseError(path.to_path_buf(), raw_err))
}

fn parse_children_then_add(
    files: &mut Vec<(unsimplified::File, PathBuf)>,
    tree: &mut FileTree,
    file_id: FileId,
) -> Result<(), ReadKantuFilesError> {
    let (file, file_path) = files
        .iter()
        .find(|(file, _)| file.id == file_id)
        .expect("Impossible: file_id is invalid");
    let mod_statements = get_mod_statements(file);
    let file_path = file_path.clone();
    let file_dir = file_path
        .parent()
        .expect("Impossible: File path should always have a parent");

    if let Some(mod_statement) = mod_statements.first() {
        if !file_path.ends_with("mod.k") {
            return Err(ReadKantuFilesError::NonModDotKHasSubmodules(
                file_path,
                mod_statement.clone(),
            ));
        }
    }

    for mod_statement in &mod_statements {
        let mod_name = &mod_statement.name.name;
        let child_file_id = get_unused_file_id(files);

        let child_path = {
            let child_leaf_file_path = file_dir.join(mod_name.src_str()).with_extension("k");
            let child_nonleaf_file_path = file_dir.join(mod_name.src_str()).join("mod.k");
            match (
                child_leaf_file_path.is_file(),
                child_nonleaf_file_path.is_file(),
            ) {
                (true, false) => child_leaf_file_path,
                (false, true) => child_nonleaf_file_path,
                (true, true) => {
                    return Err(ReadKantuFilesError::ModHasMultipleFiles(
                        child_leaf_file_path,
                        child_nonleaf_file_path,
                    ))
                }
                (false, false) => {
                    // The binder will catch this later.
                    continue;
                }
            }
        };
        tree.add_child(file_id, mod_name, child_file_id)
            .map_err(|_| {
                ReadKantuFilesError::MultipleModsWithSameName(file_path.clone(), mod_name.clone())
            })?;
        let child_file = lex_and_parse_file(&child_path, child_file_id)?;
        files.push((child_file, child_path));

        parse_children_then_add(files, tree, child_file_id)?;
    }
    Ok(())
}

fn get_unused_file_id(files: &[(unsimplified::File, PathBuf)]) -> FileId {
    let max_raw = files.iter().map(|(file, _)| file.id.0).max().unwrap_or(0);
    FileId(max_raw + 1)
}

fn get_mod_statements(file: &unsimplified::File) -> Vec<unsimplified::ModStatement> {
    let mut mod_names = vec![];
    for item in &file.items {
        if let unsimplified::FileItem::Mod(mod_) = item {
            mod_names.push(mod_.clone());
        }
    }
    mod_names
}
