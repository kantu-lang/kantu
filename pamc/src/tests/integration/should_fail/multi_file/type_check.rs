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

fn expect_type_mismatch_error(
    project_path: ProjectPath,
    expected_expression_src: &str,
    expected_expected_type_src: &str,
    expected_actual_type_src: &str,
) {
    expect_type_check_error(project_path, |registry, err| match err {
        TypeCheckError::TypeMismatch {
            expression_id,
            expected_type_id,
            actual_type_id,
        } => {
            let actual_expression_src = format_expression(
                &expand_expression(registry, expression_id),
                0,
                &FormatOptions {
                    ident_size_in_spaces: 4,
                    print_db_indices: false,
                    print_fun_body_status: false,
                },
            );
            assert_eq_up_to_white_space(&actual_expression_src, expected_expression_src);

            let actual_expected_type_src = format_expression(
                &expand_expression(registry, expected_type_id.raw()),
                0,
                &FormatOptions {
                    ident_size_in_spaces: 4,
                    print_db_indices: false,
                    print_fun_body_status: false,
                },
            );
            assert_eq_up_to_white_space(&actual_expected_type_src, expected_expected_type_src);

            let actual_actual_type_src = format_expression(
                &expand_expression(registry, actual_type_id.raw()),
                0,
                &FormatOptions {
                    ident_size_in_spaces: 4,
                    print_db_indices: false,
                    print_fun_body_status: false,
                },
            );
            assert_eq_up_to_white_space(&actual_actual_type_src, expected_actual_type_src);
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

// TODO: Fix
#[ignore]
#[test]
fn insufficient_transparency() {
    expect_type_mismatch_error(
        ProjectPath {
            callee_file_path: file!(),
            checked_unadjusted_pack_omlet_path: checked_path!(
                "../../../sample_code/should_fail/multi_file/type_check/insufficient_transparency/pack.omlet"
            ),
        },
        "Eq.Refl(T, t,)",
        "Eq(T, t, identity(T, t,),)",
        "Eq(T, t, t,)",
    );
}
