use super::*;

fn expect_type_check_error(
    project_path: ProjectPath,
    panicker: impl FnOnce(&NodeRegistry, TypeCheckError),
) {
    let (files, file_tree) = get_files_and_file_tree(project_path);
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
    let err = type_check_file_items(&file_tree, &mut registry, file_item_list_id)
        .expect_err("Type checking unexpected succeeded");
    panicker(&registry, err);
}

// TODO: Fix
#[ignore]
#[test]
fn leaky_let_type() {
    expect_type_check_error(
        ProjectPath {
            callee_file_path: file!(),
            checked_unadjusted_pack_omlet_path: checked_path!(
                "../../../sample_code/should_fail/multi_file/type_check/leaky_let_type/pack.omlet"
            ),
        },
        |_, err| match err {
            _ => panic!("Unexpected error: {:?}", err),
        },
    );
}
