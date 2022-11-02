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
