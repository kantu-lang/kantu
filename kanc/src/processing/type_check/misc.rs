use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NormalFormId(ExpressionRef<'a>);

impl NormalFormId {
    pub fn unchecked_new(expression: ExpressionRef<'a>) -> Self {
        Self(expression)
    }
}

impl NormalFormId {
    pub fn raw(self) -> ExpressionRef<'a> {
        self.0
    }
}

pub(super) fn type0_expression(state: &mut State) -> NormalFormId {
    let name_id = add_name_expression_and_overwrite_component_ids(
        state.registry,
        NonEmptyVec::singleton(Identifier {
            id: dummy_id(),
            name: IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase),
            span: None,
        }),
        state.context.type0_dbi(),
    );
    NormalFormId::unchecked_new(ExpressionRef<'a>::Name(name_id))
}

pub fn add_name_expression_and_overwrite_component_ids(
    registry: &mut NodeRegistry,
    components: NonEmptyVec<Identifier>,
    db_index: DbIndex,
) -> &'a NameExpression<'a> {
    let first_span = components.first().span;
    let last_span = components.last().span;
    let span = first_span
        .and_then(|first_span| last_span.map(|last_span| first_span.inclusive_merge(last_span)));
    let component_ids =
        components.into_mapped(|component| registry.add_and_overwrite_id(component));
    let component_list_id = registry.add_list(component_ids);
    registry.add_and_overwrite_id(NameExpression {
        id: dummy_id(),
        span,
        component_list_id,
        db_index,
    })
}

pub fn add_name_expression(
    registry: &mut NodeRegistry,
    component_ids: NonEmptyVec<&'a Identifier<'a>>,
    db_index: DbIndex,
) -> &'a NameExpression<'a> {
    let first_span = registry.get(*component_ids.first()).span;
    let last_span = registry.get(*component_ids.last()).span;
    let span = first_span
        .and_then(|first_span| last_span.map(|last_span| first_span.inclusive_merge(last_span)));
    let component_list_id = registry.add_list(component_ids);
    registry.add_and_overwrite_id(NameExpression {
        id: dummy_id(),
        span,
        component_list_id,
        db_index,
    })
}

pub fn dummy_id<T>() -> &'a T<'a> {
    NodeId::new(0)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PossiblyNullaryForall {
    pub id: &'a Self<'a>,
    pub span: Option<TextSpan>,
    pub param_list_id: Option<NonEmptyParamListId>,
    pub output_id: ExpressionRef<'a>,
}

impl PossiblyNullaryForall {
    pub fn into_id(self, registry: &mut NodeRegistry) -> ExpressionRef<'a> {
        if let Some(param_list_id) = self.param_list_id {
            let forall_id = registry.add_and_overwrite_id(Forall {
                id: dummy_id(),
                span: self.span,
                param_list_id,
                output_id: self.output_id,
            });
            ExpressionRef<'a>::Forall(forall_id)
        } else {
            self.output_id
        }
    }
}

pub(super) fn is_term_equal_to_type0_or_type1(state: &State, term: NormalFormId) -> bool {
    if let ExpressionRef<'a>::Name(name_id) = term.raw() {
        let name = state.registry.get(name_id);
        let i = name.db_index;
        i == state.context.type0_dbi() || i == state.context.type1_dbi()
    } else {
        false
    }
}

