use crate::data::bound_ast::*;

pub const INDENT_SIZE_IN_SPACES: usize = 4;

fn indent(indent_level: usize) -> String {
    " ".repeat(indent_level * INDENT_SIZE_IN_SPACES)
}

pub fn debug_expression(expression: &Expression, indent_level: usize) -> String {
    match expression {
        Expression::Name(name) => debug_name(name, indent_level),
        Expression::Call(call) => debug_call(call, indent_level),
        Expression::Fun(fun) => debug_fun(fun, indent_level),
        Expression::Match(match_) => debug_match(match_, indent_level),
        Expression::Forall(forall) => debug_forall(forall, indent_level),
    }
}

pub fn debug_name(name: &NameExpression, _indent_level: usize) -> String {
    let fully_qualified = name
        .components
        .iter()
        .map(|ident| debug_ident(ident))
        .collect::<Vec<_>>()
        .join(".");
    format!("{}<{}>", fully_qualified, name.db_index.0)
}

pub fn debug_ident(ident: &Identifier) -> String {
    match &ident.name {
        IdentifierName::Standard(s) => s.clone(),
        IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase) => "Type".to_string(),
        IdentifierName::Reserved(ReservedIdentifierName::Underscore) => "_".to_string(),
    }
}

pub fn debug_call(call: &Call, indent_level: usize) -> String {
    let callee = match &call.callee {
        Expression::Fun(fun) => debug_ident(&fun.name),
        _ => debug_expression(&call.callee, indent_level),
    };
    let i0 = indent(indent_level);
    let i1 = indent(indent_level + 1);
    let args = call
        .args
        .iter()
        .map(|arg| format!("{}{},", &i1, debug_expression(arg, indent_level + 1)))
        .collect::<Vec<_>>()
        .join("\n");
    format!("{}(\n{}\n{})", callee, args, &i0)
}

pub fn debug_fun(fun: &Fun, indent_level: usize) -> String {
    let i0 = indent(indent_level);
    let i1 = indent(indent_level + 1);
    let params = fun
        .params
        .iter()
        .map(|arg| format!("{}{},", &i1, debug_param(arg, indent_level)))
        .collect::<Vec<_>>()
        .join("\n");
    let return_type = debug_expression(&fun.return_type, indent_level + 1);
    let body = debug_expression(&fun.body, indent_level + 1);
    format!(
        "fun {}(\n{}\n{}): {} {{\n{}{}\n{}}}",
        debug_ident(&fun.name),
        params,
        &i0,
        return_type,
        &i1,
        body,
        &i0
    )
}

pub fn debug_param(param: &Param, indent_level: usize) -> String {
    format!(
        "{}: {}",
        debug_ident(&param.name),
        debug_expression(&param.type_, indent_level)
    )
}

pub fn debug_match(match_: &Match, indent_level: usize) -> String {
    let i0 = indent(indent_level);
    let i1 = indent(indent_level + 1);
    let matchee = debug_expression(&match_.matchee, indent_level + 1);
    let cases = match_
        .cases
        .iter()
        .map(|case| format!("{}{}", &i1, debug_match_case(case, indent_level + 1)))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "match {} {{\n{}\n{}}}",
        try_oneline_with_multi_parens(&matchee, indent_level),
        cases,
        &i0
    )
}

fn try_oneline_with_multi_parens(s: &str, indent_level: usize) -> String {
    let i0 = indent(indent_level);
    let i1 = indent(indent_level + 1);
    if s.contains('\n') {
        format!("(\n{}{}\n{})", &i1, s, &i0)
    } else {
        format!("{}", s)
    }
}

pub fn debug_match_case(case: &MatchCase, indent_level: usize) -> String {
    let variant_name = debug_ident(&case.variant_name);
    let params = if case.params.is_empty() {
        "".to_string()
    } else {
        let params = case
            .params
            .iter()
            .map(|param| format!("{}", debug_ident(param)))
            .collect::<Vec<_>>()
            .join(", ");
        format!("({})", params)
    };
    let output = debug_expression(&case.output, indent_level + 1);
    format!(
        "{}{} => {}",
        variant_name,
        params,
        try_oneline(&format!("{},", output), indent_level)
    )
}

fn try_oneline(s: &str, indent_level: usize) -> String {
    let i0 = indent(indent_level);
    let i1 = indent(indent_level + 1);
    if s.contains('\n') {
        format!("\n{}{}\n{}", &i1, s, &i0)
    } else {
        format!("{}", s)
    }
}

pub fn debug_forall(forall: &Forall, indent_level: usize) -> String {
    let i0 = indent(indent_level);
    let i1 = indent(indent_level + 1);
    let params = forall
        .params
        .iter()
        .map(|param| format!("{}{}", &i1, debug_param(param, indent_level)))
        .collect::<Vec<_>>()
        .join("\n");
    let output = debug_expression(&forall.output, indent_level + 1);
    format!(
        "forall (\n{}\n{}) {{\n{}{}\n{}}}",
        params, &i0, &i1, output, &i0
    )
}
