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
    let items = {
        let mut out = vec![];
        for item_id in item_ids {
            match *item_id {
                FileItemNodeId::Type(type_id) => {
                    out.extend(generate_code_for_type_statement(context, state, type_id)?);
                }
                FileItemNodeId::Let(let_id) => {
                    out.push(js_ast::FileItem::Const(generate_code_for_let_statement(
                        context, state, let_id,
                    )?));
                }
            }
        }
        out
    };
    Ok(js_ast::File {
        id: file.file_id,
        items,
    })
}

/// This produces a Const for the type constructor,
/// plus a Const for each variant constructor.
///
/// For example if we have
/// ```pamlihu
/// type List(T: Type) {
///    .Nil(T: Type): List(T),
///    .Cons(T: Type; car: T, cdr: List(T)): List(T),
/// }
/// ```
/// then we need to emit something like:
/// ```js
/// const List_37 = function List_37(T_38) {
///     return { type_: "List_37", args: [T_38] };
/// };
/// const List_37__Nil_39 = function List_37__Nil_39(T_40) {
///     return ["Nil", T_40];
/// };
/// const List_37__Cons_41 = function List_37__Cons_41(T_42, car_43, cdr_44) {
///     return ["Cons", T_42, car_43, cdr_44];
/// };
/// ```
fn generate_code_for_type_statement(
    context: &CodeGenContext,
    state: &mut CodeGenState,
    type_id: NodeId<TypeStatement>,
) -> Result<Vec<js_ast::FileItem>, CompileToJavaScriptError> {
    unimplemented!()
}

fn generate_code_for_let_statement(
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
    let identifier_name = let_statement.name_id.js_name(context, state);
    let value = generate_code_for_expression(context, state, let_statement.value_id)?;
    Ok(js_ast::ConstStatement {
        name: identifier_name,
        value,
    })
}

fn generate_code_for_expression(
    context: &CodeGenContext,
    state: &mut CodeGenState,
    id: ExpressionId,
) -> Result<js_ast::Expression, CompileToJavaScriptError> {
    let expression = context.registry.expression_ref(id);
    match expression {
        ExpressionRef::Name(name) => generate_code_for_name_expression(context, state, name),
        ExpressionRef::Call(call) => generate_code_for_call(context, state, call),
        ExpressionRef::Fun(fun) => generate_code_for_fun(context, state, fun),
        ExpressionRef::Match(match_) => generate_code_for_match(context, state, match_),
        ExpressionRef::Forall(forall) => generate_code_for_forall(context, state, forall),
    }
}

fn generate_code_for_name_expression(
    context: &CodeGenContext,
    state: &mut CodeGenState,
    name: &NameExpression,
) -> Result<js_ast::Expression, CompileToJavaScriptError> {
    let CodeGenContext {
        symbol_db,
        registry,
        ..
    } = context;
    let identifier_name = {
        let symbol = symbol_db
            .identifier_symbols
            .get_using_rightmost((name.id, *registry));

        let component_ids = registry.identifier_list(name.component_list_id);
        let preferred_name = component_ids
            .iter()
            .map(|name_id| name_id.js_name(context, state))
            .collect::<Vec<_>>()
            .join("__");
        state.unique_identifier_name(symbol, Some(&preferred_name))
    };
    Ok(js_ast::Expression::Identifier(identifier_name))
}

fn generate_code_for_call(
    context: &CodeGenContext,
    state: &mut CodeGenState,
    call: &Call,
) -> Result<js_ast::Expression, CompileToJavaScriptError> {
    let callee = generate_code_for_expression(context, state, call.callee_id)?;
    let args = {
        let arg_ids = context.registry.expression_list(call.arg_list_id);
        arg_ids
            .iter()
            .map(|arg_id| generate_code_for_expression(context, state, *arg_id))
            .collect::<Result<Vec<_>, _>>()?
    };
    Ok(js_ast::Expression::Call(Box::new(js_ast::Call {
        callee,
        args,
    })))
}

