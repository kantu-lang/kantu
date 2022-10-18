use super::*;

/// The job of `panicker` is to panic if the error is different than the expected
/// error.
fn expect_type_arg_extraction_error(src: &str, panicker: impl Fn(ExpressionRef, &NodeRegistry)) {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let file = parse_file(tokens, file_id).expect("Parsing failed");
    let file = simplify_file(file).expect("AST Simplification failed");
    let mut registry = NodeRegistry::empty();
    let file_id = register_file(&mut registry, file);
    let file = registry.file(file_id);
    let symbol_db = bind_symbols_to_identifiers(&registry, vec![file_id]).expect("Binding failed");
    let err = check_variant_return_types_for_file(&symbol_db, &registry, file)
        .expect_err("Type arg extraction unexpectedly succeeded");
    let illegal_variant_return_type = registry.expression_ref(err.0);
    panicker(illegal_variant_return_type, &registry);
}

#[test]
fn param() {
    let src = include_str!("../sample_code/should_fail/variant_return_type/param.ph");
    expect_type_arg_extraction_error(src, |return_type, registry| match return_type {
        ExpressionRef::Name(name) => {
            assert_eq!(
                component_identifier_names(registry, name.id),
                vec![standard_ident_name("a")],
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
    expect_type_arg_extraction_error(src, |return_type, _registry| {
        assert!(
            matches!(return_type, ExpressionRef::Match(_)),
            "Unexpected variant return type: {:#?}",
            return_type
        );
    });
}

#[test]
fn foreign_nullary_type() {
    let src =
        include_str!("../sample_code/should_fail/variant_return_type/foreign_nullary_type.ph");
    expect_type_arg_extraction_error(src, |return_type, registry| match return_type {
        ExpressionRef::Name(name) => {
            assert_eq!(
                component_identifier_names(registry, name.id),
                vec![standard_ident_name("U")],
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
    expect_type_arg_extraction_error(src, |return_type, registry| match return_type {
        ExpressionRef::Call(call) => {
            let arg_ids = registry.expression_list(call.arg_list_id);
            assert_eq!(arg_ids.len(), 1);
            let callee = registry.expression_ref(call.callee_id);
            match callee {
                ExpressionRef::Name(name) => {
                    assert_eq!(
                        component_identifier_names(registry, name.id),
                        vec![standard_ident_name("T")],
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
