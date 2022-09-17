use crate::data::{
    node_registry::{NodeId, NodeRegistry},
    registered_ast::*,
    unregistered_ast as ur,
};

fn dummy_id<T>() -> NodeId<T> {
    NodeId::new(0)
}

pub fn register_file(registry: &mut NodeRegistry, unregistered: ur::File) -> NodeId<File> {
    let mut items = Vec::with_capacity(unregistered.items.len());
    for unregistered_item in unregistered.items {
        match unregistered_item {
            ur::FileItem::Type(unregistered_type_statement) => {
                let type_statement_id =
                    register_type_statement(registry, unregistered_type_statement);
                let type_statement = registry.type_statement(type_statement_id);
                items.push(FileItem::Type(type_statement.clone()));
            }
            ur::FileItem::Let(unregistered_let_statement) => {
                let let_statement_id = register_let_statement(registry, unregistered_let_statement);
                let let_statement = registry.let_statement(let_statement_id);
                items.push(FileItem::Let(let_statement.clone()));
            }
        }
    }
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
    unimplemented!()
}

pub fn register_let_statement(
    registry: &mut NodeRegistry,
    unregistered: ur::LetStatement,
) -> NodeId<LetStatement> {
    unimplemented!()
}
