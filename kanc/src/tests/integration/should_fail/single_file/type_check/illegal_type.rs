use super::*;

fn expect_illegal_type_error(src: &str, expected_illegal_type_src: &str) {
    expect_type_check_error(src, |registry, err| match err {
        TypeCheckError::ExpectedTermOfTypeType0OrType1 {
            expression_id,
            non_type0_or_type1_type_id: _,
        } => {
            let actual_src = format_expression(
                &expand_expression(registry, expression_id),
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
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/illegal_type/forall_output.k"
    );
    expect_illegal_type_error(src, "U.U");
}

#[test]
fn forall_param() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/illegal_type/forall_param.k"
    );
    expect_illegal_type_error(src, "U.U");
}

#[test]
fn fun_param() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/illegal_type/fun_param.k"
    );
    expect_illegal_type_error(src, "U.U");
}

#[test]
fn fun_return() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/illegal_type/fun_return.k"
    );
    expect_illegal_type_error(src, "U.U");
}

#[test]
fn type_param() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/illegal_type/type_param.k"
    );
    expect_illegal_type_error(src, "U.U");
}

#[test]
fn variant_param() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/illegal_type/variant_param.k"
    );
    expect_illegal_type_error(src, "U.U");
}
