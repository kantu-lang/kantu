use crate::{
    data::{node_registry::NodeRegistry, FileId},
    processing::{
        bind_type_independent::bind_files,
        check_variant_return_types::check_variant_return_types_for_file,
        generate_code::{targets::javascript::JavaScript, CompileTarget},
        lex::lex,
        lighten_ast::lighten_file,
        parse::parse_file,
        simplify_ast::simplify_file,
        type_check::type_check_files,
        validate_fun_recursion::validate_fun_recursion_in_file,
    },
};

#[test]
fn hello_world() {
    let src = include_str!("sample_code/should_succeed/hello_world.ph");
    expect_success(src);
}

#[test]
fn optional_commas() {
    let src = include_str!("sample_code/should_succeed/optional_commas.ph");
    expect_success(src);
}

#[test]
fn empty_implies_anything() {
    let src = include_str!("sample_code/should_succeed/empty_implies_anything.ph");
    expect_success(src);
}

#[test]
fn match_explosion() {
    let src = include_str!("sample_code/should_succeed/match_explosion.ph");
    expect_success(src);
}

#[test]
fn coercionless_match() {
    let src = include_str!("sample_code/should_succeed/coercionless_match.ph");
    {
        use crate::{
            data::{node_registry::NodeRegistry, FileId},
            processing::{
                bind_type_independent::bind_files,
                check_variant_return_types::check_variant_return_types_for_file,
                lex::lex,
                lighten_ast::lighten_file,
                parse::parse_file,
                simplify_ast::simplify_file,
                test_utils::{
                    expand_lightened::expand_expression,
                    format::{format_expression, FormatOptions},
                },
                type_check::{type_check_files, TypeCheckError},
                validate_fun_recursion::validate_fun_recursion_in_file,
            },
        };

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
        let err = type_check_files(&mut registry, &[file_id])
            .expect_err("Type checking unexpectedly succeeded. Maybe this test case is fixed...?");
        match err {
            TypeCheckError::TypeMismatch {
                expression_id,
                expected_type_id,
                actual_type_id,
            } => {
                let registry = &registry;
                let expression_src = format_expression(
                    &expand_expression(registry, expression_id),
                    0,
                    &FormatOptions {
                        ident_size_in_spaces: 4,
                        print_db_indices: true,
                        print_fun_body_status: false,
                    },
                );
                println!("TYPE_MISMATCH.expression:\n{}", expression_src);

                let expected_type_src = format_expression(
                    &expand_expression(registry, expected_type_id.raw()),
                    0,
                    &FormatOptions {
                        ident_size_in_spaces: 4,
                        print_db_indices: true,
                        print_fun_body_status: false,
                    },
                );
                println!("TYPE_MISMATCH.expected_type:\n{}", expected_type_src);

                let actual_type_src = format_expression(
                    &expand_expression(registry, actual_type_id.raw()),
                    0,
                    &FormatOptions {
                        ident_size_in_spaces: 4,
                        print_db_indices: true,
                        print_fun_body_status: false,
                    },
                );
                println!("TYPE_MISMATCH.actual_type:\n{}", actual_type_src);

                panic!("Unexpected TypeMismatch error: {:#?}", err);
            }
            _ => panic!("Unexpected error: {:#?}", err),
        }
    }
    // expect_success(src);
}

fn expect_success(src: &str) {
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
    type_check_files(&mut registry, &[file_id]).expect("Type checking failed");
    let _js_ast = JavaScript::generate_code(&registry, &[file_id]).expect("Code generation failed");
}
