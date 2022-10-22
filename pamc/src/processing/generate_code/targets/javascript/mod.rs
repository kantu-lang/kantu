use crate::data::{
    node_free_variable_cache::NodeFreeVariableCache,
    node_hash_cache::NodeStructuralIdentityHashCache,
    node_registry::{ExpressionRef, ListId, NodeId, NodeRegistry},
    registered_sst as rst,
    symbol_database::{IdentifierToSymbolMap, Symbol, SymbolDatabase, SymbolSource},
    variant_return_type::VariantReturnTypeDatabase,
    FileId,
};
use crate::processing::generate_code::CompileTarget;

use js_ast::*;

use rustc_hash::FxHashMap;

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
        symbol_db: &SymbolDatabase,
        registry: &NodeRegistry,
        variant_db: &VariantReturnTypeDatabase,
        file_ids: &[NodeId<rst::File>],
        options: Self::Options,
    ) -> Result<Self::Ok, Self::Error> {
        code_gen_impl::generate_code_with_options(
            symbol_db, registry, variant_db, file_ids, options,
        )
    }
}
