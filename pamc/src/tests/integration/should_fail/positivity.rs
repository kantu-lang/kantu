use super::*;

/// The job of `panicker` is to panic if the error is different than the expected
/// error.
fn expect_positivity_error(src: &str, panicker: impl Fn(&NodeRegistry, TypePositivityError)) {
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
    let file = registry.get(file_id);

    let file_id = validate_variant_return_types_in_file(&registry, file)
        .expect("Variant return type validation failed");
    let file_id = validate_fun_recursion_in_file(&mut registry, file_id)
        .expect("Fun recursion validation failed");
    let err = validate_type_positivity_in_file(&mut registry, file_id)
        .expect_err("Type positivity validation unexpectedly succeeded");
    panicker(&registry, err);
}

#[test]
fn negative_recursion() {
    let src = include_str!("../../sample_code/should_fail/positivity/negative_recursion.ph");
    expect_positivity_error(src, |_registry, _err| {
        todo!();
    });
}
