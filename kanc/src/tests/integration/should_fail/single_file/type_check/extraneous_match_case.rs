use super::*;

fn expect_extraneous_match_case_error(src: &str, expected_extraneous_match_case_src: &str) {
    expect_type_check_error(src, |registry, err| match err {
        TypeCheckError::ExtraneousMatchCase { case_id } => {
            let actual_extraneous_match_case_src = format_match_case(
                &expand_match_case(registry, case_id),
                0,
                &FORMAT_OPTIONS_FOR_COMPARISON,
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
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/extraneous_match_case.k"
    );
    expect_extraneous_match_case_error(src, ".maybe => Nat.s(Nat.s(Nat.o,),),");
}
