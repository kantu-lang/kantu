use light::{DbIndex, DbLevel};

use super::*;

pub fn generate_code_with_options(
    registry: &NodeRegistry,
    file_ids: &[NodeId<light::File>],
) -> Result<Vec<File>, CompileToJavaScriptError> {
    file_ids
        .iter()
        .map(|file_id| generate_code_for_file(registry, *file_id))
        .collect()
}

fn generate_code_for_file(
    registry: &NodeRegistry,
    file_id: NodeId<light::File>,
) -> Result<File, CompileToJavaScriptError> {
    let mut context = Context::new();
    let file = registry.file(file_id);
    let item_ids = registry.file_item_list(file.item_list_id);
    let items = {
        let mut out = vec![];
        out.extend(generate_code_for_type1_and_type0_without_adding_to_context());
        out.extend(generate_code_for_explosion_thrower());
        for item_id in item_ids {
            match *item_id {
                light::FileItemNodeId::Type(type_id) => {
                    out.extend(generate_code_for_type_statement(
                        registry,
                        &mut context,
                        type_id,
                    )?);
                }
                light::FileItemNodeId::Let(let_id) => {
                    out.push(FileItem::Const(generate_code_for_let_statement(
                        registry,
                        &mut context,
                        let_id,
                    )?));
                }
            }
        }
        out
    };
    Ok(File {
        id: file.file_id,
        items,
    })
}

fn generate_code_for_type1_and_type0_without_adding_to_context() -> Vec<FileItem> {
    vec![
        FileItem::Const(ConstStatement {
            name: ValidJsIdentifierName(TYPE_SPECIES_VALUE__TYPE1.to_string()),
            value: Expression::Object(Box::new(Object {
                entries: vec![ObjectEntry {
                    key: ValidJsIdentifierName(TYPE_SPECIES_KEY.to_string()),
                    value: Expression::Literal(Literal::String {
                        unescaped: TYPE_SPECIES_VALUE__TYPE1.to_string(),
                    }),
                }],
            })),
        }),
        FileItem::Const(ConstStatement {
            name: ValidJsIdentifierName(TYPE_SPECIES_VALUE__TYPE0.to_string()),
            value: Expression::Object(Box::new(Object {
                entries: vec![ObjectEntry {
                    key: ValidJsIdentifierName(TYPE_SPECIES_KEY.to_string()),
                    value: Expression::Literal(Literal::String {
                        unescaped: TYPE_SPECIES_VALUE__TYPE0.to_string(),
                    }),
                }],
            })),
        }),
    ]
}

fn generate_code_for_explosion_thrower() -> Vec<FileItem> {
    let thrower_name = ValidJsIdentifierName(EXPLOSION_THROWER_NAME.to_string());
    vec![FileItem::Const(ConstStatement {
        name: thrower_name.clone(),
        value: Expression::Function(Box::new(Function {
            name: thrower_name,
            params: vec![],
            body: vec![FunctionStatement::Throw(Expression::New(Box::new(Call {
                callee: Expression::Identifier(ValidJsIdentifierName("Error".to_string())),
                args: vec![Expression::Literal(Literal::String {
                    unescaped: "Reached supposedly unreachable path. This likely indicates that you passed one or more illegal arguments to one or more of the generated functions.".to_string(),
                })],
            })))],
        })),
    })]
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
    registry: &NodeRegistry,
    context: &mut Context,
    type_id: NodeId<light::TypeStatement>,
) -> Result<Vec<FileItem>, CompileToJavaScriptError> {
    let type_ = registry.type_statement(type_id);
    let variant_ids = registry.variant_list(type_.variant_list_id);
    let mut out = Vec::with_capacity(variant_ids.len() + 1);

    let type_constructor = generate_code_for_type_constructor(registry, context, type_id)?;
    out.push(FileItem::Const(type_constructor));
    let type_constructor_js_name = context.js_name(DbIndex(0));

    for variant_id in variant_ids {
        let variant_constructor = generate_code_for_variant_constructor(
            registry,
            context,
            *variant_id,
            &type_constructor_js_name,
        )?;
        out.push(FileItem::Const(variant_constructor));
    }

    Ok(out)
}

