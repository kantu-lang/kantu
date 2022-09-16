use crate::{bound_ast as bound, bound_ast::SymbolId, unbound_ast as unbound};

#[derive(Clone, Debug)]
pub enum BindError {
    DuplicateSymbol(unbound::Identifier, unbound::Identifier),
}

pub fn bind_file(file: unbound::File) -> Result<bound::File, BindError> {
    let mut bound_file = bound::File {
        id: file.id,
        items: Vec::with_capacity(file.items.len()),
    };
    let mut context_stack = ContextStack::singleton_empty();
    for item in file.items {
        bound_file
            .items
            .push(bind_file_item(item, &mut context_stack)?);
    }
    Ok(bound_file)
}

fn bind_file_item(
    item: unbound::FileItem,
    context_stack: &mut ContextStack,
) -> Result<bound::FileItem, BindError> {
    match item {
        unbound::FileItem::Type(type_) => {
            bind_type_statement(type_, context_stack).map(bound::FileItem::Type)
        }
        unbound::FileItem::Let(let_) => {
            bind_let_statement(let_, context_stack).map(bound::FileItem::Let)
        }
    }
}

fn bind_type_statement(
    item: unbound::TypeStatement,
    context_stack: &mut ContextStack,
) -> Result<bound::TypeStatement, BindError> {
    let type_sid = context_stack.add_to_top(&item.name)?;
    unimplemented!();
}

fn bind_let_statement(
    item: unbound::LetStatement,
    context_stack: &mut ContextStack,
) -> Result<bound::LetStatement, BindError> {
    unimplemented!()
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
