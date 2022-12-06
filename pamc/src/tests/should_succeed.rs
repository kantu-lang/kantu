use crate::{
    data::{node_registry::NodeRegistry, FileId},
    processing::{
        bind_type_independent::bind_files,
        generate_code::{targets::javascript::JavaScript, CompileTarget},
        lex::lex,
        lighten_ast::lighten_file,
        parse::parse_file,
        simplify_ast::simplify_file,
        type_check::type_check_files,
        validate_fun_recursion::validate_fun_recursion_in_file,
        validate_variant_return_types::validate_variant_return_types_in_file,
    },
};

#[test]
fn hello_world() {
    let src = include_str!("sample_code/should_succeed/hello_world.ph");
    expect_success_with_no_warnings(src);
}

#[test]
fn optional_commas() {
    let src = include_str!("sample_code/should_succeed/optional_commas.ph");
    expect_success_with_no_warnings(src);
}

#[test]
fn empty_implies_anything() {
    let src = include_str!("sample_code/should_succeed/empty_implies_anything.ph");
    expect_success_with_no_warnings(src);
}

#[test]
fn match_explosion() {
    let src = include_str!("sample_code/should_succeed/match_explosion.ph");
    expect_success_with_no_warnings(src);
}

#[test]
fn coercionless_match() {
    let src = include_str!("sample_code/should_succeed/coercionless_match.ph");
    expect_success_with_no_warnings(src);
}

#[test]
fn ill_typed_until_substituted() {
    let src = include_str!("sample_code/should_succeed/ill_typed_until_substituted.ph");
    expect_success_with_no_warnings(src);
}

#[test]
fn forall() {
    let src = include_str!("sample_code/should_succeed/forall.ph");
    expect_success_with_no_warnings(src);
}

#[test]
fn underscore() {
    let src = include_str!("sample_code/should_succeed/underscore.ph");
    expect_success_with_no_warnings(src);
}

#[test]
fn plus_commutative() {
    let src = include_str!("sample_code/should_succeed/plus_commutative.ph");
    expect_success_with_no_warnings(src);
}

#[test]
fn exists() {
    let src = include_str!("sample_code/should_succeed/exists.ph");
    expect_success_with_no_warnings(src);
}

// TODO: Fix
#[ignore]
#[test]
fn check() {
    let src = include_str!("sample_code/should_succeed/check.ph");
    expect_success_with_no_warnings(src);
}

fn expect_success_with_no_warnings(src: &str) {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let file = parse_file(tokens, file_id).expect("Parsing failed");
    let file = simplify_file(file).expect("AST Simplification failed");
    let file = bind_files(vec![file])
        .expect("Binding failed")
        .into_iter()
        .next()
        .unwrap();
    let mut registry = NodeRegistry::empty();
    let file_id = lighten_file(&mut registry, file);
    let file = registry.file(file_id);
    let file_id = validate_variant_return_types_in_file(&registry, file)
        .expect("Variant return type validation failed");
    let file_id = validate_fun_recursion_in_file(&mut registry, file_id)
        .expect("Fun recursion validation failed");
    let warnings = type_check_files(&mut registry, &[file_id]).expect("Type checking failed");
    assert_eq!(0, warnings.len(), "One or more warnings were emitted");
    let _js_ast =
        JavaScript::generate_code(&registry, &[file_id.raw()]).expect("Code generation failed");
}