pub(super) fn is_term_equal_to_type1(state: &State, term: NormalFormId) -> bool {
    if let ExpressionRef<'a>::Name(name_id) = term.raw() {
        let name = state.registry.get(name_id);
        let i = name.db_index;
        i == state.context.type1_dbi()
    } else {
        false
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RewrittenTermEqualityStatus {
    Equal,
    NotEqual,
    Exploded,
}

impl RewrittenTermEqualityStatus {
    pub fn is_equal_or_exploded(self) -> bool {
        match self {
            RewrittenTermEqualityStatus::Equal => true,
            RewrittenTermEqualityStatus::NotEqual => false,
            RewrittenTermEqualityStatus::Exploded => true,
        }
    }
}

pub(super) fn get_rewritten_term_equality_status(
    state: &mut State,
    left: NormalFormId,
    right: NormalFormId,
) -> RewrittenTermEqualityStatus {
    let ((left,), (right,)) =
        match apply_substitutions_from_substitution_context(state, ((left,), (right,))) {
            Ok(x) => x,
            Err(self::Exploded) => return RewrittenTermEqualityStatus::Exploded,
        };

    if state
        .equality_checker
        .eq(left.raw(), right.raw(), state.registry)
    {
        RewrittenTermEqualityStatus::Equal
    } else {
        RewrittenTermEqualityStatus::NotEqual
    }
}

pub use std::convert::Infallible;

pub trait SafeUnwrap<T> {
    /// This is guaranteed to never panic.
    fn safe_unwrap(self) -> T;
}

impl<T> SafeUnwrap<T> for Result<T, Infallible> {
    fn safe_unwrap(self) -> T {
        match self {
            Ok(x) => x,
            Err(impossible) => match impossible {},
        }
    }
}

pub(super) fn normalize_optional_params_and_leave_params_in_context_dirty(
    state: &mut State,
    id: Option<NonEmptyParamListId>,
) -> Result<WithPushWarning<Option<NonEmptyParamListId>>, Tainted<TypeCheckError>> {
    if let Some(id) = id {
        Ok(normalize_params_and_leave_params_in_context_dirty(state, id)?.map(Some))
    } else {
        Ok(with_push_warning(None))
    }
}

pub(super) fn normalize_params_and_leave_params_in_context_dirty(
    state: &mut State,
    id: NonEmptyParamListId,
) -> Result<WithPushWarning<NonEmptyParamListId>, Tainted<TypeCheckError>> {
    Ok(with_push_warning(match id {
        NonEmptyParamListId::Unlabeled(id) => NonEmptyParamListId::Unlabeled(
            normalize_unlabeled_params_and_leave_params_in_context_dirty(state, id)??,
        ),
        NonEmptyParamListId::UniquelyLabeled(id) => NonEmptyParamListId::UniquelyLabeled(
            normalize_labeled_params_and_leave_params_in_context_dirty(state, id)??,
        ),
    }))
}

pub(super) fn normalize_unlabeled_params_and_leave_params_in_context_dirty(
    state: &mut State,
    param_list_id: NonEmptyListId<&'a UnlabeledParam<'a>>,
) -> Result<WithPushWarning<NonEmptyListId<&'a UnlabeledParam<'a>>>, Tainted<TypeCheckError>> {
    let param_ids = state.registry.get_list(param_list_id).to_non_empty_vec();
    let normalized_ids = param_ids.as_non_empty_slice().try_to_mapped(
        |&param_id| -> Result<&'a UnlabeledParam<'a>, Tainted<TypeCheckError>> {
            type_check_unlabeled_param_dirty(state, param_id)??;
            let type_id: ExpressionRef<'a> = state
                .context
                .get_type(DbIndex(0), state.registry)
                .downshift(1, state.registry)
                .raw();
            let old_param = state.registry.get(param_id);
            let normalized_param_with_dummy_id = UnlabeledParam {
                id: dummy_id(),
                span: None,
                is_dashed: old_param.is_dashed,
                name_id: old_param.name_id,
                type_id,
            };
            let normalized_id = state
                .registry
                .add_and_overwrite_id(normalized_param_with_dummy_id)
                .without_spans(state.registry);
            Ok(normalized_id)
        },
    )?;
    Ok(with_push_warning(state.registry.add_list(normalized_ids)))
}

pub(super) fn normalize_labeled_params_and_leave_params_in_context_dirty(
    state: &mut State,
    param_list_id: NonEmptyListId<&'a LabeledParam<'a>>,
) -> Result<WithPushWarning<NonEmptyListId<&'a LabeledParam<'a>>>, Tainted<TypeCheckError>> {
    let param_ids = state.registry.get_list(param_list_id).to_non_empty_vec();
    let normalized_ids = param_ids.as_non_empty_slice().try_to_mapped(
        |&param_id| -> Result<&'a LabeledParam<'a>, Tainted<TypeCheckError>> {
            type_check_labeled_param_dirty(state, param_id)??;
            let type_id: ExpressionRef<'a> = state
                .context
                .get_type(DbIndex(0), state.registry)
                .downshift(1, state.registry)
                .raw();
            let old_param = state.registry.get(param_id);
            let normalized_param_with_dummy_id = LabeledParam {
                id: dummy_id(),
                span: None,
                label_clause: old_param.label_clause,
                is_dashed: old_param.is_dashed,
                name_id: old_param.name_id,
                type_id,
            };
            let normalized_id = state
                .registry
                .add_and_overwrite_id(normalized_param_with_dummy_id)
                .without_spans(state.registry);
            Ok(normalized_id)
        },
    )?;
    Ok(with_push_warning(state.registry.add_list(normalized_ids)))
}

pub fn verify_variant_to_case_bijection(
    registry: &mut NodeRegistry,
    match_id: &'a Match<'a>,
    variant_name_list_id: Option<NonEmptyListId<&'a Identifier<'a>>>,
    case_list_id: Option<NonEmptyListId<&'a MatchCase<'a>>>,
) -> Result<(), TypeCheckError> {
    verify_there_are_no_duplicate_cases(registry, case_list_id)?;
    verify_that_every_variant_has_a_case(registry, match_id, variant_name_list_id, case_list_id)?;
    verify_that_every_case_has_a_variant(registry, variant_name_list_id, case_list_id)?;
    Ok(())
}

fn verify_there_are_no_duplicate_cases(
    registry: &NodeRegistry,
    case_list_id: Option<NonEmptyListId<&'a MatchCase<'a>>>,
) -> Result<(), TypeCheckError> {
    let mut visited_cases: Vec<&'a MatchCase<'a>> = Vec::with_capacity(case_list_id.len());

    let case_ids = registry.get_possibly_empty_list(case_list_id);

    for &case_id in case_ids {
        let case = registry.get(case_id);
        let case_variant_name = &registry.get(case.variant_name_id).name;

        if let Some(existing_case_id) = visited_cases
            .iter()
            .find(|&&existing_case_id| {
                let existing_case = registry.get(existing_case_id);
                let existing_case_variant_name = &registry.get(existing_case.variant_name_id).name;
                existing_case_variant_name == case_variant_name
            })
            .copied()
        {
            return Err(TypeCheckError::DuplicateMatchCase {
                existing_match_case_id: existing_case_id,
                new_match_case_id: case_id,
            });
        }

        visited_cases.push(case_id);
    }

    Ok(())
}

fn verify_that_every_variant_has_a_case(
    registry: &mut NodeRegistry,
    match_id: &'a Match<'a>,
    variant_name_list_id: Option<NonEmptyListId<&'a Identifier<'a>>>,
    case_list_id: Option<NonEmptyListId<&'a MatchCase<'a>>>,
) -> Result<(), TypeCheckError> {
    let variant_name_ids = registry.get_possibly_empty_list(variant_name_list_id);
    let case_ids = registry.get_possibly_empty_list(case_list_id);

    let missing_variant_name_ids: Vec<&'a Identifier<'a>> = variant_name_ids
        .iter()
        .copied()
        .filter_map(|variant_name_id| {
            let variant_name = &registry.get(variant_name_id).name;
            if !case_ids.iter().any(|&case_id| {
                let case = registry.get(case_id);
                let case_variant_name = &registry.get(case.variant_name_id).name;
                case_variant_name == variant_name
            }) {
                Some(variant_name_id)
            } else {
                None
            }
        })
        .collect();

    if let Ok(missing_variant_name_ids) = NonEmptyVec::try_from(missing_variant_name_ids) {
        let missing_variant_name_list_id = registry.add_list(missing_variant_name_ids);
        return Err(TypeCheckError::MissingMatchCases {
            missing_variant_name_list_id,
            match_id,
        });
    }

    Ok(())
}

fn verify_that_every_case_has_a_variant(
    registry: &NodeRegistry,
    variant_name_list_id: Option<NonEmptyListId<&'a Identifier<'a>>>,
    case_list_id: Option<NonEmptyListId<&'a MatchCase<'a>>>,
) -> Result<(), TypeCheckError> {
    let variant_name_ids = registry.get_possibly_empty_list(variant_name_list_id);
    let case_ids = registry.get_possibly_empty_list(case_list_id);

    for &case_id in case_ids {
        let case = registry.get(case_id);
        let case_variant_name = &registry.get(case.variant_name_id).name;
        if !variant_name_ids.iter().any(|&variant_name_id| {
            let variant_name = &registry.get(variant_name_id).name;
            case_variant_name == variant_name
        }) {
            return Err(TypeCheckError::ExtraneousMatchCase { case_id });
        }
    }
    Ok(())
}

pub(super) fn get_db_index_for_adt_variant_of_name(
    state: &mut State,
    adt_expression: NormalFormAdtExpression,
    target_variant_name_id: &'a Identifier<'a>,
) -> DbIndex {
    let type_dbi = state.registry.get(adt_expression.type_name_id).db_index;
    let variant_name_list_id = match state.context.get_definition(type_dbi, state.registry) {
        ContextEntryDefinition::Adt {
            variant_name_list_id,
            ..
        } => variant_name_list_id,
        _ => panic!("An ADT's NameExpression should always point to an ADT definition"),
    };

    let target_variant_name = &state.registry.get(target_variant_name_id).name;
    let variant_index = state
        .registry
        .get_possibly_empty_list(variant_name_list_id)
        .iter()
        .position(|&variant_name_id| {
            let variant_name = &state.registry.get(variant_name_id).name;
            variant_name == target_variant_name
        })
        .expect("The target variant name should always be found in the ADT's variant name list");
    DbIndex(type_dbi.0 - 1 - variant_index)
}
#[derive(Clone, Copy, Debug)]
pub struct DynamicSubstitution(pub NormalFormId, pub NormalFormId);

fn is_left_inclusive_subterm_of_right(
    state: &mut State,
    left: ExpressionRef<'a>,
    right: ExpressionRef<'a>,
) -> bool {
    if state.equality_checker.eq(left, right, state.registry) {
        return true;
    }

    match right {
        ExpressionRef<'a>::Name(_) => {
            // This must be false because we already checked for equality.
            false
        }
        ExpressionRef<'a>::Todo(_) => {
            // This must be false because we already checked for equality.
            false
        }
        ExpressionRef<'a>::Call(right_id) => {
            let right = state.registry.get(right_id).clone();

            if is_left_inclusive_subterm_of_right(state, left, right.callee_id) {
                return true;
            }

            if does_right_call_arg_list_contain_left(state, left, right.arg_list_id) {
                return true;
            }

            false
        }
        ExpressionRef<'a>::Fun(right_id) => {
            let right = state.registry.get(right_id).clone();

            if is_left_subterm_of_any_right_param_type(state, left, right.param_list_id) {
                return true;
            }

            let right_param_arity = right.param_list_id.len();

            {
                let shifted_left = left.upshift(right_param_arity, state.registry);
                if is_left_inclusive_subterm_of_right(state, shifted_left, right.return_type_id) {
                    return true;
                }
            }

            {
                let shifted_left = left.upshift(right_param_arity + 1, state.registry);
                if is_left_inclusive_subterm_of_right(state, shifted_left, right.body_id) {
                    return true;
                }
            }

            false
        }
        ExpressionRef<'a>::Match(right_id) => {
            let right = state.registry.get(right_id).clone();

            if is_left_inclusive_subterm_of_right(state, left, right.matchee_id) {
                return true;
            }

            let right_case_ids = state
                .registry
                .get_possibly_empty_list(right.case_list_id)
                .to_vec();
            if right_case_ids.iter().any(|&right_case_id| {
                let case_arity = state.registry.get(right_case_id).param_list_id.len();
                let shifted_left = left.upshift(case_arity, state.registry);
                let right_case_output_id = state.registry.get(right_case_id).output_id;
                match right_case_output_id {
                    MatchCaseOutputId::Some(right_case_output_id) => {
                        is_left_inclusive_subterm_of_right(
                            state,
                            shifted_left,
                            right_case_output_id,
                        )
                    }
                    MatchCaseOutputId::ImpossibilityClaim(_) => false,
                }
            }) {
                return true;
            }

            false
        }
        ExpressionRef<'a>::Forall(right_id) => {
            let right = state.registry.get(right_id).clone();

            if is_left_subterm_of_any_right_param_type(state, left, right.param_list_id) {
                return true;
            }

            let right_param_arity = right.param_list_id.len();

            {
                let shifted_left = left.upshift(right_param_arity, state.registry);
                if is_left_inclusive_subterm_of_right(state, shifted_left, right.output_id) {
                    return true;
                }
            }

            false
        }
        ExpressionRef<'a>::Check(right_id) => {
            let right = state.registry.get(right_id).clone();

            if is_left_inclusive_subterm_of_any_right_assertion(state, left, right_id) {
                return true;
            }

            if is_left_inclusive_subterm_of_right(state, left, right.output_id) {
                return true;
            }

            false
        }
    }
}

