use crate::data::{
    light_ast as light,
    node_registry::{
        ExpressionRef, FileItemNodeId, LabeledCallArgId, MatchCaseOutputId, NodeId, NodeRegistry,
        NonEmptyCallArgListId, NonEmptyListId, NonEmptyMatchCaseParamListId, NonEmptyParamListId,
    },
    non_empty_vec::OptionalNonEmptyVecLen,
    TextSpan,
};
use crate::processing::generate_code::CompileTarget;

use js_ast::*;

pub mod format;
pub mod js_ast;

mod code_gen_impl;

#[derive(Clone, Debug)]
pub struct JavaScript;

#[derive(Clone, Debug)]
pub enum CompileToJavaScriptError {}

impl CompileTarget for JavaScript {
    type Options = ();
    // TODO: Make this return a tree of files, instead of
    // just a single file.
    type Ok = js_ast::File;
    type Error = CompileToJavaScriptError;

    fn generate_code_with_options(
        registry: &NodeRegistry,
        file_item_list_id: Option<NonEmptyListId<FileItemNodeId>>,
        _options: Self::Options,
    ) -> Result<Self::Ok, Self::Error> {
        code_gen_impl::generate_code_with_options(registry, file_item_list_id)
    }
}
