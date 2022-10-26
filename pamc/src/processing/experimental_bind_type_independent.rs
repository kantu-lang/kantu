use crate::data::{
    bound_ast::*,
    // `ub` stands for "unbound".
    simplified_ast as ub,
    symbol_database::{Symbol, SymbolProvider, SymbolToDotTargetsMap},
    FileId,
};

// TODO: Forbid fun return type from using the fun it declares.

/// The returned `Vec<File>` is not guaranteed to be in any particular order.
pub fn bind_symbols_to_identifiers(
    files: Vec<ub::File>,
) -> Result<(Vec<File>, SymbolProvider, SymbolToDotTargetsMap), BindError> {
    let file_node_ids = sort_by_dependencies(files)?;
    let mut state = State::with_builtins();

    let files = file_node_ids
        .into_iter()
        .map(|file| bind_file(&mut state, file))
        .collect::<Result<Vec<_>, BindError>>()?;

    let (provider, dot_targets) = state.into_provider_and_dot_targets();
    Ok((files, provider, dot_targets))
}

fn sort_by_dependencies(
    files: Vec<ub::File>,
) -> Result<Vec<ub::File>, CircularFileDependencyError> {
    // TODO (distant): Actually sort, once we support `use` statements.
    Ok(files)
}

fn bind_file(state: &mut State, file: ub::File) -> Result<File, BindError> {
    state.push_scope();
    let items = file
        .items
        .into_iter()
        .map(|item| bind_file_item(state, item))
        .collect::<Result<Vec<_>, BindError>>()?;
    state.pop_scope_or_panic();
    Ok(File { id: file.id, items })
}

fn bind_file_item(state: &mut State, item: ub::FileItem) -> Result<FileItem, BindError> {
    match item {
        ub::FileItem::Type(type_statement) => {
            Ok(FileItem::Type(bind_type_statement(state, type_statement)?))
        }
        ub::FileItem::Let(let_statement) => {
            Ok(FileItem::Let(bind_let_statement(state, let_statement)?))
        }
    }
}

fn bind_type_statement(
    state: &mut State,
    type_statement: ub::TypeStatement,
) -> Result<TypeStatement, BindError> {
    let params = {
        state.push_scope();
        let out = type_statement
            .params
            .into_iter()
            .map(|param| bind_param(state, param))
            .collect::<Result<Vec<_>, BindError>>()?;
        state.pop_scope_or_panic();
        out
    };

    let name = create_name_and_add_to_scope(state, &type_statement.name)?;

    let (variants, original_names): (Vec<Variant>, Vec<ub::Identifier>) = {
        let variants_with_original_names: Vec<(Variant, ub::Identifier)> = type_statement
            .variants
            .into_iter()
            .map(|unbound| {
                let original_name = unbound.name.clone();
                let variant = bind_variant_without_declaring_dot_target(state, unbound)?;
                Ok((variant, original_name))
            })
            .collect::<Result<Vec<_>, BindError>>()?;
        variants_with_original_names.into_iter().unzip()
    };

    for (variant, original_name) in variants.iter().zip(original_names.into_iter()) {
        state.add_dot_target_to_scope(
            (name.symbol, variant.name.component.name.clone()),
            (
                variant.name.symbol,
                OwnedSymbolSource::Identifier(original_name.clone()),
            ),
        )?;
    }

    Ok(TypeStatement {
        name,
        params,
        variants,
    })
}

fn bind_param(state: &mut State, param: ub::Param) -> Result<Param, BindError> {
    let type_ = bind_expression(state, param.type_)?;
    let name = create_name_and_add_to_scope(state, &param.name)?;
    Ok(Param {
        is_dashed: param.is_dashed,
        name,
        type_,
    })
}

fn bind_variant_without_declaring_dot_target(
    state: &mut State,
    variant: ub::Variant,
) -> Result<Variant, BindError> {
    state.push_scope();
    let params = variant
        .params
        .into_iter()
        .map(|param| bind_param(state, param))
        .collect::<Result<Vec<_>, BindError>>()?;
    let return_type = bind_expression(state, variant.return_type)?;
    state.pop_scope_or_panic();

    Ok(Variant {
        name: create_name_without_adding_to_scope(state, &variant.name),
        params,
        return_type,
    })
}

