use super::*;

fn expect_bind_error(project_path: ProjectPath, panicker: impl FnOnce(BindError)) {
    let (files, file_tree) = get_files_and_file_tree(project_path);
    let err = bind_files(file_tree.root(), files, &file_tree)
        .expect_err("Binding unexpectedly succeeded");
    panicker(err);
}

#[test]
fn name_clash() {
    expect_bind_error(
        ProjectPath {
            callee_file_path: file!(),
            checked_unadjusted_pack_omlet_path: checked_path!(
                "../../../sample_code/should_fail/multi_file/bind/name_clash/pack.omlet"
            ),
        },
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
        ProjectPath {
            callee_file_path: file!(),
            checked_unadjusted_pack_omlet_path: checked_path!(
                "../../../sample_code/should_fail/multi_file/bind/leaky_use_single/pack.omlet"
            ),
        },
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
        ProjectPath {
            callee_file_path: file!(),
            checked_unadjusted_pack_omlet_path: checked_path!(
                "../../../sample_code/should_fail/multi_file/bind/use_priv/pack.omlet"
            ),
        },
        |err| match err {
            BindError::NameIsPrivate(NameIsPrivateError {
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
        ProjectPath {
            callee_file_path: file!(),
            checked_unadjusted_pack_omlet_path: checked_path!(
                "../../../sample_code/should_fail/multi_file/bind/leaky_type/pack.omlet"
            ),
        },
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
