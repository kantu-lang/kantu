use crate::processing::test_utils::expand_lightened::expand_check_assertion;

use super::*;

fn expect_success_with_one_or_more_warnings(
    src: &str,
    expected_warnings: &[TypeCheckWarningExpectation],
) {
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
}

#[derive(Clone, Debug)]
enum TypeCheckWarningExpectation<'a> {
    TypeAssertionGoalLhs {
        assertion_src: &'a str,
    },
    TypeAssertionTypeCheckFailure {
        reason: TypeCheckFailureReasonExpectation,
    },
    TypeAssertionTypeMismatch {
        original_left_src: &'a str,
        rewritten_left_src: &'a str,
        original_right_src: &'a str,
        rewritten_right_src: &'a str,
    },
    TypeAssertionTypeQuestionMark {
        original_left_src: &'a str,
        rewritten_left_src: &'a str,
    },

    NormalFormNoGoalExists {
        assertion_src: &'a str,
    },
    NormalFormCompareeTypeCheckFailure {
        reason: TypeCheckFailureReasonExpectation,
    },
    NormalFormCompareeMismatch {
        original_left_src: &'a str,
        rewritten_left_src: &'a str,
        original_right_src: &'a str,
        rewritten_right_src: &'a str,
    },
    NormalFormCompareeQuestionMark {
        original_left_src: &'a str,
        rewritten_left_src: &'a str,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TypeCheckFailureReasonExpectation {
    BindError,
    IllegalRecursionError,
    TypeError,
}

fn assert_expectations_match_actual_warnings(
    registry: &NodeRegistry,
    expected_warnings: &[TypeCheckWarningExpectation],
    actual_warnings: &[TypeCheckWarning],
) {
    assert_all_emitted_warnings_were_expected(registry, expected_warnings, actual_warnings);
    assert_all_expected_warnings_were_emitted(registry, expected_warnings, actual_warnings);
}

fn assert_all_emitted_warnings_were_expected(
    registry: &NodeRegistry,
    expected_warnings: &[TypeCheckWarningExpectation],
    actual_warnings: &[TypeCheckWarning],
) {
    for actual in actual_warnings {
        let mut mismatch_reasons = vec![];
        let mut was_found = false;
        for expected in expected_warnings {
            match try_match_warnings(registry, expected, actual) {
                WarningMatchResult::Ok => {
                    was_found = true;
                    break;
                }
                WarningMatchResult::WrongCategory => (),
                WarningMatchResult::Mismatch(reason) => mismatch_reasons.push(reason),
            }
        }
        if !was_found {
            panic!(
                "Unexpected warning: {:?}.\nMismatch reasons: {}",
                actual,
                format_reasons(&mismatch_reasons),
            );
        }
    }
}

fn assert_all_expected_warnings_were_emitted(
    registry: &NodeRegistry,
    expected_warnings: &[TypeCheckWarningExpectation],
    actual_warnings: &[TypeCheckWarning],
) {
    for expected in expected_warnings {
        let mut mismatch_reasons = vec![];
        let mut was_found = false;
        for actual in actual_warnings {
            match try_match_warnings(registry, expected, actual) {
                WarningMatchResult::Ok => {
                    was_found = true;
                    break;
                }
                WarningMatchResult::WrongCategory => (),
                WarningMatchResult::Mismatch(reason) => mismatch_reasons.push(reason),
            }
        }
        if !was_found {
            panic!(
                "Expected warning, but it was never emitted: {:?}.\nMismatch reasons: {}",
                expected,
                format_reasons(&mismatch_reasons),
            );
        }
    }
}

fn format_reasons(mismatch_reasons: &[String]) -> String {
    if mismatch_reasons.is_empty() {
        "[]".to_string()
    } else {
        format!(
            "[\n{} <<<END_REASON>>>\n]",
            mismatch_reasons.join(" <<<END_REASON>>>\n")
        )
    }
}

#[derive(Clone, Debug)]
enum WarningMatchResult {
    Ok,
    WrongCategory,
    Mismatch(String),
}

fn try_match_warnings(
    registry: &NodeRegistry,
    expected: &TypeCheckWarningExpectation,
    actual: &TypeCheckWarning,
) -> WarningMatchResult {
    use crate::processing::test_utils::{expand_lightened::*, format::*};

    const FORMAT_OPTIONS: FormatOptions = FormatOptions {
        ident_size_in_spaces: 4,
        print_db_indices: false,
        print_fun_body_status: false,
    };

    match (expected, actual) {
        (
            TypeCheckWarningExpectation::TypeAssertionGoalLhs {
                assertion_src: expected_assertion_src,
            },
            TypeCheckWarning::TypeAssertion(TypeAssertionWarning::GoalLhs(assertion_id)),
        ) => {
            let actual_assertion_src = format_check_assertion(
                &expand_check_assertion(registry, *assertion_id),
                0,
                &FORMAT_OPTIONS,
            );
            if let Err(err) =
                try_assert_eq_up_to_white_space(expected_assertion_src, &actual_assertion_src)
            {
                return WarningMatchResult::Mismatch(format!("Check assertion differs:\n{}", err));
            }
            WarningMatchResult::Ok
        }

        _ => WarningMatchResult::WrongCategory,
    }
}

