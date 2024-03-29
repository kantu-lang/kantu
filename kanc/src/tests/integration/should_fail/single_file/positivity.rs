use super::*;

/// The job of `panicker` is to panic if the error is different than the expected
/// error.
fn expect_positivity_error(src: &str, panicker: impl Fn(&NodeRegistry, TypePositivityError)) {
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
    let err = validate_type_positivity_in_file_items(&mut registry, file_item_list_id)
        .expect_err("Type positivity validation unexpectedly succeeded");
    panicker(&registry, err);
}

fn expect_illegal_variable_appearance_error(
    src: &str,
    expected_db_index: DbIndex,
    expected_name: &str,
) {
    expect_positivity_error(src, |registry, err| match err {
        TypePositivityError::IllegalVariableAppearance(name_id) => {
            let name = registry.get(name_id);

            assert_eq!(expected_db_index, name.db_index);

            let component_list_id = &registry.get(name_id).component_list_id;
            assert_eq!(1, component_list_id.len.get());
            let component_ids = registry.get_list(*component_list_id);
            assert_eq!(
                IdentifierName::new(expected_name.to_string()),
                registry.get(component_ids[0]).name
            );
        }
        _ => panic!("Unexpected error: {:?}", err),
    });
}

#[test]
fn negative_recursion() {
    let src = include_str!(
        "../../../sample_code/should_fail/single_file/positivity/negative_recursion.k"
    );
    expect_illegal_variable_appearance_error(src, DbIndex(0), "Bad");
}

#[test]
fn indirect_negative_recursion() {
    let src = include_str!(
        "../../../sample_code/should_fail/single_file/positivity/indirect_negative_recursion.k"
    );
    expect_illegal_variable_appearance_error(src, DbIndex(1), "T");
}

#[test]
fn bad_matchee() {
    let src = include_str!("../../../sample_code/should_fail/single_file/positivity/bad_matchee.k");
    expect_illegal_variable_appearance_error(src, DbIndex(0), "Bad");
}

#[test]
fn bad_match_case_output() {
    let src = include_str!(
        "../../../sample_code/should_fail/single_file/positivity/bad_match_case_output.k"
    );
    expect_illegal_variable_appearance_error(src, DbIndex(0), "Bad");
}

#[test]
fn bad_forall_output() {
    let src =
        include_str!("../../../sample_code/should_fail/single_file/positivity/bad_forall_output.k");
    expect_illegal_variable_appearance_error(src, DbIndex(1), "Bad");
}

#[test]
fn bad_check_output() {
    let src =
        include_str!("../../../sample_code/should_fail/single_file/positivity/bad_check_output.k");
    expect_illegal_variable_appearance_error(src, DbIndex(0), "Bad");
}

fn expect_non_name_variant_return_type_error(
    src: &str,
    expected_variant_name: &str,
    expected_type_arg_index: usize,
) {
    expect_positivity_error(src, |registry, err| match err {
        TypePositivityError::VariantReturnTypeHadNonNameTypeArg {
            variant_id,
            type_arg_index,
        } => {
            let variant_name_id = registry.get(variant_id).name_id;
            let actual_variant_name = &registry.get(variant_name_id).name;
            assert_eq!(
                IdentifierName::new(expected_variant_name.to_string()),
                *actual_variant_name,
            );

            assert_eq!(expected_type_arg_index, type_arg_index);
        }
        _ => panic!("Unexpected error: {:?}", err),
    });
}

#[test]
fn obscured_indirect_negative_recursion() {
    let src = include_str!(
        "../../../sample_code/should_fail/single_file/positivity/obscured_indirect_negative_recursion.k"
    );
    expect_non_name_variant_return_type_error(src, "not_c", 0);
}

fn expect_non_adt_callee_error(src: &str, expected_callee_src: &str) {
    expect_positivity_error(src, |registry, err| match err {
        TypePositivityError::NonAdtCallee {
            call_id: _,
            callee_id,
        } => {
            let actual_src = format_expression(
                &expand_expression(registry, callee_id),
                0,
                &FORMAT_OPTIONS_FOR_COMPARISON,
            );
            assert_eq_up_to_white_space(expected_callee_src, &actual_src);
        }
        _ => panic!("Unexpected error: {:?}", err),
    });
}

#[test]
fn non_adt_callee() {
    let src =
        include_str!("../../../sample_code/should_fail/single_file/positivity/non_adt_callee.k");
    let expected_call_src = "fun _(_: Unit,): Type { forall(b: Bad,) { Empty } }";
    expect_non_adt_callee_error(src, expected_call_src);
}

fn expect_expected_type_got_fun_error(src: &str, expected_fun_name: &IdentifierName) {
    expect_positivity_error(src, |registry, err| match err {
        TypePositivityError::ExpectedTypeGotFun(fun_id) => {
            let fun = registry.get(fun_id);
            let actual_fun_name = &registry.get(fun.name_id).name;
            assert_eq!(expected_fun_name, actual_fun_name);
        }
        _ => panic!("Unexpected error: {:?}", err),
    });
}

#[test]
fn expected_type_got_fun() {
    let src = include_str!(
        "../../../sample_code/should_fail/single_file/positivity/expected_type_got_fun.k"
    );
    let expected_fun_name = IdentifierName::new("this_should_not_be_here".to_string());
    expect_expected_type_got_fun_error(src, &expected_fun_name);
}

fn expect_variant_return_type_type_arg_arity_mismatch_error(
    src: &str,
    expected_expected_arity: usize,
    expected_return_type_src: &str,
) {
    expect_positivity_error(src, |registry, err| match err {
        TypePositivityError::VariantReturnTypeTypeArgArityMismatch {
            actual: _,
            expected,
            return_type_id,
        } => {
            assert_eq!(expected, expected_expected_arity);

            let actual_src = format_expression(
                &expand_expression(registry, return_type_id),
                0,
                &FORMAT_OPTIONS_FOR_COMPARISON,
            );
            assert_eq_up_to_white_space(expected_return_type_src, &actual_src);
        }
        _ => panic!("Unexpected error: {:?}", err),
    });
}

#[test]
fn expected_1_type_arg_got_0() {
    let src = include_str!(
        "../../../sample_code/should_fail/single_file/positivity/expected_1_type_arg_got_0.k"
    );
    expect_variant_return_type_type_arg_arity_mismatch_error(src, 1, "Not");
}

#[test]
fn expected_2_type_args_got_1() {
    let src = include_str!(
        "../../../sample_code/should_fail/single_file/positivity/expected_2_type_args_got_1.k"
    );
    expect_variant_return_type_type_arg_arity_mismatch_error(src, 2, "Not(Unit.c,)");
}
