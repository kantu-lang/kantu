use crate::data::{
    node_registry::{NodeId, NodeRegistry},
    registered_sst::*,
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
    UnbindableDotExpressionLhs(ExpressionId),
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

/// If this function returns `Ok`, then every identifier in the program is bound to a symbol,
/// EXCEPT for the identifiers that appear as the variant name in a match case node.
/// The reason we cannot bind variant names is because we don't know the type of the matchee.
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
        bind_file(&mut bind_state, registry, file)?;
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

fn bind_file(
    bind_state: &mut BindState,
    registry: &NodeRegistry,
    file: &File,
) -> Result<(), BindError> {
    bind_state.context.push_scope();
    let item_ids = registry.file_item_list(file.item_list_id);
    for item_id in item_ids {
        match item_id {
            FileItemNodeId::Type(type_id) => {
                let type_statement = registry.type_statement(*type_id);
                bind_type_statement(bind_state, registry, type_statement)?
            }
            FileItemNodeId::Let(let_id) => {
                let let_statement = registry.let_statement(*let_id);
                bind_let_statement(bind_state, registry, let_statement)?
            }
        }
    }
    bind_state.context.pop_scope();
    Ok(())
}

fn bind_type_statement(
    bind_state: &mut BindState,
    registry: &NodeRegistry,
    type_statement: &TypeStatement,
) -> Result<(), BindError> {
    bind_state.context.push_scope();
    let param_ids = registry.param_list(type_statement.param_list_id);
    for param_id in param_ids {
        let param = registry.param(*param_id);
        bind_param(bind_state, registry, param)?;
    }
    bind_state.context.pop_scope();

    let name_symbol = define_symbol_in_context_and_bind_to_identifier(
        bind_state,
        registry,
        type_statement.name_id,
        SymbolSource::Type(type_statement.id),
    )?;

    let variant_ids = registry.variant_list(type_statement.variant_list_id);
    for variant_id in variant_ids {
        let variant = registry.variant(*variant_id);
        bind_variant(bind_state, registry, variant, name_symbol)?;
    }

    Ok(())
}

fn bind_param(
    bind_state: &mut BindState,
    registry: &NodeRegistry,
    param: &Param,
) -> Result<(), BindError> {
    bind_expression(bind_state, registry, param.type_id)?;
    define_symbol_in_context_and_bind_to_identifier(
        bind_state,
        registry,
        param.name_id,
        SymbolSource::TypedParam(param.id),
    )?;
    Ok(())
}

fn bind_variant(
    bind_state: &mut BindState,
    registry: &NodeRegistry,
    variant: &Variant,
    declaring_type_name_symbol: Symbol,
) -> Result<(), BindError> {
    bind_state.context.push_scope();
    let param_ids = registry.param_list(variant.param_list_id);
    for param_id in param_ids {
        let param = registry.param(*param_id);
        bind_param(bind_state, registry, param)?;
    }
    bind_expression(bind_state, registry, variant.return_type_id)?;
    bind_state.context.pop_scope();

    let variant_symbol = bind_new_symbol_to_identifier(bind_state, registry, variant.name_id);
    let variant_symbol_source = SymbolSource::Variant(variant.id);
    define_symbol_source(bind_state, variant_symbol, variant_symbol_source);
    let variant_name: &IdentifierName = &registry.identifier(variant.name_id).name;
    if let Some(existing_symbol) = bind_state
        .dot_targets
        .get(declaring_type_name_symbol, variant_name)
    {
        let old_symbol_source = *bind_state.symbol_sources.get(&existing_symbol).expect("Error: Existing variant symbol does not have a symbol source defined. This indicates a serious logic bug.");
        Err(BindError::NameClash(NameClashError {
            old: old_symbol_source,
            new: variant_symbol_source,
        }))
    } else {
        bind_state.dot_targets.insert(
            declaring_type_name_symbol,
            variant_name.clone(),
            variant_symbol,
        );
        Ok(())
    }
}

fn bind_let_statement(
    bind_state: &mut BindState,
    registry: &NodeRegistry,
    let_statement: &LetStatement,
) -> Result<(), BindError> {
    bind_expression(bind_state, registry, let_statement.value_id)?;
    define_symbol_in_context_and_bind_to_identifier(
        bind_state,
        registry,
        let_statement.name_id,
        SymbolSource::Let(let_statement.id),
    )?;
    Ok(())
}

fn bind_expression(
    bind_state: &mut BindState,
    registry: &NodeRegistry,
    expression_id: ExpressionId,
) -> Result<(), BindError> {
    match expression_id {
        ExpressionId::Identifier(id) => {
            let identifier = registry.identifier(id);
            bind_identifier(bind_state, identifier)
        }
        ExpressionId::Dot(id) => {
            let dot = registry.dot(id);
            bind_dot(bind_state, registry, dot)
        }
        ExpressionId::Call(id) => {
            let call = registry.call(id);
            bind_call(bind_state, registry, call)
        }
        ExpressionId::Fun(id) => {
            let fun = registry.fun(id);
            bind_fun(bind_state, registry, fun)
        }
        ExpressionId::Match(id) => {
            let match_ = registry.match_(id);
            bind_match(bind_state, registry, match_)
        }
        ExpressionId::Forall(id) => {
            let forall = registry.forall(id);
            bind_forall(bind_state, registry, forall)
        }
    }
}

fn bind_identifier(bind_state: &mut BindState, identifier: &Identifier) -> Result<(), BindError> {
    lookup_symbol_from_context_and_bind_to_identifier(bind_state, identifier)?;
    Ok(())
}

fn bind_dot(
    bind_state: &mut BindState,
    registry: &NodeRegistry,
    dot: &Dot,
) -> Result<(), BindError> {
    let right = registry.identifier(dot.right_id);
    bind_expression(bind_state, registry, dot.left_id)?;
    let left_symbol = match dot.left_id {
        ExpressionId::Identifier(identifier_id) => bind_state.identifier_symbols.get(identifier_id),
        ExpressionId::Dot(subdot_id) => {
            let subdot = registry.dot(subdot_id);
            bind_state.identifier_symbols.get(subdot.right_id)
        }
        _ => return Err(BindError::UnbindableDotExpressionLhs(dot.left_id)),
    };
    let right_symbol = if let Some(s) = bind_state.dot_targets.get(left_symbol, &right.name) {
        s
    } else {
        return Err(BindError::InvalidDotExpressionRhs(right.id));
    };
    bind_symbol_to_identifier(bind_state, right_symbol, right);
    Ok(())
}

fn bind_call(
    bind_state: &mut BindState,
    registry: &NodeRegistry,
    call: &Call,
) -> Result<(), BindError> {
    bind_expression(bind_state, registry, call.callee_id)?;
    let arg_ids = registry.expression_list(call.arg_list_id);
    for arg_id in arg_ids {
        bind_expression(bind_state, registry, *arg_id)?;
    }
    Ok(())
}

fn bind_fun(
    bind_state: &mut BindState,
    registry: &NodeRegistry,
    fun: &Fun,
) -> Result<(), BindError> {
    bind_state.context.push_scope();
    let param_ids = registry.param_list(fun.param_list_id);
    for param_id in param_ids {
        let param = registry.param(*param_id);
        bind_param(bind_state, registry, param)?;
    }
    bind_expression(bind_state, registry, fun.return_type_id)?;
    define_symbol_in_context_and_bind_to_identifier(
        bind_state,
        registry,
        fun.name_id,
        SymbolSource::Fun(fun.id),
    )?;
    bind_expression(bind_state, registry, fun.body_id)?;
    bind_state.context.pop_scope();
    Ok(())
}

fn bind_match(
    bind_state: &mut BindState,
    registry: &NodeRegistry,
    match_: &Match,
) -> Result<(), BindError> {
    bind_expression(bind_state, registry, match_.matchee_id)?;
    let case_ids = registry.match_case_list(match_.case_list_id);
    for case_id in case_ids {
        let case = registry.match_case(*case_id);
        bind_match_case(bind_state, registry, case)?;
    }
    Ok(())
}

fn bind_match_case(
    bind_state: &mut BindState,
    registry: &NodeRegistry,
    case: &MatchCase,
) -> Result<(), BindError> {
    bind_state.context.push_scope();
    let param_ids = registry.identifier_list(case.param_list_id);
    for param_id in param_ids.iter().copied() {
        define_symbol_in_context_and_bind_to_identifier(
            bind_state,
            registry,
            param_id,
            SymbolSource::UntypedParam(param_id),
        )?;
    }
    bind_expression(bind_state, registry, case.output_id)?;
    bind_state.context.pop_scope();
    Ok(())
}

fn bind_forall(
    bind_state: &mut BindState,
    registry: &NodeRegistry,
    forall: &Forall,
) -> Result<(), BindError> {
    bind_state.context.push_scope();
    let param_ids = registry.param_list(forall.param_list_id);
    for param_id in param_ids {
        let param = registry.param(*param_id);
        bind_param(bind_state, registry, param)?;
    }
    bind_expression(bind_state, registry, forall.output_id)?;
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
    registry: &NodeRegistry,
    identifier_id: NodeId<Identifier>,
    source: SymbolSource,
) -> Result<Symbol, BindError> {
    let identifier = registry.identifier(identifier_id);
    let name_symbol = bind_state
        .context
        .assign_new_symbol_and_add(&identifier.name, source)?;
    bind_symbol_to_identifier(bind_state, name_symbol, identifier);
    define_symbol_source(bind_state, name_symbol, source);

    Ok(name_symbol)
}

fn bind_new_symbol_to_identifier(
    bind_state: &mut BindState,
    registry: &NodeRegistry,
    identifier_id: NodeId<Identifier>,
) -> Symbol {
    let identifier = registry.identifier(identifier_id);
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