fn generate_code_for_type_constructor(
    registry: &NodeRegistry,
    context: &mut Context,
    type_id: NodeId<light::TypeStatement>,
) -> Result<ConstStatement, CompileToJavaScriptError> {
    let type_ = registry.type_statement(type_id);

    let param_js_names: Vec<ValidJsIdentifierName> = {
        let type_param_ids = registry.param_list(type_.param_list_id);
        let param_js_names = type_param_ids
            .iter()
            .map(|id| {
                let param = registry.param(*id);
                let param_name = &registry.identifier(param.name_id).name;
                context.try_push_name(param_name.js_name());
                let param_js_name = context.js_name(DbIndex(0));
                param_js_name
            })
            .collect();
        context.pop_n(type_param_ids.len());
        param_js_names
    };

    let type_name = &registry.identifier(type_.name_id).name;
    context.try_push_name(type_name.js_name());
    let type_js_name = context.js_name(DbIndex(0));

    let return_value = Expression::Object(Box::new(Object {
        entries: vec![
            ObjectEntry {
                key: ValidJsIdentifierName(TYPE_SPECIES_KEY.to_string()),
                value: Expression::Literal(Literal::String {
                    unescaped: type_js_name.0.clone(),
                }),
            },
            ObjectEntry {
                key: ValidJsIdentifierName(TYPE_ARGS_KEY.to_string()),
                value: Expression::Array(Box::new(Array {
                    items: param_js_names
                        .iter()
                        .map(|param| Expression::Identifier(param.clone()))
                        .collect(),
                })),
            },
        ],
    }));

    Ok(ConstStatement {
        name: type_js_name.clone(),
        value: Function {
            name: type_js_name,
            params: param_js_names,
            body: vec![FunctionStatement::Return(return_value)],
        }
        .into_return_value_if_simple_nullary(),
    })
}

fn generate_code_for_variant_constructor(
    registry: &NodeRegistry,
    context: &mut Context,
    variant_id: NodeId<light::Variant>,
    type_constructor_js_name: &ValidJsIdentifierName,
) -> Result<ConstStatement, CompileToJavaScriptError> {
    let variant = registry.variant(variant_id);
    let arity = variant.param_list_id.len;

    let param_js_names: Vec<ValidJsIdentifierName> = {
        let type_param_ids = registry.param_list(variant.param_list_id);
        let param_js_names = type_param_ids
            .iter()
            .map(|id| {
                let param = registry.param(*id);
                let param_name = &registry.identifier(param.name_id).name;
                context.try_push_name(param_name.js_name());
                let param_js_name = context.js_name(DbIndex(0));
                param_js_name
            })
            .collect();
        param_js_names
    };
    let return_value = {
        let mut items = Vec::with_capacity(arity + 1);
        items.push(Expression::Literal(Literal::String {
            // We must use the JS name instead of the Symbol JS name
            // since we won't know the Symbol of the match case patterns
            // (at the time of writing, the type checker isn't complete,
            // so we don't have that information during code generation).
            // Later, we can fix this.
            unescaped: registry.identifier(variant.name_id).name.js_name().0,
        }));
        items.extend(
            param_js_names
                .iter()
                .map(|param| Expression::Identifier(param.clone())),
        );
        Expression::Array(Box::new(Array { items }))
    };

    context.pop_n(arity);

    let variant_name = &registry.identifier(variant.name_id).name;
    context.try_push_name(ValidJsIdentifierName(format!(
        "{}_{}",
        &type_constructor_js_name.0,
        variant_name.js_name().0,
    )));
    let variant_symbol_js_name = context.js_name(DbIndex(0));

    Ok(ConstStatement {
        name: variant_symbol_js_name.clone(),
        value: Function {
            name: variant_symbol_js_name,
            params: param_js_names,
            body: vec![FunctionStatement::Return(return_value)],
        }
        .into_return_value_if_simple_nullary(),
    })
}

