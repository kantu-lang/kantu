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

#[test]
fn missing_labeled_match_case_params_2008() {
    let path = concat_paths(
        file!(),
        checked_path!(
            "../../sample_code/should_fail/single_file/type_check/labeled_match_case_params/missing_param.k"
        ),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--file", &path]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn undefined_labeled_match_case_params_2009() {
    let path = concat_paths(
        file!(),
        checked_path!(
            "../../sample_code/should_fail/single_file/type_check/labeled_match_case_params/undefined_param.k"
        ),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--file", &path]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn undefined_labeled_match_case_params_2010() {
    let path = concat_paths(
        file!(),
        checked_path!("../../sample_code/should_fail/single_file/type_check/type_mismatch/adt.k"),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--file", &path]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn non_adt_matchee_2011() {
    let path = concat_paths(
        file!(),
        checked_path!(
            "../../sample_code/should_fail/single_file/type_check/non_adt_matchee/type1.k"
        ),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--file", &path]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn duplicate_match_case_2012() {
    let path = concat_paths(
        file!(),
        checked_path!(
            "../../sample_code/should_fail/single_file/type_check/duplicate_match_case.k"
        ),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--file", &path]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn missing_match_case_2013_missing_one() {
    let path = concat_paths(
        file!(),
        checked_path!(
            "../../sample_code/should_fail/single_file/type_check/missing_match_case/missing_one.k"
        ),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--file", &path]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn missing_match_case_2013_missing_multiple() {
    let path = concat_paths(
        file!(),
        checked_path!(
            "../../sample_code/should_fail/single_file/type_check/missing_match_case/missing_multiple.k"
        ),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--file", &path]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn extraneous_match_case_2014() {
    let path = concat_paths(
        file!(),
        checked_path!(
            "../../sample_code/should_fail/single_file/type_check/extraneous_match_case.k"
        ),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--file", &path]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn match_case_incorrectly_marked_impossible_2015() {
    let path = concat_paths(
        file!(),
        checked_path!(
            "../../sample_code/should_fail/single_file/type_check/match_case_incorrectly_marked_impossible.k"
        ),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--file", &path]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn cannot_infer_type_of_empty_match_2016() {
    let path = concat_paths(
        file!(),
        checked_path!(
            "../../sample_code/should_fail/single_file/type_check/cannot_infer_type_of_empty_match.k"
        ),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--file", &path]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn ambiguous_match_case_output_type_2017() {
    let path = concat_paths(
        file!(),
        checked_path!(
            "../../sample_code/should_fail/single_file/type_check/ambiguous_match_case_output_type.k"
        ),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--file", &path]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn cannot_infer_type_of_todo_expression_2018() {
    let path = concat_paths(
        file!(),
        checked_path!(
            "../../sample_code/should_fail/single_file/type_check/cannot_infer_todo_type/let_value.k"
        ),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--file", &path]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn unreachable_expression_2019() {
    let path = concat_paths(
        file!(),
        checked_path!(
            "../../sample_code/should_fail/single_file/type_check/unreachable_expression.k"
        ),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--file", &path]);
    insta::assert_debug_snapshot!(output);
}

#[test]
fn let_statement_type_contains_private_name_2020() {
    let path = concat_paths(
        file!(),
        checked_path!(
            "../../sample_code/should_fail/multi_file/type_check/leaky_let_type/pack.yscl"
        ),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--pack", &path]);
    insta::assert_debug_snapshot!(output);
}
