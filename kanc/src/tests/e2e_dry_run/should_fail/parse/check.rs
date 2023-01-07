use super::*;

#[test]
fn question_checkee() {
    let path = concat_paths(
        file!(),
        checked_path!(
            "../../../sample_code/should_fail/single_file/parse/check/question_checkee.k"
        ),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--file", &path]);
    insta::assert_debug_snapshot!(output);
}
