use super::*;

fn expect_match_case_incorrectly_marked_impossible_error(
    src: &str,
    expected_extraneous_match_case_src: &str,
) {
    expect_type_check_error(src, |registry, err| match err {
        TypeCheckError::AllegedlyImpossibleMatchCaseWasNotObviouslyImpossible { case_id } => {
            let actual_extraneous_match_case_src = format_match_case(
                &expand_match_case(registry, case_id),
                0,
                &FormatOptions {
                    ident_size_in_spaces: 4,
                    print_db_indices: false,
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
fn match_case_incorrectly_marked_impossible() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/match_case_incorrectly_marked_impossible.k"
    );
    expect_match_case_incorrectly_marked_impossible_error(src, ".O => impossible,");
}
