use crate::{
    data::{unsimplified_ast::*, FileId},
    processing::{lex::lex, parse::parse},
};

fn expect_expression(src: &str, panicker: impl Fn(Expression)) {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let expression = parse(tokens, file_id).expect("Parsing failed");
    panicker(expression);
}

#[test]
fn dot1() {
    let src = include_str!("../sample_code/should_succeed/expressions/dot1.phx");
    expect_expression(src, |expression| match expression {
        Expression::Identifier(_) => {}
        other => panic!("Unexpected expression {:?}", other),
    });
}

#[test]
fn dot2() {
    let src = include_str!("../sample_code/should_succeed/expressions/dot2.phx");
    expect_expression(src, |expression| match expression {
        Expression::Dot(_) => {}
        other => panic!("Unexpected expression {:?}", other),
    });
}

#[test]
fn dot3() {
    let src = include_str!("../sample_code/should_succeed/expressions/dot3.phx");
    expect_expression(src, |expression| match expression {
        Expression::Dot(_) => {}
        other => panic!("Unexpected expression {:?}", other),
    });
}

#[test]
fn call() {
    let src = include_str!("../sample_code/should_succeed/expressions/call.phx");
    expect_expression(src, |expression| match expression {
        Expression::Call(_) => {}
        other => panic!("Unexpected expression {:?}", other),
    });
}

#[test]
fn fun() {
    let src = include_str!("../sample_code/should_succeed/expressions/fun.phx");
    expect_expression(src, |expression| match expression {
        Expression::Fun(_) => {}
        other => panic!("Unexpected expression {:?}", other),
    });
}

#[test]
fn match_() {
    let src = include_str!("../sample_code/should_succeed/expressions/match.phx");
    expect_expression(src, |expression| match expression {
        Expression::Match(_) => {}
        other => panic!("Unexpected expression {:?}", other),
    });
}

#[test]
fn forall() {
    let src = include_str!("../sample_code/should_succeed/expressions/forall.phx");
    expect_expression(src, |expression| match expression {
        Expression::Forall(_) => {}
        other => panic!("Unexpected expression {:?}", other),
    });
}

// TODO: Test Check expressions