fn generate_code_for_fun(
    context: &CodeGenContext,
    state: &mut CodeGenState,
    fun: &Fun,
) -> Result<js_ast::Expression, CompileToJavaScriptError> {
    let CodeGenContext { registry, .. } = context;
    let name = fun.name_id.js_name(context, state);
    let param_names = {
        let param_ids = registry.param_list(fun.param_list_id);
        param_ids
            .iter()
            .map(|param_id| param_id.js_name(context, state))
            .collect::<Vec<_>>()
    };
    let body = generate_code_for_expression(context, state, fun.body_id)?;
    Ok(js_ast::Expression::Function(Box::new(js_ast::Function {
        name,
        params: param_names,
        body,
    })))
}

fn generate_code_for_match(
    context: &CodeGenContext,
    state: &mut CodeGenState,
    name: &Match,
) -> Result<js_ast::Expression, CompileToJavaScriptError> {
    unimplemented!()
}

fn generate_code_for_forall(
    context: &CodeGenContext,
    state: &mut CodeGenState,
    name: &Forall,
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
struct CodeGenState {
    fully_qualified_names: FxHashMap<Symbol, String>,
}

impl CodeGenState {
    fn new() -> Self {
        Self {
            fully_qualified_names: Default::default(),
        }
    }
}

impl CodeGenState {
    /// Guarantees that every Symbol will have exactly one name, and that every name will
    /// have at most one Symbol.
    fn unique_identifier_name(&mut self, symbol: Symbol, preferred_name: Option<&str>) -> String {
        if let Some(existing) = self.fully_qualified_names.get(&symbol) {
            return existing.clone();
        }

        const DEFAULT_NAME: &str = "anonymous";
        const NAME_SYMBOL_SEPARATOR: char = '_';

        let unqualified_name = preferred_name.unwrap_or(DEFAULT_NAME);
        let fully_qualified_name = format!(
            "{unq}{sep}{sym}",
            unq = unqualified_name,
            sep = NAME_SYMBOL_SEPARATOR,
            sym = symbol.0
        );
        self.fully_qualified_names
            .insert(symbol, fully_qualified_name.clone());
        fully_qualified_name
    }
}

trait JsName {
    fn js_name(&self, context: &CodeGenContext, state: &mut CodeGenState) -> String;
}

impl JsName for NodeId<Identifier> {
    fn js_name(&self, context: &CodeGenContext, state: &mut CodeGenState) -> String {
        let CodeGenContext {
            symbol_db,
            registry,
            ..
        } = context;
        let symbol = symbol_db.identifier_symbols.get(*self);
        let preferred_name = registry.identifier(*self).name.js_name(context, state);
        state.unique_identifier_name(symbol, Some(&preferred_name))
    }
}

impl JsName for NodeId<Param> {
    fn js_name(&self, context: &CodeGenContext, state: &mut CodeGenState) -> String {
        let param = context.registry.param(*self);
        param.name_id.js_name(context, state)
    }
}

impl JsName for NodeId<NameExpression> {
    fn js_name(&self, context: &CodeGenContext, state: &mut CodeGenState) -> String {
        let CodeGenContext {
            symbol_db,
            registry,
            ..
        } = context;
        let symbol = symbol_db
            .identifier_symbols
            .get_using_rightmost((*self, context.registry));
        let component_ids =
            registry.identifier_list(registry.name_expression(*self).component_list_id);
        let preferred_name = component_ids
            .iter()
            .map(|x| registry.identifier(*x).name.js_name(context, state))
            .collect::<Vec<_>>()
            .join("__");
        state.unique_identifier_name(symbol, Some(&preferred_name))
    }
}

impl JsName for IdentifierName {
    fn js_name(&self, _: &CodeGenContext, _: &mut CodeGenState) -> String {
        match self {
            IdentifierName::Standard(s) => s.to_owned(),
            IdentifierName::Reserved(ReservedIdentifierName::Underscore) => "_".to_owned(),
            IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase) => "Type0".to_owned(),
        }
    }
}
