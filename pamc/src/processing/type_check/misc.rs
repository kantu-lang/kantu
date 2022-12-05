use super::*;

use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NormalFormId(ExpressionId);

impl NormalFormId {
    pub fn unchecked_new(expression: ExpressionId) -> Self {
        Self(expression)
    }
}

impl NormalFormId {
    pub fn raw(self) -> ExpressionId {
        self.0
    }
}

pub(super) fn type0_expression(state: &mut State) -> NormalFormId {
    let name_id = add_name_expression_and_overwrite_component_ids(
        state.registry,
        vec![Identifier {
            id: dummy_id(),
            name: IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase),
            start: None,
        }],
        state.context.type0_dbi(),
    );
    NormalFormId::unchecked_new(ExpressionId::Name(name_id))
}

pub fn add_name_expression_and_overwrite_component_ids(
    registry: &mut NodeRegistry,
    components: Vec<Identifier>,
    db_index: DbIndex,
) -> NodeId<NameExpression> {
    let component_ids = components
        .into_iter()
        .map(|component| registry.add_identifier_and_overwrite_its_id(component))
        .collect();
    let component_list_id = registry.add_identifier_list(component_ids);
    registry.add_name_expression_and_overwrite_its_id(NameExpression {
        id: dummy_id(),
        component_list_id,
        db_index,
    })
}

pub fn add_name_expression(
    registry: &mut NodeRegistry,
    component_ids: Vec<NodeId<Identifier>>,
    db_index: DbIndex,
) -> NodeId<NameExpression> {
    let component_list_id = registry.add_identifier_list(component_ids);
    registry.add_name_expression_and_overwrite_its_id(NameExpression {
        id: dummy_id(),
        component_list_id,
        db_index,
    })
}

pub fn dummy_id<T>() -> NodeId<T> {
    NodeId::new(0)
}

impl Forall {
    pub fn collapse_if_nullary(self, registry: &mut NodeRegistry) -> ExpressionId {
        if self.param_list_id.len == 0 {
            self.output_id
        } else {
            let forall_id = registry.add_forall_and_overwrite_its_id(self);
            ExpressionId::Forall(forall_id)
        }
    }
}

pub(super) fn is_term_equal_to_type0_or_type1(state: &State, term: NormalFormId) -> bool {
    if let ExpressionId::Name(name_id) = term.raw() {
        let name = state.registry.name_expression(name_id);
        let i = name.db_index;
        i == state.context.type0_dbi() || i == state.context.type1_dbi()
    } else {
        false
    }
}

pub(super) fn is_left_type_assignable_to_right_type(
    state: &mut State,
    left: NormalFormId,
    right: NormalFormId,
) -> bool {
    is_term_equal_to_a_trivially_empty_type(state, left)
        || state
            .equality_checker
            .eq(left.raw(), right.raw(), state.registry)
}

