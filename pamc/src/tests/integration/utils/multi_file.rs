use super::*;

use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn get_files_and_file_tree(
    file_path: &str,
    checked_unadjusted_pack_omlet_path: &str,
) -> (Vec<simplified_ast::File>, FileTree) {
    let adjusted_pack_omlet_path = Path::new(file_path)
        .parent()
        .unwrap()
        .join(checked_unadjusted_pack_omlet_path);
    let (root_file, root_file_path) = {
        let root_file_path = adjusted_pack_omlet_path
            .parent()
            .unwrap()
            .join("src/mod.ph");
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
        tree.add_child(file_id, mod_name, child_file_id)
            .expect("Multiple modules with same name.");

        let (child_src, child_path) = {
            if !file_path.ends_with("mod.ph") {
                panic!("{:?} cannot have submodules.", file_path);
            }
            let child_leaf_file_path = file_path
                .parent()
                .unwrap()
                .join(mod_name.src_str())
                .with_extension("ph");
            let child_nonleaf_file_path = file_path
                .parent()
                .unwrap()
                .join(mod_name.src_str())
                .join("mod.ph");

            fs::read_to_string(&child_leaf_file_path)
                .map(|src| (src, child_leaf_file_path))
                .or_else(|_| {
                    fs::read_to_string(&child_nonleaf_file_path)
                        .map(|src| (src, child_nonleaf_file_path))
                })
                .expect("Failed to open file")
        };
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