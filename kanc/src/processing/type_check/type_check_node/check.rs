use super::*;

pub(in crate::processing::type_check) fn get_type_of_check_expression_dirty(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    check_id: &'a Check<'a>,
) -> Result<NormalFormId, Tainted<TypeCheckError>> {
    add_check_expression_warnings(state, coercion_target_id, check_id).map_err(Tainted::new)?;
    let check = state.registry.get(check_id).clone();
    get_type_of_expression_dirty(state, coercion_target_id, check.output_id)
}

fn add_check_expression_warnings(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    check_id: &'a Check<'a>,
) -> Result<(), TypeCheckError> {
    let warnings = get_check_expression_warnings(state, coercion_target_id, check_id);
    state.warnings.extend(warnings);
    Ok(())
}

fn get_check_expression_warnings(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    check_id: &'a Check<'a>,
) -> Vec<TypeCheckWarning> {
    let assertion_ids = {
        let check = state.registry.get(check_id);
        state.registry.get_list(check.assertion_list_id).to_vec()
    };
    assertion_ids
        .into_iter()
        .map(|assertion_id| get_check_assertion_warnings(state, coercion_target_id, assertion_id))
        .flatten()
        .collect()
}

fn get_check_assertion_warnings(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    assertion_id: &'a CheckAssertion<'a>,
) -> Vec<TypeCheckWarning> {
    let assertion = state.registry.get(assertion_id).clone();
    match assertion.kind {
        CheckAssertionKind::Type => {
            get_type_assertion_warnings(state, coercion_target_id, assertion)
                .into_iter()
                .map(TypeCheckWarning::TypeAssertion)
                .collect()
        }
        CheckAssertionKind::NormalForm => {
            get_normal_form_assertion_warnings(state, coercion_target_id, assertion)
        }
    }
}

fn get_type_assertion_warnings(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    assertion: CheckAssertion,
) -> Vec<TypeAssertionWarning> {
    match assertion.left_id {
        GoalKwOrPossiblyInvalidExpressionId::GoalKw { .. } => {
            vec![TypeAssertionWarning::GoalLhs(assertion.id)]
        }
        GoalKwOrPossiblyInvalidExpressionId::Expression(expression_id) => {
            get_non_goal_type_assertion_warnings(
                state,
                coercion_target_id,
                assertion,
                expression_id,
            )
        }
    }
}

fn get_non_goal_type_assertion_warnings(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    assertion: CheckAssertion,
    left_id: PossiblyInvalidExpressionId,
) -> Vec<TypeAssertionWarning> {
    let left_correctness =
        get_type_correctness_of_possibly_invalid_expression(state, coercion_target_id, left_id);
    let right_correctness = get_type_correctness_of_question_mark_or_possibly_invalid_expression(
        state,
        coercion_target_id,
        assertion.right_id,
    );

    match (left_correctness, right_correctness) {
        (
            Ok(CorrectlyTyped {
                expression_id: left_expression_id,
                type_id: left_type_id,
            }),
            QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness::Correct(
                right_expression_id,
                _right_type_id,
            ),
        ) => {
            let normalized_right_expression_id =
                evaluate_well_typed_expression(state, right_expression_id);
            match apply_substitutions_from_substitution_context(
                state,
                ((left_type_id,), (normalized_right_expression_id,)),
            ) {
                Ok(((rewritten_left_type_id,), (rewritten_right_id,))) => {
                    if get_rewritten_term_equality_status(
                        state,
                        rewritten_left_type_id,
                        rewritten_right_id,
                    )
                    .is_equal_or_exploded()
                    {
                        vec![]
                    } else if is_term_equal_to_type1(state, rewritten_left_type_id) {
                        vec![TypeAssertionWarning::LhsTypeIsType1(assertion.id)]
                    } else {
                        vec![TypeAssertionWarning::TypesDoNotMatch {
                            left_id: left_expression_id,
                            rewritten_left_type_id,
                            original_and_rewritten_right_ids: Ok((
                                right_expression_id,
                                rewritten_right_id,
                            )),
                        }]
                    }
                }
                Err(Exploded) => vec![],
            }
        }
        (
            Ok(CorrectlyTyped {
                expression_id: left_expression_id,
                type_id: left_type_id,
            }),
            QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness::QuestionMark,
        ) => {
            let (rewritten_left_type_id,) =
                match apply_substitutions_from_substitution_context(state, (left_type_id,)) {
                    Ok(rewritten) => rewritten,
                    Err(Exploded) => (left_type_id,),
                };
            if is_term_equal_to_type1(state, rewritten_left_type_id) {
                vec![TypeAssertionWarning::LhsTypeIsType1(assertion.id)]
            } else {
                vec![TypeAssertionWarning::TypesDoNotMatch {
                    left_id: left_expression_id,
                    rewritten_left_type_id,
                    original_and_rewritten_right_ids: Err(RhsIsQuestionMark),
                }]
            }
        }
        (other_left, other_right) => {
            let mut out = vec![];

            if let Err(reason) = other_left {
                out.push(TypeAssertionWarning::CompareeTypeCheckFailure(reason));
            }
            if let QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness::Incorrect(reason) =
                other_right
            {
                out.push(TypeAssertionWarning::CompareeTypeCheckFailure(reason));
            }

            out
        }
    }
}

