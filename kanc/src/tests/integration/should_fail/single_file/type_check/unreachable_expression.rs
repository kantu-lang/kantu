use super::*;

fn expect_unreachable_expression_error(src: &str, expected_expression_src: &str) {
    expect_type_check_error(src, |registry, err| match err {
        TypeCheckError::UnreachableExpression(expression_id) => {
            let actual_expression_src = format_expression(
                &expand_expression(registry, expression_id),
                0,
                &FORMAT_OPTIONS_FOR_COMPARISON,
            );
            assert_eq_up_to_white_space(&actual_expression_src, expected_expression_src);
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

#[test]
fn unreachable_expression() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/unreachable_expression.k"
    );
    expect_unreachable_expression_error(src, "U1.c");
}
