use super::*;

#[test]
fn type_assertion_goal_lhs() {
    use TypeCheckWarningSummary::*;
    let src =
        include_str!("../../sample_code/should_succeed/should_succeed_with_warnings/check/type_assertion_goal_lhs.ph");
    let expected_warnings = vec![
        TypeAssertionGoalLhs {
            assertion_src: "goal: Nat".to_string(),
        },
        TypeAssertionGoalLhs {
            assertion_src: "goal: ?".to_string(),
        },
    ];
    expect_success_with_warnings(src, &expected_warnings);
}

#[test]
fn type_of_type0() {
    use TypeCheckWarningSummary::*;
    let src = include_str!(
        "../../sample_code/should_succeed/should_succeed_with_warnings/check/type_of_type0.ph"
    );
    let expected_warnings = vec![TypeAssertionLhsTypeIsType1 {
        assertion_src: "Type: ?".to_string(),
    }];
    expect_success_with_warnings(src, &expected_warnings);
}

#[test]
fn type_assertion_type_check_failure() {
    use TypeCheckWarningSummary::*;
    let src = include_str!(
        "../../sample_code/should_succeed/should_succeed_with_warnings/check/type_assertion_type_check_failure.ph"
    );
    let expected_warnings = vec![
        TypeAssertionCompareeTypeCheckFailure {
            reason: TypeCheckFailureReasonSummary::BindError,
        },
        TypeAssertionCompareeTypeCheckFailure {
            reason: TypeCheckFailureReasonSummary::IllegalRecursionError,
        },
        TypeAssertionCompareeTypeCheckFailure {
            reason: TypeCheckFailureReasonSummary::TypeCheckError,
        },
    ];
    let warnings = expect_success_with_warnings(src, &expected_warnings);
    assert_eq!(8, warnings.len());
}

#[test]
fn mismatched_types() {
    use TypeCheckWarningSummary::*;
    let src = include_str!(
        "../../sample_code/should_succeed/should_succeed_with_warnings/check/mismatched_types.ph"
    );
    let expected_warnings = vec![
        TypeAssertionTypeMismatch {
            original_left_src: "m".to_string(),
            rewritten_left_type_src: "Nat".to_string(),
            original_right_src: "Unit".to_string(),
            rewritten_right_src: "Unit".to_string(),
        },
        TypeAssertionTypeQuestionMark {
            original_left_src: "m".to_string(),
            rewritten_left_type_src: "Nat".to_string(),
        },
    ];
    expect_success_with_warnings(src, &expected_warnings);
}

#[test]
fn nf_no_goal_exists() {
    use TypeCheckWarningSummary::*;
    let src = include_str!(
        "../../sample_code/should_succeed/should_succeed_with_warnings/check/nf_no_goal_exists.ph"
    );
    let expected_warnings = vec![
        NormalFormAssertionNoGoalExists {
            assertion_src: "goal = Nat".to_string(),
        },
        NormalFormAssertionNoGoalExists {
            assertion_src: "goal = ?".to_string(),
        },
    ];
    expect_success_with_warnings(src, &expected_warnings);
}

#[test]
fn nf_non_goal_assertion_type_check_failure() {
    use TypeCheckWarningSummary::*;
    let src = include_str!(
        "../../sample_code/should_succeed/should_succeed_with_warnings/check/nf_non_goal_assertion_type_check_failure.ph"
    );
    let expected_warnings = vec![
        NormalFormAssertionCompareeTypeCheckFailure {
            reason: TypeCheckFailureReasonSummary::BindError,
        },
        NormalFormAssertionCompareeTypeCheckFailure {
            reason: TypeCheckFailureReasonSummary::IllegalRecursionError,
        },
        NormalFormAssertionCompareeTypeCheckFailure {
            reason: TypeCheckFailureReasonSummary::TypeCheckError,
        },
    ];
    let warnings = expect_success_with_warnings(src, &expected_warnings);
    assert_eq!(7, warnings.len());
}

#[test]
fn nf_goal_assertion_type_check_failure() {
    use TypeCheckWarningSummary::*;
    let src = include_str!(
        "../../sample_code/should_succeed/should_succeed_with_warnings/check/nf_goal_assertion_type_check_failure.ph"
    );
    let expected_warnings = vec![
        NormalFormAssertionCompareeTypeCheckFailure {
            reason: TypeCheckFailureReasonSummary::BindError,
        },
        NormalFormAssertionCompareeTypeCheckFailure {
            reason: TypeCheckFailureReasonSummary::IllegalRecursionError,
        },
        NormalFormAssertionCompareeTypeCheckFailure {
            reason: TypeCheckFailureReasonSummary::TypeCheckError,
        },
    ];
    let warnings = expect_success_with_warnings(src, &expected_warnings);
    assert_eq!(5, warnings.len());
}

#[test]
fn mismatched_nf_comparees() {
    use TypeCheckWarningSummary::*;
    let src =
        include_str!("../../sample_code/should_succeed/should_succeed_with_warnings/check/mismatched_nf_comparees.ph");
    let expected_warnings = vec![
        NormalFormAssertionCompareeMismatch {
            original_left_src: "goal".to_string(),
            rewritten_left_src: "Nat".to_string(),
            original_right_src: "Eq(Nat, Nat.O, Nat.O,)".to_string(),
            rewritten_right_src: "Eq(Nat, Nat.O, Nat.O,)".to_string(),
        },
        NormalFormAssertionCompareeMismatch {
            original_left_src: "n".to_string(),
            rewritten_left_src: "Nat.S(n',)".to_string(),
            original_right_src: "Nat.S(n,)".to_string(),
            rewritten_right_src: "Nat.S(Nat.S(n',),)".to_string(),
        },
        NormalFormAssertionCompareeQuestionMark {
            original_left_src: "m".to_string(),
            rewritten_left_src: "Nat.O".to_string(),
        },
        NormalFormAssertionCompareeQuestionMark {
            original_left_src: "m".to_string(),
            rewritten_left_src: "Nat.S(m',)".to_string(),
        },
    ];
    expect_success_with_warnings(src, &expected_warnings);
}