fn get_normal_form_assertion_warnings(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    assertion: CheckAssertion,
) -> Vec<TypeCheckWarning> {
    let nonwrapped = match assertion.left_id {
        GoalKwOrPossiblyInvalidExpressionId::GoalKw { .. } => {
            get_goal_normal_form_assertion_warnings(state, coercion_target_id, assertion)
        }
        GoalKwOrPossiblyInvalidExpressionId::Expression(expression_id) => {
            get_non_goal_normal_form_assertion_warnings(
                state,
                coercion_target_id,
                assertion,
                expression_id,
            )
        }
    };
    nonwrapped
        .into_iter()
        .map(TypeCheckWarning::NormalFormAssertion)
        .collect()
}

// TODO: DRY

fn get_goal_normal_form_assertion_warnings(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    assertion: CheckAssertion,
) -> Vec<NormalFormAssertionWarning> {
    if let Some(coercion_target_id) = coercion_target_id {
        get_goal_normal_form_assertion_warnings_given_goal_exists(
            state,
            coercion_target_id,
            assertion,
        )
    } else {
        vec![NormalFormAssertionWarning::NoGoalExists(assertion.id)]
    }
}

fn get_goal_normal_form_assertion_warnings_given_goal_exists(
    state: &mut State,
    goal_id: NormalFormId,
    assertion: CheckAssertion,
) -> Vec<NormalFormAssertionWarning> {
    let coercion_target_id = Some(goal_id);
    // TODO: DRY (this is copied from `get_non_goal_normal_form_assertion_warnings`)
    let right_correctness = get_type_correctness_of_question_mark_or_possibly_invalid_expression(
        state,
        coercion_target_id,
        assertion.right_id,
    );

    match right_correctness {
        QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness::Correct(
            right_expression_id,
            _right_type_id,
        ) => {
            let normalized_right_expression_id =
                evaluate_well_typed_expression(state, right_expression_id);
            match apply_substitutions_from_substitution_context(
                state,
                ((goal_id,), (normalized_right_expression_id,)),
            ) {
                Ok(((rewritten_goal_id,), (rewritten_right_expression_id,))) => {
                    if get_rewritten_term_equality_status(
                        state,
                        rewritten_goal_id,
                        rewritten_right_expression_id,
                    )
                    .is_equal_or_exploded()
                    {
                        vec![]
                    } else {
                        vec![NormalFormAssertionWarning::CompareesDoNotMatch {
                            left_id: Err(LhsIsGoalKw),
                            rewritten_left_id: rewritten_goal_id,
                            original_and_rewritten_right_ids: Ok((
                                right_expression_id,
                                rewritten_right_expression_id,
                            )),
                        }]
                    }
                }
                Err(Exploded) => vec![],
            }
        }
        QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness::QuestionMark => {
            let (rewritten_goal_id,) =
                match apply_substitutions_from_substitution_context(state, (goal_id,)) {
                    Ok(rewritten) => rewritten,
                    Err(Exploded) => (goal_id,),
                };
            vec![NormalFormAssertionWarning::CompareesDoNotMatch {
                left_id: Err(LhsIsGoalKw),
                rewritten_left_id: rewritten_goal_id,
                original_and_rewritten_right_ids: Err(RhsIsQuestionMark),
            }]
        }
        other_right => {
            if let QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness::Incorrect(reason) =
                other_right
            {
                vec![NormalFormAssertionWarning::CompareeTypeCheckFailure(reason)]
            } else {
                vec![]
            }
        }
    }
}