fn bind_let_statement(
    state: &mut State,
    let_statement: ub::LetStatement,
) -> Result<LetStatement, BindError> {
    let value = bind_expression(state, let_statement.value)?;
    let name = create_name_and_add_to_scope(state, &let_statement.name)?;
    Ok(LetStatement { name, value })
}

fn bind_expression(state: &mut State, expression: ub::Expression) -> Result<Expression, BindError> {
    match expression {
        ub::Expression::Name(name) => bind_name_expression(state, name),
        ub::Expression::Call(call) => bind_call_expression(state, *call),
        ub::Expression::Fun(fun) => bind_fun(state, *fun),
        ub::Expression::Match(match_) => bind_match(state, *match_),
        ub::Expression::Forall(forall) => bind_forall(state, *forall),
    }
}

fn bind_name_expression(
    state: &mut State,
    name: ub::NameExpression,
) -> Result<Expression, BindError> {
    let (first, rest) = split_first_and_rest(&name.components)
        .expect("NameExpression must have at least one component.");
    let symbol = {
        let mut current = state.get_symbol(&first)?;
        for component in rest {
            current = state.get_dot_target_symbol((current, &component))?;
        }
        current
    };
    let db_index = state
        .get_db_index(symbol)
        .expect("Symbol should be within scope.");
    Ok(Expression::Name(NameExpression {
        components: name.components,
        symbol,
        db_index,
    }))
}

fn split_first_and_rest<T>(components: &[T]) -> Option<(&T, &[T])> {
    if components.is_empty() {
        return None;
    }
    Some((&components[0], &components[1..]))
}

fn bind_call_expression(_state: &mut State, _call: ub::Call) -> Result<Expression, BindError> {
    unimplemented!()
}

fn bind_fun(_state: &mut State, _fun: ub::Fun) -> Result<Expression, BindError> {
    unimplemented!()
}

fn bind_match(_state: &mut State, _match_: ub::Match) -> Result<Expression, BindError> {
    unimplemented!()
}

fn bind_forall(_state: &mut State, _forall: ub::Forall) -> Result<Expression, BindError> {
    unimplemented!()
}

fn create_name_without_adding_to_scope(
    state: &mut State,
    identifier: &ub::Identifier,
) -> SingletonName {
    SingletonName {
        component: identifier.clone(),
        symbol: state.new_symbol(),
    }
}

fn create_name_and_add_to_scope(
    state: &mut State,
    identifier: &ub::Identifier,
) -> Result<SingletonName, NameClashError> {
    let symbol = state.add_name_to_scope(identifier)?;
    Ok(SingletonName {
        component: identifier.clone(),
        symbol,
    })
}

use state::*;
mod state {
    use super::*;

    use rustc_hash::FxHashMap;

    #[derive(Clone, Debug)]
    pub struct State {
        provider: SymbolProvider,
        scope_stack: Vec<Scope>,
    }

    #[derive(Clone, Debug)]
    pub enum OwnedSymbolSource {
        Identifier(ub::Identifier),
        Builtin(ReservedIdentifierName),
    }

    impl State {
        pub fn with_builtins() -> Self {
            let provider = SymbolProvider::new();
            let mut bottom_scope = Scope::empty();
            bottom_scope.insert_unqualified_name(
                IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase),
                (
                    provider.type0_symbol(),
                    OwnedSymbolSource::Builtin(ReservedIdentifierName::TypeTitleCase),
                ),
            );

