use super::*;

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
fn expect_type_check_error(
    file_path: &str,
    checked_unadjusted_pack_omlet_path: &str,
    panicker: impl FnOnce(&NodeRegistry, TypeCheckError),
) {
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
    let err = type_check_file_items(&mut registry, file_item_list_id)
        .expect_err("Type checking unexpected succeeded");
    panicker(&registry, err);
}

// TODO: Fix
#[ignore]
#[test]
fn leaky_let_type() {
    expect_type_check_error(
        file!(),
        checked_path!(
            "../../../sample_code/should_fail/multi_file/type_check/leaky_let_type/pack.omlet"
        ),
        |_, err| match err {
            _ => panic!("Unexpected error: {:?}", err),
        },
    );
}
