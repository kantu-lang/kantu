use super::*;

#[test]
fn hello_world() {
    let src = include_str!(
        "../../../sample_code/should_succeed/single_file/should_succeed_without_warnings/hello_world.ph"
    );
    expect_success_with_no_warnings(src);
}

#[test]
fn optional_commas() {
    let src = include_str!(
        "../../../sample_code/should_succeed/single_file/should_succeed_without_warnings/optional_commas.ph"
    );
    expect_success_with_no_warnings(src);
}

#[test]
fn empty_match_can_be_assigned_to_anything() {
    let src = include_str!("../../../sample_code/should_succeed/single_file/should_succeed_without_warnings/empty_match_can_be_assigned_to_anything.ph");
    expect_success_with_no_warnings(src);
}

#[test]
fn match_explosion() {
    let src = include_str!(
        "../../../sample_code/should_succeed/single_file/should_succeed_without_warnings/match_explosion.ph"
    );
    expect_success_with_no_warnings(src);
}

#[test]
fn coercionless_match() {
    let src = include_str!(
        "../../../sample_code/should_succeed/single_file/should_succeed_without_warnings/coercionless_match.ph"
    );
    expect_success_with_no_warnings(src);
}

#[test]
fn ill_typed_until_substituted() {
    let src = include_str!("../../../sample_code/should_succeed/single_file/should_succeed_without_warnings/ill_typed_until_substituted.ph");
    expect_success_with_no_warnings(src);
}

#[test]
fn forall() {
    let src = include_str!(
        "../../../sample_code/should_succeed/single_file/should_succeed_without_warnings/forall.ph"
    );
    expect_success_with_no_warnings(src);
}

#[test]
fn underscore() {
    let src = include_str!(
        "../../../sample_code/should_succeed/single_file/should_succeed_without_warnings/underscore.ph"
    );
    expect_success_with_no_warnings(src);
}

#[test]
fn plus_commutative() {
    let src = include_str!(
        "../../../sample_code/should_succeed/single_file/should_succeed_without_warnings/plus_commutative.ph"
    );
    expect_success_with_no_warnings(src);
}

#[test]
fn exists() {
    let src = include_str!(
        "../../../sample_code/should_succeed/single_file/should_succeed_without_warnings/exists.ph"
    );
    expect_success_with_no_warnings(src);
}

#[test]
fn comment() {
    let src = include_str!(
        "../../../sample_code/should_succeed/single_file/should_succeed_without_warnings/comment.ph"
    );
    expect_success_with_no_warnings(src);
}

#[test]
fn check() {
    let src = include_str!(
        "../../../sample_code/should_succeed/single_file/should_succeed_without_warnings/check.ph"
    );
    expect_success_with_no_warnings(src);
}

#[test]
fn labeled_params() {
    let src = include_str!(
        "../../../sample_code/should_succeed/single_file/should_succeed_without_warnings/labeled_params.ph"
    );
    expect_success_with_no_warnings(src);
}

#[test]
fn labeled_call_args() {
    let src = include_str!(
        "../../../sample_code/should_succeed/single_file/should_succeed_without_warnings/labeled_call_args.ph"
    );
    expect_success_with_no_warnings(src);
}

#[test]
fn misordered_labeled_args() {
    let src = include_str!(
        "../../../sample_code/should_succeed/single_file/should_succeed_without_warnings/misordered_labeled_args.ph"
    );
    expect_success_with_no_warnings(src);
}

#[test]
fn nullary_variant_with_call_return_type() {
    let src = include_str!(
        "../../../sample_code/should_succeed/single_file/should_succeed_without_warnings/nullary_variant_with_call_return_type.ph"
    );
    expect_success_with_no_warnings(src);
}

#[test]
fn misordered_match_case_params() {
    let src = include_str!(
        "../../../sample_code/should_succeed/single_file/should_succeed_without_warnings/misordered_match_case_params.ph"
    );
    expect_success_with_no_warnings(src);
}

#[test]
fn nat_s_injective() {
    let src = include_str!(
        "../../../sample_code/should_succeed/single_file/should_succeed_without_warnings/nat_s_injective.ph"
    );
    expect_success_with_no_warnings(src);
}

#[test]
fn color_c_injective_despite_misordered_args() {
    let src = include_str!(
        "../../../sample_code/should_succeed/single_file/should_succeed_without_warnings/color_c_injective_despite_misordered_args.ph"
    );
    expect_success_with_no_warnings(src);
}

#[test]
fn equal_despite_misordered_args() {
    let src = include_str!(
        "../../../sample_code/should_succeed/single_file/should_succeed_without_warnings/equal_despite_misordered_args.ph"
    );
    expect_success_with_no_warnings(src);
}

#[test]
fn equal_despite_different_param_names() {
    let src = include_str!(
        "../../../sample_code/should_succeed/single_file/should_succeed_without_warnings/equal_despite_different_param_names.ph"
    );
    expect_success_with_no_warnings(src);
}

#[test]
fn dash_does_not_affect_type() {
    let src = include_str!(
        "../../../sample_code/should_succeed/single_file/should_succeed_without_warnings/dash_does_not_affect_type.ph"
    );
    expect_success_with_no_warnings(src);
}

#[test]
fn fun_recursion_right_label_wrong_order() {
    let src = include_str!(
        "../../../sample_code/should_succeed/single_file/should_succeed_without_warnings/fun_recursion_right_label_wrong_order.ph"
    );
    expect_success_with_no_warnings(src);
}

#[test]
fn recursive_index_positivity_checking() {
    let src = include_str!(
        "../../../sample_code/should_succeed/single_file/should_succeed_without_warnings/recursive_index_positivity_checking.ph"
    );
    expect_success_with_no_warnings(src);
}

fn expect_success_with_no_warnings(src: &str) {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let file = parse_file(tokens, file_id).expect("Parsing failed");
    let file = simplify_file(file).expect("AST Simplification failed");
    let file_items =
        bind_files(file_id, vec![file], &FileTree::from_root(file_id)).expect("Binding failed");
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
