use super::*;

fn expect_non_adt_matchee_error(src: &str, expected_matchee_src: &str, expected_type_src: &str) {
    expect_type_check_error(src, |registry, err| match err {
        TypeCheckError::NonAdtMatchee {
            matchee_id,
            type_id,
        } => {
            let actual_matchee_src = format_expression(
                &expand_expression(registry, matchee_id),
                0,
                &FormatOptions {
                    ident_size_in_spaces: 4,
                    print_db_indices: false,
                    print_fun_body_status: false,
                },
            );
            assert_eq_up_to_white_space(&actual_matchee_src, expected_matchee_src);

            let actual_type_src = format_expression(
                &expand_expression(registry, type_id.raw()),
                0,
                &FormatOptions {
                    ident_size_in_spaces: 4,
                    print_db_indices: false,
                    print_fun_body_status: false,
                },
            );
            assert_eq_up_to_white_space(&actual_type_src, expected_type_src);
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

#[test]
fn type0() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/non_adt_matchee/type0.ph"
    );
    expect_non_adt_matchee_error(src, "U", "Type");
}

#[test]
fn type1() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/non_adt_matchee/type1.ph"
    );
    expect_non_adt_matchee_error(src, "Type", "Type1");
}
