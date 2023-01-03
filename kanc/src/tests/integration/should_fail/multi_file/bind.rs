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

#[test]
fn wildcard_downgrades_visibility() {
    expect_bind_error(
        ProjectPath {
            callee_file_path: file!(),
            checked_unadjusted_pack_omlet_path: checked_path!(
                "../../../sample_code/should_fail/multi_file/bind/wildcard_downgrades_visibility/pack.omlet"
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
fn leaky_let_value() {
    expect_bind_error(
        ProjectPath {
            callee_file_path: file!(),
            checked_unadjusted_pack_omlet_path: checked_path!(
                "../../../sample_code/should_fail/multi_file/bind/leaky_let_value/pack.omlet"
            ),
        },
        |err| match err {
            BindError::CannotLeakPrivateName(CannotLeakPrivateNameError {
                name_component,
                required_visibility: _,
                actual_visibility: _,
            }) => {
                assert_eq!("priv_S", name_component.name.src_str());
            }
            _ => panic!("Unexpected error: {:?}", err),
        },
    );
}

#[test]
fn let_value_priv() {
    expect_bind_error(
        ProjectPath {
            callee_file_path: file!(),
            checked_unadjusted_pack_omlet_path: checked_path!(
                "../../../sample_code/should_fail/multi_file/bind/let_value_priv/pack.omlet"
            ),
        },
        |err| match err {
            BindError::NameIsPrivate(NameIsPrivateError {
                name_component,
                required_visibility: _,
                actual_visibility: _,
            }) => {
                assert_eq!("Nat", name_component.name.src_str());
            }
            _ => panic!("Unexpected error: {:?}", err),
        },
    );
}

#[test]
fn type_priv() {
    expect_bind_error(
        ProjectPath {
            callee_file_path: file!(),
            checked_unadjusted_pack_omlet_path: checked_path!(
                "../../../sample_code/should_fail/multi_file/bind/type_priv/pack.omlet"
            ),
        },
        |err| match err {
            BindError::NameIsPrivate(NameIsPrivateError {
                name_component,
                required_visibility: _,
                actual_visibility: _,
            }) => {
                assert_eq!("Nat", name_component.name.src_str());
            }
            _ => panic!("Unexpected error: {:?}", err),
        },
    );
}

#[test]
fn visibility_not_at_least_as_permissive_as_current_mod() {
    expect_bind_error(
        ProjectPath {
            callee_file_path: file!(),
            checked_unadjusted_pack_omlet_path: checked_path!(
                "../../../sample_code/should_fail/multi_file/bind/visibility_not_at_least_as_permissive_as_current_mod/pack.omlet"
            ),
        },
        |err| match err {
            BindError::VisibilityWasNotAtLeastAsPermissiveAsCurrentMod(VisibilityWasNotAtLeastAsPermissiveAsCurrentModError {
                visibility_modifier,
            }) => {
                match visibility_modifier.kind {
                    simplified_ast::ModScopeModifierKind::PackRelative { path_after_pack_kw } => {
                        assert_eq!(1, path_after_pack_kw.len());
                        assert_eq!("nat", path_after_pack_kw[0].name.src_str());
                    }
                    _ => panic!("Unexpected visibility modifier: {:?}", visibility_modifier),
                }
            }
            _ => panic!("Unexpected error: {:?}", err),
        },
    );
}

#[test]
fn transparency_not_at_least_as_permissive_as_current_mod() {
    expect_bind_error(
        ProjectPath {
            callee_file_path: file!(),
            checked_unadjusted_pack_omlet_path: checked_path!(
                "../../../sample_code/should_fail/multi_file/bind/transparency_not_at_least_as_permissive_as_current_mod/pack.omlet"
            ),
        },
        |err| match err {
            BindError::TransparencyWasNotAtLeastAsPermissiveAsCurrentMod(TransparencyWasNotAtLeastAsPermissiveAsCurrentModError {
                transparency_modifier,
            }) => {
                match transparency_modifier.kind {
                    simplified_ast::ModScopeModifierKind::PackRelative { path_after_pack_kw } => {
                        assert_eq!(1, path_after_pack_kw.len());
                        assert_eq!("nat", path_after_pack_kw[0].name.src_str());
                    }
                    _ => panic!("Unexpected transparency modifier: {:?}", transparency_modifier),
                }
            }
            _ => panic!("Unexpected error: {:?}", err),
        },
    );
}

#[test]
fn use_mod_as_is() {
    expect_bind_error(
        ProjectPath {
            callee_file_path: file!(),
            checked_unadjusted_pack_omlet_path: checked_path!(
                "../../../sample_code/should_fail/multi_file/bind/use_mod_as_is/pack.omlet"
            ),
        },
        |err| match err {
            BindError::CannotUselesslyImportItemAsSelf(CannotUselesslyImportItemAsSelfError {
                use_statement,
            }) => {
                assert_eq!(
                    simplified_ast::UseStatementFirstComponentKind::Mod,
                    use_statement.first_component.kind
                );
            }
            _ => panic!("Unexpected error: {:?}", err),
        },
    );
}

#[test]
fn use_super_as_is() {
    expect_bind_error(
        ProjectPath {
            callee_file_path: file!(),
            checked_unadjusted_pack_omlet_path: checked_path!(
                "../../../sample_code/should_fail/multi_file/bind/use_super_as_is/pack.omlet"
            ),
        },
        |err| match err {
            BindError::CannotUselesslyImportItemAsSelf(CannotUselesslyImportItemAsSelfError {
                use_statement,
            }) => {
                assert_eq!(
                    simplified_ast::UseStatementFirstComponentKind::Super(
                        NonZeroUsize::new(1).unwrap()
                    ),
                    use_statement.first_component.kind
                );
            }
            _ => panic!("Unexpected error: {:?}", err),
        },
    );
}

#[test]
fn use_pack_as_is() {
    expect_bind_error(
        ProjectPath {
            callee_file_path: file!(),
            checked_unadjusted_pack_omlet_path: checked_path!(
                "../../../sample_code/should_fail/multi_file/bind/use_pack_as_is/pack.omlet"
            ),
        },
        |err| match err {
            BindError::CannotUselesslyImportItemAsSelf(CannotUselesslyImportItemAsSelfError {
                use_statement,
            }) => {
                assert_eq!(
                    simplified_ast::UseStatementFirstComponentKind::Pack,
                    use_statement.first_component.kind
                );
            }
            _ => panic!("Unexpected error: {:?}", err),
        },
    );
}

#[test]
fn use_identifier_as_is() {
    expect_bind_error(
        ProjectPath {
            callee_file_path: file!(),
            checked_unadjusted_pack_omlet_path: checked_path!(
                "../../../sample_code/should_fail/multi_file/bind/use_identifier_as_is/pack.omlet"
            ),
        },
        |err| match &err {
            BindError::CannotUselesslyImportItemAsSelf(CannotUselesslyImportItemAsSelfError {
                use_statement,
            }) => match &use_statement.first_component.kind {
                simplified_ast::UseStatementFirstComponentKind::Identifier(name) => {
                    assert_eq!("Blat", name.src_str());
                }
                _ => panic!("Unexpected error: {:?}", err),
            },
            _ => panic!("Unexpected error: {:?}", err),
        },
    );
}

#[test]
fn mod_file_not_found() {
    expect_bind_error(
        ProjectPath {
            callee_file_path: file!(),
            checked_unadjusted_pack_omlet_path: checked_path!(
                "../../../sample_code/should_fail/multi_file/bind/mod_file_not_found/pack.omlet"
            ),
        },
        |err| match err {
            BindError::ModFileNotFound(ModFileNotFoundError { mod_name }) => {
                assert_eq!("foo", mod_name.name.src_str());
            }
            _ => panic!("Unexpected error: {:?}", err),
        },
    );
}

#[test]
fn term_in_visibility_modifier() {
    expect_bind_error(
        ProjectPath {
            callee_file_path: file!(),
            checked_unadjusted_pack_omlet_path: checked_path!(
                "../../../sample_code/should_fail/multi_file/bind/term_in_visibility_modifier/pack.omlet"
            ),
        },
        |err| match err {
            BindError::ExpectedModButNameRefersToTerm(ExpectedModButNameRefersToTermError { name_components }) => {
                assert_eq!("pack.foo.Foo", name_components.iter().map(|c| c.name.src_str()).collect::<Vec<_>>().join("."));
            }
            _ => panic!("Unexpected error: {:?}", err),
        },
    );
}

#[test]
fn expected_term_got_mod() {
    expect_bind_error(
        ProjectPath {
            callee_file_path: file!(),
            checked_unadjusted_pack_omlet_path: checked_path!(
                "../../../sample_code/should_fail/multi_file/bind/expected_term_got_mod/pack.omlet"
            ),
        },
        |err| match err {
            BindError::ExpectedTermButNameRefersToMod(ExpectedTermButNameRefersToModError {
                name_components,
            }) => {
                assert_eq!(
                    "foo",
                    name_components
                        .iter()
                        .map(|c| c.name.src_str())
                        .collect::<Vec<_>>()
                        .join(".")
                );
            }
            _ => panic!("Unexpected error: {:?}", err),
        },
    );
}

#[test]
fn visibility_stricter_than_transparency() {
    expect_bind_error(
        ProjectPath {
            callee_file_path: file!(),
            checked_unadjusted_pack_omlet_path: checked_path!(
                "../../../sample_code/should_fail/multi_file/bind/visibility_stricter_than_transparency/pack.omlet"
            ),
        },
        |err| match err {
            BindError::TransparencyWasNotAtLeastAsPermissiveAsVisibility(TransparencyWasNotAtLeastAsPermissiveAsVisibilityError {
                transparency_modifier: transparency,
            }) => {
                assert_eq!(
                    simplified_ast::ModScopeModifierKind::Global,
                    transparency.kind
                );
            }
            _ => panic!("Unexpected error: {:?}", err),
        },
    );
}
