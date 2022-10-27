use crate::data::bound_ast::*;

#[derive(Clone, Debug)]
pub enum TypeCheckError {}

pub fn type_check_files(files: &[File]) -> Result<(), TypeCheckError> {
    let mut context = Context::with_builtins();
    for file in files {
        type_check_file(&mut context, file)?;
    }
    Ok(())
}

fn type_check_file(context: &mut Context, file: &File) -> Result<(), TypeCheckError> {
    for item in &file.items {
        type_check_file_item(context, item)?;
    }
    Ok(())
}

fn type_check_file_item(context: &mut Context, item: &FileItem) -> Result<(), TypeCheckError> {
    match item {
        FileItem::Type(type_statement) => type_check_type_statement(context, type_statement),
        FileItem::Let(let_statement) => type_check_let_statement(context, let_statement),
    }
}

fn type_check_type_statement(
    _context: &mut Context,
    _type_statement: &TypeStatement,
) -> Result<(), TypeCheckError> {
    unimplemented!()
}

fn type_check_let_statement(
    _context: &mut Context,
    _let_statement: &LetStatement,
) -> Result<(), TypeCheckError> {
    unimplemented!()
}

use context::*;
mod context {
    pub struct Context {}

    impl Context {
        pub fn with_builtins() -> Self {
            Self {}
        }
    }
}