fn does_right_call_arg_list_contain_left(
    state: &mut State,
    left: ExpressionRef<'a>,
    right: NonEmptyCallArgListId,
) -> bool {
    match right {
        NonEmptyCallArgListId::Unlabeled(right) => {
            let right_arg_ids = state.registry.get_list(right).to_vec();
            right_arg_ids
                .iter()
                .copied()
                .any(|right_arg_id| is_left_inclusive_subterm_of_right(state, left, right_arg_id))
        }
        NonEmptyCallArgListId::UniquelyLabeled(right) => {
            let right_arg_ids = state.registry.get_list(right).to_vec();
            right_arg_ids.iter().copied().any(|right_arg_id| {
                let right_value_id = right_arg_id.value_id();
                is_left_inclusive_subterm_of_right(state, left, right_value_id)
            })
        }
    }
}

fn is_left_subterm_of_any_right_param_type(
    state: &mut State,
    left: ExpressionRef<'a>,
    right: NonEmptyParamListId,
) -> bool {
    match right {
        NonEmptyParamListId::Unlabeled(param_list_id) => {
            let right_param_ids = state.registry.get_list(param_list_id).to_vec();
            right_param_ids.iter().copied().enumerate().any(
                |(right_param_index, right_param_id)| {
                    let shifted_left = left.upshift(right_param_index, state.registry);
                    let right_param_type_id = state.registry.get(right_param_id).type_id;
                    is_left_inclusive_subterm_of_right(state, shifted_left, right_param_type_id)
                },
            )
        }
        NonEmptyParamListId::UniquelyLabeled(param_list_id) => {
            let right_param_ids = state.registry.get_list(param_list_id).to_vec();
            right_param_ids.iter().copied().enumerate().any(
                |(right_param_index, right_param_id)| {
                    let shifted_left = left.upshift(right_param_index, state.registry);
                    let right_param_type_id = state.registry.get(right_param_id).type_id;
                    is_left_inclusive_subterm_of_right(state, shifted_left, right_param_type_id)
                },
            )
        }
    }
}

