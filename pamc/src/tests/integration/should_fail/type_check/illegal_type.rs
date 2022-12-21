use super::*;

fn expect_illegal_type_error(src: &str, expected_illegal_type_src: &str) {
    expect_type_check_error(src, |registry, err| match err {
        TypeCheckError::IllegalTypeExpression(id) => {
            let actual_src = format_expression(
                &expand_expression(registry, id),
                0,
                &FormatOptions {
                    ident_size_in_spaces: 4,
                    print_db_indices: false,
                    print_fun_body_status: false,
                },
            );
            assert_eq_up_to_white_space(&actual_src, expected_illegal_type_src);
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

#[test]
fn forall_output() {
    let src =
        include_str!("../../../sample_code/should_fail/type_check/illegal_type/forall_output.ph");
    expect_illegal_type_error(src, "U.U");
}

#[test]
fn forall_param() {
    let src =
        include_str!("../../../sample_code/should_fail/type_check/illegal_type/forall_param.ph");
    expect_illegal_type_error(src, "U.U");
}

#[test]
fn fun_param() {
    let src = include_str!("../../../sample_code/should_fail/type_check/illegal_type/fun_param.ph");
    expect_illegal_type_error(src, "U.U");
}

#[test]
fn fun_return() {
    let src =
        include_str!("../../../sample_code/should_fail/type_check/illegal_type/fun_return.ph");
    expect_illegal_type_error(src, "U.U");
}

#[test]
fn type_param() {
    let src =
        include_str!("../../../sample_code/should_fail/type_check/illegal_type/type_param.ph");
    expect_illegal_type_error(src, "U.U");
}

#[test]
fn variant_param() {
    let src =
        include_str!("../../../sample_code/should_fail/type_check/illegal_type/variant_param.ph");
    expect_illegal_type_error(src, "U.U");
}
