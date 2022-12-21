use super::*;

fn expect_ambiguous_output_type_error(src: &str, expected_ambiguous_match_case_src: &str) {
    expect_type_check_error(src, |registry, err| match err {
        TypeCheckError::AmbiguousOutputType { case_id } => {
            let actual_ambiguous_match_case_src = format_match_case(
                &expand_match_case(registry, case_id),
                0,
                &FormatOptions {
                    ident_size_in_spaces: 4,
                    print_db_indices: false,
                    print_fun_body_status: false,
                },
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
fn ambiguous_output_type() {
    let src = include_str!("../../../sample_code/should_fail/type_check/ambiguous_output_type.ph");
    expect_ambiguous_output_type_error(src, ".S(problem) => Eq.Refl(Nat, Nat.S(problem,),),");
}
