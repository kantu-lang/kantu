use super::*;

fn expect_wrong_number_of_arguments_error(
    src: &str,
    expected_illegal_call_src: &str,
    expected_expected_arity: usize,
) {
    expect_type_check_error(src, |registry, err| match err {
        TypeCheckError::WrongNumberOfArguments {
            call_id,
            expected: actual_expected_arity,
            ..
        } => {
            let actual_src = format_expression(
                &expand_expression(registry, ExpressionId::Call(call_id)),
                0,
                &FORMAT_OPTIONS_FOR_COMPARISON,
            );
            assert_eq_up_to_white_space(&actual_src, expected_illegal_call_src);
            assert_eq!(expected_expected_arity, actual_expected_arity);
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

#[test]
fn forall() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/wrong_number_of_arguments/fun.k"
    );
    expect_wrong_number_of_arguments_error(src, "bar_(U.u, U.u,)", 1);
}

#[test]
fn type_() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/wrong_number_of_arguments/type.k"
    );
    expect_wrong_number_of_arguments_error(src, "V(U.u, U.u,)", 1);
}

#[test]
fn variant() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/wrong_number_of_arguments/variant.k"
    );
    expect_wrong_number_of_arguments_error(src, "Bar.B(Empty, Empty,)", 1);
}
