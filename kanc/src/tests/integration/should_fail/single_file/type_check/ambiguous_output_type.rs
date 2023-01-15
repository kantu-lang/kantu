use super::*;

fn expect_ambiguous_match_case_output_type_error(
    src: &str,
    expected_ambiguous_match_case_src: &str,
) {
    expect_type_check_error(src, |registry, err| match err {
        TypeCheckError::AmbiguousMatchCaseOutputType {
            case_id,
            non_shifted_output_type_id: _,
        } => {
            let actual_ambiguous_match_case_src = format_match_case(
                &expand_match_case(registry, case_id),
                0,
                &FORMAT_OPTIONS_FOR_COMPARISON,
            );
            assert_eq_up_to_white_space(
                &actual_ambiguous_match_case_src,
                expected_ambiguous_match_case_src,
            );
        }
        _ => {
            panic!("Unexpected error: {:#?}", err)
        }
    });
}

#[test]
fn ambiguous_match_case_output_type() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/ambiguous_match_case_output_type.k"
    );
    expect_ambiguous_match_case_output_type_error(
        src,
        ".S(problem) => Eq.Refl(Nat, Nat.S(problem,),),",
    );
}
