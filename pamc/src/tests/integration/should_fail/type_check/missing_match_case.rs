use super::*;

fn expect_missing_match_case_error(src: &str, expected_variant_name: &IdentifierName) {
    expect_type_check_error(src, |registry, err| match err {
        TypeCheckError::MissingMatchCase { variant_name_id } => {
            let actual_variant_name = &registry.get(variant_name_id).name;
            assert_eq!(expected_variant_name, actual_variant_name);
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

#[test]
fn missing_match_case() {
    let src = include_str!("../../../sample_code/should_fail/type_check/missing_match_case.ph");
    expect_missing_match_case_error(src, &standard_ident_name("False"));
}
