use super::*;

fn expect_undefined_match_case_params_type_error<const N: usize>(
    src: &str,
    expected_undefined_label_names: [&str; N],
) {
    expect_type_check_error(src, |registry, err| match err {
        TypeCheckError::UndefinedLabeledMatchCaseParams {
            case_param_list_id, ..
        } => {
            let case_param_ids = registry.get_list(case_param_list_id);
            assert_eq!(expected_undefined_label_names.len(), case_param_ids.len());

            for (expected_name, case_param_id) in expected_undefined_label_names
                .iter()
                .copied()
                .zip(case_param_ids.iter().copied())
            {
                let case_param_label_id = registry.get(case_param_id).label_identifier_id();
                let case_param_label_name = &registry.get(case_param_label_id).name;
                assert_eq!(
                    IdentifierName::Standard(expected_name.to_string()),
                    *case_param_label_name
                );
            }
        }
        _ => {
            panic!("Unexpected error: {:#?}", err)
        }
    });
}

#[test]
fn extraneous_arg() {
    let src = include_str!(
        "../../../sample_code/should_fail/type_check/labeled_match_case_params/undefined_param.ph"
    );
    expect_undefined_match_case_params_type_error(src, ["alpha", "hue"]);
}
