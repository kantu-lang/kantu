use crate::{lex::Token, unbound_ast::*};

#[derive(Clone, Debug)]
pub enum ParseError {}

pub fn parse_file(tokens: Vec<Token>) -> Result<File, ParseError> {
    // TODO
    Ok(File(vec![]))
}
