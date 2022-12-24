use super::*;

use crate::processing::{
    test_utils::{expand_lightened::*, format::*},
    type_check,
};

const FORMAT_OPTIONS: FormatOptions = FormatOptions {
    ident_size_in_spaces: 4,
    print_db_indices: false,
    print_fun_body_status: false,
};

#[derive(Clone, Debug)]
pub enum TypeCheckWarningSummary {
    TypeAssertionGoalLhs {
        assertion_src: String,
    },
    TypeAssertionLhsTypeIsType1 {
        assertion_src: String,
    },
    TypeAssertionCompareeTypeCheckFailure {
        reason: TypeCheckFailureReasonSummary,
    },
    TypeAssertionTypeMismatch {
        original_left_src: String,
        rewritten_left_type_src: String,
        original_right_src: String,
        rewritten_right_src: String,
    },
    TypeAssertionTypeQuestionMark {
        original_left_src: String,
        rewritten_left_type_src: String,
    },

    NormalFormAssertionNoGoalExists {
        assertion_src: String,
    },
    NormalFormAssertionCompareeTypeCheckFailure {
        reason: TypeCheckFailureReasonSummary,
    },
    NormalFormAssertionCompareeMismatch {
        original_left_src: String,
        rewritten_left_src: String,
        original_right_src: String,
        rewritten_right_src: String,
    },
    NormalFormAssertionCompareeQuestionMark {
        original_left_src: String,
        rewritten_left_src: String,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeCheckFailureReasonSummary {
    BindError,
    IllegalRecursionError,
    TypeCheckError,
}

pub fn assert_expectations_match_actual_warnings(
    registry: &NodeRegistry,
    expected_warnings: &[TypeCheckWarningSummary],
    actual_warnings: &[TypeCheckWarning],
) {
    let expected_warnings = format_expected_warnings(registry, expected_warnings);
    let actual_warnings = format_actual_warnings(registry, actual_warnings);
    assert_all_emitted_warnings_were_expected(&expected_warnings, &actual_warnings);
    assert_all_expected_warnings_were_emitted(&expected_warnings, &actual_warnings);
}

fn format_expected_warnings(
    registry: &NodeRegistry,
    expected_warnings: &[TypeCheckWarningSummary],
) -> Vec<String> {
    expected_warnings
        .iter()
        .map(|warning| format_expected_warning(registry, warning))
        .collect()
}

fn format_actual_warnings(
    registry: &NodeRegistry,
    actual_warnings: &[TypeCheckWarning],
) -> Vec<String> {
    actual_warnings
        .iter()
        .map(|warning| format_actual_warning(registry, warning))
        .collect()
}

fn assert_all_emitted_warnings_were_expected<T: AsRef<str>, U: AsRef<str>>(
    expected_warnings: &[T],
    actual_warnings: &[U],
) {
    for actual in actual_warnings.iter().map(AsRef::<str>::as_ref) {
        let mut mismatch_errors = vec![];
        let mut was_found = false;
        for expected in expected_warnings.iter().map(AsRef::<str>::as_ref) {
            match try_assert_eq_up_to_white_space(expected, actual) {
                Ok(()) => {
                    was_found = true;
                    break;
                }
                Err(err) => {
                    mismatch_errors.push(err);
                }
            }
        }
        if !was_found {
            panic!(
                "Unexpected warning: {}.\nMismatch reasons: {}",
                actual,
                format_mismatch_errors(&mismatch_errors),
            );
        }
    }
}

fn assert_all_expected_warnings_were_emitted<T: AsRef<str>, U: AsRef<str>>(
    expected_warnings: &[T],
    actual_warnings: &[U],
) {
    for expected in expected_warnings.iter().map(AsRef::<str>::as_ref) {
        let mut mismatch_errors = vec![];
        let mut was_found = false;
        for actual in actual_warnings.iter().map(AsRef::<str>::as_ref) {
            match try_assert_eq_up_to_white_space(expected, actual) {
                Ok(()) => {
                    was_found = true;
                    break;
                }
                Err(err) => {
                    mismatch_errors.push(err);
                }
            }
        }
        if !was_found {
            panic!(
                "Expected warning, but it was never emitted.\nExpected warning: {}.\nMismatch reasons: {}",
                expected,
                format_mismatch_errors(&mismatch_errors),
            );
        }
    }
}

fn format_mismatch_errors<T: AsRef<str>>(errors: &[T]) -> String {
    if errors.is_empty() {
        "[]".to_string()
    } else {
        format!(
            "[\n{}]\n\n\n",
            errors
                .iter()
                .map(AsRef::<str>::as_ref)
                .enumerate()
                .map(|(i, err)| format!(
                    "    {index}: {indented_err}\n    <END_ERR({index})>\n\n",
                    index = i,
                    indented_err = indent(err, 4)
                ))
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

fn indent(s: &str, n: usize) -> String {
    let indent = " ".repeat(n);
    s.lines()
        .map(|line| format!("{}{}", indent, line))
        .collect::<Vec<_>>()
        .join("\n")
}

fn indent_second_line_onward(s: &str, n: usize) -> String {
    let indent = " ".repeat(n);
    s.lines()
        .enumerate()
        .map(|(i, line)| {
            if i == 0 {
                line.to_string()
            } else {
                format!("{}{}", indent, line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
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

fn format_expected_warning(_: &NodeRegistry, warning: &TypeCheckWarningSummary) -> String {
    match warning {
        TypeCheckWarningSummary::TypeAssertionGoalLhs { assertion_src } => {
            format!(
                "TypeAssertion::GoalLhs {{\n    assertion: {},\n}}",
                indent_second_line_onward(assertion_src, 8),
            )
        }

        TypeCheckWarningSummary::TypeAssertionLhsTypeIsType1 { assertion_src } => {
            format!(
                "TypeAssertion::LhsTypeIsType1 {{\n    assertion: {},\n}}",
                indent_second_line_onward(assertion_src, 8),
            )
        }
        TypeCheckWarningSummary::TypeAssertionCompareeTypeCheckFailure { reason } => {
            format!(
                "TypeAssertion::CompareeTypeCheckFailure {{\n    reason: {},\n}}",
                indent_second_line_onward(&format!("{:?}", reason), 8),
            )
        }
        TypeCheckWarningSummary::TypeAssertionTypeMismatch {
            original_left_src,
            rewritten_left_type_src,
            original_right_src,
            rewritten_right_src,
        } => {
            format!(
                "TypeAssertion::TypeMismatch {{\n    original_left: {},\n    rewritten_left_type: {},\n    original_right: {},\n    rewritten_right: {},\n}}",
                indent_second_line_onward(original_left_src, 8),
                indent_second_line_onward(rewritten_left_type_src, 8),
                indent_second_line_onward(original_right_src, 8),
                indent_second_line_onward(rewritten_right_src, 8),
            )
        }
        TypeCheckWarningSummary::TypeAssertionTypeQuestionMark {
            original_left_src,
            rewritten_left_type_src,
        } => {
            format!(
                "TypeAssertion::TypeQuestionMark {{\n    original_left: {},\n    rewritten_left_type: {},\n}}",
                indent_second_line_onward(original_left_src, 8),
                indent_second_line_onward(rewritten_left_type_src, 8),
            )
        }

        TypeCheckWarningSummary::NormalFormAssertionNoGoalExists { assertion_src } => {
            format!(
                "NormalFormAssertion::NoGoalExists {{\n    assertion: {},\n}}",
                indent_second_line_onward(assertion_src, 8),
            )
        }
        TypeCheckWarningSummary::NormalFormAssertionCompareeTypeCheckFailure { reason } => {
            format!(
                "NormalFormAssertion::CompareeTypeCheckFailure {{\n    reason: {},\n}}",
                indent_second_line_onward(&format!("{:?}", reason), 8),
            )
        }
        TypeCheckWarningSummary::NormalFormAssertionCompareeMismatch {
            original_left_src,
            rewritten_left_src,
            original_right_src,
            rewritten_right_src,
        } => {
            format!(
                "NormalFormAssertion::CompareeMismatch {{\n    original_left: {},\n    rewritten_left: {},\n    original_right: {},\n    rewritten_right: {},\n}}",
                indent_second_line_onward(original_left_src, 8),
                indent_second_line_onward(rewritten_left_src, 8),
                indent_second_line_onward(original_right_src, 8),
                indent_second_line_onward(rewritten_right_src, 8),
            )
        }
        TypeCheckWarningSummary::NormalFormAssertionCompareeQuestionMark {
            original_left_src,
            rewritten_left_src,
        } => {
            format!(
                "NormalFormAssertion::CompareeQuestionMark {{\n    original_left: {},\n    rewritten_left: {},\n}}",
                indent_second_line_onward(original_left_src, 8),
                indent_second_line_onward(rewritten_left_src, 8),
            )
        }
    }
}

fn format_actual_warning(registry: &NodeRegistry, warning: &TypeCheckWarning) -> String {
    let summary = summarize_warning(registry, warning);
    format_expected_warning(registry, &summary)
}

fn summarize_warning(
    registry: &NodeRegistry,
    warning: &TypeCheckWarning,
) -> TypeCheckWarningSummary {
    match warning {
        TypeCheckWarning::TypeAssertion(warning) => {
            summarize_type_assertion_warning(registry, warning)
        }
        TypeCheckWarning::NormalFormAssertion(warning) => {
            summarize_normal_form_assertion_warning(registry, warning)
        }
    }
}

fn summarize_type_assertion_warning(
    registry: &NodeRegistry,
    warning: &TypeAssertionWarning,
) -> TypeCheckWarningSummary {
    match warning {
        TypeAssertionWarning::GoalLhs(assertion_id) => {
            TypeCheckWarningSummary::TypeAssertionGoalLhs {
                assertion_src: format_check_assertion(
                    &expand_check_assertion(registry, *assertion_id),
                    0,
                    &FORMAT_OPTIONS,
                ),
            }
        }
        TypeAssertionWarning::LhsTypeIsType1(assertion_id) => {
            TypeCheckWarningSummary::TypeAssertionLhsTypeIsType1 {
                assertion_src: format_check_assertion(
                    &expand_check_assertion(registry, *assertion_id),
                    0,
                    &FORMAT_OPTIONS,
                ),
            }
        }
        TypeAssertionWarning::CompareeTypeCheckFailure(reason) => {
            TypeCheckWarningSummary::TypeAssertionCompareeTypeCheckFailure {
                reason: summarize_type_check_failure_reason(reason),
            }
        }
        TypeAssertionWarning::TypesDoNotMatch {
            left_id,
            rewritten_left_type_id,
            original_and_rewritten_right_ids: Ok((original_right_id, rewritten_right_id)),
        } => TypeCheckWarningSummary::TypeAssertionTypeMismatch {
            original_left_src: format_expr(registry, *left_id),
            rewritten_left_type_src: format_expr(registry, rewritten_left_type_id.raw()),
            original_right_src: format_expr(registry, *original_right_id),
            rewritten_right_src: format_expr(registry, rewritten_right_id.raw()),
        },
        TypeAssertionWarning::TypesDoNotMatch {
            left_id,
            rewritten_left_type_id,
            original_and_rewritten_right_ids: Err(type_check::RhsIsQuestionMark),
        } => TypeCheckWarningSummary::TypeAssertionTypeQuestionMark {
            original_left_src: format_expr(registry, *left_id),
            rewritten_left_type_src: format_expr(registry, rewritten_left_type_id.raw()),
        },
    }
}

fn summarize_normal_form_assertion_warning(
    registry: &NodeRegistry,
    warning: &NormalFormAssertionWarning,
) -> TypeCheckWarningSummary {
    match warning {
        NormalFormAssertionWarning::NoGoalExists(assertion_id) => {
            TypeCheckWarningSummary::NormalFormAssertionNoGoalExists {
                assertion_src: format_check_assertion(
                    &expand_check_assertion(registry, *assertion_id),
                    0,
                    &FORMAT_OPTIONS,
                ),
            }
        }
        NormalFormAssertionWarning::CompareeTypeCheckFailure(reason) => {
            TypeCheckWarningSummary::NormalFormAssertionCompareeTypeCheckFailure {
                reason: summarize_type_check_failure_reason(reason),
            }
        }
        NormalFormAssertionWarning::CompareesDoNotMatch {
            left_id,
            rewritten_left_id,
            original_and_rewritten_right_ids: Ok((original_right_id, rewritten_right_id)),
        } => TypeCheckWarningSummary::NormalFormAssertionCompareeMismatch {
            original_left_src: format_goal_kw_or_expr(registry, *left_id),
            rewritten_left_src: format_expr(registry, rewritten_left_id.raw()),
            original_right_src: format_expr(registry, *original_right_id),
            rewritten_right_src: format_expr(registry, rewritten_right_id.raw()),
        },
        NormalFormAssertionWarning::CompareesDoNotMatch {
            left_id,
            rewritten_left_id,
            original_and_rewritten_right_ids: Err(type_check::RhsIsQuestionMark),
        } => TypeCheckWarningSummary::NormalFormAssertionCompareeQuestionMark {
            original_left_src: format_goal_kw_or_expr(registry, *left_id),
            rewritten_left_src: format_expr(registry, rewritten_left_id.raw()),
        },
    }
}

fn format_goal_kw_or_expr(
    registry: &NodeRegistry,
    id: Result<ExpressionId, type_check::LhsIsGoalKw>,
) -> String {
    match id {
        Ok(id) => format_expr(registry, id),
        Err(type_check::LhsIsGoalKw) => "goal".to_string(),
    }
}

fn format_expr(registry: &NodeRegistry, id: ExpressionId) -> String {
    format_expression(&expand_expression(registry, id), 0, &FORMAT_OPTIONS)
}

pub fn summarize_type_check_failure_reason(
    reason: &TypeCheckFailureReason,
) -> TypeCheckFailureReasonSummary {
    match reason {
        TypeCheckFailureReason::CannotTypeCheck(InvalidExpressionId::SymbolicallyInvalid(_)) => {
            TypeCheckFailureReasonSummary::BindError
        }

        TypeCheckFailureReason::CannotTypeCheck(InvalidExpressionId::IllegalFunRecursion(_)) => {
            TypeCheckFailureReasonSummary::IllegalRecursionError
        }

        TypeCheckFailureReason::TypeCheckError(_, _) => {
            TypeCheckFailureReasonSummary::TypeCheckError
        }
    }
}
