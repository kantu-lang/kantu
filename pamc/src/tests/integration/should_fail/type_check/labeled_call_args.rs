use super::*;

fn expect_extraneous_labeled_call_arg_type_error(
    src: &str,
    expected_extraneous_label_name_src: &str,
) {
    expect_type_check_error(src, |registry, err| match err {
        TypeCheckError::ExtraneousLabeledCallArg { arg_id, .. } => {
            let arg_label_id = arg_id.label_id();
            let actual_name = &registry.get(arg_label_id).name;
            let expected_name =
                IdentifierName::Standard(expected_extraneous_label_name_src.to_string());
            let expected_name = &expected_name;

            assert_eq!(expected_name, actual_name);
        }
        _ => {
            panic!("Unexpected error: {:#?}", err)
        }
    });
}

#[test]
fn extraneous_arg() {
    let src = include_str!(
        "../../../sample_code/should_fail/type_check/labeled_call_args/extraneous_arg.ph"
    );
    expect_extraneous_labeled_call_arg_type_error(src, "max");
}

fn expect_missing_labeled_call_arg_type_error(src: &str, expected_missing_label_name_src: &str) {
    expect_type_check_error(src, |registry, err| match err {
        TypeCheckError::MissingLabeledCallArg { label_id, .. } => {
            let actual_name = &registry.get(label_id).name;
            let expected_name =
                IdentifierName::Standard(expected_missing_label_name_src.to_string());
            let expected_name = &expected_name;

            assert_eq!(expected_name, actual_name);
        }
        _ => {
            panic!("Unexpected error: {:#?}", err)
        }
    });
}

#[test]
fn missing_arg() {
    let src = include_str!(
        "../../../sample_code/should_fail/type_check/labeled_call_args/missing_arg.ph"
    );
    expect_missing_labeled_call_arg_type_error(src, "right");
}
