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
    UnbindableDotExpressionLhs(NodeId<WrappedExpression>),
    InvalidDotExpressionRhs(NodeId<Identifier>),
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
    builtin_identifiers: &[Identifier],
) -> Result<IdentifierToSymbolMap, BindError> {
    let file_node_ids = sort_by_dependencies(registry, file_node_ids)?;
    let mut bind_state = BindState {
        map: IdentifierToSymbolMap::empty(),
        context: Context::new(Symbol(0)),
        dot_targets: SymbolToDotTargetsMap::empty(),
    };

    bind_state.context.push_frame();

    for identifier in builtin_identifiers {
        bind_state
            .context
            .add(identifier)
            .expect("Error: built-in identifiers vec contains identifiers with name clash");
    }

    for file_node_id in file_node_ids {
        let file = registry.file(file_node_id);
        bind_file(&mut bind_state, file)?;
    }

    bind_state.context.pop_frame();

    Ok(bind_state.map)
}

fn sort_by_dependencies(
    _registry: &NodeRegistry,
    file_node_ids: Vec<NodeId<File>>,
) -> Result<Vec<NodeId<File>>, CircularFileDependencyError> {
    // TODO (distant): Actually sort, once we support `use` statements.
    Ok(file_node_ids)
}

fn bind_file(bind_state: &mut BindState, file: &File) -> Result<(), BindError> {
    bind_state.context.push_frame();
    for item in &file.items {
        match item {
            FileItem::Type(type_statement) => bind_type_statement(bind_state, type_statement)?,
            FileItem::Let(let_statement) => bind_let_statement(bind_state, let_statement)?,
        }
    }
    bind_state.context.pop_frame();
    Ok(())
}

fn bind_type_statement(
    bind_state: &mut BindState,
    type_statement: &TypeStatement,
) -> Result<(), BindError> {
    let name_symbol =
        define_symbol_in_context_and_bind_to_identifier(bind_state, &type_statement.name)?;

    bind_state.context.push_frame();
    for param in &type_statement.params {
        bind_param(bind_state, param)?;
    }
    bind_state.context.pop_frame();

    bind_state.context.push_frame();
    for constructor in &type_statement.constructors {
        bind_constructor(bind_state, constructor, name_symbol)?;
    }
    bind_state.context.pop_frame();

    Ok(())
}

fn bind_param(bind_state: &mut BindState, param: &Param) -> Result<(), BindError> {
    bind_expression(bind_state, &param.type_)?;
    define_symbol_in_context_and_bind_to_identifier(bind_state, &param.name)?;
    Ok(())
}

fn bind_constructor(
    bind_state: &mut BindState,
    constructor: &Constructor,
    declaring_type_name_symbol: Symbol,
) -> Result<(), BindError> {
    let constructor_symbol = bind_new_symbol_to_identifier(bind_state, &constructor.name);

    bind_state.dot_targets.insert(
        declaring_type_name_symbol,
        constructor.name.name.clone(),
        constructor_symbol,
    );

    bind_state.context.push_frame();
    for param in &constructor.params {
        bind_param(bind_state, param)?;
    }
    bind_expression(bind_state, &constructor.return_type)?;
    bind_state.context.pop_frame();

    Ok(())
}

fn bind_let_statement(
    bind_state: &mut BindState,
    let_statement: &LetStatement,
) -> Result<(), BindError> {
    bind_expression(bind_state, &let_statement.value)?;
    define_symbol_in_context_and_bind_to_identifier(bind_state, &let_statement.name)?;
    Ok(())
}

fn bind_expression(
    bind_state: &mut BindState,
    expression: &WrappedExpression,
) -> Result<(), BindError> {
    match &expression.expression {
        Expression::Identifier(identifier) => bind_identifier(bind_state, identifier)?,
        Expression::Dot(dot) => bind_dot(bind_state, dot)?,
        Expression::Call(call) => bind_call(bind_state, call)?,
        Expression::Fun(fun) => bind_fun(bind_state, fun)?,
        Expression::Match(match_) => bind_match(bind_state, match_)?,
        Expression::Forall(forall) => bind_forall(bind_state, forall)?,
    }

    Ok(())
}

