use light::{DbIndex, DbLevel};

use super::*;

pub fn generate_code_with_options(
    registry: &NodeRegistry,
    file_item_list_id: Option<NonEmptyListId<FileItemNodeId>>,
) -> Result<File, CompileToJavaScriptError> {
    let mut context = Context::new();
    let item_ids = registry.get_possibly_empty_list(file_item_list_id);
    let items = {
        let mut out = vec![];
        out.extend(generate_code_for_type1_and_type0_without_adding_to_context());
        out.extend(generate_code_for_explosion_thrower());
        out.extend(generate_code_for_todo_error_thrower());
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
    Ok(File { items })
}

fn generate_code_for_type1_and_type0_without_adding_to_context() -> Vec<FileItem> {
    vec![
        FileItem::Const(ConstStatement {
            name: ValidJsIdentifierName(TYPE_SPECIES_VALUE__TYPE1.to_string()),
            value: Expression::Object(Box::new(Object {
                entries: vec![ObjectEntry {
                    key: ValidJsIdentifierName(TYPE_SPECIES_KEY.to_string()),
                    value: Expression::Literal(Literal::String(JsStringLiteral {
                        unescaped: TYPE_SPECIES_VALUE__TYPE1.to_string(),
                    })),
                }],
            })),
        }),
        FileItem::Const(ConstStatement {
            name: ValidJsIdentifierName(TYPE_SPECIES_VALUE__TYPE0.to_string()),
            value: Expression::Object(Box::new(Object {
                entries: vec![ObjectEntry {
                    key: ValidJsIdentifierName(TYPE_SPECIES_KEY.to_string()),
                    value: Expression::Literal(Literal::String(JsStringLiteral {
                        unescaped: TYPE_SPECIES_VALUE__TYPE0.to_string(),
                    })),
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
            params: Params::Standard(vec![
                ValidJsIdentifierName(EXPLOSION_THROWER_PARAM0_NAME.to_string()),
            ]),
            body: vec![FunctionStatement::Throw(Expression::New(Box::new(Call {
                callee: Expression::Identifier(ValidJsIdentifierName("Error".to_string())),
                args: vec![Expression::BinaryOp(Box::new(BinaryOp {
                    op: BinaryOpKind::Plus,
                    left: Expression::Literal(Literal::String(JsStringLiteral {
                        unescaped: "Reached supposedly unreachable path. This likely indicates that you passed one or more illegal arguments to one or more of the generated functions. Responsible span: ".to_string(),
                    })),
                    right: Expression::Identifier(ValidJsIdentifierName(
                        EXPLOSION_THROWER_PARAM0_NAME.to_string(),
                    )),
                }))],
            })))],
        })),
    })]
}

fn generate_code_for_todo_error_thrower() -> Vec<FileItem> {
    let thrower_name = ValidJsIdentifierName(TODO_ERROR_THROWER_NAME.to_string());
    vec![FileItem::Const(ConstStatement {
        name: thrower_name.clone(),
        value: Expression::Function(Box::new(Function {
            name: thrower_name,
            params: Params::Standard(vec![ValidJsIdentifierName(
                TODO_ERROR_THROWER_PARAM0_NAME.to_string(),
            )]),
            body: vec![FunctionStatement::Throw(Expression::New(Box::new(Call {
                callee: Expression::Identifier(ValidJsIdentifierName("Error".to_string())),
                args: vec![Expression::BinaryOp(Box::new(BinaryOp {
                    op: BinaryOpKind::Plus,
                    left: Expression::Literal(Literal::String(JsStringLiteral {
                        unescaped: "This functionality is not implemented. Source span: "
                            .to_string(),
                    })),
                    right: Expression::Identifier(ValidJsIdentifierName(
                        TODO_ERROR_THROWER_PARAM0_NAME.to_string(),
                    )),
                }))],
            })))],
        })),
    })]
}

/// This produces a Const for the type constructor,
/// plus a Const for each variant constructor.
///
/// For example if we have
/// ```kantu
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
    let type_ = registry.get(type_id);
    let variant_ids = registry.get_possibly_empty_list(type_.variant_list_id);
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
    let type_ = registry.get(type_id);

    let params = generate_code_for_optional_params_and_leave_params_in_context(
        registry,
        context,
        type_.param_list_id,
    )?;
    context.pop_n(type_.param_list_id.len());

    let type_name = &registry.get(type_.name_id).name;
    context.try_push_name(type_name.preferred_js_name());
    let type_js_name = context.js_name(DbIndex(0));

    let type_args = match &params {
        Params::Standard(param_js_names) => Expression::Array(Box::new(Array {
            items: param_js_names
                .iter()
                .map(|param| Expression::Identifier(param.clone()))
                .collect(),
        })),
        Params::DestructuredSingleton(entries) => Expression::Object(Box::new(Object {
            entries: entries
                .iter()
                .map(|entry| ObjectEntry {
                    key: entry.in_name.clone(),
                    value: Expression::Identifier(entry.out_name.clone()),
                })
                .collect(),
        })),
    };

    let return_value = Expression::Object(Box::new(Object {
        entries: vec![
            ObjectEntry {
                key: ValidJsIdentifierName(TYPE_SPECIES_KEY.to_string()),
                value: Expression::Literal(Literal::String(JsStringLiteral {
                    unescaped: type_js_name.0.clone(),
                })),
            },
            ObjectEntry {
                key: ValidJsIdentifierName(TYPE_ARGS_KEY.to_string()),
                value: type_args,
            },
        ],
    }));

    Ok(ConstStatement {
        name: type_js_name.clone(),
        value: Function {
            name: type_js_name,
            params,
            body: vec![FunctionStatement::Return(return_value)],
        }
        .into_return_value_if_simple_nullary(),
    })
}

fn generate_code_for_optional_params_and_leave_params_in_context(
    registry: &NodeRegistry,
    context: &mut Context,
    param_list_id: Option<NonEmptyParamListId>,
) -> Result<Params, CompileToJavaScriptError> {
    match param_list_id {
        Some(param_list_id) => {
            generate_code_for_params_and_leave_params_in_context(registry, context, param_list_id)
        }
        None => Ok(Params::Standard(vec![])),
    }
}

fn generate_code_for_params_and_leave_params_in_context(
    registry: &NodeRegistry,
    context: &mut Context,
    param_list_id: NonEmptyParamListId,
) -> Result<Params, CompileToJavaScriptError> {
    Ok(match param_list_id {
        NonEmptyParamListId::Unlabeled(param_list_id) => {
            let param_ids = registry.get_list(param_list_id);
            let param_js_names = param_ids
                .iter()
                .map(|id| {
                    let param = registry.get(*id);
                    let param_name = &registry.get(param.name_id).name;
                    context.try_push_name(param_name.preferred_js_name());
                    let param_js_name = context.js_name(DbIndex(0));
                    param_js_name
                })
                .collect();
            Params::Standard(param_js_names)
        }
        NonEmptyParamListId::UniquelyLabeled(param_list_id) => {
            let param_ids = registry.get_list(param_list_id);
            let entries = param_ids
                .iter()
                .map(|&id| {
                    let param = registry.get(id);
                    let param_name = &registry.get(param.name_id).name;
                    context.try_push_name(param_name.preferred_js_name());
                    let param_js_name = context.js_name(DbIndex(0));

                    let param_label_name = &registry.get(param.label()).name;
                    let param_label_js_name = param_label_name.preferred_js_name();
                    ObjectDestructureEntry {
                        in_name: param_label_js_name,
                        out_name: param_js_name,
                    }
                })
                .collect();
            Params::DestructuredSingleton(entries)
        }
    })
}

fn generate_code_for_variant_constructor(
    registry: &NodeRegistry,
    context: &mut Context,
    variant_id: NodeId<light::Variant>,
    type_constructor_js_name: &ValidJsIdentifierName,
) -> Result<ConstStatement, CompileToJavaScriptError> {
    let variant = registry.get(variant_id);
    let arity = variant.param_list_id.len();

    let params = generate_code_for_optional_params_and_leave_params_in_context(
        registry,
        context,
        variant.param_list_id,
    )?;

    let type_args: Vec<Expression> = match &params {
        Params::Standard(param_js_names) => param_js_names
            .iter()
            .map(|param| Expression::Identifier(param.clone()))
            .collect(),
        Params::DestructuredSingleton(entries) => vec![Expression::Object(Box::new(Object {
            entries: entries
                .iter()
                .map(|entry| ObjectEntry {
                    key: entry.in_name.clone(),
                    value: Expression::Identifier(entry.out_name.clone()),
                })
                .collect(),
        }))],
    };

    let return_value = {
        let mut items = Vec::with_capacity(arity + 1);
        items.push(Expression::Literal(Literal::String(JsStringLiteral {
            // We must use the variant JS name instead of the DB index's JS name
            // since we won't know the DB index of the match case patterns
            // (at the time of writing,
            // the type checker doesn't store its results,
            // so we don't have that information during code generation).
            // Later, we can fix this.
            unescaped: registry.get(variant.name_id).name.preferred_js_name().0,
            // TODO: What if 2 variant names have the same JS name?
        })));
        items.extend(type_args);
        Expression::Array(Box::new(Array { items }))
    };

    context.pop_n(arity);

    let variant_name = &registry.get(variant.name_id).name;
    context.try_push_name(ValidJsIdentifierName(format!(
        "{}_{}",
        &type_constructor_js_name.0,
        variant_name.preferred_js_name().0,
    )));
    let variant_symbol_js_name = context.js_name(DbIndex(0));

    Ok(ConstStatement {
        name: variant_symbol_js_name.clone(),
        value: Function {
            name: variant_symbol_js_name,
            params,
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
    let let_statement = registry.get(let_id);

    let value = generate_code_for_expression(registry, context, let_statement.value_id)?;

    let let_statement_name = &registry.get(let_statement.name_id).name;
    context.try_push_name(let_statement_name.preferred_js_name());
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
        ExpressionRef::Todo(todo) => generate_code_for_todo_expression(registry, context, todo),
        ExpressionRef::Call(call) => generate_code_for_call(registry, context, call),
        ExpressionRef::Fun(fun) => generate_code_for_fun(registry, context, fun),
        ExpressionRef::Match(match_) => generate_code_for_match(registry, context, match_),
        ExpressionRef::Forall(forall) => generate_code_for_forall(registry, context, forall),
        ExpressionRef::Check(check) => {
            generate_code_for_expression(registry, context, check.output_id)
        }
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

fn generate_code_for_todo_expression(
    _registry: &NodeRegistry,
    _context: &mut Context,
    todo: &light::TodoExpression,
) -> Result<Expression, CompileToJavaScriptError> {
    Ok(Expression::Call(Box::new(Call {
        callee: Expression::Identifier(ValidJsIdentifierName(TODO_ERROR_THROWER_NAME.to_string())),
        args: vec![Expression::Literal(Literal::String(JsStringLiteral {
            unescaped: todo
                .span
                .map(format_text_span)
                .unwrap_or_else(|| "<No span found>".to_string()),
        }))],
    })))
}

fn format_text_span(span: TextSpan) -> String {
    format!("{:?}", span)
}

fn generate_code_for_call(
    registry: &NodeRegistry,
    context: &mut Context,
    call: &light::Call,
) -> Result<Expression, CompileToJavaScriptError> {
    let callee = generate_code_for_expression(registry, context, call.callee_id)?;
    let args = match call.arg_list_id {
        NonEmptyCallArgListId::Unlabeled(arg_list_id) => {
            let arg_ids = registry.get_list(arg_list_id);
            arg_ids
                .iter()
                .map(|arg_id| generate_code_for_expression(registry, context, *arg_id))
                .collect::<Result<Vec<_>, _>>()?
        }
        NonEmptyCallArgListId::UniquelyLabeled(arg_list_id) => {
            let arg_ids = registry.get_list(arg_list_id);
            let entries = arg_ids
                .iter()
                .map(|arg_id| {
                    Ok(match arg_id {
                        LabeledCallArgId::Implicit {
                            label_id,
                            db_index,
                            value_id: _,
                        } => {
                            let key = registry.get(*label_id).name.preferred_js_name();
                            let value = Expression::Identifier(context.js_name(*db_index));
                            ObjectEntry { key, value }
                        }
                        LabeledCallArgId::Explicit { label_id, value_id } => {
                            let key = registry.get(*label_id).name.preferred_js_name();
                            let value = generate_code_for_expression(registry, context, *value_id)?;
                            ObjectEntry { key, value }
                        }
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            vec![Expression::Object(Box::new(Object { entries }))]
        }
    };
    Ok(Expression::Call(Box::new(Call { callee, args })))
}

fn generate_code_for_fun(
    registry: &NodeRegistry,
    context: &mut Context,
    fun: &light::Fun,
) -> Result<Expression, CompileToJavaScriptError> {
    let param_arity = fun.param_list_id.len();
    let params =
        generate_code_for_params_and_leave_params_in_context(registry, context, fun.param_list_id)?;
    let fun_js_name = {
        let fun_name = &registry.get(fun.name_id).name;
        context.try_push_name(fun_name.preferred_js_name());
        context.js_name(DbIndex(0))
    };
    let return_value = generate_code_for_expression(registry, context, fun.body_id)?;
    context.pop_n(param_arity + 1);
    Ok(Expression::Function(Box::new(Function {
        name: fun_js_name,
        params,
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
        .get_possibly_empty_list(match_.case_list_id)
        .iter()
        .map(|case_id| {
            let case = registry.get(*case_id);
            generate_code_for_match_case(registry, context, case, &matchee_temp_name)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Expression::Call(Box::new(Call {
        callee: Expression::Function(Box::new(Function {
            name: fun_temp_name,
            params: Params::Standard(vec![matchee_temp_name.clone()]),
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
        let case_js_name = registry.get(case.variant_name_id).name.preferred_js_name();
        Expression::BinaryOp(Box::new(BinaryOp {
            left: Expression::BinaryOp(Box::new(BinaryOp {
                left: Expression::Identifier(matchee_js_name.clone()),
                op: BinaryOpKind::Index,
                right: Expression::Literal(Literal::Number(0)),
            })),
            op: BinaryOpKind::TripleEqual,
            right: Expression::Literal(Literal::String(JsStringLiteral {
                unescaped: case_js_name.0,
            })),
        }))
    };

    let body = {
        let explicit_arity = case
            .param_list_id
            .map(|list_id| list_id.explicit_len())
            .unwrap_or(0);
        let mut body = Vec::with_capacity(explicit_arity + 1);

        match case.param_list_id {
            None => {}
            Some(NonEmptyMatchCaseParamListId::Unlabeled(param_list_id)) => {
                for (param_index, param_id) in
                    registry.get_list(param_list_id).iter().copied().enumerate()
                {
                    let param_name = &registry.get(param_id).name;
                    context.try_push_name(param_name.preferred_js_name());
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
            }
            Some(NonEmptyMatchCaseParamListId::UniquelyLabeled {
                param_list_id,
                triple_dot: _,
            }) => {
                let args_obj = Expression::BinaryOp(Box::new(BinaryOp {
                    left: Expression::Identifier(matchee_js_name.clone()),
                    op: BinaryOpKind::Index,
                    right: Expression::Literal(Literal::Number(1)),
                }));
                for param_id in registry
                    .get_possibly_empty_list(param_list_id)
                    .iter()
                    .copied()
                {
                    let param_name_id = registry.get(param_id).name_id;
                    let param_name = &registry.get(param_name_id).name;
                    context.try_push_name(param_name.preferred_js_name());
                    let param_js_name = context.js_name(DbIndex(0));
                    let param_label_id = registry.get(param_id).label();
                    let param_label_name = &registry.get(param_label_id).name;
                    let param_value = Expression::Dot(Box::new(Dot {
                        left: args_obj.clone(),
                        right: param_label_name.preferred_js_name(),
                    }));
                    body.push(FunctionStatement::Const(ConstStatement {
                        name: param_js_name,
                        value: param_value,
                    }));
                }
            }
        }

        body.push(FunctionStatement::Return(
            generate_code_for_match_case_output(registry, context, case.output_id)?,
        ));

        context.pop_n(explicit_arity);

        body
    };

    Ok(IfStatement { condition, body })
}

fn generate_code_for_match_case_output(
    registry: &NodeRegistry,
    context: &mut Context,
    id: MatchCaseOutputId,
) -> Result<Expression, CompileToJavaScriptError> {
    match id {
        MatchCaseOutputId::Some(id) => generate_code_for_expression(registry, context, id),
        MatchCaseOutputId::ImpossibilityClaim(kw_span) => {
            Ok(generate_code_to_throw_explosion(kw_span))
        }
    }
}

fn generate_code_to_throw_explosion(responsible_span: Option<TextSpan>) -> Expression {
    Expression::Call(Box::new(Call {
        callee: Expression::Identifier(ValidJsIdentifierName(EXPLOSION_THROWER_NAME.to_string())),
        args: vec![Expression::Literal(Literal::String(JsStringLiteral {
            unescaped: responsible_span
                .map(format_text_span)
                .unwrap_or_else(|| "<No span found>".to_string()),
        }))],
    }))
}

fn generate_code_for_forall(
    _registry: &NodeRegistry,
    _context: &mut Context,
    _name: &light::Forall,
) -> Result<Expression, CompileToJavaScriptError> {
    Ok(Expression::Object(Box::new(Object {
        entries: vec![ObjectEntry {
            key: ValidJsIdentifierName(TYPE_SPECIES_KEY.to_string()),
            value: Expression::Literal(Literal::String(JsStringLiteral {
                unescaped: TYPE_SPECIES_VALUE__FORALL.to_string(),
            })),
        }],
    })))
}

const TYPE_SPECIES_KEY: &str = "type_species";
const TYPE_SPECIES_VALUE__TYPE1: &str = "Type1";
const TYPE_SPECIES_VALUE__TYPE0: &str = "Type";
const TYPE_SPECIES_VALUE__FORALL: &str = "forall";
const TYPE_ARGS_KEY: &str = "type_args";
const EXPLOSION_THROWER_NAME: &str = "unreachable";
const EXPLOSION_THROWER_PARAM0_NAME: &str = "unreachable_span";
const TODO_ERROR_THROWER_NAME: &str = "unimplemented";
const TODO_ERROR_THROWER_PARAM0_NAME: &str = "unimplemented_span";
const DISPOSABLE_NAME_PREFIX: &str = "temp";

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
    fn preferred_js_name(&self) -> ValidJsIdentifierName {
        bijectively_sanitize_js_identifier_name(self.src_str())
    }
}

fn bijectively_sanitize_js_identifier_name(s: &str) -> ValidJsIdentifierName {
    let mut out = String::new();

    // The first character cannot be a digit
    for c in s.chars().take(1) {
        // '$' would also be a legal character.
        // However, we use it for escape sequences,
        // so we escape it to avoid name collisions.
        //
        // For example, if we didn't escape '$', then
        // the Kantu names `a$u0027$` and `a'` would both
        // be translated to the JavaScript name `a$u0027$`.
        if c.is_ascii_alphabetic() || c == '_' {
            out.push(c);
        } else {
            out.push_str(&format!("$u{:04x}$", u32::from(c)));
        }
    }

    // ...but the rest can be digits.
    for c in s.chars().skip(1) {
        // '$' would also be a legal character.
        // However, we use it for escape sequences,
        // so we escape it to avoid name collisions.
        //
        // For example, if we didn't escape '$', then
        // the Kantu names `a$u0027$` and `a'` would both
        // be translated to the JavaScript name `a$u0027$`.
        if c.is_ascii_alphanumeric() || c == '_' {
            out.push(c);
        } else {
            out.push_str(&format!("$u{:04x}$", u32::from(c)));
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
    js_name: ValidJsIdentifierName,
}

impl Context {
    fn new() -> Self {
        Self {
            stack: vec![
                ContextEntry {
                    js_name: ValidJsIdentifierName(TYPE_SPECIES_VALUE__TYPE1.to_string()),
                },
                ContextEntry {
                    js_name: ValidJsIdentifierName(TYPE_SPECIES_VALUE__TYPE0.to_string()),
                },
            ],
            other_reserved_names: vec![
                ValidJsIdentifierName(EXPLOSION_THROWER_NAME.to_string()),
                ValidJsIdentifierName(EXPLOSION_THROWER_PARAM0_NAME.to_string()),
                ValidJsIdentifierName(TODO_ERROR_THROWER_NAME.to_string()),
                ValidJsIdentifierName(TODO_ERROR_THROWER_PARAM0_NAME.to_string()),
            ],
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
        self.stack[level.0].js_name.clone()
    }

    fn try_push_name(&mut self, preferred: ValidJsIdentifierName) {
        if !self.contains(&preferred) {
            self.push(ContextEntry { js_name: preferred });
            return;
        }

        let mut i = 2;
        loop {
            let name = ValidJsIdentifierName(format!("{}{}", preferred.0, i));
            if !self.contains(&name) {
                self.push(ContextEntry { js_name: name });
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
        self.stack.iter().any(|x| x.js_name == *name)
            || self.other_reserved_names.iter().any(|x| x == name)
    }

    fn push(&mut self, entry: ContextEntry) {
        self.stack.push(entry);
    }

    fn pop_n(&mut self, n: usize) {
        self.stack.truncate(self.stack.len() - n);
    }
}
