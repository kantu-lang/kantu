use crate::data::{
    identifier_to_symbol_map::{IdentifierToSymbolMap, Symbol},
    node_registry::{NodeId, NodeRegistry},
    registered_ast::*,
};
use rustc_hash::FxHashMap;

#[derive(Clone, Debug)]
pub enum BindError {
    CircularFileDependency(CircularFileDependencyError),
    NameClash(NameClashError),
    NameNotFound(NameNotFoundError),
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

#[derive(Clone, Debug)]
pub struct NameClashError {
    pub old: Identifier,
    pub new: Identifier,
}

impl From<NameClashError> for BindError {
    fn from(error: NameClashError) -> Self {
        Self::NameClash(error)
    }
}

#[derive(Clone, Debug)]
pub struct NameNotFoundError {
    pub name: IdentifierName,
}

impl From<NameNotFoundError> for BindError {
    fn from(error: NameNotFoundError) -> Self {
        Self::NameNotFound(error)
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
    (registry, map): (&NodeRegistry, &mut IdentifierToSymbolMap),
    file: &File,
) -> Result<(), BindError> {
    let mut bind_state = BindState {
        registry,
        map,
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
    define_symbol_and_bind_to_identifier(bind_state, &type_statement.name);

    bind_state.context.push_frame();
    for param in &type_statement.params {
        bind_param(bind_state, param)?;
    }
    bind_state.context.pop_frame();

    bind_state.context.push_frame();
    for constructor in &type_statement.constructors {
        bind_constructor(bind_state, constructor)?;
    }
    bind_state.context.pop_frame();

    Ok(())
}

fn bind_param(bind_state: &mut BindState, param: &Param) -> Result<(), BindError> {
    bind_expression(bind_state, &param.type_);
    define_symbol_and_bind_to_identifier(bind_state, &param.name);
    Ok(())
}

fn bind_constructor(
    bind_state: &mut BindState,
    constructor: &Constructor,
) -> Result<(), BindError> {
    // TODO: Register as constructor of parent in caller of this function.

    bind_state.context.push_frame();
    for param in &constructor.params {
        bind_param(bind_state, param)?;
    }
    bind_expression(bind_state, &constructor.return_type);
    bind_state.context.pop_frame();

    Ok(())
}

fn bind_let_statement(
    bind_state: &mut BindState,
    let_statement: &LetStatement,
) -> Result<(), BindError> {
    bind_expression(bind_state, &let_statement.value)?;
    define_symbol_and_bind_to_identifier(bind_state, &let_statement.name);
    Ok(())
}

fn bind_expression(
    bind_state: &mut BindState,
    expression: &WrappedExpression,
) -> Result<(), BindError> {
    match &expression.expression {
        Expression::Identifier(identifier) => bind_identifier(bind_state, identifier),
        Expression::Dot(dot) => bind_dot(bind_state, dot),
        Expression::Call(call) => bind_call(bind_state, call),
        Expression::Fun(fun) => bind_fun(bind_state, fun),
        Expression::Match(match_) => bind_match(bind_state, match_),
        Expression::Forall(forall) => bind_forall(bind_state, forall),
    }

    Ok(())
}

fn bind_identifier(bind_state: &mut BindState, identifier: &Identifier) -> Result<(), BindError> {
    lookup_symbol_and_bind_to_identifier(bind_state, identifier)?;
    Ok(())
}

fn bind_dot(bind_state: &mut BindState, dot: &Dot) -> Result<(), BindError> {
    unimplemented!()
}

fn bind_call(bind_state: &mut BindState, call: &Call) -> Result<(), BindError> {
    bind_expression(bind_state, &call.callee)?;
    for arg in &call.args {
        bind_expression(bind_state, arg)?;
    }
    Ok(())
}

fn bind_fun(bind_state: &mut BindState, fun: &Fun) -> Result<(), BindError> {
    bind_state.context.push_frame();
    define_symbol_and_bind_to_identifier(bind_state, &fun.name);
    for param in &fun.params {
        bind_param(bind_state, param)?;
    }
    bind_expression(bind_state, &fun.return_type)?;
    bind_expression(bind_state, &fun.return_value)?;
    bind_state.context.pop_frame();
    Ok(())
}

fn bind_match(bind_state: &mut BindState, match_: &Match) -> Result<(), BindError> {
    bind_expression(bind_state, &match_.matchee)?;
    for case in &match_.cases {
        bind_match_case(bind_state, case)?;
    }
    Ok(())
}

fn bind_match_case(bind_state: &mut BindState, case: &MatchCase) -> Result<(), BindError> {
    bind_state.context.push_frame();
    // TODO
    bind_state.context.pop_frame();
}

#[derive(Debug)]
struct BindState<'a> {
    registry: &'a NodeRegistry,
    map: &'a mut IdentifierToSymbolMap,
    context: Context,
}

fn define_symbol_and_bind_to_identifier(
    bind_state: &mut BindState,
    identifier: &Identifier,
) -> Result<Symbol, BindError> {
    if bind_state.map.contains(identifier.id) {
        panic!("Impossible: Tried to assign symbol to identifier that already had a symbol assigned to it.");
    }
    let name_symbol = bind_state.context.add(identifier)?;
    bind_state.map.insert(identifier.id, name_symbol);
    Ok(name_symbol)
}

fn lookup_symbol_and_bind_to_identifier(
    bind_state: &mut BindState,
    identifier: &Identifier,
) -> Result<Symbol, BindError> {
    if bind_state.map.contains(identifier.id) {
        panic!("Impossible: Tried to assign symbol to identifier that already had a symbol assigned to it.");
    }
    let name_symbol = bind_state.context.lookup(identifier)?;
    bind_state.map.insert(identifier.id, name_symbol);
    Ok(name_symbol)
}

use context::*;
mod context {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct Context {
        stack: Vec<FxHashMap<IdentifierName, (Identifier, Symbol)>>,
        lowest_available_symbol_id: Symbol,
    }

    impl Context {
        pub fn empty() -> Self {
            Context {
                stack: vec![FxHashMap::default()],
                lowest_available_symbol_id: Symbol(0),
            }
        }
    }

    impl Context {
        pub fn add(&mut self, identifier: &Identifier) -> Result<Symbol, NameClashError> {
            let existing_symbol: Option<&(Identifier, Symbol)> = self
                .stack
                .iter()
                .find_map(|frame| frame.get(&identifier.name));
            if let Some(existing_symbol) = existing_symbol {
                return Err(NameClashError {
                    old: existing_symbol.0.clone(),
                    new: identifier.clone().into(),
                });
            }
            let symbol = self.lowest_available_symbol_id;
            self.lowest_available_symbol_id.0 += 1;
            Ok(symbol)
        }

        pub fn lookup(&self, identifier: &Identifier) -> Result<Symbol, NameNotFoundError> {
            let existing_symbol: Option<&(Identifier, Symbol)> = self
                .stack
                .iter()
                .find_map(|frame| frame.get(&identifier.name));
            if let Some(existing_symbol) = existing_symbol {
                Ok(existing_symbol.1)
            } else {
                Err(NameNotFoundError {
                    name: identifier.name.clone(),
                })
            }
        }

        pub fn push_frame(&mut self) {
            self.stack.push(FxHashMap::default());
        }

        pub fn pop_frame(&mut self) {
            self.stack.pop();
        }
    }
}
