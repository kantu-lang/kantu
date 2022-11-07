use super::*;

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
    is_term_equal_to_an_empty_type(state, left)
        || state
            .equality_checker
            .eq(left.raw(), right.raw(), state.registry)
}

fn is_term_equal_to_an_empty_type(state: &mut State, term_id: NormalFormId) -> bool {
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

pub(super) fn normalize_params(
    state: &mut State,
    param_list_id: ListId<NodeId<Param>>,
) -> Result<ListId<NodeId<Param>>, TypeCheckError> {
    let normalized_list_id = normalize_params_and_leave_params_in_context(state, param_list_id)?;
    state.context.pop_n(param_list_id.len);
    Ok(normalized_list_id)
}

pub(super) fn normalize_params_and_leave_params_in_context(
    state: &mut State,
    param_list_id: ListId<NodeId<Param>>,
) -> Result<ListId<NodeId<Param>>, TypeCheckError> {
    let param_ids = state.registry.param_list(param_list_id).to_vec();
    let normalized_ids = param_ids
        .iter()
        .copied()
        .map(|param_id| {
            type_check_param(state, param_id)?;
            let type_id: ExpressionId = state.context.get_type(DbIndex(0), state.registry).raw();
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
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(state.registry.add_param_list(normalized_ids))
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
    DbIndex(type_dbi.0 + 1 + variant_index)
}

#[derive(Clone, Debug)]
pub struct Fusion {
    pub has_exploded: bool,
    pub substitutions: Vec<DynamicSubstitution>,
}

#[derive(Clone, Copy, Debug)]
pub struct DynamicSubstitution(pub ExpressionId, pub ExpressionId);

impl std::ops::AddAssign<Fusion> for Fusion {
    fn add_assign(&mut self, rhs: Fusion) {
        self.has_exploded |= rhs.has_exploded;
        self.substitutions.extend(rhs.substitutions);
    }
}

pub(super) fn fuse_left_to_right(
    state: &mut State,
    left: NormalFormId,
    right: NormalFormId,
) -> Fusion {
    if let (Some(left_ve), Some(right_ve)) = (
        try_as_variant_expression(state, left),
        try_as_variant_expression(state, right),
    ) {
        let left_name: &IdentifierName = &state.registry.identifier(left_ve.0).name;
        let right_name: &IdentifierName = &state.registry.identifier(right_ve.0).name;
        if left_name == right_name {
            match (left_ve.1, right_ve.1) {
                (
                    PossibleArgListId::Some(left_arg_list_id),
                    PossibleArgListId::Some(right_arg_list_id),
                ) => {
                    let mut out = Fusion {
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
                        out += fuse_left_to_right(
                            state,
                            // This is safe because an arg to a normal
                            // form Call node is always a normal form itself.
                            NormalFormId::unchecked_new(left_arg_id),
                            NormalFormId::unchecked_new(right_arg_id),
                        );
                    }
                    out
                }
                (PossibleArgListId::Nullary, PossibleArgListId::Nullary) => Fusion {
                    has_exploded: false,
                    substitutions: vec![],
                },
                other => panic!("Invalid fusion: {:?}", other),
            }
        } else {
            Fusion {
                has_exploded: true,
                substitutions: vec![],
            }
        }
    } else {
        Fusion {
            has_exploded: false,
            substitutions: vec![DynamicSubstitution(left.raw(), right.raw())],
        }
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
            if right_param_ids.iter().any(|&right_param_id| {
                let right_param_type_id = state.registry.param(right_param_id).type_id;
                is_left_inclusive_subterm_of_right(state, left, right_param_type_id)
            }) {
                return true;
            }

            if is_left_inclusive_subterm_of_right(state, left, right.return_type_id) {
                return true;
            }

            if is_left_inclusive_subterm_of_right(state, left, right.body_id) {
                return true;
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
                let right_case_output_id = state.registry.match_case(right_case_id).output_id;
                is_left_inclusive_subterm_of_right(state, left, right_case_output_id)
            }) {
                return true;
            }

            false
        }
        ExpressionId::Forall(right_id) => {
            let right = state.registry.forall(right_id).clone();

            let right_param_ids = state.registry.param_list(right.param_list_id).to_vec();
            if right_param_ids.iter().any(|&right_param_id| {
                let right_param_type_id = state.registry.param(right_param_id).type_id;
                is_left_inclusive_subterm_of_right(state, left, right_param_type_id)
            }) {
                return true;
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
    expression_id: NormalFormId,
) -> Option<(NodeId<Identifier>, PossibleArgListId)> {
    match expression_id.raw() {
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

pub(super) fn apply_dynamic_substitutions_with_compounding(
    state: &mut State,
    substitutions: Vec<DynamicSubstitution>,
    shifted_coercion_target_id: Option<ExpressionId>,
) -> (Context, Option<ExpressionId>) {
    let original_state = state;
    let n = substitutions.len();

    let mut substitutions = substitutions;
    let mut context = original_state.context.clone();
    let mut state = State {
        context: &mut context,
        registry: original_state.registry,
        equality_checker: original_state.equality_checker,
    };
    let mut shifted_coercion_target_id = shifted_coercion_target_id;

    for i in 0..n {
        let substitution = get_concrete_substitution(&mut state, substitutions[i]);
        let remaining_substitutions = &mut substitutions[i + 1..];
        loop {
            let mut was_no_op = WasNoOp(true);

            if let Some(id) = shifted_coercion_target_id.as_mut() {
                was_no_op &= id.subst_in_place(substitution, &mut state.registry);
            }

            for remaining in remaining_substitutions.iter_mut() {
                was_no_op &= remaining.subst_in_place(substitution, &mut state.registry);
            }

            was_no_op &= state
                .context
                .subst_in_place(substitution, &mut state.registry);

            if was_no_op.0 {
                break;
            }
        }
    }

    (context, shifted_coercion_target_id)
}

fn get_concrete_substitution(_state: &mut State, _dynamic: DynamicSubstitution) -> Substitution {
    unimplemented!()
}

trait SubstituteInPlace {
    fn subst_in_place(
        &mut self,
        substitution: Substitution,
        registry: &mut NodeRegistry,
    ) -> WasNoOp;
}

impl<T> SubstituteInPlace for T
where
    T: Clone + Substitute<Output = T>,
{
    fn subst_in_place(
        &mut self,
        substitution: Substitution,
        registry: &mut NodeRegistry,
    ) -> WasNoOp {
        let (substituted, was_no_op) = self.clone().subst(substitution, registry);
        *self = substituted;
        was_no_op
    }
}
