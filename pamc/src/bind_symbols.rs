use crate::{bound_ast as bound, unbound_ast as unbound};

#[derive(Clone, Debug)]
pub enum BindError {}

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
        context_stack: ContextStack::empty(),
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
        unimplemented!()
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

    #[derive(Clone, Debug)]
    pub struct ContextStack(Vec<Context>);

    impl ContextStack {
        pub fn empty() -> Self {
            Self(vec![])
        }
    }

    #[derive(Clone, Debug)]
    pub struct Context {}
}
