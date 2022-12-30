use super::*;

#[test]
fn todo_expressions() {
    use TypeCheckWarningSummary::*;
    let src = include_str!(
        "../../../sample_code/should_succeed/single_file/should_succeed_with_warnings/todo_expressions.ph"
    );
    let expected_warnings = vec![TodoExpressionWarning];
    let actual_warnings = expect_success_with_warnings(src, &expected_warnings);
    assert_eq!(5, actual_warnings.len());
}
