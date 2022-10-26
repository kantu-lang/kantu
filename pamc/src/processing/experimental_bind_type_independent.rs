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
    struct Scope {
        map: FxHashMap<IdentifierName, (OwnedSymbolSource, Symbol)>,
    }

    impl Scope {
        fn empty() -> Self {
            Self {
                map: FxHashMap::default(),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub enum OwnedSymbolSource {
        Identifier(ub::Identifier),
        Builtin(ReservedIdentifierName),
    }

    impl Context {
        pub fn with_builtins() -> Self {
            let mut provider = SymbolProvider::new();
            let mut bottom_scope = Scope::empty();
            bottom_scope.map.insert(
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
            self.scope_stack.push(Scope::empty());
        }

        pub fn pop_scope_or_panic(&mut self) {
            self.scope_stack
                .pop()
                .expect("Tried to pop scope from empty stack");
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
