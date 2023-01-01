use super::*;

fn expect_success_with_no_warnings(project_path: ProjectPath) {
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
    let warnings = type_check_file_items(&file_tree, &mut registry, file_item_list_id)
        .expect("Type checking failed");
    assert_eq!(0, warnings.len(), "One or more warnings were emitted");
    let _js_ast = JavaScript::generate_code(&registry, file_item_list_id.raw())
        .expect("Code generation failed");
}

#[test]
fn factorial() {
    expect_success_with_no_warnings(ProjectPath {
        callee_file_path: file!(),
        checked_unadjusted_pack_omlet_path: checked_path!(
            "../../../sample_code/should_succeed/multi_file/no_warnings/factorial/pack.omlet"
        ),
    });
}

#[test]
fn import_merging() {
    expect_success_with_no_warnings(ProjectPath {
        callee_file_path: file!(),
        checked_unadjusted_pack_omlet_path: checked_path!(
            "../../../sample_code/should_succeed/multi_file/no_warnings/import_merging/pack.omlet"
        ),
    });
}

#[test]
fn alternate_name() {
    expect_success_with_no_warnings(ProjectPath {
        callee_file_path: file!(),
        checked_unadjusted_pack_omlet_path: checked_path!(
            "../../../sample_code/should_succeed/multi_file/no_warnings/alternate_name/pack.omlet"
        ),
    });
}

#[test]
fn plus_commutative() {
    expect_success_with_no_warnings(ProjectPath {
        callee_file_path: file!(),
        checked_unadjusted_pack_omlet_path: checked_path!(
            "../../../sample_code/should_succeed/multi_file/no_warnings/plus_commutative/pack.omlet"
        ),
    });
}

#[test]
fn no_clash_because_priv() {
    expect_success_with_no_warnings(ProjectPath{
        callee_file_path:   file!(),
        checked_unadjusted_pack_omlet_path:   checked_path!(
            "../../../sample_code/should_succeed/multi_file/no_warnings/no_clash_because_priv/pack.omlet"
        ),
   } );
}

#[test]
fn opaque_nat() {
    expect_success_with_no_warnings(ProjectPath {
        callee_file_path: file!(),
        checked_unadjusted_pack_omlet_path: checked_path!(
            "../../../sample_code/should_succeed/multi_file/no_warnings/opaque_nat/pack.omlet"
        ),
    });
}

#[test]
fn let_not_leaky_because_its_opaque() {
    expect_success_with_no_warnings(ProjectPath {
        callee_file_path: file!(),
        checked_unadjusted_pack_omlet_path: checked_path!(
            "../../../sample_code/should_succeed/multi_file/no_warnings/let_not_leaky_because_its_opaque/pack.omlet"
        ),
    });
}

#[test]
fn pack_relative_with_alias() {
    expect_success_with_no_warnings(ProjectPath {
        callee_file_path: file!(),
        checked_unadjusted_pack_omlet_path: checked_path!(
            "../../../sample_code/should_succeed/multi_file/no_warnings/pack_relative_with_alias/pack.omlet"
        ),
    });
}

#[test]
fn identity_eq_transparent() {
    expect_success_with_no_warnings(ProjectPath {
        callee_file_path: file!(),
        checked_unadjusted_pack_omlet_path: checked_path!(
            "../../../sample_code/should_succeed/multi_file/no_warnings/identity_eq_transparent/pack.omlet"
        ),
    });
}