fn generate_code_for_let_statement(
    registry: &NodeRegistry,
    context: &mut Context,
    let_id: NodeId<light::LetStatement>,
) -> Result<ConstStatement, CompileToJavaScriptError> {
    let let_statement = registry.let_statement(let_id);

    let value = generate_code_for_expression(registry, context, let_statement.value_id)?;

    let let_statement_name = &registry.identifier(let_statement.name_id).name;
    context.try_push_name(let_statement_name.js_name());
    let let_statement_js_name = context.js_name(DbIndex(0));
    Ok(ConstStatement {
        name: let_statement_js_name,
        value,
    })
}

fn generate_code_for_expression(
    registry: &NodeRegistry,
    context: &mut Context,
    id: light::ExpressionId,
) -> Result<Expression, CompileToJavaScriptError> {
    let expression = registry.expression_ref(id);
    match expression {
        ExpressionRef::Name(name) => generate_code_for_name_expression(registry, context, name),
        ExpressionRef::Call(call) => generate_code_for_call(registry, context, call),
        ExpressionRef::Fun(fun) => generate_code_for_fun(registry, context, fun),
        ExpressionRef::Match(match_) => generate_code_for_match(registry, context, match_),
        ExpressionRef::Forall(forall) => generate_code_for_forall(registry, context, forall),
    }
}

fn generate_code_for_name_expression(
    _registry: &NodeRegistry,
    context: &mut Context,
    name: &light::NameExpression,
) -> Result<Expression, CompileToJavaScriptError> {
    let identifier_name = context.js_name(name.db_index);
    Ok(Expression::Identifier(identifier_name))
}

fn generate_code_for_call(
    registry: &NodeRegistry,
    context: &mut Context,
    call: &light::Call,
) -> Result<Expression, CompileToJavaScriptError> {
    let callee = generate_code_for_expression(registry, context, call.callee_id)?;
    let args = {
        let arg_ids = registry.expression_list(call.arg_list_id);
        arg_ids
            .iter()
            .map(|arg_id| generate_code_for_expression(registry, context, *arg_id))
            .collect::<Result<Vec<_>, _>>()?
    };
    Ok(Expression::Call(Box::new(Call { callee, args })))
}

fn generate_code_for_fun(
    registry: &NodeRegistry,
    context: &mut Context,
    fun: &light::Fun,
) -> Result<Expression, CompileToJavaScriptError> {
    let param_arity = fun.param_list_id.len;
    let param_js_names = registry
        .param_list(fun.param_list_id)
        .iter()
        .map(|id| {
            let param = registry.param(*id);
            let param_name = &registry.identifier(param.name_id).name;
            context.try_push_name(param_name.js_name());
            let param_js_name = context.js_name(DbIndex(0));
            param_js_name
        })
        .collect();
    let fun_js_name = {
        let fun_name = &registry.identifier(fun.name_id).name;
        context.try_push_name(fun_name.js_name());
        context.js_name(DbIndex(0))
    };
    let return_value = generate_code_for_expression(registry, context, fun.body_id)?;
    context.pop_n(param_arity + 1);
    Ok(Expression::Function(Box::new(Function {
        name: fun_js_name,
        params: param_js_names,
        body: vec![FunctionStatement::Return(return_value)],
    })))
}