fn bind_identifier(bind_state: &mut BindState, identifier: &Identifier) -> Result<(), BindError> {
    lookup_symbol_from_context_and_bind_to_identifier(bind_state, identifier)?;
    Ok(())
}

fn bind_dot(bind_state: &mut BindState, dot: &Dot) -> Result<(), BindError> {
    bind_expression(bind_state, &dot.left)?;
    let left_symbol = match &dot.left.expression {
        Expression::Identifier(identifier) => bind_state.map.get(identifier.id),
        Expression::Dot(dot) => bind_state.map.get(dot.right.id),
        _ => return Err(BindError::UnbindableDotExpressionLhs(dot.left.id)),
    };
    let right_symbol = if let Some(s) = bind_state.dot_targets.get(left_symbol, &dot.right.name) {
        s
    } else {
        return Err(BindError::InvalidDotExpressionRhs(dot.right.id));
    };
    bind_symbol_to_identifier(bind_state, right_symbol, &dot.right);
    Ok(())
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
    define_symbol_in_context_and_bind_to_identifier(bind_state, &fun.name)?;
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
    for param in &case.params {
        define_symbol_in_context_and_bind_to_identifier(bind_state, param)?;
    }
    bind_expression(bind_state, &case.output)?;
    bind_state.context.pop_frame();
    Ok(())
}

fn bind_forall(bind_state: &mut BindState, forall: &Forall) -> Result<(), BindError> {
    bind_state.context.push_frame();
    for param in &forall.params {
        bind_param(bind_state, param)?;
    }
    bind_expression(bind_state, &forall.output)?;
    bind_state.context.pop_frame();
    Ok(())
}

#[derive(Debug)]
struct BindState {
    map: IdentifierToSymbolMap,
    context: Context,
    dot_targets: SymbolToDotTargetsMap,
}

fn define_symbol_in_context_and_bind_to_identifier(
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

fn bind_new_symbol_to_identifier(bind_state: &mut BindState, identifier: &Identifier) -> Symbol {
    if bind_state.map.contains(identifier.id) {
        panic!("Impossible: Tried to assign symbol to identifier that already had a symbol assigned to it.");
    }
    let symbol = bind_state.context.new_symbol();
    bind_state.map.insert(identifier.id, symbol);
    symbol
}

fn bind_symbol_to_identifier(bind_state: &mut BindState, symbol: Symbol, identifier: &Identifier) {
    if bind_state.map.contains(identifier.id) {
        panic!("Impossible: Tried to assign symbol to identifier that already had a symbol assigned to it.");
    }
    bind_state.map.insert(identifier.id, symbol);
}

fn lookup_symbol_from_context_and_bind_to_identifier(
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
        lowest_available_symbol: Symbol,
    }

    impl Context {
        pub fn new(lowest_available_symbol: Symbol) -> Self {
            Context {
                stack: vec![],
                lowest_available_symbol,
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
            let symbol = self.new_symbol();
            self.stack
                .last_mut()
                .expect("Error: Context::add was called when the stack was empty.")
                .insert(identifier.name.clone(), (identifier.clone(), symbol));
            Ok(symbol)
        }

        pub fn new_symbol(&mut self) -> Symbol {
            let symbol = self.lowest_available_symbol;
            self.lowest_available_symbol.0 += 1;
            symbol
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

use dot_targets::*;
mod dot_targets {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct SymbolToDotTargetsMap(FxHashMap<Symbol, FxHashMap<IdentifierName, Symbol>>);

    impl SymbolToDotTargetsMap {
        pub fn empty() -> Self {
            SymbolToDotTargetsMap(FxHashMap::default())
        }
    }

    impl SymbolToDotTargetsMap {
        pub fn insert(
            &mut self,
            symbol: Symbol,
            target_name: IdentifierName,
            target_symbol: Symbol,
        ) {
            if let Some(targets) = self.0.get_mut(&symbol) {
                targets.insert(target_name, target_symbol);
            } else {
                let mut targets = FxHashMap::default();
                targets.insert(target_name, target_symbol);
                self.0.insert(symbol, targets);
            }
        }

        pub fn get(&self, symbol: Symbol, target_name: &IdentifierName) -> Option<Symbol> {
            self.0
                .get(&symbol)
                .and_then(|targets| targets.get(target_name))
                .copied()
        }
    }
}
