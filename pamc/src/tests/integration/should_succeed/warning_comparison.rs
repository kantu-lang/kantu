use super::*;

use crate::processing::test_utils::{expand_lightened::*, format::*};

const FORMAT_OPTIONS: FormatOptions = FormatOptions {
    ident_size_in_spaces: 4,
    print_db_indices: false,
    print_fun_body_status: false,
};

#[derive(Clone, Debug)]
pub enum TypeCheckWarningExpectation<'a> {
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

    NormalFormAssertionNoGoalExists {
        assertion_src: &'a str,
    },
    NormalFormAssertionTypeCheckFailure {
        reason: TypeCheckFailureReasonExpectation,
    },
    NormalFormAssertionCompareeMismatch {
        original_left_src: &'a str,
        rewritten_left_src: &'a str,
        original_right_src: &'a str,
        rewritten_right_src: &'a str,
    },
    NormalFormAssertionCompareeQuestionMark {
        original_left_src: &'a str,
        rewritten_left_src: &'a str,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypeCheckFailureReasonExpectation {
    BindError,
    IllegalRecursionError,
    TypeCheckError,
}

impl TypeCheckFailureReasonExpectation {
    pub fn new(reason: &TypeCheckFailureReason) -> Self {
        match reason {
            TypeCheckFailureReason::CannotTypeCheck(InvalidExpressionId::SymbolicallyInvalid(
                _,
            )) => Self::BindError,

            TypeCheckFailureReason::CannotTypeCheck(InvalidExpressionId::IllegalFunRecursion(
                _,
            )) => Self::IllegalRecursionError,

            TypeCheckFailureReason::TypeCheckError(_, _) => Self::TypeCheckError,
        }
    }
}

