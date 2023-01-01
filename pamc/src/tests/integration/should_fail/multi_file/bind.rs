use super::*;

/// `file_path` should **always** be `file!()`.
///
/// This is so it will be consistent with `checked_unadjusted_pack_omlet_path`.
///
/// The reason we make `file_path` a parameter rather than simply
/// hardcoding `file!()` in the function is because the value
/// of `file!()` will change depending on where it is written.
/// The value of `checked_unadjusted_pack_omlet_path` is relative to the
/// calling file, so the value of `file_path` must also be relative to the calling file.
/// Thus, we cannot hardcode `file!()` in the function definition,
/// and must instead require the caller to pass it in as an argument.
fn expect_bind_error(
    file_path: &str,
    checked_unadjusted_pack_omlet_path: &str,
    panicker: impl FnOnce(BindError),
) {
    let (files, file_tree) = get_files_and_file_tree(file_path, checked_unadjusted_pack_omlet_path);
    let err = bind_files(file_tree.root(), files, &file_tree)
        .expect_err("Binding unexpectedly succeeded");
    panicker(err);
}

#[test]
fn name_clash() {
    expect_bind_error(
        file!(),
        checked_path!("../../../sample_code/should_fail/multi_file/bind/name_clash/pack.omlet"),
        |err| match err {
            BindError::NameClash(NameClashError {
                name,
                old: OwnedSymbolSource::WildcardImport(_),
                new: OwnedSymbolSource::WildcardImport(_),
            }) => {
                assert_eq!("Nat", name.src_str());
            }
            _ => panic!("Unexpected error: {:?}", err),
        },
    );
}

#[test]
fn leaky_use_single() {
    expect_bind_error(
        file!(),
        checked_path!(
            "../../../sample_code/should_fail/multi_file/bind/leaky_use_single/pack.omlet"
        ),
        |err| match err {
            BindError::CannotLeakPrivateName(CannotLeakPrivateNameError {
                name_component,
                required_visibility: _,
                actual_visibility: _,
            }) => {
                assert_eq!("Foo", name_component.name.src_str());
            }
            _ => panic!("Unexpected error: {:?}", err),
        },
    );
}

#[test]
fn leaky_use_single_nested() {
    expect_bind_error(
        file!(),
        checked_path!(
            "../../../sample_code/should_fail/multi_file/bind/leaky_use_single_nested/pack.omlet"
        ),
        |err| match err {
            BindError::CannotLeakPrivateName(CannotLeakPrivateNameError {
                name_component,
                required_visibility: _,
                actual_visibility: _,
            }) => {
                assert_eq!("Foo", name_component.name.src_str());
            }
            _ => panic!("Unexpected error: {:?}", err),
        },
    );
}

#[test]
fn locally_leaky_use_single_nested() {
    expect_bind_error(
        file!(),
        checked_path!(
            "../../../sample_code/should_fail/multi_file/bind/locally_leaky_use_single_nested/pack.omlet"
        ),
        |err| match err {
            BindError::CannotLeakPrivateName(CannotLeakPrivateNameError {
                name_component,
                required_visibility: _,
                actual_visibility: _,
            }) => {
                assert_eq!("Foo", name_component.name.src_str());
            }
            _ => panic!("Unexpected error: {:?}", err),
        },
    );
}

#[test]
fn leaky_type() {
    expect_bind_error(
        file!(),
        checked_path!("../../../sample_code/should_fail/multi_file/bind/leaky_type/pack.omlet"),
        |err| match err {
            BindError::CannotLeakPrivateName(CannotLeakPrivateNameError {
                name_component,
                required_visibility: _,
                actual_visibility: _,
            }) => {
                assert_eq!("Private", name_component.name.src_str());
            }
            _ => panic!("Unexpected error: {:?}", err),
        },
    );
}