fn is_left_inclusive_subterm_of_any_right_assertion(
    state: &mut State,
    left: ExpressionRef<'a>,
    right: &'a Check<'a>,
) -> bool {
    let right = state.registry.get(right).clone();
    let right_assertion_ids = state.registry.get_list(right.assertion_list_id).to_vec();
    right_assertion_ids.into_iter().any(|right_assertion_id| {
        let assertion = state.registry.get(right_assertion_id).clone();
        if let GoalKwOrPossiblyInvalidExpressionId::Expression(
            PossiblyInvalidExpressionId::Valid(assertion_left_id),
        ) = assertion.left_id
        {
            if is_left_inclusive_subterm_of_right(state, left, assertion_left_id) {
                return true;
            }
        }

        if let QuestionMarkOrPossiblyInvalidExpressionId::Expression(
            PossiblyInvalidExpressionId::Valid(assertion_right_id),
        ) = assertion.right_id
        {
            if is_left_inclusive_subterm_of_right(state, left, assertion_right_id) {
                return true;
            }
        }

        false
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NormalFormAdtExpression {
    pub type_name_id: &'a NameExpression<'a>,
    pub variant_name_list_id: Option<NonEmptyListId<&'a Identifier<'a>>>,
    pub arg_list_id: Option<NonEmptyCallArgListId>,
}

/// If the provided expression has an ADT constructor at
/// the top level, this returns the appropriate `AdtExpression`.
/// Otherwise, returns `None`.
pub(super) fn try_as_normal_form_adt_expression(
    state: &mut State,
    expression_id: NormalFormId,
) -> Option<NormalFormAdtExpression> {
    match expression_id.raw() {
        ExpressionRef<'a>::Name(name_id) => {
            let db_index = state.registry.get(name_id).db_index;
            let definition = state.context.get_definition(db_index, state.registry);
            match definition {
                ContextEntryDefinition::Adt {
                    variant_name_list_id,
                    ..
                } => Some(NormalFormAdtExpression {
                    type_name_id: name_id,
                    variant_name_list_id,
                    arg_list_id: None,
                }),
                _ => None,
            }
        }
        ExpressionRef<'a>::Call(call_id) => {
            let call = state.registry.get(call_id).clone();
            match call.callee_id {
                ExpressionRef<'a>::Name(name_id) => {
                    let db_index = state.registry.get(name_id).db_index;
                    let definition = state.context.get_definition(db_index, state.registry);
                    match definition {
                        ContextEntryDefinition::Adt {
                            variant_name_list_id,
                            ..
                        } => Some(NormalFormAdtExpression {
                            type_name_id: name_id,
                            variant_name_list_id: variant_name_list_id,
                            arg_list_id: Some(call.arg_list_id),
                        }),
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}

/// Returns whether the provided expression
/// has an ADT constructor at the top level.
pub(super) fn is_normal_form_adt_expression(
    state: &mut State,
    expression_id: NormalFormId,
) -> bool {
    try_as_normal_form_adt_expression(state, expression_id).is_some()
}

/// If the provided expression has a variant at
/// the top level, this returns IDs for the variant name
/// and the variant's argument list.
/// Otherwise, returns `None`.
pub(super) fn try_as_variant_expression(
    state: &mut State,
    expression_id: ExpressionRef<'a>,
) -> Option<(&'a Identifier<'a>, Option<NonEmptyCallArgListId>)> {
    try_as_variant_expression_with_node_registry_and_definition_getter(
        state.registry,
        |db_index, registry| state.context.get_definition(db_index, registry),
        expression_id,
    )
}

/// If the provided expression has a variant at
/// the top level, this returns IDs for the variant name
/// and the variant's argument list.
/// Otherwise, returns `None`.
pub(super) fn try_as_variant_expression_with_node_registry_and_definition_getter(
    registry: &mut NodeRegistry,
    mut get_definition: impl FnMut(DbIndex, &mut NodeRegistry) -> ContextEntryDefinition,
    expression_id: ExpressionRef<'a>,
) -> Option<(&'a Identifier<'a>, Option<NonEmptyCallArgListId>)> {
    match expression_id {
        ExpressionRef<'a>::Name(name_id) => {
            let db_index = registry.get(name_id).db_index;
            let definition = get_definition(db_index, registry);
            match definition {
                ContextEntryDefinition::Variant { name_id, .. } => Some((name_id, None)),
                _ => None,
            }
        }
        ExpressionRef<'a>::Call(call_id) => {
            let call = registry.get(call_id).clone();
            match call.callee_id {
                ExpressionRef<'a>::Name(name_id) => {
                    let db_index = registry.get(name_id).db_index;
                    let definition = get_definition(db_index, registry);
                    match definition {
                        ContextEntryDefinition::Variant { name_id, .. } => {
                            Some((name_id, Some(call.arg_list_id)))
                        }
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}

/// If the provided expression has a variant at
/// the top level,this returns true.
pub(super) fn is_variant_expression(state: &mut State, expression_id: NormalFormId) -> bool {
    try_as_variant_expression(state, expression_id.raw()).is_some()
}

/// If the provided expression has a variant at
/// the top level,this returns true.
pub(super) fn determine_whether_expression_is_variant_using_node_registry_and_definition_getter(
    registry: &mut NodeRegistry,
    get_definition: impl FnMut(DbIndex, &mut NodeRegistry) -> ContextEntryDefinition,
    expression_id: NormalFormId,
) -> bool {
    try_as_variant_expression_with_node_registry_and_definition_getter(
        registry,
        get_definition,
        expression_id.raw(),
    )
    .is_some()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Exploded;

#[derive(Clone, Copy, Debug)]
struct TaggedDynamicSubstitution {
    substitution: DynamicSubstitution,
    applied: bool,
}

impl TaggedDynamicSubstitution {
    fn unapplied(substitution: DynamicSubstitution) -> Self {
        Self {
            substitution,
            applied: false,
        }
    }
}

pub(super) fn apply_substitutions_from_substitution_context<
    E: Copy + Map<NormalFormId, Output = E>,
>(
    state: &mut State,
    expressions_to_substitute: E,
) -> Result<E, Exploded> {
    let mut substitutions: Vec<TaggedDynamicSubstitution> = state
        .substitution_context
        .get_adjusted_substitutions(state.registry, state.context.len())
        .expect("SubstitutionContext and Context should be in-sync.")
        .into_iter()
        .map(TaggedDynamicSubstitution::unapplied)
        .collect();
    let mut expressions_to_substitute = expressions_to_substitute;

    loop {
        let Some(tagged_sub_index) = substitutions.iter().position(|substitution| !substitution.applied) else {
            return Ok(expressions_to_substitute);
        };
        let tagged_sub = substitutions.remove(tagged_sub_index);
        apply_tagged_substitution(
            state,
            &mut substitutions,
            &mut expressions_to_substitute,
            tagged_sub,
        )?;
    }
}

fn apply_tagged_substitution<E: MapInPlace<NormalFormId, Output = E>>(
    state: &mut State,
    substitutions: &mut Vec<TaggedDynamicSubstitution>,
    expressions_to_substitute: &mut E,
    tagged_sub: TaggedDynamicSubstitution,
) -> Result<(), Exploded> {
    match expand_dynamic_substitution_shallow(state, tagged_sub.substitution) {
        DynamicSubstitutionExpansionResult::Exploded => Err(Exploded),
        DynamicSubstitutionExpansionResult::Replace(replacements) => {
            mark_all_as_unapplied(substitutions);
            substitutions.extend(
                replacements
                    .into_iter()
                    .map(TaggedDynamicSubstitution::unapplied),
            );
            Ok(())
        }
        DynamicSubstitutionExpansionResult::ApplyConcrete(concrete_sub) => {
            loop {
                let was_no_op = apply_concrete_substitution(
                    state,
                    substitutions,
                    expressions_to_substitute,
                    concrete_sub,
                );
                if was_no_op.0 {
                    break;
                } else {
                    mark_all_as_unapplied(substitutions);
                }
            }
            substitutions.push(TaggedDynamicSubstitution {
                substitution: tagged_sub.substitution,
                applied: true,
            });
            Ok(())
        }
    }
}

fn mark_all_as_unapplied(substitutions: &mut [TaggedDynamicSubstitution]) {
    for substitution in substitutions {
        substitution.applied = false;
    }
}

fn apply_concrete_substitution<E>(
    state: &mut State,
    substitutions: &mut [TaggedDynamicSubstitution],
    expressions_to_substitute: &mut E,
    concrete_sub: Substitution,
) -> WasSyntacticNoOp
where
    E: MapInPlace<NormalFormId, Output = E>,
{
    let mut was_no_op = WasSyntacticNoOp(true);

    expressions_to_substitute.map_in_place(|id| {
        let mut raw = id.raw();
        was_no_op &= raw.subst_in_place_and_get_status(concrete_sub, &mut state.without_context());
        evaluate_well_typed_expression(state, raw)
    });

    for remaining in substitutions.iter_mut() {
        was_no_op &= remaining
            .substitution
            .subst_in_place_and_get_status(concrete_sub, state);
    }

    was_no_op
}

#[derive(Clone, Debug)]
enum DynamicSubstitutionExpansionResult {
    ApplyConcrete(Substitution),
    /// If the original substitution was a no-op, the
    /// expansion will be `DynamicSubstitutionExpansionResult::Replace(vec![])`.
    Replace(Vec<DynamicSubstitution>),
    Exploded,
}

fn expand_dynamic_substitution_shallow(
    state: &mut State,
    d: DynamicSubstitution,
) -> DynamicSubstitutionExpansionResult {
    if let (Some(left), Some(right)) = (
        try_as_normal_form_adt_expression(state, d.0),
        try_as_normal_form_adt_expression(state, d.1),
    ) {
        expand_dynamic_adt_substitution_shallow(state, left, right)
    } else if let (Some(left), Some(right)) = (
        try_as_variant_expression(state, d.0.raw()),
        try_as_variant_expression(state, d.1.raw()),
    ) {
        expand_dynamic_normal_form_variant_substitution_shallow(state, left, right)
    }
    // TODO: Add else-ifs to handle cases Fun, Forall, etc.
    else {
        // TODO: Refactor
        match get_concrete_substitution(state, d) {
            Some(conc_sub) => DynamicSubstitutionExpansionResult::ApplyConcrete(conc_sub),
            None => DynamicSubstitutionExpansionResult::Replace(vec![]),
        }
    }
}

fn expand_dynamic_adt_substitution_shallow(
    state: &mut State,
    left: NormalFormAdtExpression,
    right: NormalFormAdtExpression,
) -> DynamicSubstitutionExpansionResult {
    let left_db_index = state.registry.get(left.type_name_id).db_index;
    let right_db_index = state.registry.get(right.type_name_id).db_index;
    if left_db_index != right_db_index {
        return DynamicSubstitutionExpansionResult::Exploded;
    }

    match (left.arg_list_id, right.arg_list_id) {
        (
            Some(NonEmptyCallArgListId::Unlabeled(left_arg_list_id)),
            Some(NonEmptyCallArgListId::Unlabeled(right_arg_list_id)),
        ) => expand_dynamic_normal_form_unlabeled_call_arg_list_substitution_shallow(
            state,
            left_arg_list_id,
            right_arg_list_id,
        ),
        (
            Some(NonEmptyCallArgListId::UniquelyLabeled(left_arg_list_id)),
            Some(NonEmptyCallArgListId::UniquelyLabeled(right_arg_list_id)),
        ) => expand_dynamic_normal_form_labeled_call_arg_list_substitution_shallow(
            state,
            left_arg_list_id,
            right_arg_list_id,
        ),
        (None, None) => DynamicSubstitutionExpansionResult::Replace(vec![]),
        _ => DynamicSubstitutionExpansionResult::Exploded,
    }
}

fn expand_dynamic_normal_form_variant_substitution_shallow(
    state: &mut State,
    left: (&'a Identifier<'a>, Option<NonEmptyCallArgListId>),
    right: (&'a Identifier<'a>, Option<NonEmptyCallArgListId>),
) -> DynamicSubstitutionExpansionResult {
    // We only need to compare name (rather than DB index) because
    // `left` and `right` are assumed to have the same type, and
    // every type can only have at most one variant associated with
    // a given name.
    let left_name = &state.registry.get(left.0).name;
    let right_name = &state.registry.get(right.0).name;
    if left_name != right_name {
        return DynamicSubstitutionExpansionResult::Exploded;
    }

    match (left.1, right.1) {
        (
            Some(NonEmptyCallArgListId::Unlabeled(left_arg_list_id)),
            Some(NonEmptyCallArgListId::Unlabeled(right_arg_list_id)),
        ) => expand_dynamic_normal_form_unlabeled_call_arg_list_substitution_shallow(
            state,
            left_arg_list_id,
            right_arg_list_id,
        ),
        (
            Some(NonEmptyCallArgListId::UniquelyLabeled(left_arg_list_id)),
            Some(NonEmptyCallArgListId::UniquelyLabeled(right_arg_list_id)),
        ) => expand_dynamic_normal_form_labeled_call_arg_list_substitution_shallow(
            state,
            left_arg_list_id,
            right_arg_list_id,
        ),
        (None, None) => DynamicSubstitutionExpansionResult::Replace(vec![]),
        _ => DynamicSubstitutionExpansionResult::Exploded,
    }
}

fn expand_dynamic_normal_form_unlabeled_call_arg_list_substitution_shallow(
    state: &mut State,
    normalized_left_arg_list_id: NonEmptyListId<ExpressionRef<'a>>,
    normalized_right_arg_list_id: NonEmptyListId<ExpressionRef<'a>>,
) -> DynamicSubstitutionExpansionResult {
    let left_args = state.registry.get_list(normalized_left_arg_list_id);
    let right_args = state.registry.get_list(normalized_right_arg_list_id);
    assert_eq!(left_args.len(), right_args.len(), "Two well-typed Call expressions with the same callee should have the same number of arguments.");
    let arg_substitutions = left_args
        .iter()
        .copied()
        .zip(right_args.iter().copied())
        .map(|(left_arg_id, right_arg_id)| {
            DynamicSubstitution(
                // This is safe because the argument of a normal
                // form Call expression is always itself a normal form.
                NormalFormId::unchecked_new(left_arg_id),
                NormalFormId::unchecked_new(right_arg_id),
            )
        })
        .collect();
    DynamicSubstitutionExpansionResult::Replace(arg_substitutions)
}

fn expand_dynamic_normal_form_labeled_call_arg_list_substitution_shallow(
    state: &mut State,
    normalized_left_arg_list_id: NonEmptyListId<LabeledCallArgId>,
    normalized_right_arg_list_id: NonEmptyListId<LabeledCallArgId>,
) -> DynamicSubstitutionExpansionResult {
    let normalized_left_arg_ids = state
        .registry
        .get_list(normalized_left_arg_list_id)
        .to_non_empty_vec();
    let normalized_right_arg_ids = state
        .registry
        .get_list(normalized_right_arg_list_id)
        .to_non_empty_vec();
    assert_eq!(normalized_left_arg_ids.len(), normalized_right_arg_ids.len(), "Two well-typed Call expressions with the same callee should have the same number of arguments.");

    let mut subs = vec![];
    for &normalized_left_arg_id in normalized_left_arg_ids.iter() {
        let left_value_id = NormalFormId::unchecked_new(normalized_left_arg_id.value_id());
        let left_label_id = normalized_left_arg_id.label_id();
        let left_label_name = &state.registry.get(left_label_id).name;
        let corresponding_normalized_right_arg = normalized_right_arg_ids
            .iter()
            .find(|&right_arg_id| {
                let right_label_id = right_arg_id.label_id();
                left_label_name == &state.registry.get(right_label_id).name
            })
            .expect(
                "Two well-typed Call expressions with the same callee should have the same labels.",
            );
        let right_value_id =
            NormalFormId::unchecked_new(corresponding_normalized_right_arg.value_id());
        subs.push(DynamicSubstitution(left_value_id, right_value_id))
    }
    DynamicSubstitutionExpansionResult::Replace(subs)
}

/// Returns `None` if the dynamic substitution is a no-op.
fn get_concrete_substitution(state: &mut State, d: DynamicSubstitution) -> Option<Substitution> {
    if d.0.raw() == d.1.raw() {
        return None;
    }
    if is_left_inclusive_subterm_of_right(state, d.0.raw(), d.1.raw()) {
        return Some(Substitution {
            from: d.1.raw(),
            to: d.0.raw(),
        });
    }
    if is_left_inclusive_subterm_of_right(state, d.1.raw(), d.0.raw()) {
        return Some(Substitution {
            from: d.0.raw(),
            to: d.1.raw(),
        });
    }

    if is_variant_expression(state, d.0) {
        return Some(Substitution {
            from: d.1.raw(),
            to: d.0.raw(),
        });
    }
    if is_variant_expression(state, d.1) {
        return Some(Substitution {
            from: d.0.raw(),
            to: d.1.raw(),
        });
    }

    if is_normal_form_adt_expression(state, d.0) {
        return Some(Substitution {
            from: d.1.raw(),
            to: d.0.raw(),
        });
    }
    if is_normal_form_adt_expression(state, d.1) {
        return Some(Substitution {
            from: d.0.raw(),
            to: d.1.raw(),
        });
    }

    Some(Substitution {
        from: d.0.raw(),
        to: d.1.raw(),
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WasSyntacticNoOp(pub bool);

impl std::ops::BitAndAssign for WasSyntacticNoOp {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl std::ops::BitAnd for WasSyntacticNoOp {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self {
        self &= rhs;
        self
    }
}

pub(super) trait SubstituteInPlaceAndGetNoOpStatus {
    fn subst_in_place_and_get_status(
        &mut self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> WasSyntacticNoOp;
}

impl SubstituteInPlaceAndGetNoOpStatus for ExpressionRef<'a> {
    fn subst_in_place_and_get_status(
        &mut self,
        substitution: Substitution,
        state: &mut ContextlessState,
    ) -> WasSyntacticNoOp {
        let original = *self;
        let new = original.subst(substitution, state);
        let was_no_op = WasSyntacticNoOp(original == new);

        *self = new;
        was_no_op
    }
}

// We can't `impl SubstituteInPlace` here, because applying a substitution to
// a `DynamicSubstitution` requires evaluation, which requires a `State` (which
// is too strong of a requirement to satisfy the `SubstituteInPlace` trait).
impl DynamicSubstitution {
    fn subst_in_place_and_get_status(
        &mut self,
        substitution: Substitution,
        state: &mut State,
    ) -> WasSyntacticNoOp {
        let (substituted, was_no_op) = self.subst_and_get_status(substitution, state);
        *self = substituted;
        was_no_op
    }

    fn subst_and_get_status(
        self,
        substitution: Substitution,
        state: &mut State,
    ) -> (Self, WasSyntacticNoOp) {
        let original_t1 = self.0;
        let original_t2 = self.1;
        let t1 = self
            .0
            .raw()
            .subst(substitution, &mut state.without_context());
        let t2 = self
            .1
            .raw()
            .subst(substitution, &mut state.without_context());
        let t1 = evaluate_well_typed_expression(state, t1);
        let t2 = evaluate_well_typed_expression(state, t2);
        let was_no_op =
            WasSyntacticNoOp(original_t1.raw() == t1.raw() && original_t2.raw() == t2.raw());
        (DynamicSubstitution(t1, t2), was_no_op)
    }
}

pub trait Map<I, O = I> {
    type Output;

    fn map(self, f: impl FnMut(I) -> O) -> Self::Output;
}

impl<I, O> Map<I, O> for Vec<I> {
    type Output = Vec<O>;

    fn map(self, f: impl FnMut(I) -> O) -> Self::Output {
        self.into_iter().map(f).collect()
    }
}

impl<I, O> Map<I, O> for () {
    type Output = ();

    fn map(self, _f: impl FnMut(I) -> O) -> Self::Output {
        ()
    }
}

impl<I, O> Map<I, O> for (I,) {
    type Output = (O,);

    fn map(self, mut f: impl FnMut(I) -> O) -> Self::Output {
        (f(self.0),)
    }
}

impl<I, O> Map<I, O> for Option<I> {
    type Output = Option<O>;

    fn map(self, f: impl FnMut(I) -> O) -> Self::Output {
        self.map(f)
    }
}

impl<I, O, U1, U2> Map<I, O> for (U1, U2)
where
    U1: Map<I, O>,
    U2: Map<I, O>,
{
    type Output = (U1::Output, U2::Output);

    fn map(self, mut f: impl FnMut(I) -> O) -> Self::Output {
        (self.0.map(&mut f), self.1.map(&mut f))
    }
}

impl<I, O, U1, U2, U3> Map<I, O> for (U1, U2, U3)
where
    U1: Map<I, O>,
    U2: Map<I, O>,
    U3: Map<I, O>,
{
    type Output = (U1::Output, U2::Output, U3::Output);

    fn map(self, mut f: impl FnMut(I) -> O) -> Self::Output {
        (self.0.map(&mut f), self.1.map(&mut f), self.2.map(&mut f))
    }
}

impl<I, O, U1, U2, U3, U4> Map<I, O> for (U1, U2, U3, U4)
where
    U1: Map<I, O>,
    U2: Map<I, O>,
    U3: Map<I, O>,
    U4: Map<I, O>,
{
    type Output = (U1::Output, U2::Output, U3::Output, U4::Output);

    fn map(self, mut f: impl FnMut(I) -> O) -> Self::Output {
        (
            self.0.map(&mut f),
            self.1.map(&mut f),
            self.2.map(&mut f),
            self.3.map(&mut f),
        )
    }
}

impl<I, O, U1, U2, U3, U4, U5> Map<I, O> for (U1, U2, U3, U4, U5)
where
    U1: Map<I, O>,
    U2: Map<I, O>,
    U3: Map<I, O>,
    U4: Map<I, O>,
    U5: Map<I, O>,
{
    type Output = (U1::Output, U2::Output, U3::Output, U4::Output, U5::Output);

    fn map(self, mut f: impl FnMut(I) -> O) -> Self::Output {
        (
            self.0.map(&mut f),
            self.1.map(&mut f),
            self.2.map(&mut f),
            self.3.map(&mut f),
            self.4.map(&mut f),
        )
    }
}

impl<I, O, U1, U2, U3, U4, U5, U6> Map<I, O> for (U1, U2, U3, U4, U5, U6)
where
    U1: Map<I, O>,
    U2: Map<I, O>,
    U3: Map<I, O>,
    U4: Map<I, O>,
    U5: Map<I, O>,
    U6: Map<I, O>,
{
    type Output = (
        U1::Output,
        U2::Output,
        U3::Output,
        U4::Output,
        U5::Output,
        U6::Output,
    );

    fn map(self, mut f: impl FnMut(I) -> O) -> Self::Output {
        (
            self.0.map(&mut f),
            self.1.map(&mut f),
            self.2.map(&mut f),
            self.3.map(&mut f),
            self.4.map(&mut f),
            self.5.map(&mut f),
        )
    }
}

impl<I, O, U1, U2, U3, U4, U5, U6, U7> Map<I, O> for (U1, U2, U3, U4, U5, U6, U7)
where
    U1: Map<I, O>,
    U2: Map<I, O>,
    U3: Map<I, O>,
    U4: Map<I, O>,
    U5: Map<I, O>,
    U6: Map<I, O>,
    U7: Map<I, O>,
{
    type Output = (
        U1::Output,
        U2::Output,
        U3::Output,
        U4::Output,
        U5::Output,
        U6::Output,
        U7::Output,
    );

    fn map(self, mut f: impl FnMut(I) -> O) -> Self::Output {
        (
            self.0.map(&mut f),
            self.1.map(&mut f),
            self.2.map(&mut f),
            self.3.map(&mut f),
            self.4.map(&mut f),
            self.5.map(&mut f),
            self.6.map(&mut f),
        )
    }
}

impl<I, O, U1, U2, U3, U4, U5, U6, U7, U8> Map<I, O> for (U1, U2, U3, U4, U5, U6, U7, U8)
where
    U1: Map<I, O>,
    U2: Map<I, O>,
    U3: Map<I, O>,
    U4: Map<I, O>,
    U5: Map<I, O>,
    U6: Map<I, O>,
    U7: Map<I, O>,
    U8: Map<I, O>,
{
    type Output = (
        U1::Output,
        U2::Output,
        U3::Output,
        U4::Output,
        U5::Output,
        U6::Output,
        U7::Output,
        U8::Output,
    );

    fn map(self, mut f: impl FnMut(I) -> O) -> Self::Output {
        (
            self.0.map(&mut f),
            self.1.map(&mut f),
            self.2.map(&mut f),
            self.3.map(&mut f),
            self.4.map(&mut f),
            self.5.map(&mut f),
            self.6.map(&mut f),
            self.7.map(&mut f),
        )
    }
}

pub trait MapInPlace<I, O = I> {
    type Output;

    fn map_in_place(&mut self, f: impl FnMut(I) -> O) -> Self::Output;
}

impl<I, O, T> MapInPlace<I, O> for T
where
    T: Copy + Map<I, O, Output = T>,
{
    type Output = T::Output;

    fn map_in_place(&mut self, f: impl FnMut(I) -> O) -> Self::Output {
        *self = self.map(f);
        *self
    }
}

pub(super) fn get_unlabeled_param_type_ids(
    state: &State,
    param_list_id: NonEmptyListId<&'a UnlabeledParam<'a>>,
) -> NonEmptyVec<ExpressionRef<'a>> {
    state
        .registry
        .get_list(param_list_id)
        .to_mapped(|&param_id| {
            let param = state.registry.get(param_id);
            param.type_id
        })
}

pub(super) fn get_names_and_types_of_params(
    state: &State,
    param_list_id: NonEmptyParamListId,
) -> (NonEmptyVec<&'a Identifier<'a>>, NonEmptyVec<ExpressionRef<'a>>) {
    match param_list_id {
        NonEmptyParamListId::Unlabeled(id) => get_names_and_types_of_unlabeled_params(state, id),
        NonEmptyParamListId::UniquelyLabeled(id) => {
            get_names_and_types_of_labeled_params(state, id)
        }
    }
}

pub(super) fn get_names_and_types_of_unlabeled_params(
    state: &State,
    param_list_id: NonEmptyListId<&'a UnlabeledParam<'a>>,
) -> (NonEmptyVec<&'a Identifier<'a>>, NonEmptyVec<ExpressionRef<'a>>) {
    let param_ids = state.registry.get_list(param_list_id);
    param_ids.map_to_unzipped(|param_id| {
        let param = state.registry.get(*param_id);
        (param.name_id, param.type_id)
    })
}

pub(super) fn get_names_and_types_of_labeled_params(
    state: &State,
    param_list_id: NonEmptyListId<&'a LabeledParam<'a>>,
) -> (NonEmptyVec<&'a Identifier<'a>>, NonEmptyVec<ExpressionRef<'a>>) {
    let param_ids = state.registry.get_list(param_list_id);
    param_ids.map_to_unzipped(|param_id| {
        let param = state.registry.get(*param_id);
        (param.name_id, param.type_id)
    })
}

pub(super) fn get_arg_corresponding_to_label(
    state: &State,
    target_label_id: &'a Identifier<'a>,
    arg_ids: &[LabeledCallArgId],
) -> Option<(usize, LabeledCallArgId)> {
    let target_label_name = &state.registry.get(target_label_id).name;
    arg_ids.iter().copied().enumerate().find(|(_, arg_id)| {
        let arg_label_id = arg_id.label_id();
        let arg_label_name = &state.registry.get(arg_label_id).name;
        arg_label_name == target_label_name
    })
}

pub fn is_left_at_least_as_permissive_as_right(
    file_tree: &FileTree,
    left: ModScope,
    right: ModScope,
) -> bool {
    match (left, right) {
        (ModScope::Global, ModScope::Global) => true,
        (ModScope::Global, ModScope::Mod(_)) => true,
        (ModScope::Mod(_), ModScope::Global) => false,
        (ModScope::Mod(left), ModScope::Mod(right)) => {
            file_tree.is_left_non_strict_descendant_of_right(right, left)
        }
    }
}
