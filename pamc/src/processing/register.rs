use crate::data::{
    node_registry::{NodeId, NodeRegistry},
    registered_ast::*,
    unregistered_ast as ur,
};

fn dummy_id<T>() -> NodeId<T> {
    NodeId::new(0)
}

pub fn register_file(registry: &mut NodeRegistry, unregistered: ur::File) -> NodeId<File> {
    let item_ids = unregistered
        .items
        .into_iter()
        .map(|unregistered| register_file_item(registry, unregistered))
        .collect();
    let item_list_id = registry.add_file_item_list(item_ids);
    registry.add_file_and_overwrite_its_id(File {
        id: dummy_id(),
        file_id: unregistered.id,
        item_list_id,
    })
}

pub fn register_file_item(
    registry: &mut NodeRegistry,
    unregistered: ur::FileItem,
) -> FileItemNodeId {
    match unregistered {
        ur::FileItem::Type(unregistered) => {
            FileItemNodeId::Type(register_type_statement(registry, unregistered))
        }
        ur::FileItem::Let(unregistered) => {
            FileItemNodeId::Let(register_let_statement(registry, unregistered))
        }
    }
}

pub fn register_type_statement(
    registry: &mut NodeRegistry,
    unregistered: ur::TypeStatement,
) -> NodeId<TypeStatement> {
    let name_id = register_identifier(registry, unregistered.name);
    let param_ids = unregistered
        .params
        .into_iter()
        .map(|unregistered| register_param(registry, unregistered))
        .collect();
    let param_list_id = registry.add_param_list(param_ids);
    let variant_ids = unregistered
        .variants
        .into_iter()
        .map(|unregistered_variant| register_variant(registry, unregistered_variant))
        .collect();
    let variant_list_id = registry.add_variant_list(variant_ids);
    registry.add_type_statement_and_overwrite_its_id(TypeStatement {
        id: dummy_id(),
        name_id,
        param_list_id,
        variant_list_id,
    })
}

pub fn register_identifier(
    registry: &mut NodeRegistry,
    unregistered: ur::Identifier,
) -> NodeId<Identifier> {
    registry.add_identifier_and_overwrite_its_id(Identifier {
        id: dummy_id(),
        start: Some(unregistered.start),
        name: unregistered.name,
    })
}

pub fn register_param(registry: &mut NodeRegistry, unregistered: ur::Param) -> NodeId<Param> {
    let name_id = register_identifier(registry, unregistered.name);
    let type_id = register_expression(registry, unregistered.type_);
    registry.add_param_and_overwrite_its_id(Param {
        id: dummy_id(),
        is_dashed: unregistered.is_dashed,
        name_id,
        type_id,
    })
}

pub fn register_variant(registry: &mut NodeRegistry, unregistered: ur::Variant) -> NodeId<Variant> {
    let name_id = register_identifier(registry, unregistered.name);
    let param_ids = unregistered
        .params
        .into_iter()
        .map(|unregistered| register_param(registry, unregistered))
        .collect();
    let param_list_id = registry.add_param_list(param_ids);
    let return_type_id = register_expression(registry, unregistered.return_type);
    registry.add_variant_and_overwrite_its_id(Variant {
        id: dummy_id(),
        name_id,
        param_list_id,
        return_type_id,
    })
}

pub fn register_let_statement(
    registry: &mut NodeRegistry,
    unregistered: ur::LetStatement,
) -> NodeId<LetStatement> {
    let name_id = register_identifier(registry, unregistered.name);
    let value_id = register_expression(registry, unregistered.value);
    registry.add_let_statement_and_overwrite_its_id(LetStatement {
        id: dummy_id(),
        name_id,
        value_id,
    })
}

