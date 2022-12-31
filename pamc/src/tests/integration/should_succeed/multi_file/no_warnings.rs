use super::*;

/// This macro takes a string literal as input.
///
/// If the string literal is a valid path to a file, it returns
/// the string literal as-is.
///
/// If the string literal is not a valid path to a file, it will
/// cause a compilation error.
#[macro_export]
macro_rules! checked_path {
    ($path_:literal) => {{
        let _ = include_str!($path_);
        $path_
    }};
}

/// `file_path` should **always** be `file!()`.
///
/// This is so it will be consistent with `checked_unadjusted_pack_omlet_path`.
///
/// The reason we make `file_path` a parameter rather than simply
/// hardcoding `file!()` in the function is because the value
/// of `file!()` will change depending on where it is written.
/// The value of `checked_unadjusted_pack_omlet_path` is relative to the
/// calling file, so the value of `file_path` must also be relative to the calling file.
/// Thus, we cannot hardcode `file!()` in the function definition,
/// and must instead require the caller to pass it in as an argument.
fn expect_success_with_no_warnings(file_path: &str, checked_unadjusted_pack_omlet_path: &str) {
    let (files, file_tree) = get_files_and_file_tree(file_path, checked_unadjusted_pack_omlet_path);
    let file_items = bind_files(file_tree.root(), files, &file_tree).expect("Binding failed");
    let mut registry = NodeRegistry::empty();
    let file_item_list_id = register_file_items(&mut registry, file_items);

    let file_item_list_id =
        validate_variant_return_types_in_file_items(&registry, file_item_list_id)
            .expect("Variant return type validation failed");
    let file_item_list_id = validate_fun_recursion_in_file_items(&mut registry, file_item_list_id)
        .expect("Fun recursion validation failed");
    let file_item_list_id =
        validate_type_positivity_in_file_items(&mut registry, file_item_list_id)
            .expect("Type positivity validation failed");
    let warnings =
        type_check_file_items(&mut registry, file_item_list_id).expect("Type checking failed");
    assert_eq!(0, warnings.len(), "One or more warnings were emitted");
    let _js_ast = JavaScript::generate_code(&registry, file_item_list_id.raw())
        .expect("Code generation failed");
}

#[test]
fn factorial() {
    expect_success_with_no_warnings(
        file!(),
        checked_path!(
            "../../../sample_code/should_succeed/multi_file/no_warnings/factorial/pack.omlet"
        ),
    );
}

#[test]
fn import_merging() {
    expect_success_with_no_warnings(
        file!(),
        checked_path!(
            "../../../sample_code/should_succeed/multi_file/no_warnings/import_merging/pack.omlet"
        ),
    );
}
