use super::*;

#[test]
fn expected_term_of_type_type0_or_type1_2000() {
    let path = concat_paths(
        file!(),
        checked_path!(
            "../../sample_code/should_fail/single_file/type_check/illegal_type/forall_output.k"
        ),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--file", &path]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn illegal_callee_2001() {
    let path = concat_paths(
        file!(),
        checked_path!(
            "../../sample_code/should_fail/single_file/type_check/illegal_callee/forall.k"
        ),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--file", &path]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn wrong_number_of_arguments_2002() {
    let path = concat_paths(
        file!(),
        checked_path!(
            "../../sample_code/should_fail/single_file/type_check/wrong_number_of_arguments/fun.k"
        ),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--file", &path]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn call_labeledness_mismatch_2003() {
    let path = concat_paths(
        file!(),
        checked_path!(
            "../../sample_code/should_fail/single_file/type_check/call_arg_labeledness_mismatch/labeled_fun_unlabeled_args.k"
        ),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--file", &path]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn missing_labeled_call_args_2004_one_arg() {
    let path = concat_paths(
        file!(),
        checked_path!(
            "../../sample_code/should_fail/single_file/type_check/labeled_call_args/missing_arg.k"
        ),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--file", &path]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn missing_labeled_call_args_2004_multiple_args() {
    let path = concat_paths(
        file!(),
        checked_path!(
            "../../sample_code/should_fail/single_file/type_check/labeled_call_args/multiple_missing_args.k"
        ),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--file", &path]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn extraneous_labeled_call_arg_2005() {
    let path = concat_paths(
        file!(),
        checked_path!(
            "../../sample_code/should_fail/single_file/type_check/labeled_call_args/extraneous_arg.k"
        ),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--file", &path]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn wrong_number_of_match_case_params_2006_expected_zero() {
    let path = concat_paths(
        file!(),
        checked_path!(
            "../../sample_code/should_fail/single_file/type_check/wrong_number_of_case_params/expected_1_actually_0.k"
        ),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--file", &path]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn wrong_number_of_match_case_params_2006_expected_one() {
    let path = concat_paths(
        file!(),
        checked_path!(
            "../../sample_code/should_fail/single_file/type_check/wrong_number_of_case_params/expected_0_actually_1.k"
        ),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--file", &path]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn match_case_param_labeledness_mismatch_2007() {
    let path = concat_paths(
        file!(),
        checked_path!(
            "../../sample_code/should_fail/single_file/type_check/match_case_param_labeledness_mismatch/labeled_variant_unlabeled_case.k"
        ),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--file", &path]);
    insta::assert_debug_snapshot!(output);
}
