use crate::data::{
    node_registry::{NodeId, NodeRegistry},
    registered_ast::*,
    symbol_database::{
        IdentifierToSymbolMap, Symbol, SymbolDatabase, SymbolProvider, SymbolSource,
        SymbolSourceMap, SymbolToDotTargetsMap,
    },
};

// TODO: Forbid fun return type from using the fun it declares.

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
    pub old: SymbolSource,
    pub new: SymbolSource,
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
) -> Result<SymbolDatabase, BindError> {
    let file_node_ids = sort_by_dependencies(registry, file_node_ids)?;
    let mut provider = SymbolProvider::new();
    let builtin_identifiers = get_builtin_identifiers(&mut provider);
    let mut bind_state = BindState {
        identifier_symbols: IdentifierToSymbolMap::empty(),
        dot_targets: SymbolToDotTargetsMap::empty(),
        context: Context::new(&mut provider),
        symbol_sources: SymbolSourceMap::default(),
    };

    bind_state.context.push_scope();

    for (name, source, symbol) in builtin_identifiers {
        bind_state
            .context
            .assign_symbol_and_add(&name, source, symbol)?;
    }

    for file_node_id in file_node_ids {
        let file = registry.file(file_node_id);
        bind_file(&mut bind_state, file)?;
    }

    bind_state.context.pop_scope();

    Ok(SymbolDatabase {
        identifier_symbols: bind_state.identifier_symbols,
        symbol_dot_targets: bind_state.dot_targets,
        symbol_sources: bind_state.symbol_sources,
        provider,
    })
}

fn sort_by_dependencies(
    _registry: &NodeRegistry,
    file_node_ids: Vec<NodeId<File>>,
) -> Result<Vec<NodeId<File>>, CircularFileDependencyError> {
    // TODO (distant): Actually sort, once we support `use` statements.
    Ok(file_node_ids)
}

fn get_builtin_identifiers(
    provider: &SymbolProvider,
) -> Vec<(IdentifierName, SymbolSource, Symbol)> {
    vec![(
        IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase),
        SymbolSource::BuiltinTypeTitleCase,
        provider.type0_symbol(),
    )]
}

fn bind_file(bind_state: &mut BindState, file: &File) -> Result<(), BindError> {
    bind_state.context.push_scope();
    for item in &file.items {
        match item {
            FileItem::Type(type_statement) => bind_type_statement(bind_state, type_statement)?,
            FileItem::Let(let_statement) => bind_let_statement(bind_state, let_statement)?,
        }
    }
    bind_state.context.pop_scope();
    Ok(())
}

fn bind_type_statement(
    bind_state: &mut BindState,
    type_statement: &TypeStatement,
) -> Result<(), BindError> {
    bind_state.context.push_scope();
    for param in &type_statement.params {
        bind_param(bind_state, param)?;
    }
    bind_state.context.pop_scope();

    let name_symbol = define_symbol_in_context_and_bind_to_identifier(
        bind_state,
        &type_statement.name,
        SymbolSource::Type(type_statement.id),
    )?;

    for variant in &type_statement.variants {
        bind_variant(bind_state, variant, name_symbol)?;
    }

    Ok(())
}

fn bind_param(bind_state: &mut BindState, param: &Param) -> Result<(), BindError> {
    bind_expression(bind_state, &param.type_)?;
    define_symbol_in_context_and_bind_to_identifier(
        bind_state,
        &param.name,
        SymbolSource::TypedParam(param.id),
    )?;
    Ok(())
}

fn bind_variant(
    bind_state: &mut BindState,
    variant: &Variant,
    declaring_type_name_symbol: Symbol,
) -> Result<(), BindError> {
    let variant_symbol = bind_new_symbol_to_identifier(bind_state, &variant.name);
    define_symbol_source(
        bind_state,
        variant_symbol,
        SymbolSource::Variant(variant.id),
    );

    bind_state.dot_targets.insert(
        declaring_type_name_symbol,
        variant.name.name.clone(),
        variant_symbol,
    );

    bind_state.context.push_scope();
    for param in &variant.params {
        bind_param(bind_state, param)?;
    }
    bind_expression(bind_state, &variant.return_type)?;
    bind_state.context.pop_scope();

    Ok(())
}

fn bind_let_statement(
    bind_state: &mut BindState,
    let_statement: &LetStatement,
) -> Result<(), BindError> {
    bind_expression(bind_state, &let_statement.value)?;
    define_symbol_in_context_and_bind_to_identifier(
        bind_state,
        &let_statement.name,
        SymbolSource::Let(let_statement.id),
    )?;
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
        Expression::Identifier(identifier) => bind_state.identifier_symbols.get(identifier.id),
        Expression::Dot(dot) => bind_state.identifier_symbols.get(dot.right.id),
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
    bind_state.context.push_scope();
    for param in &fun.params {
        bind_param(bind_state, param)?;
    }
    bind_expression(bind_state, &fun.return_type)?;
    define_symbol_in_context_and_bind_to_identifier(
        bind_state,
        &fun.name,
        SymbolSource::Fun(fun.id),
    )?;
    bind_expression(bind_state, &fun.body)?;
    bind_state.context.pop_scope();
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
    bind_state.context.push_scope();
    for param in &case.params {
        define_symbol_in_context_and_bind_to_identifier(
            bind_state,
            param,
            SymbolSource::UntypedParam(param.id),
        )?;
    }
    bind_expression(bind_state, &case.output)?;
    bind_state.context.pop_scope();
    Ok(())
}

