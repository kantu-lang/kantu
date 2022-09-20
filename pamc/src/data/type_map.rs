use crate::data::registered_ast::*;
use rustc_hash::FxHashMap;

#[derive(Clone, Debug)]
pub struct TypeMap {
    raw: FxHashMap<usize, WrappedExpression>,
}
