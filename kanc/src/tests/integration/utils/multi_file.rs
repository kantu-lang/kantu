use super::*;

use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Clone, Copy, Debug)]
pub struct ProjectPath<'a> {
    /// `checked_unadjusted_pack_yscl_path` should **always** be created using the
    /// `checked_path!` macro.
    pub checked_unadjusted_pack_yscl_path: &'a str,

    /// `callee_file_path` should **always** be `file!()`.
    ///
    /// This is so it will be consistent with `checked_unadjusted_pack_yscl_path`.
    ///
    /// The reason we make `callee_file_path` a parameter rather than simply
    /// hardcoding `file!()` in the function is because the value
    /// of `file!()` will change depending on where it is written.
    /// The value of `checked_unadjusted_pack_yscl_path` is relative to the
    /// calling file, so the value of `callee_file_path` must also be relative to the calling file.
    /// Thus, we cannot hardcode `file!()` in the function definition,
    /// and must instead require the caller to pass it in as an argument.
    pub callee_file_path: &'a str,
}

pub fn get_files_and_file_tree(project_path: ProjectPath) -> (Vec<simplified_ast::File>, FileTree) {
    let ProjectPath {
        callee_file_path,
        checked_unadjusted_pack_yscl_path,
    } = project_path;
    let adjusted_pack_yscl_path = Path::new(callee_file_path)
        .parent()
        .unwrap()
        .join(checked_unadjusted_pack_yscl_path);
    let (root_file, root_file_path) = {
        let root_file_path = adjusted_pack_yscl_path.parent().unwrap().join("src/mod.k");
        let src = fs::read_to_string(&root_file_path).expect("Failed to open file");
        let root_file = lex_and_parse_file(&src, FileId(0));
        (root_file, root_file_path)
    };

    let root_file_id = root_file.id;
    let mut files_and_paths = vec![(root_file, root_file_path)];
    let mut file_tree = FileTree::from_root(root_file_id);
    parse_children_then_add(&mut files_and_paths, &mut file_tree, root_file_id);
    let files = files_and_paths
        .into_iter()
        .map(|(file, _)| file)
        .collect::<Vec<_>>();
    (files, file_tree)
}

fn lex_and_parse_file(src: &str, id: FileId) -> simplified_ast::File {
    let tokens = lex(src).expect("Lexing failed");
    let file = parse_file(tokens, id).expect("Parsing failed");
    simplify_file(file).expect("AST Simplification failed")
}

fn parse_children_then_add(
    files: &mut Vec<(simplified_ast::File, PathBuf)>,
    tree: &mut FileTree,
    file_id: FileId,
) {
    let (file, file_path) = files
        .iter()
        .find(|(file, _)| file.id == file_id)
        .expect("Invalid file_id");
    let mod_names = get_mod_names(file);
    let file_path = file_path.clone();

    for mod_name in &mod_names {
        let child_file_id = get_unused_file_id(files);

        let (child_src, child_path) = {
            if !file_path.ends_with("mod.k") {
                panic!("{:?} cannot have submodules.", file_path);
            }
            let child_leaf_file_path = file_path
                .parent()
                .unwrap()
                .join(mod_name.src_str())
                .with_extension("k");
            let child_nonleaf_file_path = file_path
                .parent()
                .unwrap()
                .join(mod_name.src_str())
                .join("mod.k");

            let file_res = fs::read_to_string(&child_leaf_file_path)
                .map(|src| (src, child_leaf_file_path))
                .or_else(|_| {
                    fs::read_to_string(&child_nonleaf_file_path)
                        .map(|src| (src, child_nonleaf_file_path))
                });
            match file_res {
                Ok(file_res) => file_res,
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => continue,
                Err(err) => panic!("Could not read file: {:?}", err),
            }
        };
        tree.add_child(file_id, mod_name, child_file_id)
            .expect("Multiple modules with same name.");
        let child_file = lex_and_parse_file(&child_src, child_file_id);
        files.push((child_file, child_path));

        parse_children_then_add(files, tree, child_file_id);
    }
}

fn get_unused_file_id(files: &[(simplified_ast::File, PathBuf)]) -> FileId {
    let max_raw = files.iter().map(|(file, _)| file.id.0).max().unwrap_or(0);
    FileId(max_raw + 1)
}

fn get_mod_names(file: &simplified_ast::File) -> Vec<IdentifierName> {
    let mut mod_names = vec![];
    for item in &file.items {
        if let simplified_ast::FileItem::Mod(mod_) = item {
            mod_names.push(mod_.name.name.clone());
        }
    }
    mod_names
}
