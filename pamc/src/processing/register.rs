use crate::data::{
    node_registry::{NodeId, NodeRegistry},
    registered_ast::*,
    unregistered_ast as ur,
};

fn dummy_id<T>() -> NodeId<T> {
    NodeId::new(0)
}

pub fn register_file(registry: &mut NodeRegistry, unregistered: ur::File) -> NodeId<File> {
    let items = unregistered
        .items
        .into_iter()
        .map(|unregistered| match unregistered {
            ur::FileItem::Type(unregistered_type_statement) => {
                let id = register_type_statement(registry, unregistered_type_statement);
                let registered = registry.type_statement(id);
                FileItem::Type(registered.clone())
            }
            ur::FileItem::Let(unregistered_let_statement) => {
                let id = register_let_statement(registry, unregistered_let_statement);
                let registered = registry.let_statement(id);
                FileItem::Let(registered.clone())
            }
        })
        .collect();
    registry.add_file_and_overwrite_its_id(File {
        id: dummy_id(),
        file_id: unregistered.id,
        items,
    })
}

pub fn register_type_statement(
    registry: &mut NodeRegistry,
    unregistered: ur::TypeStatement,
) -> NodeId<TypeStatement> {
    let name_id = register_identifier(registry, unregistered.name);
    let name = registry.identifier(name_id).clone();
    let params = unregistered
        .params
        .into_iter()
        .map(|unregistered| {
            let id = register_param(registry, unregistered);
            registry.param(id).clone()
        })
        .collect();
    let constructors = unregistered
        .constructors
        .into_iter()
        .map(|unregistered_constructor| {
            let id = register_constructor(registry, unregistered_constructor);
            registry.constructor(id).clone()
        })
        .collect();
    registry.add_type_statement_and_overwrite_its_id(TypeStatement {
        id: dummy_id(),
        name,
        params,
        constructors,
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
    let name = registry.identifier(name_id).clone();
    let type_id = register_expression(registry, unregistered.type_);
    let type_ = registry.wrapped_expression(type_id).clone();
    registry.add_param_and_overwrite_its_id(Param {
        id: dummy_id(),
        name,
        type_,
    })
}

pub fn register_constructor(
    registry: &mut NodeRegistry,
    unregistered: ur::Constructor,
) -> NodeId<Constructor> {
    let name_id = register_identifier(registry, unregistered.name);
    let name = registry.identifier(name_id).clone();
    let params = unregistered
        .params
        .into_iter()
        .map(|unregistered| {
            let id = register_param(registry, unregistered);
            registry.param(id).clone()
        })
        .collect();
    let return_type_id = register_expression(registry, unregistered.return_type);
    let return_type = registry.wrapped_expression(return_type_id).clone();
    registry.add_constructor_and_overwrite_its_id(Constructor {
        id: dummy_id(),
        name,
        params,
        return_type,
    })
}

pub fn register_let_statement(
    registry: &mut NodeRegistry,
    unregistered: ur::LetStatement,
) -> NodeId<LetStatement> {
    let name_id = register_identifier(registry, unregistered.name);
    let name = registry.identifier(name_id).clone();
    let value_id = register_expression(registry, unregistered.value);
    let value = registry.wrapped_expression(value_id).clone();
    registry.add_let_statement_and_overwrite_its_id(LetStatement {
        id: dummy_id(),
        name,
        value,
    })
}

pub fn register_expression(
    registry: &mut NodeRegistry,
    unregistered: ur::Expression,
) -> NodeId<WrappedExpression> {
    let expression = match unregistered {
        ur::Expression::Identifier(unregistered) => {
            let id = register_identifier(registry, unregistered);
            let registered = registry.identifier(id);
            Expression::Identifier(registered.clone())
        }
        ur::Expression::Dot(unregistered) => {
            let id = register_dot(registry, *unregistered);
            let registered = registry.dot(id).clone();
            Expression::Dot(Box::new(registered))
        }
        ur::Expression::Call(unregistered) => {
            let id = register_call(registry, *unregistered);
            let registered = registry.call(id).clone();
            Expression::Call(Box::new(registered))
        }
        ur::Expression::Fun(unregistered) => {
            let id = register_fun(registry, *unregistered);
            let registered = registry.fun(id).clone();
            Expression::Fun(Box::new(registered))
        }
        ur::Expression::Match(unregistered) => {
            let id = register_match(registry, *unregistered);
            let registered = registry.match_(id).clone();
            Expression::Match(Box::new(registered))
        }
        ur::Expression::Forall(unregistered) => {
            let id = register_forall(registry, *unregistered);
            let registered = registry.forall(id).clone();
            Expression::Forall(Box::new(registered))
        }
    };
    registry.add_wrapped_expression_and_overwrite_its_id(WrappedExpression {
        id: dummy_id(),
        expression,
    })
}

pub fn register_dot(registry: &mut NodeRegistry, unregistered: ur::Dot) -> NodeId<Dot> {
    let left_id = register_expression(registry, unregistered.left);
    let left = registry.wrapped_expression(left_id).clone();
    let right_id = register_identifier(registry, unregistered.right);
    let right = registry.identifier(right_id).clone();
    registry.add_dot_and_overwrite_its_id(Dot {
        id: dummy_id(),
        left,
        right,
    })
}

pub fn register_call(registry: &mut NodeRegistry, unregistered: ur::Call) -> NodeId<Call> {
    let callee_id = register_expression(registry, unregistered.callee);
    let callee = registry.wrapped_expression(callee_id).clone();
    let args = unregistered
        .args
        .into_iter()
        .map(|unregistered| {
            let id = register_expression(registry, unregistered);
            registry.wrapped_expression(id).clone()
        })
        .collect();
    registry.add_call_and_overwrite_its_id(Call {
        id: dummy_id(),
        callee,
        args,
    })
}

pub fn register_fun(registry: &mut NodeRegistry, unregistered: ur::Fun) -> NodeId<Fun> {
    let name_id = register_identifier(registry, unregistered.name);
    let name = registry.identifier(name_id).clone();
    let params = unregistered
        .params
        .into_iter()
        .map(|unregistered| {
            let id = register_param(registry, unregistered);
            registry.param(id).clone()
        })
        .collect();
    let return_type_id = register_expression(registry, unregistered.return_type);
    let return_type = registry.wrapped_expression(return_type_id).clone();
    let return_value_id = register_expression(registry, unregistered.return_value);
    let return_value = registry.wrapped_expression(return_value_id).clone();
    registry.add_fun_and_overwrite_its_id(Fun {
        id: dummy_id(),
        name,
        params,
        return_type,
        return_value,
    })
}

pub fn register_match(registry: &mut NodeRegistry, unregistered: ur::Match) -> NodeId<Match> {
    let matchee_id = register_expression(registry, unregistered.matchee);
    let matchee = registry.wrapped_expression(matchee_id).clone();
    let cases = unregistered
        .cases
        .into_iter()
        .map(|unregistered| {
            let id = register_match_case(registry, unregistered);
            registry.match_case(id).clone()
        })
        .collect();
    registry.add_match_and_overwrite_its_id(Match {
        id: dummy_id(),
        matchee,
        cases,
    })
}

pub fn register_forall(registry: &mut NodeRegistry, unregistered: ur::Forall) -> NodeId<Forall> {
    let params = unregistered
        .params
        .into_iter()
        .map(|unregistered| {
            let id = register_param(registry, unregistered);
            registry.param(id).clone()
        })
        .collect();
    let output_id = register_expression(registry, unregistered.output);
    let output = registry.wrapped_expression(output_id).clone();
    registry.add_forall_and_overwrite_its_id(Forall {
        id: dummy_id(),
        params,
        output,
    })
}

pub fn register_match_case(
    registry: &mut NodeRegistry,
    unregistered: ur::MatchCase,
) -> NodeId<MatchCase> {
    let constructor_name_id = register_identifier(registry, unregistered.constructor_name);
    let constructor_name = registry.identifier(constructor_name_id).clone();
    let params = unregistered
        .params
        .into_iter()
        .map(|unregistered| {
            let id = register_identifier(registry, unregistered);
            registry.identifier(id).clone()
        })
        .collect();
    let output_id = register_expression(registry, unregistered.output);
    let output = registry.wrapped_expression(output_id).clone();
    registry.add_match_case_and_overwrite_its_id(MatchCase {
        id: dummy_id(),
        constructor_name,
        params,
        output,
    })
}
