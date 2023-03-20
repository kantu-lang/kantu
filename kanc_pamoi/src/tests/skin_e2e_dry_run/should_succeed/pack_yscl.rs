use super::*;

#[test]
fn plus_commutative() {
    let path = concat_paths(
        file!(),
        checked_path!(
            "../../sample_code/should_succeed/multi_file/no_warnings/plus_commutative/pack.yscl"
        ),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--pack", &path]);
    insta::assert_debug_snapshot!(output);
}
