use super::*;

fn expect_extraneous_match_case_error(src: &str, expected_extraneous_match_case_src: &str) {
    expect_type_check_error(src, |registry, err| match err {
        TypeCheckError::ExtraneousMatchCase { case_id } => {
            let actual_extraneous_match_case_src = format_match_case(
                &expand_match_case(registry, case_id),
                0,
                &FormatOptions {
                    ident_size_in_spaces: 4,
                    print_db_indices: false,
                    print_fun_body_status: false,
                },
            );
            assert_eq_up_to_white_space(
                &actual_extraneous_match_case_src,
                expected_extraneous_match_case_src,
            );
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

#[test]
fn extraneous_match_case() {
    let src = include_str!("../../../sample_code/should_fail/type_check/extraneous_match_case.ph");
    expect_extraneous_match_case_error(src, "Maybe => Nat.S(Nat.S(Nat.O,),),");
}
