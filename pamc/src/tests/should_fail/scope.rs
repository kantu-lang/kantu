use super::*;

fn expect_name_not_found_error(src: &str, expected_unfindable_name: &str) {
    expect_bind_error(src, |err, _registry| match err {
        BindError::NameNotFound(err) => {
            assert_eq!(
                err.name,
                standard_ident_name(expected_unfindable_name),
                "Unexpected param name"
            );
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

fn expect_invalid_dot_rhs_error(src: &str, expected_unfindable_name: &str) {
    expect_bind_error(src, |err, registry| match err {
        BindError::InvalidDotExpressionRhs(rhs_id) => {
            let invalid_rhs = registry.identifier(rhs_id);
            assert_eq!(
                invalid_rhs.name,
                standard_ident_name(expected_unfindable_name),
                "Unexpected param name"
            );
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

#[test]
fn reference_let_in_body() {
    let src = include_str!("../sample_code/should_fail/scope/ref_let_in_body.ph");
    expect_name_not_found_error(src, "a");
}

#[test]
fn reference_type_in_param() {
    let src = include_str!("../sample_code/should_fail/scope/ref_type_in_param.ph");
    expect_name_not_found_error(src, "U");
}

#[test]
fn reference_fun_in_param() {
    let src = include_str!("../sample_code/should_fail/scope/ref_fun_in_param.ph");
    expect_name_not_found_error(src, "g");
}

#[test]
fn reference_fun_in_return_type() {
    let src = include_str!("../sample_code/should_fail/scope/ref_fun_in_return_type.ph");
    expect_name_not_found_error(src, "g");
}

fn expect_bind_error(src: &str, panicker: impl Fn(BindError, &NodeRegistry)) {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let file = parse_file(tokens, file_id).expect("Parsing failed");
    let mut registry = NodeRegistry::empty();
    let file_id = register_file(&mut registry, file);
    let err = bind_symbols_to_identifiers(&registry, vec![file_id])
        .expect_err("Binding unexpectedly succeeded");
    panicker(err, &registry);
}

#[test]
fn reference_variant_in_prev_variant() {
    let src = include_str!("../sample_code/should_fail/scope/ref_variant_in_prev_variant.ph");
    expect_invalid_dot_rhs_error(src, "C");
}

#[test]
fn reference_variant_in_variant_return_type() {
    let src =
        include_str!("../sample_code/should_fail/scope/ref_variant_in_variant_return_type.ph");
    expect_invalid_dot_rhs_error(src, "B");
}

#[test]
fn reference_variant_in_variant_param_type() {
    let src = include_str!("../sample_code/should_fail/scope/ref_variant_in_variant_param_type.ph");
    expect_invalid_dot_rhs_error(src, "D");
}

#[test]
fn reference_unbindable_dot_lhs() {
    let src = include_str!("../sample_code/should_fail/scope/unbindable_dot_lhs.ph");
    expect_bind_error(src, |err, registry| match err {
        BindError::UnbindableDotExpressionLhs(lhs_id) => {
            let invalid_lhs = &registry.wrapped_expression(lhs_id).expression;
            assert!(
                matches!(invalid_lhs, Expression::Match(_)),
                "Unexpected lhs {:?}",
                invalid_lhs
            );
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}
