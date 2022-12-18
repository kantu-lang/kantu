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
    let args = match &call.args {
        NonEmptyCallArgVec::Unlabeled(args) => args
            .iter()
            .map(|arg| {
                format!(
                    "{}{},",
                    &i1,
                    format_expression(arg, indent_level + 1, options)
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
        NonEmptyCallArgVec::UniquelyLabeled(args) => args
            .iter()
            .map(|arg| {
                let label = match &arg.label {
                    ParamLabel::Implicit => ":".to_string(),
                    ParamLabel::Explicit(label) => format!("{}: ", format_ident(label)),
                };
                format!(
                    "{}{}{},",
                    &i1,
                    label,
                    format_expression(&arg.value, indent_level + 1, options)
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
    };
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
    let params = format_params(&fun.params, indent_level + 1, options);
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

pub fn format_params(
    params: &NonEmptyParamVec,
    indent_level: usize,
    options: &FormatOptions,
) -> String {
    let i0 = indent(indent_level, options);

    match params {
        NonEmptyParamVec::Unlabeled(params) => params
            .iter()
            .map(|param| {
                format!(
                    "{}{},",
                    &i0,
                    format_unlabeled_param(param, indent_level, options)
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
        NonEmptyParamVec::UniquelyLabeled(params) => params
            .iter()
            .map(|param| {
                format!(
                    "{}{},",
                    &i0,
                    format_labeled_param(param, indent_level, options)
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
    }
}

pub fn format_unlabeled_param(
    param: &UnlabeledParam,
    indent_level: usize,
    options: &FormatOptions,
) -> String {
    let is_dashed = if param.is_dashed { "-" } else { "" };
    format!(
        "{}{}: {}",
        is_dashed,
        format_ident(&param.name),
        format_expression(&param.type_, indent_level, options)
    )
}

pub fn format_labeled_param(
    param: &LabeledParam,
    indent_level: usize,
    options: &FormatOptions,
) -> String {
    let explicit_label = match &param.label {
        ParamLabel::Explicit(ident) => format_ident(ident),
        ParamLabel::Implicit => "".to_string(),
    };
    let is_dashed = if param.is_dashed { "-" } else { "" };
    format!(
        "{}~{}{}: {}",
        explicit_label,
        is_dashed,
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
    let params = format_params(&forall.params, indent_level + 1, options);
    let output = format_expression(&forall.output, indent_level + 1, options);
    format!(
        "forall (\n{}\n{}) {{\n{}{}\n{}}}",
        params, &i0, &i1, output, &i0
    )
}

pub fn format_check(check: &Check, indent_level: usize, options: &FormatOptions) -> String {
    let i0 = indent(indent_level, options);
    let i1 = indent(indent_level + 1, options);
    let assertions = format_check_assertions(&check.assertions, indent_level + 1, options);
    let output = format_expression(&check.output, indent_level + 1, options);
    format!("case {} {{\n{}{}\n{}}}", assertions, &i1, output, &i0,)
}

pub fn format_check_assertions(
    assertions: &[CheckAssertion],
    indent_level: usize,
    options: &FormatOptions,
) -> String {
    let i0 = indent(indent_level, options);
    let i1 = indent(indent_level + 1, options);
    format!(
        "({}\n{})",
        assertions
            .iter()
            .map(|assertion| format!(
                "\n{}{},",
                &i1,
                format_check_assertion(assertion, indent_level + 1, options)
            ))
            .collect::<Vec<_>>()
            .join(""),
        i0
    )
}

pub fn format_check_assertion(
    assertion: &CheckAssertion,
    indent_level: usize,
    options: &FormatOptions,
) -> String {
    let i1 = indent(indent_level + 1, options);
    let kind = match assertion.kind {
        CheckAssertionKind::Type => ":",
        CheckAssertionKind::NormalForm => " =",
    };
    let left = format_goal_kw_or_expression(&assertion.left, indent_level + 1, options);
    let right = format_question_mark_or_possibly_invalid_expression(
        &assertion.right,
        indent_level + 2,
        options,
    );
    if left.contains('\n') || right.contains('\n') {
        format!("{}{}\n{}{}", left, kind, &i1, right)
    } else {
        format!("{}{} {}", left, kind, right)
    }
}

pub fn format_goal_kw_or_expression(
    expression: &GoalKwOrPossiblyInvalidExpression,
    indent_level: usize,
    options: &FormatOptions,
) -> String {
    match expression {
        GoalKwOrPossiblyInvalidExpression::GoalKw { span: _ } => "goal".to_string(),
        GoalKwOrPossiblyInvalidExpression::Expression(expression) => {
            format_possibly_invalid_expression(expression, indent_level, options)
        }
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
