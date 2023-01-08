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
