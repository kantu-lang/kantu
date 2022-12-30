use super::*;

fn expect_name_not_found_error<'a, N>(src: &str, name_components: N)
where
    N: Copy + IntoIterator<Item = &'a str>,
{
    expect_bind_error(src, |err| match err {
        BindError::NameNotFound(err) => {
            assert_eq!(
                err.name_components
                    .iter()
                    .map(|identifier| identifier.name.clone())
                    .collect::<Vec<_>>(),
                name_components
                    .into_iter()
                    .map(standard_ident_name)
                    .collect::<Vec<_>>(),
                "Unexpected param name"
            );
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

fn expect_underscore_not_found_error(src: &str) {
    expect_bind_error(src, |err| match err {
        BindError::NameNotFound(err) => {
            assert_eq!(
                err.name_components
                    .iter()
                    .map(|identifier| identifier.name.clone())
                    .collect::<Vec<_>>(),
                vec![IdentifierName::Reserved(ReservedIdentifierName::Underscore)],
                "Unexpected param name"
            );
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

#[test]
fn reference_let_in_body() {
    let src = include_str!("../../sample_code/should_fail/scope/ref_let_in_body.ph");
    expect_name_not_found_error(src, ["a"]);
}

#[test]
fn reference_type_in_param() {
    let src = include_str!("../../sample_code/should_fail/scope/ref_type_in_param.ph");
    expect_name_not_found_error(src, ["U"]);
}

#[test]
fn reference_fun_in_param() {
    let src = include_str!("../../sample_code/should_fail/scope/ref_fun_in_param.ph");
    expect_name_not_found_error(src, ["g"]);
}

#[test]
fn reference_fun_in_return_type() {
    let src = include_str!("../../sample_code/should_fail/scope/ref_fun_in_return_type.ph");
    expect_name_not_found_error(src, ["g"]);
}

fn expect_bind_error(src: &str, panicker: impl Fn(BindError)) {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let file = parse_file(tokens, file_id).expect("Parsing failed");
    let file = simplify_file(file).expect("AST Simplification failed");
    let err = bind_files(file_id, vec![file], &FileGraph::from_root(file_id))
        .expect_err("Binding unexpectedly succeeded");
    panicker(err);
}

#[test]
fn reference_variant_in_prev_variant() {
    let src = include_str!("../../sample_code/should_fail/scope/ref_variant_in_prev_variant.ph");
    expect_name_not_found_error(src, ["Bar", "C"]);
}

#[test]
fn reference_variant_in_variant_return_type() {
    let src =
        include_str!("../../sample_code/should_fail/scope/ref_variant_in_variant_return_type.ph");
    expect_name_not_found_error(src, ["Bar", "B"]);
}

#[test]
fn reference_variant_in_variant_param_type() {
    let src =
        include_str!("../../sample_code/should_fail/scope/ref_variant_in_variant_param_type.ph");
    expect_name_not_found_error(src, ["Bar", "D"]);
}

fn expect_name_clash_error(src: &str, expected_source_name: &str) {
    expect_bind_error(src, |err| match err {
        BindError::NameClash(err) => {
            assert!(
                matches!(err.old, OwnedSymbolSource::Identifier(identifier) if identifier.name == standard_ident_name(expected_source_name)),
                "Unexpected old name"
            );
            assert!(
                matches!(err.new, OwnedSymbolSource::Identifier(identifier) if identifier.name == standard_ident_name(expected_source_name)),
                "Unexpected new name"
            );
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

#[test]
fn fun_shadows_own_param() {
    let src = include_str!("../../sample_code/should_fail/scope/fun_shadows_own_param.ph");
    expect_name_clash_error(src, "g");
}

#[test]
fn duplicate_variants() {
    let src = include_str!("../../sample_code/should_fail/scope/duplicate_variants.ph");
    expect_name_clash_error(src, "F");
}

#[test]
fn duplicate_type_params() {
    let src = include_str!("../../sample_code/should_fail/scope/duplicate_type_params.ph");
    expect_name_clash_error(src, "T");
}

#[test]
fn duplicate_variant_params() {
    let src = include_str!("../../sample_code/should_fail/scope/duplicate_variant_params.ph");
    expect_name_clash_error(src, "R");
}

#[test]
fn duplicate_fun_params() {
    let src = include_str!("../../sample_code/should_fail/scope/duplicate_fun_params.ph");
    expect_name_clash_error(src, "U");
}

#[test]
fn duplicate_forall_params() {
    let src = include_str!("../../sample_code/should_fail/scope/duplicate_forall_params.ph");
    expect_name_clash_error(src, "Q");
}

#[test]
fn duplicate_match_case_params() {
    let src = include_str!("../../sample_code/should_fail/scope/duplicate_match_case_params.ph");
    expect_name_clash_error(src, "x");
}

#[test]
fn ref_underscore_param() {
    let src = include_str!("../../sample_code/should_fail/scope/ref_underscore_param.ph");
    expect_underscore_not_found_error(src);
}

#[test]
fn ref_underscore_fun() {
    let src = include_str!("../../sample_code/should_fail/scope/ref_underscore_fun.ph");
    expect_underscore_not_found_error(src);
}
