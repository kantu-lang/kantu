use super::*;

fn expect_success_with_warnings(
    src: &str,
    expected_warnings: &[TypeCheckWarningSummary],
) -> Vec<TypeCheckWarning> {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let file = parse_file(tokens, file_id).expect("Parsing failed");
    let file = simplify_file(file).expect("AST Simplification failed");
    let file = bind_files(vec![file])
        .expect("Binding failed")
        .into_iter()
        .next()
        .unwrap();
    let mut registry = NodeRegistry::empty();
    let file_id = lighten_file(&mut registry, file);
    let file = registry.get(file_id);
    let file_id = validate_variant_return_types_in_file(&registry, file)
        .expect("Variant return type validation failed");
    let file_id = validate_fun_recursion_in_file(&mut registry, file_id)
        .expect("Fun recursion validation failed");
    let file_id = validate_type_positivity_in_file(&mut registry, file_id)
        .expect("Type positivity validation failed");
    let warnings = type_check_files(&mut registry, &[file_id]).expect("Type checking failed");
    assert_expectations_match_actual_warnings(&registry, expected_warnings, &warnings);
    let _js_ast =
        JavaScript::generate_code(&registry, &[file_id.raw()]).expect("Code generation failed");
    warnings
}

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
    assert_eq!(7, warnings.len());
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
