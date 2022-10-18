use super::*;

/// The job of `panicker` is to panic if the error is different than the expected
/// error.
fn expect_recursion_error(src: &str, panicker: impl Fn(&NodeRegistry, IllegalFunRecursionError)) {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let file = parse_file(tokens, file_id).expect("Parsing failed");
    let file = simplify_file(file).expect("AST Simplification failed");
    let mut registry = NodeRegistry::empty();
    let file_id = register_file(&mut registry, file);
    let file = registry.file(file_id);
    let symbol_db = bind_symbols_to_identifiers(&registry, vec![file_id]).expect("Binding failed");
    let _variant_return_type_map = check_variant_return_types_for_file(&symbol_db, &registry, file);
    let err = validate_fun_recursion_in_file(&symbol_db, &registry, file)
        .expect_err("Fun recursion validation unexpectedly succeeded");
    panicker(&registry, err);
}

#[test]
fn rec_fun_same_param() {
    let src = include_str!("../sample_code/should_fail/illegal_recursion/rec_fun_same_param.ph");
    expect_recursion_error(src, |registry, err| match err {
        IllegalFunRecursionError::NonSubstructPassedToDecreasingParam {
            callee: callee_id,
            arg: arg_id,
        } => {
            let arg = &registry.expression_ref(arg_id);
            assert_eq!(
                component_identifier_names(registry, callee_id),
                vec![standard_ident_name("x")],
                "Unexpected param name"
            );
            assert!(
                matches!(arg, ExpressionRef::Name(name) if component_identifier_names(registry, name.id) == vec![standard_ident_name("a")]),
                "Unexpected arg: {:#?}",
                arg
            );
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

#[test]
fn rec_fun_non_substruct() {
    let src = include_str!("../sample_code/should_fail/illegal_recursion/rec_fun_non_substruct.ph");
    expect_recursion_error(src, |registry, err| match err {
        IllegalFunRecursionError::NonSubstructPassedToDecreasingParam {
            callee: callee_id,
            arg: arg_id,
        } => {
            let arg = &registry.expression_ref(arg_id);
            assert_eq!(
                component_identifier_names(registry, callee_id),
                vec![standard_ident_name("x")],
                "Unexpected param name"
            );
            assert!(
                matches!(arg, ExpressionRef::Name(name) if component_identifier_names(registry, name.id) == vec![standard_ident_name("b")]),
                "Unexpected arg: {:#?}",
                arg
            );
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

#[test]
fn rec_fun_non_ident() {
    let src = include_str!("../sample_code/should_fail/illegal_recursion/rec_fun_non_ident.ph");
    expect_recursion_error(src, |registry, err| match err {
        IllegalFunRecursionError::NonSubstructPassedToDecreasingParam {
            callee: callee_id,
            arg: arg_id,
        } => {
            let arg = &registry.expression_ref(arg_id);
            assert_eq!(
                component_identifier_names(registry, callee_id),
                vec![standard_ident_name("x")],
                "Unexpected param name"
            );
            assert!(
                matches!(arg, ExpressionRef::Name(name) if component_identifier_names(registry, name.id) == vec![standard_ident_name("Nat"), standard_ident_name("O")]),
                "Unexpected arg: {:#?}",
                arg
            );
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

#[test]
fn rec_fun_no_decreasing_param() {
    let src =
        include_str!("../sample_code/should_fail/illegal_recursion/rec_fun_no_decreasing_param.ph");
    expect_recursion_error(src, |registry, err| match err {
        IllegalFunRecursionError::RecursivelyCalledFunctionWithoutDecreasingParam {
            callee: callee_id,
        } => {
            assert_eq!(
                component_identifier_names(registry, callee_id),
                vec![standard_ident_name("x")],
                "Unexpected param name"
            );
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}
