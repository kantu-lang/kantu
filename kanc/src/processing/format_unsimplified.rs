use crate::data::unsimplified_ast::*;

fn indent(indent_level: usize, options: &FormatOptions) -> String {
    " ".repeat(indent_level * options.ident_size_in_spaces)
}

#[derive(Clone, Debug)]
pub struct FormatOptions {
    pub ident_size_in_spaces: usize,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            ident_size_in_spaces: 4,
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
        Expression::Identifier(ident) => format_identifier(ident),
        Expression::Dot(dot) => format_dot(dot, indent_level, options),
        Expression::Todo(_) => format!("todo"),
        Expression::Call(call) => format_call(call, indent_level, options),
        Expression::Fun(fun) => format_fun(fun, indent_level, options),
        Expression::Match(match_) => format_match(match_, indent_level, options),
        Expression::Forall(forall) => format_forall(forall, indent_level, options),
        Expression::Check(check) => format_check(check, indent_level, options),
    }
}

pub fn format_dot(dot: &Dot, indent_level: usize, options: &FormatOptions) -> String {
    format!(
        "{}.{}",
        format_expression(&dot.left, indent_level + 1, options),
        format_identifier(&dot.right),
    )
}

pub fn format_identifier(ident: &Identifier) -> String {
    ident.name.src_str().to_owned()
}

pub fn format_call(call: &Call, indent_level: usize, options: &FormatOptions) -> String {
    let callee = match &call.callee {
        Expression::Fun(fun) => {
            format!("{}", format_identifier(&fun.name))
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
                format_call_arg(arg, indent_level + 1, options)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!("{}(\n{}\n{})", callee, args, &i0)
}

pub fn format_call_arg(arg: &CallArg, indent_level: usize, options: &FormatOptions) -> String {
    match &arg.label {
        None => format_expression(&arg.value, indent_level, options),
        Some(ParamLabel::Implicit) => {
            format!(":{}", format_expression(&arg.value, indent_level, options))
        }
        Some(ParamLabel::Explicit(label)) => {
            format!(
                "{}: {}",
                label.name.src_str(),
                format_expression(&arg.value, indent_level, options)
            )
        }
    }
}

pub fn format_fun(fun: &Fun, indent_level: usize, options: &FormatOptions) -> String {
    let i0 = indent(indent_level, options);
    let i1 = indent(indent_level + 1, options);
    let params = format_params(&fun.params, indent_level + 1, options);
    let return_type = format_expression(&fun.return_type, indent_level + 1, options);
    let body = format_expression(&fun.body, indent_level + 1, options);
    format!(
        "fun {}(\n{}\n{}): {} {{\n{}{}\n{}}}",
        format_identifier(&fun.name),
        params,
        &i0,
        return_type,
        &i1,
        body,
        &i0
    )
}

pub fn format_params(params: &Vec<Param>, indent_level: usize, options: &FormatOptions) -> String {
    let i0 = indent(indent_level, options);

    params
        .iter()
        .map(|param| format!("{}{},", &i0, format_param(param, indent_level, options)))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn format_param(param: &Param, indent_level: usize, options: &FormatOptions) -> String {
    let label = match &param.label {
        Some(ParamLabel::Explicit(ident)) => format!("{}~", format_identifier(ident)),
        Some(ParamLabel::Implicit) => "~".to_string(),
        None => "".to_string(),
    };
    let is_dashed = if param.is_dashed { "-" } else { "" };
    format!(
        "{}{}{}: {}",
        label,
        is_dashed,
        format_identifier(&param.name),
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
    let variant_name = format_identifier(&case.variant_name);
    let params = if case.params.is_empty() {
        "".to_string()
    } else {
        format_optional_match_case_params(case.params.as_ref(), case.triple_dot.is_some())
    };
    let output = format_match_case_output(&case.output, indent_level + 1, options);
    format!(
        ".{}{} => {}",
        variant_name,
        params,
        try_oneline(&format!("{},", output), indent_level, options)
    )
}

pub fn format_optional_match_case_params(
    params: Option<&Vec<MatchCaseParam>>,
    has_triple_dot: bool,
) -> String {
    match params {
        Some(params) => {
            let params = params
                .iter()
                .map(format_match_case_param)
                .collect::<Vec<_>>()
                .join(", ");
            let triple_dot = if has_triple_dot { ", ..." } else { "" };
            format!("({params}{triple_dot})")
        }
        None => {
            if has_triple_dot {
                "(...)".to_string()
            } else {
                "".to_string()
            }
        }
    }
}

pub fn format_match_case_param(param: &MatchCaseParam) -> String {
    match &param.label {
        None => format_identifier(&param.name),
        Some(ParamLabel::Implicit) => format!(":{}", format_identifier(&param.name)),
        Some(ParamLabel::Explicit(label)) => {
            format!(
                "{}: {}",
                label.name.src_str(),
                format_identifier(&param.name)
            )
        }
    }
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

pub fn format_match_case_output(
    output: &MatchCaseOutput,
    indent_level: usize,
    options: &FormatOptions,
) -> String {
    match output {
        MatchCaseOutput::Some(expression) => format_expression(expression, indent_level, options),
        MatchCaseOutput::ImpossibilityClaim(_) => "impossible".to_string(),
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
    let right = format_question_mark_or_expression(&assertion.right, indent_level + 2, options);
    if left.contains('\n') || right.contains('\n') {
        format!("{}{}\n{}{}", left, kind, &i1, right)
    } else {
        format!("{}{} {}", left, kind, right)
    }
}

pub fn format_goal_kw_or_expression(
    expression: &GoalKwOrExpression,
    indent_level: usize,
    options: &FormatOptions,
) -> String {
    match expression {
        GoalKwOrExpression::GoalKw { span: _ } => "goal".to_string(),
        GoalKwOrExpression::Expression(expression) => {
            format_expression(expression, indent_level, options)
        }
    }
}

pub fn format_question_mark_or_expression(
    expression: &QuestionMarkOrExpression,
    indent_level: usize,
    options: &FormatOptions,
) -> String {
    match expression {
        QuestionMarkOrExpression::QuestionMark { span: _ } => "?".to_string(),
        QuestionMarkOrExpression::Expression(expression) => {
            format_expression(expression, indent_level, options)
        }
    }
}
