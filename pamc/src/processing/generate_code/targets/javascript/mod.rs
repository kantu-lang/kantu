use crate::data::{
    light_ast as light,
    node_registry::{
        ExpressionRef, LabeledCallArgId, NodeId, NodeRegistry, NonEmptyCallArgListId,
        NonEmptyMatchCaseParamListId, NonEmptyParamListId,
    },
    non_empty_vec::OptionalNonEmptyVecLen,
    FileId,
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
    type Ok = Vec<js_ast::File>;
    type Error = CompileToJavaScriptError;

    fn generate_code_with_options(
        registry: &NodeRegistry,
        file_ids: &[NodeId<light::File>],
        _options: Self::Options,
    ) -> Result<Self::Ok, Self::Error> {
        code_gen_impl::generate_code_with_options(registry, file_ids)
    }
}
