use super::*;

fn expect_name_not_found_error(src: &str, expected_unfindable_name: &str) {
    expect_bind_error(src, |err, _registry| match err {
        BindError::NameNotFound(err) => {
            assert_eq!(
                err.name,
                standard_ident_name(expected_unfindable_name),
                "Unexpected param name"
            );
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

fn expect_invalid_dot_rhs_error(src: &str, expected_unfindable_name: &str) {
    expect_bind_error(src, |err, registry| match err {
        BindError::InvalidDotExpressionRhs(rhs_id) => {
            let invalid_rhs = registry.identifier(rhs_id);
            assert_eq!(
                invalid_rhs.name,
                standard_ident_name(expected_unfindable_name),
                "Unexpected param name"
            );
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

#[test]
fn reference_let_in_body() {
    let src = include_str!("../sample_code/should_fail/scope/ref_let_in_body.ph");
    expect_name_not_found_error(src, "a");
}

#[test]
fn reference_type_in_param() {
    let src = include_str!("../sample_code/should_fail/scope/ref_type_in_param.ph");
    expect_name_not_found_error(src, "U");
}

#[test]
fn reference_fun_in_param() {
    let src = include_str!("../sample_code/should_fail/scope/ref_fun_in_param.ph");
    expect_name_not_found_error(src, "g");
}

#[test]
fn reference_fun_in_return_type() {
    let src = include_str!("../sample_code/should_fail/scope/ref_fun_in_return_type.ph");
    expect_name_not_found_error(src, "g");
}

fn expect_bind_error(src: &str, panicker: impl Fn(BindError, &NodeRegistry)) {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let file = parse_file(tokens, file_id).expect("Parsing failed");
    let file = simplify_file(file).expect("AST Simplification failed");
    let mut registry = NodeRegistry::empty();
    let file_id = register_file(&mut registry, file);
    let err = bind_symbols_to_identifiers(&registry, vec![file_id])
        .expect_err("Binding unexpectedly succeeded");
    panicker(err, &registry);
}

#[test]
fn reference_variant_in_prev_variant() {
    let src = include_str!("../sample_code/should_fail/scope/ref_variant_in_prev_variant.ph");
    expect_invalid_dot_rhs_error(src, "C");
}

#[test]
fn reference_variant_in_variant_return_type() {
    let src =
        include_str!("../sample_code/should_fail/scope/ref_variant_in_variant_return_type.ph");
    expect_invalid_dot_rhs_error(src, "B");
}

#[test]
fn reference_variant_in_variant_param_type() {
    let src = include_str!("../sample_code/should_fail/scope/ref_variant_in_variant_param_type.ph");
    expect_invalid_dot_rhs_error(src, "D");
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SymbolSourceKind {
    Type,
    Variant,
    Param,
    Let,
    Fun,
    BuiltinTypeTitleCase,
}

fn expect_name_clash_error(
    src: &str,
    expected_name: &str,
    expected_old_kind: SymbolSourceKind,
    expected_new_kind: SymbolSourceKind,
) {
    use crate::data::symbol_database::SymbolSource;
    fn symbol_name(
        registry: &NodeRegistry,
        sym_src: SymbolSource,
    ) -> (&IdentifierName, SymbolSourceKind) {
        match sym_src {
            SymbolSource::Type(type_id) => {
                let name_id = registry.type_statement(type_id).name_id;
                (&registry.identifier(name_id).name, SymbolSourceKind::Type)
            }
            SymbolSource::Variant(variant_id) => {
                let name_id = registry.variant(variant_id).name_id;
                (
                    &registry.identifier(name_id).name,
                    SymbolSourceKind::Variant,
                )
            }
            SymbolSource::TypedParam(param_id) => {
                let name_id = registry.param(param_id).name_id;
                (&registry.identifier(name_id).name, SymbolSourceKind::Param)
            }
            SymbolSource::UntypedParam(param_id) => {
                (&registry.identifier(param_id).name, SymbolSourceKind::Param)
            }
            SymbolSource::Let(let_id) => {
                let name_id = registry.let_statement(let_id).name_id;
                (&registry.identifier(name_id).name, SymbolSourceKind::Let)
            }
            SymbolSource::Fun(fun_id) => {
                let name_id = registry.fun(fun_id).name_id;
                (&registry.identifier(name_id).name, SymbolSourceKind::Fun)
            }
            SymbolSource::BuiltinTypeTitleCase => (
                &IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase),
                SymbolSourceKind::BuiltinTypeTitleCase,
            ),
        }
    }

    expect_bind_error(src, |err, registry| match err {
        BindError::NameClash(err) => {
            let (old_name, old_kind) = symbol_name(registry, err.old);
            assert_eq!(
                old_name,
                &standard_ident_name(expected_name),
                "Unexpected old name"
            );
            assert_eq!(old_kind, expected_old_kind, "Unexpected old kind");
            let (new_name, new_kind) = symbol_name(registry, err.new);
            assert_eq!(
                new_name,
                &standard_ident_name(expected_name),
                "Unexpected new name"
            );
            assert_eq!(new_kind, expected_new_kind, "Unexpected new kind");
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

#[test]
fn fun_shadows_own_param() {
    let src = include_str!("../sample_code/should_fail/scope/fun_shadows_own_param.ph");
    expect_name_clash_error(src, "g", SymbolSourceKind::Param, SymbolSourceKind::Fun);
}

#[test]
fn duplicate_variants() {
    let src = include_str!("../sample_code/should_fail/scope/duplicate_variants.ph");
    expect_name_clash_error(
        src,
        "F",
        SymbolSourceKind::Variant,
        SymbolSourceKind::Variant,
    );
}

#[test]
fn duplicate_type_params() {
    let src = include_str!("../sample_code/should_fail/scope/duplicate_type_params.ph");
    expect_name_clash_error(src, "T", SymbolSourceKind::Param, SymbolSourceKind::Param);
}

#[test]
fn duplicate_variant_params() {
    let src = include_str!("../sample_code/should_fail/scope/duplicate_variant_params.ph");
    expect_name_clash_error(src, "R", SymbolSourceKind::Param, SymbolSourceKind::Param);
}

#[test]
fn duplicate_fun_params() {
    let src = include_str!("../sample_code/should_fail/scope/duplicate_fun_params.ph");
    expect_name_clash_error(src, "U", SymbolSourceKind::Param, SymbolSourceKind::Param);
}

#[test]
fn duplicate_forall_params() {
    let src = include_str!("../sample_code/should_fail/scope/duplicate_forall_params.ph");
    expect_name_clash_error(src, "Q", SymbolSourceKind::Param, SymbolSourceKind::Param);
}

#[test]
fn duplicate_match_case_params() {
    let src = include_str!("../sample_code/should_fail/scope/duplicate_match_case_params.ph");
    expect_name_clash_error(src, "x", SymbolSourceKind::Param, SymbolSourceKind::Param);
}

// TODO: Move this to the AST simplification test.
#[test]
#[ignore]
fn reference_unbindable_dot_lhs() {
    let src = include_str!("../sample_code/should_fail/scope/unbindable_dot_lhs.ph");
    expect_bind_error(src, |err, registry| match err {
        BindError::UnbindableDotExpressionLhs(lhs_id) => {
            let invalid_lhs = &registry.expression_ref(lhs_id);
            assert!(
                matches!(invalid_lhs, ExpressionRef::Match(_)),
                "Unexpected lhs {:?}",
                invalid_lhs
            );
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}
