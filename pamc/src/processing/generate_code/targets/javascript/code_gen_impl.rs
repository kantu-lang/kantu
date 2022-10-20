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
    let CodeGenContext {
        symbol_db,
        registry,
        ..
    } = context;
    let let_statement = registry.let_statement(let_id);
    let identifier_name = {
        let symbol = symbol_db.identifier_symbols.get(let_statement.name_id);
        let preferred_name = registry.identifier(let_statement.name_id).name.to_js_name();
        state.unique_identifier_name(symbol, Some(&preferred_name))
    };
    let value = generate_code_for_expression(context, state, let_statement.value_id)?;
    Ok(js_ast::ConstStatement {
        name: identifier_name,
        value,
    })
}

fn generate_code_for_expression(
    context: &CodeGenContext,
    state: &mut CodeGenState,
    let_id: ExpressionId,
) -> Result<js_ast::Expression, CompileToJavaScriptError> {
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

impl CodeGenState {
    fn unique_identifier_name(&mut self, symbol: Symbol, preferred_name: Option<&str>) -> String {
        format!("{}_{}", preferred_name.unwrap_or("anonymous"), symbol.0)
    }
}

impl IdentifierName {
    fn to_js_name(&self) -> String {
        match self {
            IdentifierName::Standard(s) => s.to_owned(),
            IdentifierName::Reserved(ReservedIdentifierName::Underscore) => "_".to_owned(),
            IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase) => "Type0".to_owned(),
        }
    }
}