            Self {
                provider,
                scope_stack: vec![bottom_scope],
            }
        }
    }

    impl State {
        pub fn push_scope(&mut self) {
            self.scope_stack.push(Scope::empty());
        }

        pub fn pop_scope_or_panic(&mut self) {
            self.scope_stack
                .pop()
                .expect("Tried to pop scope from empty stack");
        }
    }

    impl State {
        pub fn get_symbol(&self, identifier: &Identifier) -> Result<Symbol, NameNotFoundError> {
            if let Some(data) = self.get_symbol_data_for_name(&identifier.name) {
                Ok(data.symbol)
            } else {
                Err(NameNotFoundError {
                    name: identifier.clone(),
                })
            }
        }

        fn get_symbol_data_for_name(&self, name: &IdentifierName) -> Option<&SymbolData> {
            for scope in self.scope_stack.iter().rev() {
                if let Some(data) = scope.get_symbol_data_by_name(name) {
                    return Some(data);
                }
            }
            None
        }

        pub fn add_name_to_scope(
            &mut self,
            identifier: &ub::Identifier,
        ) -> Result<Symbol, NameClashError> {
            self.check_for_name_clash(identifier)?;

            let symbol = self.provider.new_symbol();
            self.scope_stack
                .last_mut()
                .expect("Tried to declare name in a zero-scope state.")
                .insert_unqualified_name(
                    identifier.name.clone(),
                    (symbol, OwnedSymbolSource::Identifier(identifier.clone())),
                );

            Ok(symbol)
        }

        fn check_for_name_clash(&self, identifier: &ub::Identifier) -> Result<(), NameClashError> {
            if let Some(data) = self.get_symbol_data_for_name(&identifier.name) {
                return Err(NameClashError {
                    old: data.source.clone(),
                    new: OwnedSymbolSource::Identifier(identifier.clone()),
                });
            } else {
                Ok(())
            }
        }

        pub fn new_symbol(&mut self) -> Symbol {
            self.provider.new_symbol()
        }
    }

    impl State {
        pub fn get_dot_target_symbol(
            &self,
            input: (Symbol, &Identifier),
        ) -> Result<Symbol, InvalidDotExpressionRhsError> {
            if let Some(data) = self.get_dot_target_symbol_data((input.0, &input.1.name)) {
                Ok(data.symbol)
            } else {
                Err(InvalidDotExpressionRhsError {
                    rhs: input.1.clone(),
                })
            }
        }

        pub fn get_dot_target_symbol_data(
            &self,
            input: (Symbol, &IdentifierName),
        ) -> Option<&SymbolData> {
            for scope in self.scope_stack.iter().rev() {
                if let Some(symbol) = scope.get_dot_target_symbol_data(input) {
                    return Some(symbol);
                }
            }
            None
        }

        pub fn add_dot_target_to_scope(
            &mut self,
            input: (Symbol, IdentifierName),
            output: (Symbol, OwnedSymbolSource),
        ) -> Result<(), DotExpressionRhsClashError> {
            self.check_for_dot_target_clash(&input, &output.1)?;

            self.scope_stack
                .last_mut()
                .expect("Tried to declare name in a zero-scope state.")
                .insert_dot_target(input, output);

            Ok(())
        }

        fn check_for_dot_target_clash(
            &self,
            input: &(Symbol, IdentifierName),
            output_source: &OwnedSymbolSource,
        ) -> Result<(), DotExpressionRhsClashError> {
            if let Some(data) = self.get_symbol_data_for_name(&input.1) {
                return Err(DotExpressionRhsClashError {
                    old: data.source.clone(),
                    new: output_source.clone(),
                });
            } else {
                Ok(())
            }
        }
    }

    impl State {
        /// Returns the De Bruijn index of the given symbol.
        pub fn get_db_index(&self, symbol: Symbol) -> Option<usize> {
            for (scope_index, scope) in self.scope_stack.iter().enumerate().rev() {
                if let Some(data) = scope.get_symbol_data_by_symbol(symbol) {
                    let local_index = data.index_within_scope;
                    let offset: usize = self.scope_stack[0..scope_index]
                        .iter()
                        .map(|scope| scope.len())
                        .sum();
                    return Some(offset + local_index);
                }
            }
            None
        }
    }

    impl State {
        pub fn into_provider_and_dot_targets(self) -> (SymbolProvider, SymbolToDotTargetsMap) {
            let dot_targets = {
                let mut out = SymbolToDotTargetsMap::empty();
                for scope in self.scope_stack {
                    for ((left, name), data) in scope.into_dot_targets() {
                        out.insert(left, name, data.symbol);
                    }
                }
                out
            };
            (self.provider, dot_targets)
        }
    }

    use scope::*;
    mod scope {
        use super::*;

        #[derive(Clone, Debug)]
        pub struct Scope {
            /// Unqualified names are names that can be accessed without a dot.
            /// For example, in
            ///
            /// ```pamlihu
            /// type Nat {
            ///    .O: Nat,
            ///    .S(n: Nat): Nat,
            /// }
            /// ```
            ///
            /// ...`Nat` is the only unqualified name (excluding the implicitly defined `Type`, of course).
            unqualified_names: FxHashMap<IdentifierName, SymbolData>,

            dot_targets: FxHashMap<(Symbol, IdentifierName), SymbolData>,
        }

        #[derive(Clone, Debug)]
        pub struct SymbolData {
            pub source: OwnedSymbolSource,
            pub symbol: Symbol,
            pub index_within_scope: usize,
        }

        impl Scope {
            pub fn empty() -> Self {
                Self {
                    unqualified_names: FxHashMap::default(),
                    dot_targets: FxHashMap::default(),
                }
            }
        }

        impl Scope {
            pub fn get_symbol_data_by_name(&self, name: &IdentifierName) -> Option<&SymbolData> {
                self.unqualified_names.get(name)
            }

            pub fn get_symbol_data_by_symbol(&self, symbol: Symbol) -> Option<&SymbolData> {
                self.unqualified_names
                    .values()
                    .chain(self.dot_targets.values())
                    .find(|data| data.symbol == symbol)
            }

            pub fn len(&self) -> usize {
                self.unqualified_names.len() + self.dot_targets.len()
            }

            /// Panics if the name is already in the scope.
            pub fn insert_unqualified_name(
                &mut self,
                input: IdentifierName,
                output: (Symbol, OwnedSymbolSource),
            ) {
                let (symbol, source) = output;
                if self.unqualified_names.contains_key(&input) {
                    panic!("Tried to insert a name that already exists in the scope.");
                }
                let data = SymbolData {
                    source,
                    symbol,
                    index_within_scope: self.len(),
                };
                self.unqualified_names.insert(input, data);
            }

            /// Panics if the name is already in the scope.
            pub fn insert_dot_target(
                &mut self,
                input: (Symbol, IdentifierName),
                output: (Symbol, OwnedSymbolSource),
            ) {
                let data = SymbolData {
                    symbol: output.0,
                    source: output.1,
                    index_within_scope: self.len(),
                };
                self.dot_targets.insert(input, data);
            }

            pub fn get_dot_target_symbol_data(
                &self,
                input: (Symbol, &IdentifierName),
            ) -> Option<&SymbolData> {
                self.dot_targets.get(&(input.0, input.1.clone()))
            }
        }

        impl Scope {
            pub fn into_dot_targets(self) -> FxHashMap<(Symbol, IdentifierName), SymbolData> {
                self.dot_targets
            }
        }
    }
}

