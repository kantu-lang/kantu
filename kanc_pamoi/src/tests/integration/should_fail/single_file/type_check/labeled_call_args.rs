use super::*;

fn expect_extraneous_labeled_call_arg_type_error(
    src: &str,
    expected_extraneous_label_name_src: &str,
) {
    expect_type_check_error(src, |registry, err| match err {
        TypeCheckError::ExtraneousLabeledCallArg { arg_id, .. } => {
            let arg_label_id = arg_id.label_id();
            let actual_name = &registry.get(arg_label_id).name;
            let expected_name = IdentifierName::new(expected_extraneous_label_name_src.to_string());
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
        "../../../../sample_code/should_fail/single_file/type_check/labeled_call_args/extraneous_arg.k"
    );
    expect_extraneous_labeled_call_arg_type_error(src, "max");
}

fn expect_missing_labeled_call_arg_type_error<const N: usize>(
    src: &str,
    expected_missing_label_names: [&str; N],
) {
    expect_type_check_error(src, |registry, err| match err {
        TypeCheckError::MissingLabeledCallArgs {
            missing_label_list_id,
            ..
        } => {
            let missing_label_ids = registry.get_list(missing_label_list_id);
            for (expected_name_src, missing_label_id) in expected_missing_label_names
                .iter()
                .copied()
                .zip(missing_label_ids.iter().copied())
            {
                let actual_name = &registry.get(missing_label_id).name;
                let expected_name = IdentifierName::new(expected_name_src.to_string());
                let expected_name = &expected_name;

                assert_eq!(expected_name, actual_name);
            }
        }
        _ => {
            panic!("Unexpected error: {:#?}", err)
        }
    });
}

#[test]
fn missing_arg() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/labeled_call_args/missing_arg.k"
    );
    expect_missing_labeled_call_arg_type_error(src, ["right"]);
}

#[test]
fn multiple_missing_args() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/labeled_call_args/multiple_missing_args.k"
    );
    expect_missing_labeled_call_arg_type_error(src, ["g", "b"]);
}
