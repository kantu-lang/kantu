use super::*;

/// The job of `panicker` is to panic if the error is different than the expected
/// error.
fn expect_recursion_error(src: &str, panicker: impl Fn(&NodeRegistry, IllegalFunRecursionError)) {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let file = parse_file(tokens, file_id).expect("Parsing failed");
    let file = simplify_file(file).expect("AST Simplification failed");
    let file_items =
        bind_files(file_id, vec![file], &FileGraph::from_root(file_id)).expect("Binding failed");
    let mut registry = NodeRegistry::empty();
    let file_item_list_id = register_file_items(&mut registry, file_items);

    let file_item_list_id =
        validate_variant_return_types_in_file_items(&registry, file_item_list_id)
            .expect("Variant return type validation failed");
    let err = validate_fun_recursion_in_file_items(&mut registry, file_item_list_id)
        .expect_err("Fun recursion validation unexpectedly succeeded");
    panicker(&registry, err);
}

#[test]
fn rec_fun_same_param() {
    let src = include_str!("../../sample_code/should_fail/illegal_recursion/rec_fun_same_param.ph");
    expect_recursion_error(src, |registry, err| match err {
        IllegalFunRecursionError::NonSubstructPassedToDecreasingParam { callee_id, arg_id } => {
            let arg = &registry.expression_ref(arg_id);
            assert_eq!(
                component_identifier_names(registry, callee_id),
                vec![IdentifierName::new("x".to_string())],
                "Unexpected param name"
            );
            assert!(
                matches!(arg, ExpressionRef::Name(name) if component_identifier_names(registry, name.id) == vec![IdentifierName::new("a".to_string())]),
                "Unexpected arg: {:#?}",
                arg
            );
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

#[test]
fn rec_fun_non_substruct() {
    let src =
        include_str!("../../sample_code/should_fail/illegal_recursion/rec_fun_non_substruct.ph");
    expect_recursion_error(src, |registry, err| match err {
        IllegalFunRecursionError::NonSubstructPassedToDecreasingParam { callee_id, arg_id } => {
            let arg = &registry.expression_ref(arg_id);
            assert_eq!(
                component_identifier_names(registry, callee_id),
                vec![IdentifierName::new("x".to_string())],
                "Unexpected param name"
            );
            assert!(
                matches!(arg, ExpressionRef::Name(name) if component_identifier_names(registry, name.id) == vec![IdentifierName::new("b".to_string())]),
                "Unexpected arg: {:#?}",
                arg
            );
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

#[test]
fn rec_fun_non_ident() {
    let src = include_str!("../../sample_code/should_fail/illegal_recursion/rec_fun_non_ident.ph");
    expect_recursion_error(src, |registry, err| match err {
        IllegalFunRecursionError::NonSubstructPassedToDecreasingParam { callee_id, arg_id } => {
            let arg = &registry.expression_ref(arg_id);
            assert_eq!(
                component_identifier_names(registry, callee_id),
                vec![IdentifierName::new("x".to_string())],
                "Unexpected param name"
            );
            assert!(
                matches!(arg, ExpressionRef::Name(name) if component_identifier_names(registry, name.id) == vec![IdentifierName::new("Nat".to_string()), IdentifierName::new("O".to_string())]),
                "Unexpected arg: {:#?}",
                arg
            );
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

#[test]
fn rec_fun_no_decreasing_param() {
    let src = include_str!(
        "../../sample_code/should_fail/illegal_recursion/rec_fun_no_decreasing_param.ph"
    );
    expect_recursion_error(src, |registry, err| match err {
        IllegalFunRecursionError::RecursivelyCalledFunctionWithoutDecreasingParam { callee_id } => {
            assert_eq!(
                component_identifier_names(registry, callee_id),
                vec![IdentifierName::new("x".to_string())],
                "Unexpected param name"
            );
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

#[test]
fn rec_fun_right_index_wrong_label() {
    let src = include_str!(
        "../../sample_code/should_fail/illegal_recursion/rec_fun_right_index_wrong_label.ph"
    );
    expect_recursion_error(src, |registry, err| match err {
        IllegalFunRecursionError::NonSubstructPassedToDecreasingParam { arg_id, callee_id } => {
            let arg = &registry.expression_ref(arg_id);
            assert_eq!(
                component_identifier_names(registry, callee_id),
                vec![IdentifierName::new("f".to_string())],
                "Unexpected param name"
            );
            assert!(
                matches!(arg, ExpressionRef::Name(name) if component_identifier_names(registry, name.id) == vec![IdentifierName::new("a".to_string())]),
                "Unexpected arg: {:#?}",
                arg
            );
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}
