use super::*;
// TODO: Use js_ast::* instead of registered_sst::*.

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
    match_: &Match,
) -> Result<js_ast::Expression, CompileToJavaScriptError> {
    let case_ids = context.registry.match_case_list(match_.case_list_id);
    let matchee = generate_code_for_expression(context, state, match_.matchee_id)?;
    generate_ternary_for_match_cases(context, state, &matchee, case_ids)
}

fn generate_ternary_for_match_cases(
    context: &CodeGenContext,
    state: &mut CodeGenState,
    matchee: &js_ast::Expression,
    case_ids: &[NodeId<MatchCase>],
) -> Result<js_ast::Expression, CompileToJavaScriptError> {
    if case_ids.is_empty() {
        return Ok(generate_code_for_explosion_error(context, state));
    }

    let CodeGenContext { registry, .. } = context;

    let case = registry.match_case(case_ids[0]);
    let case_variant_name: String = registry.identifier(case.variant_name_id).name.js_name();
    let condition = js_ast::Expression::BinaryOp(Box::new(js_ast::BinaryOp {
        left: js_ast::Expression::BinaryOp(Box::new(js_ast::BinaryOp {
            left: matchee.clone(),
            op: js_ast::BinaryOpKind::Index,
            right: js_constant_zero(),
        })),
        op: js_ast::BinaryOpKind::TripleEqual,
        right: js_ast::Expression::Literal(js_ast::Literal::String {
            unescaped: case_variant_name,
        }),
    }));
    let true_body = generate_code_for_expression(context, state, case.output_id)?;
    let false_body = generate_ternary_for_match_cases(context, state, matchee, &case_ids[1..])?;

    Ok(js_ast::Expression::Ternary(Box::new(js_ast::Ternary {
        condition,
        true_body,
        false_body,
    })))
}

fn generate_code_for_explosion_error(
    context: &CodeGenContext,
    state: &mut CodeGenState,
) -> js_ast::Expression {
    js_ast::Expression::Call(Box::new(js_ast::Call {
        callee: js_ast::Expression::Identifier(state.explosion_error_thrower_function_name()),
        args: vec![],
    }))
}

fn generate_code_for_forall(
    context: &CodeGenContext,
    state: &mut CodeGenState,
    name: &Forall,
) -> Result<js_ast::Expression, CompileToJavaScriptError> {
    unimplemented!()
}

fn js_constant_zero() -> js_ast::Expression {
    js_ast::Expression::Literal(js_ast::Literal::Number(0))
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

    fn explosion_error_thrower_function_name(&mut self) -> String {
        /// This is safe because all non-special identifier names end with `_`
        /// followed by one or more digits.
        /// Since this does not end with a digit, it is guaranteed to not collide.
        const EXPLOSION_THROWER_NAME: &str = "unreachable_path";
        EXPLOSION_THROWER_NAME.to_owned()
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
        let preferred_name = registry.identifier(*self).name.js_name();
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
            .map(|x| registry.identifier(*x).name.js_name())
            .collect::<Vec<_>>()
            .join("__");
        state.unique_identifier_name(symbol, Some(&preferred_name))
    }
}

impl IdentifierName {
    fn js_name(&self) -> String {
        match self {
            IdentifierName::Standard(s) => s.to_owned(),
            IdentifierName::Reserved(ReservedIdentifierName::Underscore) => "_".to_owned(),
            IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase) => "Type0".to_owned(),
        }
    }
}
