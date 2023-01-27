pub use crate::data::{
    bind_error::*, file_id::*, file_tree::FileTree,
    fun_recursion_validation_result::IllegalFunRecursionError, light_ast::*, simplified_ast,
    token::TokenKind, type_positivity_validation_result::TypePositivityError,
};
pub use crate::processing::{
    bind_type_independent::bind_files,
    bind_type_independent::{BindError, OwnedSymbolSource},
    generate_code::{targets::javascript::JavaScript, CompileTarget},
    lex::lex,
    lex::LexError,
    lighten_ast::register_file_items,
    parse::parse_file,
    parse::ParseError,
    simplify_ast::simplify_file,
    simplify_ast::SimplifyAstError,
    skin::processing::test_utils::dry_run::run_pipeline_without_writing_files,
    test_utils::{
        expand_lightened::{expand_expression, expand_match, expand_match_case},
        format::{format_expression, format_match, format_match_case, FormatOptions},
    },
    type_check::TypeCheckError,
    type_check::{
        type_check_file_items, NormalFormAssertionWarning, TypeAssertionWarning,
        TypeCheckFailureReason, TypeCheckWarning,
    },
    validate_fun_recursion::validate_fun_recursion_in_file_items,
    validate_type_positivity::validate_type_positivity_in_file_items,
    validate_variant_return_types::validate_variant_return_types_in_file_items,
};

pub use std::{
    num::NonZeroUsize,
    path::{Path, PathBuf},
};

pub const DUMMY_EXEC_PATH: &str = "<CARGO_MANIFEST_DIR>/target/debug/kanc";

pub fn get_manifest_path_and_backslash_normalized_output(args: Vec<&str>) -> String {
    let args: Vec<String> = args.into_iter().map(|s| s.to_string()).collect();
    let output = match run_pipeline_without_writing_files(&args) {
        Ok(s) => s,
        Err(s) => s,
    };
    normalize_manifest_path_and_backslashes(&output)
}

fn normalize_manifest_path_and_backslashes(s: &str) -> String {
    let cargo_manifest_dir = env!("CARGO_MANIFEST_DIR");
    s.replace(cargo_manifest_dir, "<CARGO_MANIFEST_DIR>")
        .replace("\\", "/")
}

pub fn concat_paths(file_path: &str, checked_rel_path: &str) -> String {
    Path::new(file_path)
        .parent()
        .unwrap()
        .join(checked_rel_path)
        .to_string_lossy()
        .to_string()
}
