use super::*;

type Options = <JavaScript as CompileTarget>::Options;

pub fn generate_code_with_options(
    symbol_db: &SymbolDatabase,
    registry: &NodeRegistry,
    variant_db: &VariantReturnTypeDatabase,
    file_ids: &[NodeId<File>],
    options: Options,
) -> Result<Vec<js_ast::File>, CompileToJavaScriptError> {
    let context = CodeGenContext {
        symbol_db,
        registry,
        variant_db,
        options: &options,
    };
    let mut state = CodeGenState::new();
    file_ids
        .iter()
        .map(|file_id| generate_code_for_file(&context, &mut state, *file_id))
        .collect()
}

fn generate_code_for_file(
    context: &CodeGenContext,
    state: &mut CodeGenState,
    file_id: NodeId<File>,
) -> Result<js_ast::File, CompileToJavaScriptError> {
    let file = context.registry.file(file_id);
    let item_ids = context.registry.file_item_list(file.item_list_id);
    let items = item_ids
        .iter()
        .copied()
        .filter_map(|item_id| match item_id {
            FileItemNodeId::Type(_) => None,
            FileItemNodeId::Let(let_id) => Some(
                generate_code_for_let_declaration(context, state, let_id)
                    .map(js_ast::FileItem::Const),
            ),
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(js_ast::File {
        id: file.file_id,
        items,
    })
}

fn generate_code_for_let_declaration(
    context: &CodeGenContext,
    state: &mut CodeGenState,
    let_id: NodeId<LetStatement>,
) -> Result<js_ast::ConstStatement, CompileToJavaScriptError> {
    unimplemented!()
}

#[derive(Clone, Debug)]
struct CodeGenContext<'a> {
    symbol_db: &'a SymbolDatabase,
    registry: &'a NodeRegistry,
    variant_db: &'a VariantReturnTypeDatabase,
    options: &'a Options,
}

#[derive(Clone, Debug)]
struct CodeGenState;

impl CodeGenState {
    fn new() -> Self {
        Self {}
    }
}