fn try_assert_eq_up_to_white_space(left: &str, right: &str) -> Result<(), String> {
    let mut left_non_whitespace = left.chars().enumerate().filter(|(_, c)| !c.is_whitespace());
    let left_non_whitespace_len = left_non_whitespace.clone().count();
    let mut right_non_whitespace = right
        .chars()
        .enumerate()
        .filter(|(_, c)| !c.is_whitespace());
    let right_non_whitespace_len = right_non_whitespace.clone().count();

    loop {
        let left_char = left_non_whitespace.next();
        let right_char = right_non_whitespace.next();

        match (left_char, right_char) {
            (Some((left_original_index, left_char)), Some((right_original_index, right_char))) => {
                if left_char != right_char {
                    return Err(format!(
                        "Strings differ (after removing whitespace): left_index = {}; right_index = {};\nleft = {:?};\nright = {:?};\nleft_remaining = {:?};\nright_remaining = {:?}",
                        left_original_index, right_original_index, left, right, &left[left_original_index..], &right[right_original_index..]
                    ));
                }
            }
            (None, None) => {
                break Ok(());
            }
            (Some((left_original_index, _)), None) => {
                return Err(format!(
                    "Strings differ in length after removing whitespace: left_len = {}; right_len = {};\nleft = {:?};\nright = {:?};\nleft_remaining = {:?};\nright_remaining = {:?}",
                    left_non_whitespace_len,
                    right_non_whitespace_len,
                    left,
                    right,
                    &left[left_original_index..],
                    "",
                ));
            }
            (None, Some((right_original_index, _))) => {
                return Err(format!(
                    "Strings differ in length after removing whitespace: left_len = {}; right_len = {};\nleft = {:?};\nright = {:?};\nleft_remaining = {:?};\nright_remaining = {:?}",
                    left_non_whitespace_len,
                    right_non_whitespace_len,
                    left,
                    right,
                    "",
                    &right[right_original_index..],
                ));
            }
        }
    }
}

#[test]
fn type_assertion_goal_lhs() {
    use TypeCheckWarningExpectation::*;
    let src =
        include_str!("../../sample_code/should_succeed/should_succeed_with_warnings/check/type_assertion_goal_lhs.ph");
    let expected_warnings = vec![
        TypeAssertionGoalLhs {
            assertion_src: "goal: Nat",
        },
        TypeAssertionGoalLhs {
            assertion_src: "goal: ?",
        },
    ];
    expect_success_with_one_or_more_warnings(src, &expected_warnings);
}

// TODO: Delete
// fn check() {
//     use TypeCheckWarningExpectation::*;
//     let src =
//         include_str!("../../sample_code/should_succeed/should_succeed_with_warnings/check/type_assertion_goal_lhs.ph");
//     let expected_warnings = vec![
//         // type_assertion_goal_lhs
//         TypeAssertionGoalLhs {
//             assertion_src: "goal: Nat",
//         },
//         TypeAssertionGoalLhs {
//             assertion_src: "goal: ?",
//         },
//         // mismatched_comparees
//         NormalFormCompareeMismatch {
//             original_left_src: "goal",
//             rewritten_left_src: "Nat",
//             original_right_src: "Eq(Nat, Nat.O, Nat.O,)",
//             rewritten_right_src: "Eq(Nat, Nat.O, Nat.O,)",
//         },
//         NormalFormCompareeMismatch {
//             original_left_src: "n",
//             // TODO: Check
//             rewritten_left_src: "Nat.S(n')",
//             original_right_src: "Nat.S(n)",
//             // TODO: Check
//             rewritten_right_src: "Nat.S(Nat.S(n'))",
//         },
//         // question_mark_rhs
//         NormalFormCompareeQuestionMark {
//             original_left_src: "m",
//             // TODO: Check
//             rewritten_left_src: "Nat.O",
//         },
//         NormalFormCompareeQuestionMark {
//             original_left_src: "m",
//             // TODO: Check
//             rewritten_left_src: "Nat.S(m')",
//         },
//         // goal_checkee
//         NormalFormCompareeQuestionMark {
//             original_left_src: "goal",
//             rewritten_left_src: "Nat",
//         },
//         // symbolically_invalid_annotations1
//         // TODO
//     ];
//     expect_success_with_one_or_more_warnings(src, &expected_warnings);
// }