fn bind_forall(bind_state: &mut BindState, forall: &Forall) -> Result<(), BindError> {
    bind_state.context.push_scope();
    for param in &forall.params {
        bind_param(bind_state, param)?;
    }
    bind_expression(bind_state, &forall.output)?;
    bind_state.context.pop_scope();
    Ok(())
}

#[derive(Debug)]
struct BindState<'a> {
    identifier_symbols: IdentifierToSymbolMap,
    dot_targets: SymbolToDotTargetsMap,
    symbol_sources: SymbolSourceMap,
    context: Context<'a>,
}

fn define_symbol_in_context_and_bind_to_identifier(
    bind_state: &mut BindState,
    identifier: &Identifier,
    source: SymbolSource,
) -> Result<Symbol, BindError> {
    let name_symbol = bind_state
        .context
        .assign_new_symbol_and_add(&identifier.name, source)?;
    bind_symbol_to_identifier(bind_state, name_symbol, identifier);
    define_symbol_source(bind_state, name_symbol, source);

    Ok(name_symbol)
}

fn bind_new_symbol_to_identifier(bind_state: &mut BindState, identifier: &Identifier) -> Symbol {
    let symbol = bind_state.context.new_symbol();
    bind_symbol_to_identifier(bind_state, symbol, identifier);
    symbol
}

fn bind_symbol_to_identifier(bind_state: &mut BindState, symbol: Symbol, identifier: &Identifier) {
    if bind_state.identifier_symbols.contains(identifier.id) {
        panic!("Impossible: Tried to assign symbol to identifier that already had a symbol assigned to it.");
    }
    bind_state.identifier_symbols.insert(identifier.id, symbol);
}

fn lookup_symbol_from_context_and_bind_to_identifier(
    bind_state: &mut BindState,
    identifier: &Identifier,
) -> Result<Symbol, BindError> {
    if bind_state.identifier_symbols.contains(identifier.id) {
        panic!("Impossible: Tried to assign symbol to identifier that already had a symbol assigned to it.");
    }
    let name_symbol = bind_state.context.lookup(identifier)?;
    bind_state
        .identifier_symbols
        .insert(identifier.id, name_symbol);
    Ok(name_symbol)
}

fn define_symbol_source(bind_state: &mut BindState, symbol: Symbol, source: SymbolSource) {
    if bind_state.symbol_sources.contains_key(&symbol) {
        panic!("Impossible: Tried to define symbol source for symbol that already had a source defined.");
    }
    bind_state.symbol_sources.insert(symbol, source);
}

use context::*;
mod context {
    use super::*;

    use rustc_hash::FxHashMap;

    #[derive(Debug)]
    pub struct Context<'a> {
        scope_stack: Vec<FxHashMap<IdentifierName, (SymbolSource, Symbol)>>,
        provider: &'a mut SymbolProvider,
    }

    impl Context<'_> {
        pub fn new(provider: &mut SymbolProvider) -> Context {
            Context {
                scope_stack: vec![],
                provider,
            }
        }
    }

    impl Context<'_> {
        pub fn assign_new_symbol_and_add(
            &mut self,
            name: &IdentifierName,
            source: SymbolSource,
        ) -> Result<Symbol, NameClashError> {
            let existing_symbol: Option<&(SymbolSource, Symbol)> =
                self.scope_stack.iter().find_map(|scope| scope.get(&name));
            if let Some((existing_symbol_source, _existing_symbol)) = existing_symbol {
                return Err(NameClashError {
                    old: *existing_symbol_source,
                    new: source,
                });
            }
            let symbol = self.new_symbol();
            self.assign_symbol_and_add(name, source, symbol)
        }

        pub fn assign_symbol_and_add(
            &mut self,
            name: &IdentifierName,
            source: SymbolSource,
            symbol: Symbol,
        ) -> Result<Symbol, NameClashError> {
            let existing_symbol: Option<&(SymbolSource, Symbol)> =
                self.scope_stack.iter().find_map(|scope| scope.get(&name));
            if let Some((existing_symbol_source, _existing_symbol)) = existing_symbol {
                return Err(NameClashError {
                    old: *existing_symbol_source,
                    new: source,
                });
            }
            self.scope_stack
                .last_mut()
                .expect("Error: Context::add was called when the stack was empty.")
                .insert(name.clone(), (source, symbol));
            Ok(symbol)
        }

        pub fn new_symbol(&mut self) -> Symbol {
            self.provider.new_symbol()
        }

        pub fn lookup(&self, identifier: &Identifier) -> Result<Symbol, NameNotFoundError> {
            let existing_symbol: Option<&(SymbolSource, Symbol)> = self
                .scope_stack
                .iter()
                .find_map(|scope| scope.get(&identifier.name));
            if let Some(existing_symbol) = existing_symbol {
                Ok(existing_symbol.1)
            } else {
                Err(NameNotFoundError {
                    name: identifier.name.clone(),
                })
            }
        }

        pub fn push_scope(&mut self) {
            self.scope_stack.push(FxHashMap::default());
        }

        pub fn pop_scope(&mut self) {
            self.scope_stack.pop();
        }
    }
}
