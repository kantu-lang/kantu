use super::*;

fn expect_illegal_callee_error(src: &str, expected_illegal_callee_src: &str) {
    expect_type_check_error(src, |registry, err| match err {
        TypeCheckError::IllegalCallee(id) => {
            let actual_src = format_expression(
                &expand_expression(registry, id),
                0,
                &FormatOptions {
                    ident_size_in_spaces: 4,
                    print_db_indices: false,
                    print_fun_body_status: false,
                },
            );
            assert_eq_up_to_white_space(&actual_src, expected_illegal_callee_src);
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

#[test]
fn forall() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/illegal_callee/forall.ph"
    );
    expect_illegal_callee_error(src, "forall(T: Type,) { Type }");
}

#[test]
fn non_nullary_adt_instance() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/illegal_callee/non_nullary_adt_instance.ph"
    );
    expect_illegal_callee_error(src, "Bar.B(U.U,)");
}

#[test]
fn nullary_adt_instance() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/illegal_callee/nullary_adt_instance.ph"
    );
    expect_illegal_callee_error(src, "U.U");
}

#[test]
fn nullary_type() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/illegal_callee/nullary_type.ph"
    );
    expect_illegal_callee_error(src, "U");
}

#[test]
fn type0() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/illegal_callee/type0.ph"
    );
    expect_illegal_callee_error(src, "Type");
}