pub use error::*;
mod error {
    use super::*;

    #[derive(Clone, Debug)]
    pub enum BindError {
        CircularFileDependency(CircularFileDependencyError),
        NameNotFound(NameNotFoundError),
        InvalidDotExpressionRhs(InvalidDotExpressionRhsError),
        NameClash(NameClashError),
        DotExpressionRhsClash(DotExpressionRhsClashError),
    }

    #[derive(Clone, Debug)]
    pub struct CircularFileDependencyError {
        pub ids: Vec<FileId>,
    }
    impl From<CircularFileDependencyError> for BindError {
        fn from(error: CircularFileDependencyError) -> Self {
            Self::CircularFileDependency(error)
        }
    }

    #[derive(Clone, Debug)]
    pub struct NameClashError {
        pub old: OwnedSymbolSource,
        pub new: OwnedSymbolSource,
    }
    impl From<NameClashError> for BindError {
        fn from(error: NameClashError) -> Self {
            Self::NameClash(error)
        }
    }

    #[derive(Clone, Debug)]
    pub struct DotExpressionRhsClashError {
        pub old: OwnedSymbolSource,
        pub new: OwnedSymbolSource,
    }
    impl From<DotExpressionRhsClashError> for BindError {
        fn from(error: DotExpressionRhsClashError) -> Self {
            Self::DotExpressionRhsClash(error)
        }
    }

    #[derive(Clone, Debug)]
    pub struct NameNotFoundError {
        pub name: Identifier,
    }
    impl From<NameNotFoundError> for BindError {
        fn from(error: NameNotFoundError) -> Self {
            Self::NameNotFound(error)
        }
    }

    #[derive(Clone, Debug)]
    pub struct InvalidDotExpressionRhsError {
        pub rhs: Identifier,
    }
    impl From<InvalidDotExpressionRhsError> for BindError {
        fn from(error: InvalidDotExpressionRhsError) -> Self {
            Self::InvalidDotExpressionRhs(error)
        }
    }
}
