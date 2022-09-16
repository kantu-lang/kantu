use crate::{bound_ast as bound, bound_ast::SymbolId, unbound_ast as unbound};

#[derive(Clone, Debug)]
pub enum BindError {
    DuplicateSymbol(unbound::Identifier, unbound::Identifier),
}

/// The returned vector of files has an arbitrary order--it is **NOT**
/// guaranteed to have the same order as the input vector.
pub fn bind_symbols(
    files: Vec<unbound::File>,
) -> Result<(SymbolDatabase, Vec<bound::File>), BindError> {
    let files = sort_files_by_dependency(files);
    let mut db = SymbolDatabase::empty();
    let mut bound_files = Vec::with_capacity(files.len());
    for file in files {
        match bind_file(file, &mut db) {
            Ok(bound) => bound_files.push(bound),
            Err(err) => return Err(err),
        };
    }
    Ok((db, bound_files))
}

fn sort_files_by_dependency(files: Vec<unbound::File>) -> Vec<unbound::File> {
    // TODO: Sort by dependency once we support `use` statements.
    files
}

fn bind_file(file: unbound::File, db: &mut SymbolDatabase) -> Result<bound::File, BindError> {
    let mut binder = FileBinder {
        db,
        context_stack: ContextStack::singleton_empty(),
    };
    binder.bind_file(file)
}

struct FileBinder<'a> {
    db: &'a mut SymbolDatabase,
    context_stack: ContextStack,
}

impl FileBinder<'_> {
    fn bind_file(&mut self, file: unbound::File) -> Result<bound::File, BindError> {
        let mut bound_file = bound::File {
            id: file.id,
            items: Vec::with_capacity(file.items.len()),
        };
        for item in file.items {
            match self.bind_file_item(item) {
                Ok(bound) => bound_file.items.push(bound),
                Err(err) => return Err(err),
            }
        }
        Ok(bound_file)
    }

    fn bind_file_item(&mut self, item: unbound::FileItem) -> Result<bound::FileItem, BindError> {
        match item {
            unbound::FileItem::Type(type_) => {
                self.bind_type_statement(type_).map(bound::FileItem::Type)
            }
            unbound::FileItem::Let(let_) => self.bind_let_statement(let_).map(bound::FileItem::Let),
        }
    }

    fn bind_type_statement(
        &mut self,
        item: unbound::TypeStatement,
    ) -> Result<bound::TypeStatement, BindError> {
        let type_sid = self.context_stack.add_to_top(&item.name)?;
        self.db
            .declare_symbol(type_sid, SymbolSource::Type(item.clone()));
    }

    fn bind_let_statement(
        &mut self,
        item: unbound::LetStatement,
    ) -> Result<bound::LetStatement, BindError> {
        unimplemented!()
    }
}

// fn bind_file(file: unbound::File, db: &mut SymbolDatabase) -> Result<bound::File, BindError> {
//     let mut context_stack = ContextStack::empty();
//     let mut bound_file = bound::File {
//         id: file.id,
//         items: Vec::with_capacity(file.items.len()),
//     };
//     for item in file.items {
//         match bind_file_item(item, db, &mut context_stack) {
//             Ok(bound) => bound_file.items.push(bound),
//             Err(err) => return Err(err),
//         }
//     }
//     Ok(bound_file)
// }

// fn bind_file_item(
//     item: unbound::FileItem,
//     db: &mut SymbolDatabase,
//     context_stack: &mut ContextStack,
// ) -> Result<bound::FileItem, BindError> {
//     match item {
//         unbound::FileItem::Type(type_) => {
//             bind_type_statement(type_, db, context_stack).map(bound::FileItem::Type)
//         }
//         unbound::FileItem::Let(let_) => {
//             bind_let_statement(let_, db, context_stack).map(bound::FileItem::Let)
//         }
//     }
// }

// fn bind_type_statement(
//     item: unbound::TypeStatement,
//     db: &mut SymbolDatabase,
//     context_stack: &mut ContextStack,
// ) -> Result<bound::TypeStatement, BindError> {
// }

// fn bind_let_statement(
//     item: unbound::LetStatement,
//     db: &mut SymbolDatabase,
//     context_stack: &mut ContextStack,
// ) -> Result<bound::LetStatement, BindError> {
// }

pub use symbol_database::*;
mod symbol_database {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct SymbolDatabase {}

    impl SymbolDatabase {
        pub fn empty() -> Self {
            SymbolDatabase {}
        }
    }

    impl SymbolDatabase {}
}

use context::*;
mod context {
    use super::*;
    use rustc_hash::FxHashMap;

    #[derive(Clone, Debug)]
    pub struct ContextStack(Vec<Context>);

    impl ContextStack {
        pub fn singleton_empty() -> Self {
            Self(vec![Context::empty()])
        }
    }

    impl ContextStack {
        pub fn add_to_top(&mut self, ident: &unbound::Identifier) -> Result<SymbolId, BindError> {
            let top = self
                .0
                .last_mut()
                .expect("Impossible: ContextStack is empty");
            top.add(ident)
        }
    }

    #[derive(Clone, Debug)]
    pub struct Context {
        map: FxHashMap<String, (unbound::Identifier, SymbolId)>,
        lowest_available_id: SymbolId,
    }

    impl Context {
        pub fn empty() -> Self {
            Self {
                map: FxHashMap::default(),
                lowest_available_id: SymbolId(0),
            }
        }
    }

    impl Context {
        pub fn add(&mut self, ident: &unbound::Identifier) -> Result<SymbolId, BindError> {
            if let Some((existing_ident, _)) = self.map.get(&ident.content) {
                Err(BindError::DuplicateSymbol(
                    existing_ident.clone(),
                    ident.clone(),
                ))
            } else {
                let new_id = self.lowest_available_id;
                self.lowest_available_id.0 += 1;
                self.map
                    .insert(ident.content.clone(), (ident.clone(), new_id));
                Ok(new_id)
            }
        }
    }
}