fn generate_code_for_match(
    registry: &NodeRegistry,
    context: &mut Context,
    match_: &light::Match,
) -> Result<Expression, CompileToJavaScriptError> {
    let matchee_temp_name = context.get_disposable_name();
    let fun_temp_name = context.get_disposable_name();

    let matchee = generate_code_for_expression(registry, context, match_.matchee_id)?;
    let cases = registry
        .match_case_list(match_.case_list_id)
        .iter()
        .map(|case_id| {
            let case = registry.match_case(*case_id);
            generate_code_for_match_case(registry, context, case, &matchee_temp_name)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Expression::Call(Box::new(Call {
        callee: Expression::Function(Box::new(Function {
            name: fun_temp_name,
            params: vec![matchee_temp_name.clone()],
            body: cases.into_iter().map(FunctionStatement::If).collect(),
        })),
        args: vec![matchee],
    })))
}

fn generate_code_for_match_case(
    registry: &NodeRegistry,
    context: &mut Context,
    case: &light::MatchCase,
    matchee_js_name: &ValidJsIdentifierName,
) -> Result<IfStatement, CompileToJavaScriptError> {
    let condition = {
        let case_js_name = registry.identifier(case.variant_name_id).name.js_name();
        Expression::BinaryOp(Box::new(BinaryOp {
            left: Expression::BinaryOp(Box::new(BinaryOp {
                left: Expression::Identifier(matchee_js_name.clone()),
                op: BinaryOpKind::Index,
                right: Expression::Literal(Literal::Number(0)),
            })),
            op: BinaryOpKind::TripleEqual,
            right: Expression::Literal(Literal::String {
                unescaped: case_js_name.0,
            }),
        }))
    };

    let body = {
        let arity = case.param_list_id.len;
        let mut body = Vec::with_capacity(arity + 1);

        for (param_index, param_id) in registry
            .identifier_list(case.param_list_id)
            .iter()
            .copied()
            .enumerate()
        {
            let param_name = &registry.identifier(param_id).name;
            context.try_push_name(param_name.js_name());
            let param_js_name = context.js_name(DbIndex(0));
            let field_index = i32::try_from(1 + param_index)
                .expect("The param index should not be absurdly large.");
            let param_value = Expression::BinaryOp(Box::new(BinaryOp {
                left: Expression::Identifier(matchee_js_name.clone()),
                op: BinaryOpKind::Index,
                right: Expression::Literal(Literal::Number(field_index)),
            }));
            body.push(FunctionStatement::Const(ConstStatement {
                name: param_js_name,
                value: param_value,
            }));
        }

        body.push(FunctionStatement::Return(generate_code_for_expression(
            registry,
            context,
            case.output_id,
        )?));

        context.pop_n(arity);

        body
    };

    Ok(IfStatement { condition, body })
}

const TYPE_SPECIES_KEY: &str = "type_species";
const TYPE_SPECIES_VALUE__TYPE1: &str = "Type1";
const TYPE_SPECIES_VALUE__TYPE0: &str = "Type0";
const TYPE_SPECIES_VALUE__FORALL: &str = "forall";
const TYPE_ARGS_KEY: &str = "type_args";
const EXPLOSION_THROWER_NAME: &str = "unreachable";
const DISPOSABLE_NAME_PREFIX: &str = "temp";

fn generate_code_for_forall(
    _registry: &NodeRegistry,
    _context: &mut Context,
    _name: &light::Forall,
) -> Result<Expression, CompileToJavaScriptError> {
    Ok(Expression::Object(Box::new(Object {
        entries: vec![ObjectEntry {
            key: ValidJsIdentifierName(TYPE_SPECIES_KEY.to_string()),
            value: Expression::Literal(Literal::String {
                unescaped: TYPE_SPECIES_VALUE__FORALL.to_string(),
            }),
        }],
    })))
}

impl Function {
    /// A "simple" function is one that has a body that has a return statement
    /// and no other statements.
    /// This method returns the return value of the function if it is a simple
    /// function with zero parameters, otherwise it returns the function itself.
    ///
    /// Invocations of nullary type constructors and variant constructors
    /// are represented as identifiers, not calls.
    /// Thus, when we're declaring these constructors, we must make sure
    /// to declare non-nullary constructors as values rather than nullary
    /// functions.
    fn into_return_value_if_simple_nullary(self) -> Expression {
        if self.params.is_empty() && matches!(&self.body[..], [FunctionStatement::Return(_)]) {
            match self.body.into_iter().next() {
                Some(FunctionStatement::Return(value)) => value,
                _ => unreachable!(),
            }
        } else {
            Expression::Function(Box::new(self))
        }
    }
}

impl light::IdentifierName {
    fn js_name(&self) -> ValidJsIdentifierName {
        match self {
            light::IdentifierName::Standard(s) => sanitize_js_identifier_name(s),
            light::IdentifierName::Reserved(light::ReservedIdentifierName::Underscore) => {
                ValidJsIdentifierName("_".to_string())
            }
            light::IdentifierName::Reserved(light::ReservedIdentifierName::TypeTitleCase) => {
                ValidJsIdentifierName(TYPE_SPECIES_VALUE__TYPE0.to_string())
            }
        }
    }
}

fn sanitize_js_identifier_name(s: &str) -> ValidJsIdentifierName {
    let mut out = String::new();

    // The first character cannot be a digit
    for c in s.chars().take(1) {
        if c.is_ascii_alphabetic() || c == '_' || c == '$' {
            out.push(c);
        } else {
            out.push('_');
        }
    }

    // ...but the rest can be digits.
    for c in s.chars().skip(1) {
        if c.is_ascii_alphanumeric() || c == '_' || c == '$' {
            out.push(c);
        } else {
            out.push('_');
        }
    }

    while is_js_reserved_word(&out) {
        out.push('_');
    }

    ValidJsIdentifierName(out)
}

fn is_js_reserved_word(s: &str) -> bool {
    [
        "abstract",
        "arguments",
        "as",
        "async",
        "await",
        "boolean",
        "break",
        "byte",
        "case",
        "catch",
        "char",
        "class",
        "const",
        "continue",
        "debugger",
        "default",
        "delete",
        "do",
        "double",
        "else",
        "enum",
        "eval",
        "export",
        "extends",
        "false",
        "final",
        "finally",
        "float",
        "for",
        "from",
        "function",
        "get",
        "goto",
        "if",
        "implements",
        "import",
        "in",
        "instanceof",
        "int",
        "interface",
        "let",
        "long",
        "native",
        "new",
        "null",
        "of",
        "package",
        "private",
        "protected",
        "public",
        "return",
        "set",
        "short",
        "static",
        "super",
        "switch",
        "synchronized",
        "this",
        "throw",
        "throws",
        "transient",
        "true",
        "try",
        "typeof",
        "undefined",
        "var",
        "void",
        "volatile",
        "while",
        "with",
        "yield",
    ]
    .iter()
    .any(|&reserved| reserved == s)
}

#[derive(Clone, Debug)]
struct Context {
    stack: Vec<ContextEntry>,
    other_reserved_names: Vec<ValidJsIdentifierName>,
}

#[derive(Clone, Debug)]
struct ContextEntry {
    name: ValidJsIdentifierName,
}

impl Context {
    fn new() -> Self {
        Self {
            stack: vec![
                ContextEntry {
                    name: ValidJsIdentifierName(TYPE_SPECIES_VALUE__TYPE1.to_string()),
                },
                ContextEntry {
                    name: ValidJsIdentifierName(TYPE_SPECIES_VALUE__TYPE0.to_string()),
                },
            ],
            other_reserved_names: vec![ValidJsIdentifierName(EXPLOSION_THROWER_NAME.to_string())],
        }
    }
}

impl Context {
    fn index_to_level(&self, index: DbIndex) -> DbLevel {
        DbLevel(self.stack.len() - index.0 - 1)
    }
}

impl Context {
    fn js_name(&self, index: DbIndex) -> ValidJsIdentifierName {
        let level = self.index_to_level(index);
        self.stack[level.0].name.clone()
    }

    fn try_push_name(&mut self, preferred: ValidJsIdentifierName) {
        if !self.contains(&preferred) {
            self.push(ContextEntry { name: preferred });
            return;
        }

        let mut i = 2;
        loop {
            let name = ValidJsIdentifierName(format!("{}{}", preferred.0, i));
            if !self.contains(&name) {
                self.push(ContextEntry { name });
                return;
            }
            i += 1;
        }
    }

    fn get_disposable_name(&mut self) -> ValidJsIdentifierName {
        let mut i = 0;
        loop {
            let name = ValidJsIdentifierName(format!("{}_{:x}", DISPOSABLE_NAME_PREFIX, i));
            if !self.contains(&name) {
                self.other_reserved_names.push(name.clone());
                return name;
            }
            i += 1;
        }
    }

    fn contains(&self, name: &ValidJsIdentifierName) -> bool {
        self.stack.iter().any(|x| x.name == *name)
            || self.other_reserved_names.iter().any(|x| x == name)
    }

    fn push(&mut self, entry: ContextEntry) {
        self.stack.push(entry);
    }

    fn pop_n(&mut self, n: usize) {
        self.stack.truncate(self.stack.len() - n);
    }
}
