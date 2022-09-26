use super::*;

/// The job of `panicker` is to panic if the error is different than the expected
/// error.
fn expect_type_arg_extraction_error(src: &str, panicker: impl Fn(&Expression)) {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let file = parse_file(tokens, file_id).expect("Parsing failed");
    let mut registry = NodeRegistry::empty();
    let file_id = register_file(&mut registry, file);
    let file = registry.file(file_id);
    let mut provider = SymbolProvider::new();
    let symbol_db = bind_symbols_to_identifiers(&registry, vec![file_id], &mut provider)
        .expect("Binding failed");
    let err = extract_variant_type_args_for_file(&symbol_db, file)
        .expect_err("Type arg extraction unexpectedly succeeded");
    let illegal_variant_return_type = &registry.wrapped_expression(err.0).expression;
    panicker(illegal_variant_return_type);
}

#[test]
fn param() {
    let src = include_str!("../sample_code/should_fail/variant_return_type/param.ph");
    expect_type_arg_extraction_error(src, |return_type| match return_type {
        Expression::Identifier(identifier) => {
            assert_eq!(
                identifier.name,
                standard_ident_name("a"),
                "Unexpected variant return type: {:#?}",
                return_type
            );
        }
        _ => panic!("Unexpected variant return type: {:#?}", return_type),
    });
}

#[test]
fn complex_expression() {
    let src = include_str!("../sample_code/should_fail/variant_return_type/complex_expression.ph");
    expect_type_arg_extraction_error(src, |return_type| {
        assert!(
            matches!(return_type, Expression::Match(_)),
            "Unexpected variant return type: {:#?}",
            return_type
        );
    });
}

#[test]
fn foreign_nullary_type() {
    let src =
        include_str!("../sample_code/should_fail/variant_return_type/foreign_nullary_type.ph");
    expect_type_arg_extraction_error(src, |return_type| match return_type {
        Expression::Identifier(identifier) => {
            assert_eq!(
                identifier.name,
                standard_ident_name("U"),
                "Unexpected variant return type: {:#?}",
                return_type
            );
        }
        _ => panic!("Unexpected variant return type: {:#?}", return_type),
    });
}

#[test]
fn foreign_non_nullary_type() {
    let src =
        include_str!("../sample_code/should_fail/variant_return_type/foreign_non_nullary_type.ph");
    expect_type_arg_extraction_error(src, |return_type| match return_type {
        Expression::Call(call) => {
            assert_eq!(call.args.len(), 1);
            match &call.callee.expression {
                Expression::Identifier(identifier) => {
                    assert_eq!(
                        identifier.name,
                        standard_ident_name("T"),
                        "Unexpected variant return type: {:#?}",
                        return_type
                    );
                }
                _ => panic!("Unexpected variant return type: {:#?}", return_type),
            }
        }
        _ => panic!("Unexpected variant return type: {:#?}", return_type),
    });
}
