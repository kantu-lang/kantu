use super::*;

// TODO: Fix
#[ignore]
#[test]
fn check() {
    let src = include_str!("../sample_code/should_succeed/check.ph");
    expect_success_with_check_warnings(src, vec![]);
}

fn expect_success_with_check_warnings(src: &str, expected_warnings: Vec<CheckWarningBuilder>) {
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
    assert_warnings_equal_except_for_order(expected_warnings, warnings);
    let _js_ast =
        JavaScript::generate_code(&registry, &[file_id.raw()]).expect("Code generation failed");
}

#[derive(Clone, Debug)]
enum CheckWarningBuilder<'a> {
    NoGoal,
    // TODO: Rename "missing" to "incomplete"?
    MissingCheckeeType,
    UntypecheckableExpression(InvalidExpressionBuilder<'a>),
    IllTypedCheckeeType(&'a str),
    IncorrectCheckeeType {
        checkee_type_src: &'a str,
        expected_normalized_src: &'a str,
        actual_normalized_src: &'a str,
    },
    MissingCheckeeValue,
    IllTypedCheckeeValue(&'a str),
    IncorrectCheckeeValue {
        checkee_type_src: &'a str,
        expected_src: &'a str,
        actual_src: &'a str,
    },
}

#[derive(Clone, Debug)]
enum InvalidExpressionBuilder<'a> {
    SymbolicallyInvalid(&'a str),
    IllegalFunRecursion(&'a str),
}

fn assert_warnings_equal_except_for_order(
    _expected: Vec<CheckWarningBuilder>,
    _actual: Vec<TypeCheckWarning>,
) {
    unimplemented!()
}
