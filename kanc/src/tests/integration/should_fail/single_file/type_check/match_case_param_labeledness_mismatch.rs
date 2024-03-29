use super::*;

fn expect_match_case_param_labeledness_mismatch_error(src: &str, expected_case_src: &str) {
    expect_type_check_error(src, |registry, err| match err {
        TypeCheckError::MatchCaseLabelednessMismatch {
            case_id,
            param_list_id: _,
        } => {
            let actual_call_src = format_match_case(
                &expand_match_case(registry, case_id),
                0,
                &FORMAT_OPTIONS_FOR_COMPARISON,
            );
            assert_eq_up_to_white_space(&actual_call_src, expected_case_src);
        }
        _ => {
            panic!("Unexpected error: {:#?}", err)
        }
    });
}

#[test]
fn labeled_variant_unlabeled_case() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/match_case_param_labeledness_mismatch/labeled_variant_unlabeled_case.k"
    );
    expect_match_case_param_labeledness_mismatch_error(
        src,
        ".s(a') => Nat.s(pred: plus(a', b,),),",
    );
}

#[test]
fn unlabeled_variant_labeled_case() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/match_case_param_labeledness_mismatch/unlabeled_variant_labeled_case.k"
    );
    expect_match_case_param_labeledness_mismatch_error(src, ".s(:pred) => Nat.s(plus(pred, b,),),");
}