pub fn assert_expectations_match_actual_warnings(
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
                Ok(()) => {
                    was_found = true;
                    break;
                }
                Err(WarningMismatch::WrongCategory) => (),
                Err(WarningMismatch::SameCategoryWrongParams(reason)) => {
                    mismatch_reasons.push(reason)
                }
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
                Ok(()) => {
                    was_found = true;
                    break;
                }
                Err(WarningMismatch::WrongCategory) => (),
                Err(WarningMismatch::SameCategoryWrongParams(reason)) => {
                    mismatch_reasons.push(reason)
                }
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
pub enum WarningMismatch {
    WrongCategory,
    SameCategoryWrongParams(String),
}

fn try_match_warnings(
    registry: &NodeRegistry,
    expected: &TypeCheckWarningExpectation,
    actual: &TypeCheckWarning,
) -> Result<(), WarningMismatch> {
    match (expected, actual) {
        (
            TypeCheckWarningExpectation::TypeAssertionGoalLhs {
                assertion_src: expected_assertion_src,
            },
            TypeCheckWarning::TypeAssertion(TypeAssertionWarning::GoalLhs(assertion_id)),
        ) => try_match_assertions(registry, *assertion_id, expected_assertion_src),

        (
            TypeCheckWarningExpectation::TypeAssertionTypeCheckFailure {
                reason: expected_reason,
            },
            TypeCheckWarning::TypeAssertion(TypeAssertionWarning::CompareeTypeCheckFailure(
                actual_reason,
            )),
        ) => {
            let actual_reason = TypeCheckFailureReasonExpectation::new(actual_reason);
            if *expected_reason != actual_reason {
                return Err(WarningMismatch::SameCategoryWrongParams(format!(
                    "Got different TypeAssertion type check failure reason than expected. Expected reason: {:?}, actual reason: {:?}",
                    expected_reason, actual_reason
                )));
            }
            Ok(())
        }

        (
            TypeCheckWarningExpectation::TypeAssertionTypeMismatch {
                original_left_src: expected_original_left_src,
                rewritten_left_src: expected_rewritten_left_src,
                original_right_src: expected_original_right_src,
                rewritten_right_src: expected_rewritten_right_src,
            },
            TypeCheckWarning::TypeAssertion(TypeAssertionWarning::TypesDoNotMatch {
                left_id: actual_original_left_id,
                rewritten_left_type_id: actual_rewritten_left_type_id,
                original_and_rewritten_right_ids:
                    Ok((actual_original_right_id, actual_rewritten_right_type_id)),
            }),
        ) => {
            try_match_expressions(
                registry,
                "TypeAssertion original LHSs",
                *actual_original_left_id,
                expected_original_left_src,
            )?;
            try_match_expressions(
                registry,
                "TypeAssertion rewritten LHSs",
                actual_rewritten_left_type_id.raw(),
                expected_rewritten_left_src,
            )?;
            try_match_expressions(
                registry,
                "TypeAssertion original RHSs",
                *actual_original_right_id,
                expected_original_right_src,
            )?;
            try_match_expressions(
                registry,
                "TypeAssertion rewritten RHSs",
                actual_rewritten_right_type_id.raw(),
                expected_rewritten_right_src,
            )?;
            Ok(())
        }

        (
            TypeCheckWarningExpectation::TypeAssertionTypeQuestionMark {
                original_left_src: expected_original_left_src,
                rewritten_left_src: expected_rewritten_left_src,
            },
            TypeCheckWarning::TypeAssertion(TypeAssertionWarning::TypesDoNotMatch {
                left_id: actual_original_left_id,
                rewritten_left_type_id: actual_rewritten_left_type_id,
                original_and_rewritten_right_ids:
                    Err(crate::processing::type_check::RhsIsQuestionMark),
            }),
        ) => {
            try_match_expressions(
                registry,
                "TypeAssertion original LHSs",
                *actual_original_left_id,
                expected_original_left_src,
            )?;
            try_match_expressions(
                registry,
                "TypeAssertion rewritten LHSs",
                actual_rewritten_left_type_id.raw(),
                expected_rewritten_left_src,
            )?;
            Ok(())
        }

        (
            TypeCheckWarningExpectation::NormalFormAssertionNoGoalExists {
                assertion_src: expected_assertion_src,
            },
            TypeCheckWarning::NormalFormAssertion(NormalFormAssertionWarning::NoGoalExists(
                assertion_id,
            )),
        ) => try_match_assertions(registry, *assertion_id, expected_assertion_src),

        (
            TypeCheckWarningExpectation::NormalFormAssertionTypeCheckFailure {
                reason: expected_reason,
            },
            TypeCheckWarning::NormalFormAssertion(
                NormalFormAssertionWarning::CompareeTypeCheckFailure(actual_reason),
            ),
        ) => {
            let actual_reason = TypeCheckFailureReasonExpectation::new(actual_reason);
            if *expected_reason != actual_reason {
                return Err(WarningMismatch::SameCategoryWrongParams(format!(
                    "Got different TypeAssertion type check failure reason than expected. Expected reason: {:?}, actual reason: {:?}",
                    expected_reason, actual_reason
                )));
            }
            Ok(())
        }

        (
            TypeCheckWarningExpectation::NormalFormAssertionCompareeMismatch {
                original_left_src: expected_original_left_src,
                rewritten_left_src: expected_rewritten_left_src,
                original_right_src: expected_original_right_src,
                rewritten_right_src: expected_rewritten_right_src,
            },
            TypeCheckWarning::NormalFormAssertion(
                NormalFormAssertionWarning::CompareesDoNotMatch {
                    left_id: actual_original_left_id,
                    rewritten_left_id: actual_rewritten_left_id,
                    original_and_rewritten_right_ids:
                        Ok((actual_original_right_id, actual_rewritten_right_type_id)),
                },
            ),
        ) => {
            try_match_expressions_or_goal_kws(
                registry,
                "NormalFormAssertion original LHSs",
                *actual_original_left_id,
                expected_original_left_src,
            )?;
            try_match_expressions(
                registry,
                "NormalFormAssertion rewritten LHSs",
                actual_rewritten_left_id.raw(),
                expected_rewritten_left_src,
            )?;
            try_match_expressions(
                registry,
                "NormalFormAssertion original RHSs",
                *actual_original_right_id,
                expected_original_right_src,
            )?;
            try_match_expressions(
                registry,
                "NormalFormAssertion rewritten RHSs",
                actual_rewritten_right_type_id.raw(),
                expected_rewritten_right_src,
            )?;
            Ok(())
        }

        (
            TypeCheckWarningExpectation::NormalFormAssertionCompareeQuestionMark {
                original_left_src: expected_original_left_src,
                rewritten_left_src: expected_rewritten_left_src,
            },
            TypeCheckWarning::NormalFormAssertion(
                NormalFormAssertionWarning::CompareesDoNotMatch {
                    left_id: actual_original_left_id,
                    rewritten_left_id: actual_rewritten_left_id,
                    original_and_rewritten_right_ids:
                        Err(crate::processing::type_check::RhsIsQuestionMark),
                },
            ),
        ) => {
            try_match_expressions_or_goal_kws(
                registry,
                "NormalFormAssertion original LHSs",
                *actual_original_left_id,
                expected_original_left_src,
            )?;
            try_match_expressions(
                registry,
                "NormalFormAssertion rewritten LHSs",
                actual_rewritten_left_id.raw(),
                expected_rewritten_left_src,
            )?;
            Ok(())
        }

        _ => Err(WarningMismatch::WrongCategory),
    }
}

fn try_match_assertions(
    registry: &NodeRegistry,
    assertion_id: NodeId<CheckAssertion>,
    expected_assertion_src: &str,
) -> Result<(), WarningMismatch> {
    let actual_assertion_src = format_check_assertion(
        &expand_check_assertion(registry, assertion_id),
        0,
        &FORMAT_OPTIONS,
    );
    if let Err(err) = try_assert_eq_up_to_white_space(expected_assertion_src, &actual_assertion_src)
    {
        return Err(WarningMismatch::SameCategoryWrongParams(format!(
            "Check assertion differs:\n{}",
            err
        )));
    }
    Ok(())
}

fn try_match_expressions(
    registry: &NodeRegistry,
    component_debug_name_plural: &str,
    id: ExpressionId,
    expected_assertion_src: &str,
) -> Result<(), WarningMismatch> {
    let actual_assertion_src =
        format_expression(&expand_expression(registry, id), 0, &FORMAT_OPTIONS);
    if let Err(err) = try_assert_eq_up_to_white_space(expected_assertion_src, &actual_assertion_src)
    {
        return Err(WarningMismatch::SameCategoryWrongParams(format!(
            "{} differ:\n{}",
            component_debug_name_plural, err
        )));
    }
    Ok(())
}

fn try_match_expressions_or_goal_kws(
    registry: &NodeRegistry,
    component_debug_name_plural: &str,
    id: Result<ExpressionId, LhsIsGoalKw>,
    expected_assertion_src: &str,
) -> Result<(), WarningMismatch> {
    match id {
        Ok(id) => try_match_expressions(
            registry,
            component_debug_name_plural,
            id,
            expected_assertion_src,
        ),
        Err(_) => try_match_goal_kw(component_debug_name_plural, expected_assertion_src),
    }
}

fn try_match_goal_kw(
    component_debug_name_plural: &str,
    expected_assertion_src: &str,
) -> Result<(), WarningMismatch> {
    let actual_assertion_src = "goal".to_string();
    if let Err(err) = try_assert_eq_up_to_white_space(expected_assertion_src, &actual_assertion_src)
    {
        return Err(WarningMismatch::SameCategoryWrongParams(format!(
            "{} differ:\n{}",
            component_debug_name_plural, err
        )));
    }
    Ok(())
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
