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

pub fn type0_expression(context: &Context, registry: &mut NodeRegistry) -> NormalFormId {
    let name_id = add_name_expression_and_overwrite_component_ids(
        registry,
        vec![Identifier {
            id: dummy_id(),
            name: IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase),
            start: None,
        }],
        context.type0_dbi(),
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

pub fn is_term_equal_to_type0_or_type1(
    context: &Context,
    registry: &NodeRegistry,
    term: NormalFormId,
) -> bool {
    if let ExpressionId::Name(name_id) = term.raw() {
        let name = registry.name_expression(name_id);
        let i = name.db_index;
        i == context.type0_dbi() || i == context.type1_dbi()
    } else {
        false
    }
}

pub fn is_left_type_assignable_to_right_type(
    _context: &Context,
    _registry: &NodeRegistry,
    _left: NormalFormId,
    _right: NormalFormId,
) -> bool {
    unimplemented!()
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

pub fn normalize_params(
    context: &mut Context,
    registry: &mut NodeRegistry,
    param_list_id: ListId<NodeId<Param>>,
) -> Result<ListId<NodeId<Param>>, TypeCheckError> {
    let normalized_list_id =
        normalize_params_and_leave_params_in_context(context, registry, param_list_id)?;
    context.pop_n(param_list_id.len);
    Ok(normalized_list_id)
}

pub fn normalize_params_and_leave_params_in_context(
    context: &mut Context,
    registry: &mut NodeRegistry,
    param_list_id: ListId<NodeId<Param>>,
) -> Result<ListId<NodeId<Param>>, TypeCheckError> {
    let param_ids = registry.param_list(param_list_id).to_vec();
    let normalized_ids = param_ids
        .iter()
        .copied()
        .map(|param_id| {
            type_check_param(context, registry, param_id)?;
            let type_id: ExpressionId = context.get_type(DbIndex(0), registry).raw();
            let old_param = registry.param(param_id);
            let normalized_param_with_dummy_id = Param {
                id: dummy_id(),
                is_dashed: old_param.is_dashed,
                name_id: old_param.name_id,
                type_id,
            };
            let normalized_id =
                registry.add_param_and_overwrite_its_id(normalized_param_with_dummy_id);
            Ok(normalized_id)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(registry.add_param_list(normalized_ids))
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

pub fn get_db_index_for_adt_variant_of_name(
    context: &Context,
    registry: &mut NodeRegistry,
    adt_expression: AdtExpression,
    target_variant_name_id: NodeId<Identifier>,
) -> DbIndex {
    let type_dbi = registry
        .name_expression(adt_expression.type_name_id)
        .db_index;
    let variant_name_list_id = match context.get_definition(type_dbi, registry) {
        ContextEntryDefinition::Adt {
            variant_name_list_id,
        } => variant_name_list_id,
        _ => panic!("An ADT's NameExpression should always point to an ADT definition"),
    };

    let target_variant_name = &registry.identifier(target_variant_name_id).name;
    let variant_index = registry
        .identifier_list(variant_name_list_id)
        .iter()
        .position(|&variant_name_id| {
            let variant_name = &registry.identifier(variant_name_id).name;
            variant_name == target_variant_name
        })
        .expect("The target variant name should always be found in the ADT's variant name list");
    DbIndex(type_dbi.0 + 1 + variant_index)
}

pub fn fuse_left_to_right(
    _context: &mut Context,
    _registry: &mut NodeRegistry,
    _left: NormalFormId,
    _right: NormalFormId,
) -> Vec<Substitution> {
    unimplemented!()
}
