use super::*;

#[test]
fn illegal_dot_lhs_0400() {
    let path = concat_paths(
        file!(),
        checked_path!(
            "../../sample_code/should_fail/single_file/ast_simplification/dot/illegal_dot_lhs.k"
        ),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--file", &path]);
    insta::assert_debug_snapshot!(output);
}
