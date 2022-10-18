use super::*;

pub fn generate_code_with_options(
    symbol_db: &SymbolDatabase,
    registry: &NodeRegistry,
    variant_db: &VariantReturnTypeDatabase,
    file_ids: &[NodeId<File>],
    options: (),
) -> Result<Vec<js_ast::File>, CompileToJavaScriptError> {
    unimplemented!()
}
