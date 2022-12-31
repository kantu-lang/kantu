use super::*;

fn expect_missing_match_case_error(src: &str, expected_variant_names: &[&IdentifierName]) {
    expect_type_check_error(src, |registry, err| match err {
        TypeCheckError::MissingMatchCases {
            missing_variant_name_list_id,
        } => {
            let missing_variant_name_ids = registry.get_list(missing_variant_name_list_id);
            for (expected_variant_name, actual_missing_variant_name_id) in expected_variant_names
                .iter()
                .copied()
                .zip(missing_variant_name_ids.iter().copied())
            {
                let actual_variant_name = &registry.get(actual_missing_variant_name_id).name;
                assert_eq!(expected_variant_name, actual_variant_name);
            }
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

#[test]
fn missing_match_case() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/missing_match_case.ph"
    );
    expect_missing_match_case_error(src, &[&IdentifierName::new("False".to_string())]);
}
