use crate::data::{
    identifier_to_symbol_map::{IdentifierToSymbolMap, SymbolId},
    node_registry::{NodeId, NodeRegistry},
    registered_ast::*,
};
use rustc_hash::FxHashMap;

#[derive(Clone, Debug)]
pub enum BindError {
    CircularFileDependency(CircularFileDependencyError),
}

#[derive(Clone, Debug)]
pub struct CircularFileDependencyError {
    pub ids: Vec<NodeId<File>>,
}

impl From<CircularFileDependencyError> for BindError {
    fn from(error: CircularFileDependencyError) -> Self {
        Self::CircularFileDependency(error)
    }
}

pub fn bind_symbols_to_identifiers(
    registry: &NodeRegistry,
    file_node_ids: Vec<NodeId<File>>,
) -> Result<IdentifierToSymbolMap, BindError> {
    let file_node_ids = sort_by_dependencies(registry, file_node_ids)?;
    let mut map = IdentifierToSymbolMap::empty();

    for file_node_id in file_node_ids {
        let file = registry.file(file_node_id);
        bind_file((&registry, &mut map), file)?;
    }

    Ok(map)
}

fn sort_by_dependencies(
    _registry: &NodeRegistry,
    file_node_ids: Vec<NodeId<File>>,
) -> Result<Vec<NodeId<File>>, CircularFileDependencyError> {
    // TODO (distant): Actually sort, once we support `use` statements.
    Ok(file_node_ids)
}

fn bind_file(
    (registry, map_builder): (&NodeRegistry, &mut IdentifierToSymbolMap),
    file: &File,
) -> Result<(), BindError> {
    let mut bind_state = BindState {
        registry,
        map_builder,
        context: Context::empty(),
    };
    for item in &file.items {
        match item {
            FileItem::Type(type_statement) => bind_type_statement(&mut bind_state, type_statement)?,
            FileItem::Let(let_statement) => bind_let_statement(&mut bind_state, let_statement)?,
        }
    }
    Ok(())
}

fn bind_type_statement(
    bind_state: &mut BindState,
    type_statement: &TypeStatement,
) -> Result<(), BindError> {
    unimplemented!();
}

fn bind_let_statement(
    bind_state: &mut BindState,
    let_statement: &LetStatement,
) -> Result<(), BindError> {
    unimplemented!()
}

#[derive(Debug)]
struct BindState<'a> {
    registry: &'a NodeRegistry,
    map_builder: &'a mut IdentifierToSymbolMap,
    context: Context,
}

use context::*;
mod context {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct Context {
        stack: Vec<ContextFrame>,
        lowest_available_symbol_id: SymbolId,
    }

    impl Context {
        pub fn empty() -> Self {
            Context {
                stack: vec![ContextFrame::empty()],
                lowest_available_symbol_id: SymbolId(0),
            }
        }
    }

    #[derive(Clone, Debug)]
    struct ContextFrame {
        map: FxHashMap<String, SymbolId>,
    }

    impl ContextFrame {
        fn empty() -> Self {
            ContextFrame {
                map: FxHashMap::default(),
            }
        }
    }
}
