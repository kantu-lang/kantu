use super::*;

fn expect_duplicate_match_case_error(
    src: &str,
    expected_existing_match_case_src: &str,
    expected_new_match_case_src: &str,
) {
    expect_type_check_error(src, |registry, err| match err {
        TypeCheckError::DuplicateMatchCase {
            existing_match_case_id,
            new_match_case_id,
        } => {
            let actual_existing_match_case_src = format_match_case(
                &expand_match_case(registry, existing_match_case_id),
                0,
                &FormatOptions {
                    ident_size_in_spaces: 4,
                    print_db_indices: false,
                    print_fun_body_status: false,
                },
            );
            assert_eq_up_to_white_space(
                &actual_existing_match_case_src,
                expected_existing_match_case_src,
            );

            let actual_new_match_case_src = format_match_case(
                &expand_match_case(registry, new_match_case_id),
                0,
                &FormatOptions {
                    ident_size_in_spaces: 4,
                    print_db_indices: false,
                    print_fun_body_status: false,
                },
            );
            assert_eq_up_to_white_space(&actual_new_match_case_src, expected_new_match_case_src);
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

#[test]
fn duplicate_match_case() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/duplicate_match_case.k"
    );
    expect_duplicate_match_case_error(src, ".U => Bool.True,", ".U => Bool.False,");
}