fn is_term_equal_to_a_trivially_empty_type(state: &mut State, term_id: NormalFormId) -> bool {
    if let Some(adt) = try_as_adt_expression(state, term_id) {
        adt.variant_name_list_id.len == 0
    } else {
        false
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

pub(super) fn normalize_params_and_leave_params_in_context_dirty(
    state: &mut State,
    param_list_id: ListId<NodeId<Param>>,
) -> Result<WithPushWarning<ListId<NodeId<Param>>>, Tainted<TypeCheckError>> {
    let param_ids = state.registry.param_list(param_list_id).to_vec();
    let normalized_ids = param_ids
        .iter()
        .copied()
        .map(
            |param_id| -> Result<NodeId<Param>, Tainted<TypeCheckError>> {
                type_check_param_dirty(state, param_id)??;
                let type_id: ExpressionId = state
                    .context
                    .get_type(DbIndex(0), state.registry)
                    .downshift(1, state.registry)
                    .raw();
                let old_param = state.registry.param(param_id);
                let normalized_param_with_dummy_id = Param {
                    id: dummy_id(),
                    is_dashed: old_param.is_dashed,
                    name_id: old_param.name_id,
                    type_id,
                };
                let normalized_id = state
                    .registry
                    .add_param_and_overwrite_its_id(normalized_param_with_dummy_id);
                Ok(normalized_id)
            },
        )
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Ok(state.registry.add_param_list(normalized_ids)))
}

pub fn verify_variant_to_case_bijection(
    registry: &NodeRegistry,
    variant_name_list_id: ListId<NodeId<Identifier>>,
    case_list_id: ListId<NodeId<MatchCase>>,
) -> Result<(), TypeCheckError> {
    verify_there_are_no_duplicate_cases(registry, case_list_id)?;
    verify_that_every_variant_has_a_case(registry, variant_name_list_id, case_list_id)?;
    verify_that_every_case_has_a_variant(registry, variant_name_list_id, case_list_id)?;
    Ok(())
}

fn verify_there_are_no_duplicate_cases(
    registry: &NodeRegistry,
    case_list_id: ListId<NodeId<MatchCase>>,
) -> Result<(), TypeCheckError> {
    let mut visited_cases: Vec<NodeId<MatchCase>> = Vec::with_capacity(case_list_id.len);

    let case_ids = registry.match_case_list(case_list_id);

    for &case_id in case_ids {
        let case = registry.match_case(case_id);
        let case_variant_name = &registry.identifier(case.variant_name_id).name;

        if let Some(existing_case_id) = visited_cases
            .iter()
            .find(|&&existing_case_id| {
                let existing_case = registry.match_case(existing_case_id);
                let existing_case_variant_name =
                    &registry.identifier(existing_case.variant_name_id).name;
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
    registry: &NodeRegistry,
    variant_name_list_id: ListId<NodeId<Identifier>>,
    case_list_id: ListId<NodeId<MatchCase>>,
) -> Result<(), TypeCheckError> {
    let variant_name_ids = registry.identifier_list(variant_name_list_id);
    let case_ids = registry.match_case_list(case_list_id);

    for &variant_name_id in variant_name_ids {
        let variant_name = &registry.identifier(variant_name_id).name;
        if !case_ids.iter().any(|&case_id| {
            let case = registry.match_case(case_id);
            let case_variant_name = &registry.identifier(case.variant_name_id).name;
            case_variant_name == variant_name
        }) {
            return Err(TypeCheckError::MissingMatchCase { variant_name_id });
        }
    }
    Ok(())
}

fn verify_that_every_case_has_a_variant(
    registry: &NodeRegistry,
    variant_name_list_id: ListId<NodeId<Identifier>>,
    case_list_id: ListId<NodeId<MatchCase>>,
) -> Result<(), TypeCheckError> {
    let variant_name_ids = registry.identifier_list(variant_name_list_id);
    let case_ids = registry.match_case_list(case_list_id);

    for &case_id in case_ids {
        let case = registry.match_case(case_id);
        let case_variant_name = &registry.identifier(case.variant_name_id).name;
        if !variant_name_ids.iter().any(|&variant_name_id| {
            let variant_name = &registry.identifier(variant_name_id).name;
            case_variant_name == variant_name
        }) {
            return Err(TypeCheckError::ExtraneousMatchCase { case_id });
        }
    }
    Ok(())
}

pub(super) fn get_db_index_for_adt_variant_of_name(
    state: &mut State,
    adt_expression: AdtExpression,
    target_variant_name_id: NodeId<Identifier>,
) -> DbIndex {
    let type_dbi = state
        .registry
        .name_expression(adt_expression.type_name_id)
        .db_index;
    let variant_name_list_id = match state.context.get_definition(type_dbi, state.registry) {
        ContextEntryDefinition::Adt {
            variant_name_list_id,
        } => variant_name_list_id,
        _ => panic!("An ADT's NameExpression should always point to an ADT definition"),
    };

    let target_variant_name = &state.registry.identifier(target_variant_name_id).name;
    let variant_index = state
        .registry
        .identifier_list(variant_name_list_id)
        .iter()
        .position(|&variant_name_id| {
            let variant_name = &state.registry.identifier(variant_name_id).name;
            variant_name == target_variant_name
        })
        .expect("The target variant name should always be found in the ADT's variant name list");
    DbIndex(type_dbi.0 - 1 - variant_index)
}

#[derive(Clone, Debug)]
pub struct BackwardFusion {
    pub has_exploded: bool,
    pub substitutions: Vec<DynamicSubstitution>,
}

#[derive(Clone, Copy, Debug)]
pub struct DynamicSubstitution(pub NormalFormId, pub NormalFormId);

impl std::ops::AddAssign<BackwardFusion> for BackwardFusion {
    fn add_assign(&mut self, rhs: BackwardFusion) {
        self.has_exploded |= rhs.has_exploded;
        self.substitutions.extend(rhs.substitutions);
    }
}

pub(super) fn backfuse(
    state: &mut State,
    left: NormalFormId,
    right: NormalFormId,
) -> BackwardFusion {
    if let (Some(left_ve), Some(right_ve)) = (
        try_as_variant_expression(state, left.raw()),
        try_as_variant_expression(state, right.raw()),
    ) {
        let left_name: &IdentifierName = &state.registry.identifier(left_ve.0).name;
        let right_name: &IdentifierName = &state.registry.identifier(right_ve.0).name;
        if left_name == right_name {
            backfuse_arg_list(state, left_ve.1, right_ve.1)
        } else {
            BackwardFusion {
                has_exploded: true,
                substitutions: vec![],
            }
        }
    } else if let (Some(left_ae), Some(right_ae)) = (
        try_as_adt_expression(state, left),
        try_as_adt_expression(state, right),
    ) {
        let left_type_db_index = state
            .registry
            .name_expression(left_ae.type_name_id)
            .db_index;
        let right_type_db_index = state
            .registry
            .name_expression(right_ae.type_name_id)
            .db_index;
        if left_type_db_index == right_type_db_index {
            backfuse_arg_list(state, left_ae.arg_list_id, right_ae.arg_list_id)
        } else {
            BackwardFusion {
                has_exploded: true,
                substitutions: vec![],
            }
        }
    } else {
        BackwardFusion {
            has_exploded: false,
            substitutions: vec![DynamicSubstitution(left, right)],
        }
    }
}

fn backfuse_arg_list(
    state: &mut State,
    left_arg_list_id: PossibleArgListId,
    right_arg_list_id: PossibleArgListId,
) -> BackwardFusion {
    match (left_arg_list_id, right_arg_list_id) {
        (PossibleArgListId::Some(left_arg_list_id), PossibleArgListId::Some(right_arg_list_id)) => {
            let mut out = BackwardFusion {
                has_exploded: false,
                substitutions: vec![],
            };
            let left_arg_ids = state.registry.expression_list(left_arg_list_id).to_vec();
            let right_arg_ids = state.registry.expression_list(right_arg_list_id).to_vec();
            for (left_arg_id, right_arg_id) in left_arg_ids
                .iter()
                .copied()
                .zip(right_arg_ids.iter().copied())
            {
                out += backfuse(
                    state,
                    // This is safe because an arg to a normal
                    // form Call node is always a normal form itself.
                    NormalFormId::unchecked_new(left_arg_id),
                    NormalFormId::unchecked_new(right_arg_id),
                );
            }
            out
        }
        (PossibleArgListId::Nullary, PossibleArgListId::Nullary) => BackwardFusion {
            has_exploded: false,
            substitutions: vec![],
        },
        other => panic!("Invalid fusion: {:?}", other),
    }
}

fn is_left_inclusive_subterm_of_right(
    state: &mut State,
    left: ExpressionId,
    right: ExpressionId,
) -> bool {
    if state.equality_checker.eq(left, right, state.registry) {
        return true;
    }

    match right {
        ExpressionId::Name(_) => {
            // This must be false because we already checked for equality.
            false
        }
        ExpressionId::Call(right_id) => {
            let right = state.registry.call(right_id).clone();

            if is_left_inclusive_subterm_of_right(state, left, right.callee_id) {
                return true;
            }

            let right_arg_ids = state.registry.expression_list(right.arg_list_id).to_vec();
            if right_arg_ids
                .iter()
                .any(|&right_arg_id| is_left_inclusive_subterm_of_right(state, left, right_arg_id))
            {
                return true;
            }

            false
        }
        ExpressionId::Fun(right_id) => {
            let right = state.registry.fun(right_id).clone();

            let right_param_ids = state.registry.param_list(right.param_list_id).to_vec();
            if right_param_ids.iter().copied().enumerate().any(
                |(right_param_index, right_param_id)| {
                    let shifted_left = left.upshift(right_param_index, state.registry);
                    let right_param_type_id = state.registry.param(right_param_id).type_id;
                    is_left_inclusive_subterm_of_right(state, shifted_left, right_param_type_id)
                },
            ) {
                return true;
            }

            {
                let shifted_left = left.upshift(right_param_ids.len(), state.registry);
                if is_left_inclusive_subterm_of_right(state, shifted_left, right.return_type_id) {
                    return true;
                }
            }

            {
                let shifted_left = left.upshift(right_param_ids.len() + 1, state.registry);
                if is_left_inclusive_subterm_of_right(state, shifted_left, right.body_id) {
                    return true;
                }
            }

            false
        }
        ExpressionId::Match(right_id) => {
            let right = state.registry.match_(right_id).clone();

            if is_left_inclusive_subterm_of_right(state, left, right.matchee_id) {
                return true;
            }

            let right_case_ids = state.registry.match_case_list(right.case_list_id).to_vec();
            if right_case_ids.iter().any(|&right_case_id| {
                let case_arity = state.registry.match_case(right_case_id).param_list_id.len;
                let shifted_left = left.upshift(case_arity, state.registry);
                let right_case_output_id = state.registry.match_case(right_case_id).output_id;
                is_left_inclusive_subterm_of_right(state, shifted_left, right_case_output_id)
            }) {
                return true;
            }

            false
        }
        ExpressionId::Forall(right_id) => {
            let right = state.registry.forall(right_id).clone();

            let right_param_ids = state.registry.param_list(right.param_list_id).to_vec();
            if right_param_ids.iter().copied().enumerate().any(
                |(right_param_index, right_param_id)| {
                    let shifted_left = left.upshift(right_param_index, state.registry);
                    let right_param_type_id = state.registry.param(right_param_id).type_id;
                    is_left_inclusive_subterm_of_right(state, shifted_left, right_param_type_id)
                },
            ) {
                return true;
            }

            {
                let shifted_left = left.upshift(right.param_list_id.len, state.registry);
                if is_left_inclusive_subterm_of_right(state, shifted_left, right.output_id) {
                    return true;
                }
            }

            false
        }
        ExpressionId::Check(right_id) => {
            let right = state.registry.check(right_id).clone();

            if let CheckeeAnnotationId::Goal(right_annotation_id) = right.checkee_annotation_id {
                let right_annotation = state
                    .registry
                    .goal_checkee_annotation(right_annotation_id)
                    .clone();

                if let QuestionMarkOrPossiblyInvalidExpressionId::Expression(
                    PossiblyInvalidExpressionId::Valid(right_checkee_type_id),
                ) = right_annotation.checkee_type_id
                {
                    if is_left_inclusive_subterm_of_right(state, left, right_checkee_type_id) {
                        return true;
                    }
                }
            }

            if let CheckeeAnnotationId::Expression(right_annotation_id) =
                right.checkee_annotation_id
            {
                let right_annotation = state
                    .registry
                    .expression_checkee_annotation(right_annotation_id)
                    .clone();

                if is_left_inclusive_subterm_of_right(state, left, right_annotation.checkee_id) {
                    return true;
                }

                if let QuestionMarkOrPossiblyInvalidExpressionId::Expression(
                    PossiblyInvalidExpressionId::Valid(right_checkee_type_id),
                ) = right_annotation.checkee_type_id
                {
                    if is_left_inclusive_subterm_of_right(state, left, right_checkee_type_id) {
                        return true;
                    }
                }

                if let Some(QuestionMarkOrPossiblyInvalidExpressionId::Expression(
                    PossiblyInvalidExpressionId::Valid(right_checkee_value_id),
                )) = right_annotation.checkee_value_id
                {
                    if is_left_inclusive_subterm_of_right(state, left, right_checkee_value_id) {
                        return true;
                    }
                }
            }

            if is_left_inclusive_subterm_of_right(state, left, right.output_id) {
                return true;
            }

            false
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PossibleArgListId {
    Nullary,
    Some(ListId<ExpressionId>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdtExpression {
    pub type_name_id: NodeId<NameExpression>,
    pub variant_name_list_id: ListId<NodeId<Identifier>>,
    pub arg_list_id: PossibleArgListId,
}

/// If the provided expression is has an ADT constructor at
/// the top level, this returns the appropriate `AdtExpression`.
/// Otherwise, returns `None`.
pub(super) fn try_as_adt_expression(
    state: &mut State,
    expression_id: NormalFormId,
) -> Option<AdtExpression> {
    match expression_id.raw() {
        ExpressionId::Name(name_id) => {
            let db_index = state.registry.name_expression(name_id).db_index;
            let definition = state.context.get_definition(db_index, state.registry);
            match definition {
                ContextEntryDefinition::Adt {
                    variant_name_list_id,
                } => Some(AdtExpression {
                    type_name_id: name_id,
                    variant_name_list_id,
                    arg_list_id: PossibleArgListId::Nullary,
                }),
                _ => None,
            }
        }
        ExpressionId::Call(call_id) => {
            let call = state.registry.call(call_id).clone();
            match call.callee_id {
                ExpressionId::Name(name_id) => {
                    let db_index = state.registry.name_expression(name_id).db_index;
                    let definition = state.context.get_definition(db_index, state.registry);
                    match definition {
                        ContextEntryDefinition::Adt {
                            variant_name_list_id,
                        } => Some(AdtExpression {
                            type_name_id: name_id,
                            variant_name_list_id: variant_name_list_id,
                            arg_list_id: PossibleArgListId::Some(call.arg_list_id),
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

/// If the provided expression is has a variant at
/// the top level,this returns IDs for the variant name
/// and the variant's argument list.
/// Otherwise, returns `None`.
pub(super) fn try_as_variant_expression(
    state: &mut State,
    expression_id: ExpressionId,
) -> Option<(NodeId<Identifier>, PossibleArgListId)> {
    match expression_id {
        ExpressionId::Name(name_id) => {
            let db_index = state.registry.name_expression(name_id).db_index;
            let definition = state.context.get_definition(db_index, state.registry);
            match definition {
                ContextEntryDefinition::Variant { name_id } => {
                    Some((name_id, PossibleArgListId::Nullary))
                }
                _ => None,
            }
        }
        ExpressionId::Call(call_id) => {
            let call = state.registry.call(call_id).clone();
            match call.callee_id {
                ExpressionId::Name(name_id) => {
                    let db_index = state.registry.name_expression(name_id).db_index;
                    let definition = state.context.get_definition(db_index, state.registry);
                    match definition {
                        ContextEntryDefinition::Variant { name_id } => {
                            Some((name_id, PossibleArgListId::Some(call.arg_list_id)))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HasExploded(pub bool);

pub(super) fn apply_dynamic_substitutions_with_compounding<E: Map<ExpressionId, Output = E>>(
    state: &mut State,
    substitutions: Vec<DynamicSubstitution>,
    expressions_to_substitute: E,
) -> (HasExploded, Context, E) {
    let original_state = state;
    let n = substitutions.len();

    let mut substitutions = substitutions;
    let mut context = original_state.context.clone();
    let mut state = State {
        context: &mut context,
        registry: original_state.registry,
        equality_checker: original_state.equality_checker,
        warnings: original_state.warnings,
    };
    let mut expressions_to_substitute = expressions_to_substitute;
    let mut has_exploded = HasExploded(false);

    for i in 0..n {
        let dynamic_substitution = substitutions[i];

        let causes_explosion =
            backfuse(&mut state, dynamic_substitution.0, dynamic_substitution.1).has_exploded;
        has_exploded.0 |= causes_explosion;

        let substitution = if let Some(substitution) =
            get_concrete_substitution(&mut state, dynamic_substitution)
        {
            substitution
        } else {
            continue;
        };

        let remaining_substitutions = &mut substitutions[i + 1..];
        loop {
            let mut was_no_op = WasSyntacticNoOp(true);

            expressions_to_substitute = expressions_to_substitute.map(|mut id| {
                was_no_op &=
                    id.subst_in_place_and_get_status(substitution, &mut state.without_context());
                id
            });

            was_no_op &= state.context.subst_in_place_and_get_status(
                substitution,
                &mut ContextlessState {
                    registry: state.registry,
                    equality_checker: state.equality_checker,
                    warnings: state.warnings,
                },
            );

            for remaining in remaining_substitutions.iter_mut() {
                was_no_op &= remaining.subst_in_place_and_get_status(substitution, &mut state);
            }

            if was_no_op.0 {
                break;
            }
        }
    }

    (has_exploded, context, expressions_to_substitute)
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

    if min_free_db_index_in_expression(state.registry, d.0.raw()).0
        <= min_free_db_index_in_expression(state.registry, d.1.raw()).0
    {
        Some(Substitution {
            from: d.0.raw(),
            to: d.1.raw(),
        })
    } else {
        Some(Substitution {
            from: d.1.raw(),
            to: d.0.raw(),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MinDbIndex {
    Infinity,
    Some(DbIndex),
}

impl PartialOrd for MinDbIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MinDbIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        match (*self, *other) {
            (MinDbIndex::Some(a), MinDbIndex::Some(b)) => a.cmp(&b),
            (MinDbIndex::Some(_), MinDbIndex::Infinity) => Ordering::Less,
            (MinDbIndex::Infinity, MinDbIndex::Some(_)) => Ordering::Greater,
            (MinDbIndex::Infinity, MinDbIndex::Infinity) => Ordering::Equal,
        }
    }
}

impl MinDbIndex {
    fn expect(self, message: &str) -> DbIndex {
        self.into_option().expect(message)
    }

    fn into_option(self) -> Option<DbIndex> {
        match self {
            MinDbIndex::Infinity => None,
            MinDbIndex::Some(db_index) => Some(db_index),
        }
    }
}

// TODO: Maybe cache this.
fn min_free_db_index_in_expression(registry: &NodeRegistry, id: ExpressionId) -> DbIndex {
    min_db_index_in_expression_relative_to_cutoff(registry, id, 0)
        .expect("Nothing is less than zero.")
}

fn min_db_index_in_expression_relative_to_cutoff(
    registry: &NodeRegistry,
    id: ExpressionId,
    cutoff: usize,
) -> MinDbIndex {
    match id {
        ExpressionId::Name(id) => min_db_index_in_name_relative_to_cutoff(registry, id, cutoff),
        ExpressionId::Call(id) => min_db_index_in_call_relative_to_cutoff(registry, id, cutoff),
        ExpressionId::Fun(id) => min_db_index_in_fun_relative_to_cutoff(registry, id, cutoff),
        ExpressionId::Match(id) => min_db_index_in_match_relative_to_cutoff(registry, id, cutoff),
        ExpressionId::Forall(id) => min_db_index_in_forall_relative_to_cutoff(registry, id, cutoff),
        ExpressionId::Check(id) => min_db_index_in_check_relative_to_cutoff(registry, id, cutoff),
    }
}

fn min_db_index_in_name_relative_to_cutoff(
    registry: &NodeRegistry,
    id: NodeId<NameExpression>,
    cutoff: usize,
) -> MinDbIndex {
    let original = registry.name_expression(id).db_index;
    match original.0.checked_sub(cutoff) {
        Some(relative) => MinDbIndex::Some(DbIndex(relative)),
        None => MinDbIndex::Infinity,
    }
}

fn min_db_index_in_call_relative_to_cutoff(
    registry: &NodeRegistry,
    id: NodeId<Call>,
    cutoff: usize,
) -> MinDbIndex {
    let call = registry.call(id);
    let callee_min =
        min_db_index_in_expression_relative_to_cutoff(registry, call.callee_id, cutoff);
    let arg_ids = registry.expression_list(call.arg_list_id);
    let args_min = arg_ids
        .iter()
        .map(|&arg_id| min_db_index_in_expression_relative_to_cutoff(registry, arg_id, cutoff))
        .min();
    min_or_first(callee_min, args_min)
}

fn min_db_index_in_fun_relative_to_cutoff(
    registry: &NodeRegistry,
    id: NodeId<Fun>,
    cutoff: usize,
) -> MinDbIndex {
    let fun = registry.fun(id);
    let param_ids = registry.param_list(fun.param_list_id);
    let param_types_min = param_ids
        .iter()
        .copied()
        .enumerate()
        .map(|(param_index, param_id)| {
            let param = registry.param(param_id);
            min_db_index_in_expression_relative_to_cutoff(
                registry,
                param.type_id,
                cutoff + param_index,
            )
        })
        .min();
    let return_type_min = min_db_index_in_expression_relative_to_cutoff(
        registry,
        fun.return_type_id,
        cutoff + param_ids.len(),
    );
    let body_min = min_db_index_in_expression_relative_to_cutoff(
        registry,
        fun.body_id,
        cutoff + param_ids.len() + 1,
    );
    min_or_first(return_type_min.min(body_min), param_types_min)
}

fn min_db_index_in_match_relative_to_cutoff(
    registry: &NodeRegistry,
    id: NodeId<Match>,
    cutoff: usize,
) -> MinDbIndex {
    let match_ = registry.match_(id);
    let matchee_min =
        min_db_index_in_expression_relative_to_cutoff(registry, match_.matchee_id, cutoff);
    let case_ids = registry.match_case_list(match_.case_list_id);
    let case_outputs_min = case_ids
        .iter()
        .map(|case_id| {
            let case = registry.match_case(*case_id);
            min_db_index_in_expression_relative_to_cutoff(
                registry,
                case.output_id,
                cutoff + case.param_list_id.len,
            )
        })
        .min();
    min_or_first(matchee_min, case_outputs_min)
}

fn min_db_index_in_forall_relative_to_cutoff(
    registry: &NodeRegistry,
    id: NodeId<Forall>,
    cutoff: usize,
) -> MinDbIndex {
    let forall = registry.forall(id);
    let param_ids = registry.param_list(forall.param_list_id);
    let param_types_min = param_ids
        .iter()
        .copied()
        .enumerate()
        .map(|(param_index, param_id)| {
            let param = registry.param(param_id);
            min_db_index_in_expression_relative_to_cutoff(
                registry,
                param.type_id,
                cutoff + param_index,
            )
        })
        .min();
    let output_min = min_db_index_in_expression_relative_to_cutoff(
        registry,
        forall.output_id,
        cutoff + param_ids.len(),
    );
    min_or_first(output_min, param_types_min)
}

fn min_db_index_in_check_relative_to_cutoff(
    registry: &NodeRegistry,
    id: NodeId<Check>,
    cutoff: usize,
) -> MinDbIndex {
    let check = registry.check(id).clone();
    let annotation_min = min_db_index_in_checkee_annotation_relative_to_cutoff(
        registry,
        check.checkee_annotation_id,
        cutoff,
    );
    let output_min =
        min_db_index_in_expression_relative_to_cutoff(registry, check.output_id, cutoff);
    annotation_min.min(output_min)
}

fn min_db_index_in_checkee_annotation_relative_to_cutoff(
    registry: &NodeRegistry,
    id: CheckeeAnnotationId,
    cutoff: usize,
) -> MinDbIndex {
    match id {
        CheckeeAnnotationId::Goal(id) => {
            min_db_index_in_goal_checkee_annotation_relative_to_cutoff(registry, id, cutoff)
        }
        CheckeeAnnotationId::Expression(id) => {
            min_db_index_in_expression_checkee_annotation_relative_to_cutoff(registry, id, cutoff)
        }
    }
}

fn min_db_index_in_goal_checkee_annotation_relative_to_cutoff(
    registry: &NodeRegistry,
    id: NodeId<GoalCheckeeAnnotation>,
    cutoff: usize,
) -> MinDbIndex {
    let annotation = registry.goal_checkee_annotation(id);
    min_db_index_in_question_mark_or_possibly_invalid_expression_relative_to_cutoff(
        registry,
        annotation.checkee_type_id,
        cutoff,
    )
}

fn min_db_index_in_expression_checkee_annotation_relative_to_cutoff(
    registry: &NodeRegistry,
    id: NodeId<ExpressionCheckeeAnnotation>,
    cutoff: usize,
) -> MinDbIndex {
    let annotation = registry.expression_checkee_annotation(id);
    let checkee_min =
        min_db_index_in_expression_relative_to_cutoff(registry, annotation.checkee_id, cutoff);
    let type_min = min_db_index_in_question_mark_or_possibly_invalid_expression_relative_to_cutoff(
        registry,
        annotation.checkee_type_id,
        cutoff,
    );
    let value_min = if let Some(value_id) = annotation.checkee_value_id {
        min_db_index_in_question_mark_or_possibly_invalid_expression_relative_to_cutoff(
            registry, value_id, cutoff,
        )
    } else {
        MinDbIndex::Infinity
    };
    checkee_min.min(type_min).min(value_min)
}

fn min_db_index_in_question_mark_or_possibly_invalid_expression_relative_to_cutoff(
    registry: &NodeRegistry,
    id: QuestionMarkOrPossiblyInvalidExpressionId,
    cutoff: usize,
) -> MinDbIndex {
    match id {
        QuestionMarkOrPossiblyInvalidExpressionId::QuestionMark { start: _ } => {
            MinDbIndex::Infinity
        }
        QuestionMarkOrPossiblyInvalidExpressionId::Expression(id) => {
            min_db_index_in_possibly_invalid_expression_relative_to_cutoff(registry, id, cutoff)
        }
    }
}

fn min_db_index_in_possibly_invalid_expression_relative_to_cutoff(
    registry: &NodeRegistry,
    id: PossiblyInvalidExpressionId,
    cutoff: usize,
) -> MinDbIndex {
    match id {
        PossiblyInvalidExpressionId::Invalid(_) => MinDbIndex::Infinity,
        PossiblyInvalidExpressionId::Valid(id) => {
            min_db_index_in_expression_relative_to_cutoff(registry, id, cutoff)
        }
    }
}

fn min_or_first<T: Ord>(first: T, second: Option<T>) -> T {
    match second {
        Some(second) => first.min(second),
        None => first,
    }
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

impl SubstituteInPlaceAndGetNoOpStatus for ExpressionId {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ForwardReferencingSubstitution(pub Substitution);

pub(super) fn apply_forward_referencing_substitution<E: Map<ExpressionId, Output = E>>(
    state: &mut State,
    substitution: ForwardReferencingSubstitution,
    num_of_forward_references: usize,
    expressions_to_substitute: E,
) -> (Context, Bishift, E) {
    let min_db_index = min_free_db_index_in_expression(state.registry, substitution.0.from);

    let context = {
        let mut c = state.context.clone();
        c.push_top_n_down(
            num_of_forward_references,
            min_db_index,
            &mut state.without_context(),
        );
        c
    };
    let bishift = Bishift {
        len: num_of_forward_references,
        pivot: min_db_index,
    };
    let substitution = substitution
        .try_shift_with_cutoff(bishift, 0, state.registry)
        .expect("Should be able to bishift substitution");
    let expressions_to_substitute = expressions_to_substitute.map(|e| {
        e.try_shift_with_cutoff(bishift, 0, state.registry)
            .expect("Should be able to bishift expression")
    });

    let expressions_to_substitute = expressions_to_substitute.map(|e| e);

    let context = {
        let mut c = context;
        c.subst_in_place_and_get_status(substitution.0, &mut state.without_context());
        c
    };
    let expressions_to_substitute =
        expressions_to_substitute.map(|e| e.subst(substitution.0, &mut state.without_context()));

    (context, bishift, expressions_to_substitute)
}

impl ShiftDbIndices for ForwardReferencingSubstitution {
    type Output = Self;

    fn try_shift_with_cutoff<A: ShiftFn>(
        self,
        amount: A,
        cutoff: usize,
        registry: &mut NodeRegistry,
    ) -> Result<Self::Output, A::ShiftError> {
        Ok(Self(
            self.0.try_shift_with_cutoff(amount, cutoff, registry)?,
        ))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bishift {
    pub len: usize,
    pub pivot: DbIndex,
}

impl Bishift {
    pub fn distance(self) -> usize {
        self.pivot.0 - self.len
    }

    pub fn inverse(self) -> Self {
        Self {
            len: self.distance(),
            pivot: self.pivot,
        }
    }
}

impl ShiftFn for Bishift {
    type ShiftError = DbIndexTooSmallForDownshiftError;
    fn try_apply(
        &self,
        i: DbIndex,
        cutoff: usize,
    ) -> Result<DbIndex, DbIndexTooSmallForDownshiftError> {
        if (0..cutoff).contains(&i.0) {
            Ok(i)
        } else if (cutoff..cutoff + self.len).contains(&i.0) {
            Ok(Upshift(self.distance()).try_apply(i, cutoff).safe_unwrap())
        } else if (cutoff + self.len..cutoff + self.pivot.0).contains(&i.0) {
            Downshift(self.len).try_apply(i, cutoff)
        } else {
            // Indices equal to or greater than the pivot are left as-is.
            Ok(i)
        }
    }
}

pub(super) fn evaluate_well_typed_expressions<E: Map<ExpressionId, NormalFormId>>(
    state: &mut State,
    expressions: E,
) -> E::Output {
    expressions.map(|e| evaluate_well_typed_expression(state, e))
}
