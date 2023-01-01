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

#[test]
fn leaky_let_type() {
    expect_type_check_error(
        ProjectPath {
            callee_file_path: file!(),
            checked_unadjusted_pack_omlet_path: checked_path!(
                "../../../sample_code/should_fail/multi_file/type_check/leaky_let_type/pack.omlet"
            ),
        },
        |registry, err| match err {
            TypeCheckError::LetStatementTypeContainsPrivateName(let_id, private_name_id) => {
                let let_statement = registry.get(let_id);
                let let_name = registry.get(let_statement.name_id);
                assert_eq!("_2", let_name.name.src_str());

                let private_name = registry.get(private_name_id);
                let private_name_components = registry.get_list(private_name.component_list_id);
                assert_eq!(
                    "Nat",
                    private_name_components
                        .iter()
                        .map(|&component_id| registry.get(component_id).name.src_str())
                        .collect::<Vec<_>>()
                        .join(".")
                );
            }
            _ => panic!("Unexpected error: {:?}", err),
        },
    );
}
