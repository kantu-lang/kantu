use super::*;

/// The job of `panicker` is to panic if the error is different than the expected
/// error.
fn expect_recursion_error(src: &str, panicker: impl Fn(&NodeRegistry, TypeCheckError)) {
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

    check_variant_return_types_for_file(&registry, file)
        .expect("Variant return type validation failed");
    validate_fun_recursion_in_file(&registry, file).expect("Fun recursion validation failed");
    let err = type_check_files(&mut registry, &[file_id]).expect_err("Type checking failed");
    panicker(&registry, err);
}

fn expect_illegal_type_error(src: &str, expected_illegal_type_src: &str) {
    expect_recursion_error(src, |registry, err| match err {
        TypeCheckError::IllegalTypeExpression(id) => {
            let actual_src = format_expression(
                &expand_expression(registry, id),
                0,
                &FormatOptions {
                    ident_size_in_spaces: 4,
                    print_db_indices: false,
                    print_fun_body_status: false,
                },
            );
            assert_eq_up_to_white_space(&actual_src, expected_illegal_type_src);
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

#[test]
fn illegal_type_forall_output() {
    let src = include_str!("../sample_code/should_fail/type_check/illegal_type/forall_output.ph");
    expect_illegal_type_error(src, "U.U");
}

#[test]
fn illegal_type_forall_param() {
    let src = include_str!("../sample_code/should_fail/type_check/illegal_type/forall_param.ph");
    expect_illegal_type_error(src, "U.U");
}

#[test]
fn illegal_type_fun_param() {
    let src = include_str!("../sample_code/should_fail/type_check/illegal_type/fun_param.ph");
    expect_illegal_type_error(src, "U.U");
}

#[test]
fn illegal_type_fun_return() {
    let src = include_str!("../sample_code/should_fail/type_check/illegal_type/fun_return.ph");
    expect_illegal_type_error(src, "U.U");
}

#[test]
fn illegal_type_type_param() {
    let src = include_str!("../sample_code/should_fail/type_check/illegal_type/type_param.ph");
    expect_illegal_type_error(src, "U.U");
}

#[test]
fn illegal_type_variant_param() {
    let src = include_str!("../sample_code/should_fail/type_check/illegal_type/variant_param.ph");
    expect_illegal_type_error(src, "U.U");
}

// TODO: Add other tests
