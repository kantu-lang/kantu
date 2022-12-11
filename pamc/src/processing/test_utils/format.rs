use crate::data::bound_ast::*;

fn indent(indent_level: usize, options: &FormatOptions) -> String {
    " ".repeat(indent_level * options.ident_size_in_spaces)
}

#[derive(Clone, Debug)]
pub struct FormatOptions {
    pub ident_size_in_spaces: usize,
    pub print_db_indices: bool,
    pub print_fun_body_status: bool,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            ident_size_in_spaces: 4,
            print_db_indices: true,
            print_fun_body_status: true,
        }
    }
}

pub fn format_expression_with_default_options(expression: &Expression) -> String {
    format_expression(expression, 0, &FormatOptions::default())
}

pub fn format_expression(
    expression: &Expression,
    indent_level: usize,
    options: &FormatOptions,
) -> String {
    match expression {
        Expression::Name(name) => format_name(name, indent_level, options),
        Expression::Call(call) => format_call(call, indent_level, options),
        Expression::Fun(fun) => format_fun(fun, indent_level, options),
        Expression::Match(match_) => format_match(match_, indent_level, options),
        Expression::Forall(forall) => format_forall(forall, indent_level, options),
        Expression::Check(check) => format_check(check, indent_level, options),
    }
}

pub fn format_name(name: &NameExpression, _indent_level: usize, options: &FormatOptions) -> String {
    let fully_qualified = name
        .components
        .iter()
        .map(|ident| format_ident(ident))
        .collect::<Vec<_>>()
        .join(".");
    if options.print_db_indices {
        format!("{}<{}>", fully_qualified, name.db_index.0)
    } else {
        format!("{}", fully_qualified)
    }
}

pub fn format_ident(ident: &Identifier) -> String {
    match &ident.name {
        IdentifierName::Standard(s) => s.clone(),
        IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase) => "Type".to_string(),
        IdentifierName::Reserved(ReservedIdentifierName::Underscore) => "_".to_string(),
    }
}