fn get_non_goal_normal_form_assertion_warnings(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    assertion: CheckAssertion,
    left_id: PossiblyInvalidExpressionId,
) -> Vec<NormalFormAssertionWarning> {
    let left_correctness =
        get_type_correctness_of_possibly_invalid_expression(state, coercion_target_id, left_id);
    let right_correctness = get_type_correctness_of_question_mark_or_possibly_invalid_expression(
        state,
        coercion_target_id,
        assertion.right_id,
    );

    match (left_correctness, right_correctness) {
        (
            Ok(CorrectlyTyped {
                expression_id: left_expression_id,
                type_id: _,
            }),
            QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness::Correct(
                right_expression_id,
                _right_type_id,
            ),
        ) => {
            let normalized_left_expression_id =
                evaluate_well_typed_expression(state, left_expression_id);
            let normalized_right_expression_id =
                evaluate_well_typed_expression(state, right_expression_id);
            match apply_substitutions_from_substitution_context(
                state,
                (
                    (normalized_left_expression_id,),
                    (normalized_right_expression_id,),
                ),
            ) {
                Ok(((rewritten_left_expression_id,), (rewritten_right_expression_id,))) => {
                    if get_rewritten_term_equality_status(
                        state,
                        rewritten_left_expression_id,
                        rewritten_right_expression_id,
                    )
                    .is_equal_or_exploded()
                    {
                        vec![]
                    } else {
                        vec![NormalFormAssertionWarning::CompareesDoNotMatch {
                            left_id: Ok(left_expression_id),
                            rewritten_left_id: rewritten_left_expression_id,
                            original_and_rewritten_right_ids: Ok((
                                right_expression_id,
                                rewritten_right_expression_id,
                            )),
                        }]
                    }
                }
                Err(Exploded) => vec![],
            }
        }
        (
            Ok(CorrectlyTyped {
                expression_id: left_expression_id,
                type_id: _,
            }),
            QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness::QuestionMark,
        ) => {
            let normalized_left_expression_id =
                evaluate_well_typed_expression(state, left_expression_id);
            let (rewritten_left_type_id,) = match apply_substitutions_from_substitution_context(
                state,
                (normalized_left_expression_id,),
            ) {
                Ok(rewritten) => rewritten,
                Err(Exploded) => (normalized_left_expression_id,),
            };
            vec![NormalFormAssertionWarning::CompareesDoNotMatch {
                left_id: Ok(left_expression_id),
                rewritten_left_id: rewritten_left_type_id,
                original_and_rewritten_right_ids: Err(RhsIsQuestionMark),
            }]
        }
        (other_left, other_right) => {
            let mut out = vec![];

            if let Err(reason) = other_left {
                out.push(NormalFormAssertionWarning::CompareeTypeCheckFailure(reason));
            }
            if let QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness::Incorrect(reason) =
                other_right
            {
                out.push(NormalFormAssertionWarning::CompareeTypeCheckFailure(reason));
            }

            out
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct CorrectlyTyped {
    expression_id: ExpressionRef<'a>,
    type_id: NormalFormId,
}

fn get_type_correctness_of_possibly_invalid_expression(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    id: PossiblyInvalidExpressionId,
) -> Result<CorrectlyTyped, TypeCheckFailureReason> {
    match id {
        PossiblyInvalidExpressionId::Invalid(untypecheckable) => {
            Err(TypeCheckFailureReason::CannotTypeCheck(untypecheckable))
        }
        PossiblyInvalidExpressionId::Valid(expression_id) => {
            let type_id_or_err = get_type_of_expression(state, coercion_target_id, expression_id);
            match type_id_or_err {
                Ok(type_id) => Ok(CorrectlyTyped {
                    expression_id,
                    type_id,
                }),
                Err(type_check_err) => Err(TypeCheckFailureReason::TypeCheckError(
                    expression_id,
                    type_check_err,
                )),
            }
        }
    }
}

enum QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness {
    Correct(ExpressionRef<'a>, NormalFormId),
    Incorrect(TypeCheckFailureReason),
    QuestionMark,
}

fn get_type_correctness_of_question_mark_or_possibly_invalid_expression(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    id: QuestionMarkOrPossiblyInvalidExpressionId,
) -> QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness {
    match id {
        QuestionMarkOrPossiblyInvalidExpressionId::QuestionMark { .. } => {
            QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness::QuestionMark
        }
        QuestionMarkOrPossiblyInvalidExpressionId::Expression(possibly_typecheckable) => {
            match possibly_typecheckable {
                PossiblyInvalidExpressionId::Invalid(untypecheckable) => {
                    QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness::Incorrect(
                        TypeCheckFailureReason::CannotTypeCheck(untypecheckable),
                    )
                }
                PossiblyInvalidExpressionId::Valid(typecheckable) => {
                    match get_type_of_expression(state, coercion_target_id, typecheckable) {
                        Ok(type_id) => {
                            QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness::Correct(
                                typecheckable,
                                type_id,
                            )
                        }
                        Err(err) => {
                            QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness::Incorrect(
                                TypeCheckFailureReason::TypeCheckError(typecheckable, err),
                            )
                        }
                    }
                }
            }
        }
    }
}
