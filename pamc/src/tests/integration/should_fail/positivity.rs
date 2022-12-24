use super::*;

/// The job of `panicker` is to panic if the error is different than the expected
/// error.
fn expect_positivity_error(src: &str, panicker: impl Fn(&NodeRegistry, TypePositivityError)) {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let file = parse_file(tokens, file_id).expect("Parsing failed");
    let file = simplify_file(file).expect("AST Simplification failed");
    let file = bind_files(vec![file])
        .expect("Binding failed")
        .into_iter()
        .next()
        .unwrap();
    let mut registry = NodeRegistry::empty();
    let file_id = lighten_file(&mut registry, file);
    let file = registry.get(file_id);

    let file_id = validate_variant_return_types_in_file(&registry, file)
        .expect("Variant return type validation failed");
    let file_id = validate_fun_recursion_in_file(&mut registry, file_id)
        .expect("Fun recursion validation failed");
    let err = validate_type_positivity_in_file(&mut registry, file_id)
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
                IdentifierName::Standard(expected_name.to_string()),
                registry.get(component_ids[0]).name
            );
        }
        _ => panic!("Unexpected error: {:?}", err),
    });
}

#[test]
fn negative_recursion() {
    let src = include_str!("../../sample_code/should_fail/positivity/negative_recursion.ph");
    expect_illegal_variable_appearance_error(src, DbIndex(0), "Bad");
}

#[test]
fn indirect_negative_recursion() {
    let src =
        include_str!("../../sample_code/should_fail/positivity/indirect_negative_recursion.ph");
    expect_illegal_variable_appearance_error(src, DbIndex(1), "T");
}

#[test]
fn bad_matchee() {
    let src = include_str!("../../sample_code/should_fail/positivity/bad_matchee.ph");
    expect_illegal_variable_appearance_error(src, DbIndex(0), "Bad");
}

#[test]
fn bad_match_case_output() {
    let src = include_str!("../../sample_code/should_fail/positivity/bad_match_case_output.ph");
    expect_illegal_variable_appearance_error(src, DbIndex(0), "Bad");
}

#[test]
fn bad_forall_output() {
    let src = include_str!("../../sample_code/should_fail/positivity/bad_forall_output.ph");
    expect_illegal_variable_appearance_error(src, DbIndex(1), "Bad");
}

fn expect_non_name_variant_return_type_error(
    src: &str,
    expected_variant_name: &str,
    expected_type_arg_index: usize,
) {
    expect_positivity_error(src, |registry, err| match err {
        TypePositivityError::VariantReturnTypeHadNonNameElement {
            variant_id,
            type_arg_index,
        } => {
            let variant_name_id = registry.get(variant_id).name_id;
            let actual_variant_name = &registry.get(variant_name_id).name;
            assert_eq!(
                IdentifierName::Standard(expected_variant_name.to_string()),
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
        "../../sample_code/should_fail/positivity/obscured_indirect_negative_recursion.ph"
    );
    expect_non_name_variant_return_type_error(src, "NotC", 0);
}

fn expect_non_adt_callee_error(src: &str, expected_callee_src: &str) {
    expect_positivity_error(src, |registry, err| match err {
        TypePositivityError::NonAdtCallee {
            call_id: _,
            callee_id,
        } => {
            let actual_extraneous_match_case_src = format_expression(
                &expand_expression(registry, callee_id),
                0,
                &FormatOptions {
                    ident_size_in_spaces: 4,
                    print_db_indices: false,
                    print_fun_body_status: false,
                },
            );
            assert_eq_up_to_white_space(expected_callee_src, &actual_extraneous_match_case_src);
        }
        _ => panic!("Unexpected error: {:?}", err),
    });
}

#[test]
fn non_adt_callee() {
    let src = include_str!("../../sample_code/should_fail/positivity/non_adt_callee.ph");
    let expected_call_src = "fun _(_: Unit,): Type { forall(b: Bad,) { Empty } }";
    expect_non_adt_callee_error(src, expected_call_src);
}
