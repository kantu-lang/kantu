use super::*;

fn expect_wrong_number_of_case_params_error(
    src: &str,
    expected_illegal_match_case_src: &str,
    expected_expected_arity: usize,
) {
    expect_type_check_error(src, |registry, err| match err {
        TypeCheckError::WrongNumberOfMatchCaseParams {
            case_id,
            expected: actual_expected_arity,
            ..
        } => {
            let actual_src = format_match_case(
                &expand_match_case(registry, case_id),
                0,
                &FORMAT_OPTIONS_FOR_COMPARISON,
            );
            assert_eq_up_to_white_space(&actual_src, expected_illegal_match_case_src);
            assert_eq!(expected_expected_arity, actual_expected_arity);
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

#[test]
fn expected_0_actually_1() {
    let src = include_str!("../../../../sample_code/should_fail/single_file/type_check/wrong_number_of_case_params/expected_0_actually_1.k");
    expect_wrong_number_of_case_params_error(src, ".O(n) => n,", 0);
}

#[test]
fn expected_1_actually_0() {
    let src = include_str!("../../../../sample_code/should_fail/single_file/type_check/wrong_number_of_case_params/expected_1_actually_0.k");
    expect_wrong_number_of_case_params_error(src, ".S => Nat.O,", 1);
}

#[test]
fn expected_1_actually_2() {
    let src = include_str!("../../../../sample_code/should_fail/single_file/type_check/wrong_number_of_case_params/expected_1_actually_2.k");
    expect_wrong_number_of_case_params_error(src, ".S(n, m) => n,", 1);
}

#[test]
fn expected_2_actually_1() {
    let src = include_str!("../../../../sample_code/should_fail/single_file/type_check/wrong_number_of_case_params/expected_2_actually_1.k");
    expect_wrong_number_of_case_params_error(src, ".Refl(O) => O,", 2);
}