pub fn format_call(call: &Call, indent_level: usize, options: &FormatOptions) -> String {
    let callee = match &call.callee {
        Expression::Fun(fun) => {
            let body_status = if options.print_fun_body_status {
                format!("<<{}>>", fun.skip_type_checking_body)
            } else {
                "".to_string()
            };
            format!("{}{}", format_ident(&fun.name), body_status)
        }
        _ => format_expression(&call.callee, indent_level, options),
    };
    let i0 = indent(indent_level, options);
    let i1 = indent(indent_level + 1, options);
    let args = call
        .args
        .iter()
        .map(|arg| {
            format!(
                "{}{},",
                &i1,
                format_expression(arg, indent_level + 1, options)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!("{}(\n{}\n{})", callee, args, &i0)
}

pub fn format_fun(fun: &Fun, indent_level: usize, options: &FormatOptions) -> String {
    let i0 = indent(indent_level, options);
    let i1 = indent(indent_level + 1, options);
    let body_status = if options.print_fun_body_status {
        format!("<<{}>>", fun.skip_type_checking_body)
    } else {
        "".to_string()
    };
    let params = fun
        .params
        .iter()
        .map(|arg| format!("{}{},", &i1, format_param(arg, indent_level, options)))
        .collect::<Vec<_>>()
        .join("\n");
    let return_type = format_expression(&fun.return_type, indent_level + 1, options);
    let body = format_expression(&fun.body, indent_level + 1, options);
    format!(
        "fun {}{}(\n{}\n{}): {} {{\n{}{}\n{}}}",
        format_ident(&fun.name),
        body_status,
        params,
        &i0,
        return_type,
        &i1,
        body,
        &i0
    )
}

pub fn format_param(param: &Param, indent_level: usize, options: &FormatOptions) -> String {
    format!(
        "{}: {}",
        format_ident(&param.name),
        format_expression(&param.type_, indent_level, options)
    )
}

pub fn format_match(match_: &Match, indent_level: usize, options: &FormatOptions) -> String {
    let i0 = indent(indent_level, options);
    let i1 = indent(indent_level + 1, options);
    let matchee = format_expression(&match_.matchee, indent_level + 1, options);
    let cases = match_
        .cases
        .iter()
        .map(|case| {
            format!(
                "{}{}",
                &i1,
                format_match_case(case, indent_level + 1, options)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "match {} {{\n{}\n{}}}",
        try_oneline_with_multi_parens(&matchee, indent_level, options),
        cases,
        &i0
    )
}

fn try_oneline_with_multi_parens(s: &str, indent_level: usize, options: &FormatOptions) -> String {
    let i0 = indent(indent_level, options);
    let i1 = indent(indent_level + 1, options);
    if s.contains('\n') {
        format!("(\n{}{}\n{})", &i1, s, &i0)
    } else {
        format!("{}", s)
    }
}

pub fn format_match_case(case: &MatchCase, indent_level: usize, options: &FormatOptions) -> String {
    let variant_name = format_ident(&case.variant_name);
    let params = if case.params.is_empty() {
        "".to_string()
    } else {
        let params = case
            .params
            .iter()
            .map(|param| format!("{}", format_ident(param)))
            .collect::<Vec<_>>()
            .join(", ");
        format!("({})", params)
    };
    let output = format_expression(&case.output, indent_level + 1, options);
    format!(
        "{}{} => {}",
        variant_name,
        params,
        try_oneline(&format!("{},", output), indent_level, options)
    )
}

fn try_oneline(s: &str, indent_level: usize, options: &FormatOptions) -> String {
    let i0 = indent(indent_level, options);
    let i1 = indent(indent_level + 1, options);
    if s.contains('\n') {
        format!("\n{}{}\n{}", &i1, s, &i0)
    } else {
        format!("{}", s)
    }
}

pub fn format_forall(forall: &Forall, indent_level: usize, options: &FormatOptions) -> String {
    let i0 = indent(indent_level, options);
    let i1 = indent(indent_level + 1, options);
    let params = forall
        .params
        .iter()
        .map(|param| format!("{}{}", &i1, format_param(param, indent_level, options)))
        .collect::<Vec<_>>()
        .join("\n");
    let output = format_expression(&forall.output, indent_level + 1, options);
    format!(
        "forall (\n{}\n{}) {{\n{}{}\n{}}}",
        params, &i0, &i1, output, &i0
    )
}

pub fn format_check(check: &Check, indent_level: usize, options: &FormatOptions) -> String {
    let i0 = indent(indent_level, options);
    let i1 = indent(indent_level + 1, options);
    let annotation =
        format_checkee_annotation(&check.checkee_annotation, indent_level + 1, options);
    let output = format_expression(&check.output, indent_level + 1, options);
    format!(
        "case {} {{\n{}{}\n{}}}",
        try_oneline_with_multi_parens(&annotation, indent_level, options),
        &i1,
        output,
        &i0,
    )
}

pub fn format_checkee_annotation(
    annotation: &CheckeeAnnotation,
    indent_level: usize,
    options: &FormatOptions,
) -> String {
    match annotation {
        CheckeeAnnotation::Goal(annotation) => {
            format_goal_checkee_annotation(annotation, indent_level, options)
        }
        CheckeeAnnotation::Expression(annotation) => {
            format_expression_checkee_annotation(annotation, indent_level, options)
        }
    }
}

pub fn format_goal_checkee_annotation(
    annotation: &GoalCheckeeAnnotation,
    indent_level: usize,
    options: &FormatOptions,
) -> String {
    let i1 = indent(indent_level + 1, options);
    let checkee_type = format_question_mark_or_possibly_invalid_expression(
        &annotation.checkee_type,
        indent_level + 2,
        options,
    );
    if checkee_type.contains('\n') {
        format!("goal:\n{}{}", &i1, checkee_type)
    } else {
        format!("goal: {}", checkee_type)
    }
}

pub fn format_expression_checkee_annotation(
    annotation: &ExpressionCheckeeAnnotation,
    indent_level: usize,
    options: &FormatOptions,
) -> String {
    let i1 = indent(indent_level + 1, options);
    let checkee = format_expression(&annotation.checkee, indent_level + 1, options);
    let checkee_type = format_question_mark_or_possibly_invalid_expression(
        &annotation.checkee_type,
        indent_level + 2,
        options,
    );
    let checkee_value = if let Some(value) = &annotation.checkee_value {
        format!(
            " = {}",
            format_question_mark_or_possibly_invalid_expression(value, indent_level + 2, options)
        )
    } else {
        "".to_string()
    };
    let attempted_oneliner = format!("{}: {}{}", checkee, checkee_type, checkee_value);
    if attempted_oneliner.contains('\n') {
        let checkee_value = if checkee_value == "" {
            checkee_value
        } else {
            format!("\n{}{}", &i1, checkee_value)
        };
        format!("{}:\n{}{}{}", checkee, &i1, checkee_type, checkee_value)
    } else {
        attempted_oneliner
    }
}

pub fn format_question_mark_or_possibly_invalid_expression(
    expression: &QuestionMarkOrPossiblyInvalidExpression,
    indent_level: usize,
    options: &FormatOptions,
) -> String {
    match expression {
        QuestionMarkOrPossiblyInvalidExpression::QuestionMark { span: _ } => "?".to_string(),
        QuestionMarkOrPossiblyInvalidExpression::Expression(expression) => {
            format_possibly_invalid_expression(expression, indent_level, options)
        }
    }
}

pub fn format_possibly_invalid_expression(
    expression: &PossiblyInvalidExpression,
    indent_level: usize,
    options: &FormatOptions,
) -> String {
    match expression {
        PossiblyInvalidExpression::Valid(expression) => {
            format_expression(expression, indent_level, options)
        }
        PossiblyInvalidExpression::Invalid(_) => "<INVALID>".to_string(),
    }
}
