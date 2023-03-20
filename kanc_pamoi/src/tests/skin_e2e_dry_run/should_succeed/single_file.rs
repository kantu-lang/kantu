use super::*;

#[test]
fn hello_world() {
    let path = concat_paths(
        file!(),
        checked_path!("../../sample_code/should_succeed/single_file/no_warnings/hello_world.k"),
    );
    let output =
        get_manifest_path_and_backslash_normalized_output(vec![DUMMY_EXEC_PATH, "--file", &path]);
    insta::assert_debug_snapshot!(output);
}
