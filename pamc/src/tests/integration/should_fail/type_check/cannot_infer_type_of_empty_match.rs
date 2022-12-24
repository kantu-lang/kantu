use super::*;

fn expect_cannot_infer_type_of_empty_match_error(src: &str, expected_match_src: &str) {
    expect_type_check_error(src, |registry, err| match err {
        TypeCheckError::CannotInferTypeOfEmptyMatch { match_id } => {
            let actual_src = format_match(
                &expand_match(registry, match_id),
                0,
                &FormatOptions {
                    ident_size_in_spaces: 4,
                    print_db_indices: false,
                    print_fun_body_status: false,
                },
            );
            assert_eq_up_to_white_space(expected_match_src, &actual_src);
        }
        _ => {
            panic!("Unexpected error: {:#?}", err)
        }
    });
}

#[test]
fn cannot_infer_type_of_empty_match() {
    let src = include_str!(
        "../../../sample_code/should_fail/type_check/cannot_infer_type_of_empty_match.ph"
    );
    expect_cannot_infer_type_of_empty_match_error(src, "match e {}");
}