pub fn register_expression(
    registry: &mut NodeRegistry,
    unregistered: ur::Expression,
) -> ExpressionId {
    match unregistered {
        ur::Expression::Identifier(unregistered) => {
            let id = register_identifier(registry, unregistered);
            ExpressionId::Identifier(id)
        }
        ur::Expression::Dot(unregistered) => {
            let id = register_dot(registry, *unregistered);
            ExpressionId::Dot(id)
        }
        ur::Expression::Call(unregistered) => {
            let id = register_call(registry, *unregistered);
            ExpressionId::Call(id)
        }
        ur::Expression::Fun(unregistered) => {
            let id = register_fun(registry, *unregistered);
            ExpressionId::Fun(id)
        }
        ur::Expression::Match(unregistered) => {
            let id = register_match(registry, *unregistered);
            ExpressionId::Match(id)
        }
        ur::Expression::Forall(unregistered) => {
            let id = register_forall(registry, *unregistered);
            ExpressionId::Forall(id)
        }
    }
}

pub fn register_dot(registry: &mut NodeRegistry, unregistered: ur::Dot) -> NodeId<Dot> {
    let left_id = register_expression(registry, unregistered.left);
    let right_id = register_identifier(registry, unregistered.right);
    registry.add_dot_and_overwrite_its_id(Dot {
        id: dummy_id(),
        left_id,
        right_id,
    })
}

pub fn register_call(registry: &mut NodeRegistry, unregistered: ur::Call) -> NodeId<Call> {
    let callee_id = register_expression(registry, unregistered.callee);
    let arg_ids = unregistered
        .args
        .into_iter()
        .map(|unregistered| register_expression(registry, unregistered))
        .collect();
    let arg_list_id = registry.add_expression_list(arg_ids);
    registry.add_call_and_overwrite_its_id(Call {
        id: dummy_id(),
        callee_id,
        arg_list_id,
    })
}

pub fn register_fun(registry: &mut NodeRegistry, unregistered: ur::Fun) -> NodeId<Fun> {
    let name_id = register_identifier(registry, unregistered.name);
    let param_ids = unregistered
        .params
        .into_iter()
        .map(|unregistered| register_param(registry, unregistered))
        .collect();
    let param_list_id = registry.add_param_list(param_ids);
    let return_type_id = register_expression(registry, unregistered.return_type);
    let body_id = register_expression(registry, unregistered.body);
    registry.add_fun_and_overwrite_its_id(Fun {
        id: dummy_id(),
        name_id,
        param_list_id,
        return_type_id,
        body_id,
    })
}

pub fn register_match(registry: &mut NodeRegistry, unregistered: ur::Match) -> NodeId<Match> {
    let matchee_id = register_expression(registry, unregistered.matchee);
    let case_ids = unregistered
        .cases
        .into_iter()
        .map(|unregistered| register_match_case(registry, unregistered))
        .collect();
    let case_list_id = registry.add_match_case_list(case_ids);
    registry.add_match_and_overwrite_its_id(Match {
        id: dummy_id(),
        matchee_id,
        case_list_id,
    })
}

pub fn register_forall(registry: &mut NodeRegistry, unregistered: ur::Forall) -> NodeId<Forall> {
    let param_ids = unregistered
        .params
        .into_iter()
        .map(|unregistered| register_param(registry, unregistered))
        .collect();
    let param_list_id = registry.add_param_list(param_ids);
    let output_id = register_expression(registry, unregistered.output);
    registry.add_forall_and_overwrite_its_id(Forall {
        id: dummy_id(),
        param_list_id,
        output_id,
    })
}

pub fn register_match_case(
    registry: &mut NodeRegistry,
    unregistered: ur::MatchCase,
) -> NodeId<MatchCase> {
    let variant_name_id = register_identifier(registry, unregistered.variant_name);
    let param_ids = unregistered
        .params
        .into_iter()
        .map(|unregistered| register_identifier(registry, unregistered))
        .collect();
    let param_list_id = registry.add_identifier_list(param_ids);
    let output_id = register_expression(registry, unregistered.output);
    registry.add_match_case_and_overwrite_its_id(MatchCase {
        id: dummy_id(),
        variant_name_id,
        param_list_id,
        output_id,
    })
}
