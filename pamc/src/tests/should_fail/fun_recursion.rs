use super::*;

/// The job of `panicker` is to panic if the error is different than the expected
/// error.
fn expect_recursion_error(src: &str, panicker: impl Fn(&NodeRegistry, IllegalFunRecursionError)) {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let file = parse_file(tokens, file_id).expect("Parsing failed");
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
            let callee = registry.identifier(callee_id);
            let arg = &registry.wrapped_expression(arg_id).expression;
            assert_eq!(
                callee.name,
                standard_ident_name("x"),
                "Unexpected param name"
            );
            assert!(
                matches!(arg, Expression::Identifier(identifier) if identifier.name == standard_ident_name("a")),
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
            let callee = registry.identifier(callee_id);
            let arg = &registry.wrapped_expression(arg_id).expression;
            assert_eq!(
                callee.name,
                standard_ident_name("x"),
                "Unexpected param name"
            );
            assert!(
                matches!(arg, Expression::Identifier(identifier) if identifier.name == standard_ident_name("b")),
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
            let callee = registry.identifier(callee_id);
            let arg = &registry.wrapped_expression(arg_id).expression;
            assert_eq!(
                callee.name,
                standard_ident_name("x"),
                "Unexpected param name"
            );
            assert!(
                matches!(arg, Expression::Dot(dot) if registry.identifier(dot.right_id).name == standard_ident_name("O")),
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
            let callee = registry.identifier(callee_id);
            assert_eq!(
                callee.name,
                standard_ident_name("x"),
                "Unexpected param name"
            );
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}
