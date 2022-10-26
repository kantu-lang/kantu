use crate::data::{
    light_ast as rst,
    node_registry::{ExpressionRef, NodeId, NodeRegistry},
    symbol_database::{Symbol, SymbolDatabase, SymbolSource},
    variant_return_type::VariantReturnTypeDatabase,
    FileId,
};
use crate::processing::generate_code::CompileTarget;

use js_ast::*;

use rustc_hash::FxHashMap;

pub mod format;
pub mod js_ast;

mod code_gen_impl;

#[derive(Clone, Debug)]
pub struct JavaScript;

#[derive(Clone, Debug)]
pub enum CompileToJavaScriptError {}

impl CompileTarget for JavaScript {
    type Options = ();
    type Ok = Vec<js_ast::File>;
    type Error = CompileToJavaScriptError;

    fn generate_code_with_options(
        registry: &NodeRegistry,
        symbol_db: &SymbolDatabase,
        _variant_db: &VariantReturnTypeDatabase,
        file_ids: &[NodeId<rst::File>],
        _options: Self::Options,
    ) -> Result<Self::Ok, Self::Error> {
        code_gen_impl::generate_code_with_options(registry, symbol_db, file_ids)
    }
}
