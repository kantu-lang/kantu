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
    let mut state = BindState {
        dot_targets: SymbolToDotTargetsMap::empty(),
        context: Context::with_builtins(),
    };

    let files = file_node_ids
        .into_iter()
        .map(|file| bind_file(&mut state, file))
        .collect::<Result<Vec<_>, BindError>>()?;

    Ok((files, state.context.into_provider(), state.dot_targets))
}

fn sort_by_dependencies(
    files: Vec<ub::File>,
) -> Result<Vec<ub::File>, CircularFileDependencyError> {
    // TODO (distant): Actually sort, once we support `use` statements.
    Ok(files)
}

struct BindState {
    dot_targets: SymbolToDotTargetsMap,
    context: Context,
}

fn bind_file(state: &mut BindState, file: ub::File) -> Result<File, BindError> {
    state.context.push_scope();
    let items = file
        .items
        .into_iter()
        .map(|item| bind_file_item(state, item))
        .collect::<Result<Vec<_>, BindError>>()?;
    state.context.pop_scope_or_panic();
    Ok(File { id: file.id, items })
}

fn bind_file_item(state: &mut BindState, item: ub::FileItem) -> Result<FileItem, BindError> {
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
    state: &mut BindState,
    type_statement: ub::TypeStatement,
) -> Result<TypeStatement, BindError> {
    let params = {
        state.context.push_scope();
        let out = type_statement
            .params
            .into_iter()
            .map(|param| bind_param(state, param))
            .collect::<Result<Vec<_>, BindError>>()?;
        state.context.pop_scope_or_panic();
        out
    };

    let name = state.context.declare_name(&type_statement.name)?;
    let variants = type_statement
        .variants
        .into_iter()
        .map(|variant| bind_variant_without_declaring_dot_target(state, variant))
        .collect::<Result<Vec<_>, BindError>>()?;

    for variant in &variants {
        state.dot_targets.insert(
            name.symbol,
            variant.name.component.name.clone(),
            variant.name.symbol,
        );
    }

    Ok(TypeStatement {
        name,
        params,
        variants,
    })
}

fn bind_param(state: &mut BindState, param: ub::Param) -> Result<Param, BindError> {
    let type_ = bind_expression(state, param.type_)?;
    let name = state.context.declare_name(&param.name)?;
    Ok(Param {
        is_dashed: param.is_dashed,
        name,
        type_,
    })
}

fn bind_variant_without_declaring_dot_target(
    state: &mut BindState,
    variant: ub::Variant,
) -> Result<Variant, BindError> {
    unimplemented!()
}

fn bind_let_statement(
    state: &mut BindState,
    let_statement: ub::LetStatement,
) -> Result<LetStatement, BindError> {
    let value = bind_expression(state, let_statement.value)?;
    let name = state.context.declare_name(&let_statement.name)?;
    Ok(LetStatement { name, value })
}

fn bind_expression(
    state: &mut BindState,
    expression: ub::Expression,
) -> Result<Expression, BindError> {
    unimplemented!()
}

use context::*;
mod context {
    use super::*;

    use rustc_hash::FxHashMap;

    #[derive(Clone, Debug)]
    pub struct Context {
        provider: SymbolProvider,
        scope_stack: Vec<Scope>,
    }

    #[derive(Clone, Debug)]
    struct Scope(FxHashMap<IdentifierName, (OwnedSymbolSource, Symbol)>);

    #[derive(Clone, Debug)]
    pub enum OwnedSymbolSource {
        Identifier(ub::Identifier),
        Builtin(ReservedIdentifierName),
    }

    impl Context {
        pub fn with_builtins() -> Self {
            let mut provider = SymbolProvider::new();
            let mut bottom_scope = Scope(FxHashMap::default());
            bottom_scope.0.insert(
                IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase),
                (
                    OwnedSymbolSource::Builtin(ReservedIdentifierName::TypeTitleCase),
                    provider.type0_symbol(),
                ),
            );

            Self {
                provider,
                scope_stack: vec![bottom_scope],
            }
        }
    }

    impl Context {
        pub fn push_scope(&mut self) {
            self.scope_stack.push(Scope(FxHashMap::default()));
        }

        pub fn pop_scope_or_panic(&mut self) {
            self.scope_stack
                .pop()
                .expect("Tried to pop scope from empty stack");
        }

        /// The total number of names in the context.
        pub fn len(&self) -> usize {
            self.scope_stack.iter().map(|scope| scope.0.len()).sum()
        }
    }

    impl Context {
        pub fn get_symbol(&self, name: &IdentifierName) -> Result<Symbol, NameNotFoundError> {
            unimplemented!()
        }

        fn get(&self, name: &IdentifierName) -> Option<(OwnedSymbolSource, Symbol)> {
            unimplemented!()
        }

        pub fn declare_name(
            &mut self,
            name: &ub::Identifier,
        ) -> Result<SingletonName, NameClashError> {
            if let Some((existing_source, _)) = self.get(&name.name) {
                return Err(NameClashError {
                    old: existing_source,
                    new: OwnedSymbolSource::Identifier(name.clone()),
                });
            }

            let symbol = self.provider.new_symbol();
            self.scope_stack
                .last_mut()
                .expect("Tried to declare name in a zero-scope context.")
                .0
                .insert(
                    name.name.clone(),
                    (OwnedSymbolSource::Identifier(name.clone()), symbol),
                );
            Ok(SingletonName {
                component: name.clone(),
                symbol,
            })
        }
    }

    impl Context {
        pub fn into_provider(self) -> SymbolProvider {
            self.provider
        }
    }
}

pub use error::*;
mod error {
    use super::*;

    #[derive(Clone, Debug)]
    pub enum BindError {
        CircularFileDependency(CircularFileDependencyError),
        NameClash(NameClashError),
        NameNotFound(NameNotFoundError),
        InvalidDotExpressionRhs(Identifier),
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
    pub struct NameNotFoundError {
        pub name: IdentifierName,
    }

    impl From<NameNotFoundError> for BindError {
        fn from(error: NameNotFoundError) -> Self {
            Self::NameNotFound(error)
        }
    }
}
