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
        .map(|unregistered_item| match unregistered_item {
            ur::FileItem::Type(unregistered_type_statement) => {
                let type_statement_id =
                    register_type_statement(registry, unregistered_type_statement);
                let type_statement = registry.type_statement(type_statement_id);
                FileItem::Type(type_statement.clone())
            }
            ur::FileItem::Let(unregistered_let_statement) => {
                let let_statement_id = register_let_statement(registry, unregistered_let_statement);
                let let_statement = registry.let_statement(let_statement_id);
                FileItem::Let(let_statement.clone())
            }
        })
        .collect();
    let file = File {
        id: dummy_id(),
        file_id: unregistered.id,
        items,
    };
    registry.add_file_and_overwrite_its_id(file)
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
        .map(|unregistered_param| {
            let id = register_param(registry, unregistered_param);
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
    let type_statement = TypeStatement {
        id: dummy_id(),
        name,
        params,
        constructors,
    };
    registry.add_type_statement_and_overwrite_its_id(type_statement)
}

pub fn register_identifier(
    registry: &mut NodeRegistry,
    unregistered: ur::Identifier,
) -> NodeId<Identifier> {
    let identifier = Identifier {
        id: dummy_id(),
        start: unregistered.start,
        content: unregistered.content,
    };
    registry.add_identifier_and_overwrite_its_id(identifier)
}

pub fn register_param(registry: &mut NodeRegistry, unregistered: ur::Param) -> NodeId<Param> {
    let name_id = register_identifier(registry, unregistered.name);
    let name = registry.identifier(name_id).clone();
    let type_id = register_expression(registry, unregistered.type_);
    let type_ = registry.wrapped_expression(type_id).clone();
    let param = Param {
        id: dummy_id(),
        name,
        type_,
    };
    registry.add_param_and_overwrite_its_id(param)
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
        .map(|unregistered_param| {
            let id = register_param(registry, unregistered_param);
            registry.param(id).clone()
        })
        .collect();
    let return_type_id = register_expression(registry, unregistered.return_type);
    let return_type = registry.wrapped_expression(return_type_id).clone();
    let constructor = Constructor {
        id: dummy_id(),
        name,
        params,
        return_type,
    };
    registry.add_constructor_and_overwrite_its_id(constructor)
}

pub fn register_let_statement(
    registry: &mut NodeRegistry,
    unregistered: ur::LetStatement,
) -> NodeId<LetStatement> {
    let name_id = register_identifier(registry, unregistered.name);
    let name = registry.identifier(name_id).clone();
    let value_id = register_expression(registry, unregistered.value);
    let value = registry.wrapped_expression(value_id).clone();
    let let_statement = LetStatement {
        id: dummy_id(),
        name,
        value,
    };
    registry.add_let_statement_and_overwrite_its_id(let_statement)
}

pub fn register_expression(
    registry: &mut NodeRegistry,
    unregistered: ur::Expression,
) -> NodeId<WrappedExpression> {
    unimplemented!()
}
