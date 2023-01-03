use super::*;

fn expect_cannot_infer_type_todo_expression_error(src: &str) {
    expect_type_check_error(src, |_registry, err| match err {
        TypeCheckError::CannotInferTypeOfTodoExpression(_) => {}
        _ => {
            panic!("Unexpected error: {:#?}", err)
        }
    });
}

#[test]
fn param_type() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/cannot_infer_todo_type/param_type.ph"
    );
    expect_cannot_infer_type_todo_expression_error(src);
}

#[test]
fn let_value() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/cannot_infer_todo_type/let_value.ph"
    );
    expect_cannot_infer_type_todo_expression_error(src);
}

#[test]
fn matchee() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/cannot_infer_todo_type/matchee.ph"
    );
    expect_cannot_infer_type_todo_expression_error(src);
}
