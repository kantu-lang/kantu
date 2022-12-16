use std::ops::Deref;

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
    let src = include_str!("../../sample_code/should_succeed/subterms/expressions/dot1.x.pht");
    expect_expression(src, |expression| match &expression {
        Expression::Identifier(name) => {
            assert_eq!(&IdentifierName::Standard("a".to_string()), &name.name);
        }
        _ => panic!("Unexpected expression {:?}", expression),
    });
}

#[test]
fn dot2() {
    let src = include_str!("../../sample_code/should_succeed/subterms/expressions/dot2.x.pht");
    expect_expression(src, |expression| match &expression {
        Expression::Dot(dot) => {
            assert_eq!(&IdentifierName::Standard("b".to_string()), &dot.right.name);
            match &dot.left {
                Expression::Identifier(name) => {
                    assert_eq!(&IdentifierName::Standard("a".to_string()), &name.name);
                }
                _ => panic!("Unexpected expression {:?}", expression),
            }
        }
        _ => panic!("Unexpected expression {:?}", expression),
    });
}

#[test]
fn dot3() {
    let src = include_str!("../../sample_code/should_succeed/subterms/expressions/dot3.x.pht");
    expect_expression(src, |expression| match &expression {
        Expression::Dot(dot) => {
            assert_eq!(&IdentifierName::Standard("c".to_string()), &dot.right.name);
            match &dot.left {
                Expression::Dot(left) => {
                    assert_eq!(&IdentifierName::Standard("b".to_string()), &left.right.name);
                    match &left.left {
                        Expression::Identifier(name) => {
                            assert_eq!(&IdentifierName::Standard("a".to_string()), &name.name);
                        }
                        _ => panic!("Unexpected expression {:?}", expression),
                    }
                }
                _ => panic!("Unexpected expression {:?}", expression),
            }
        }
        _ => panic!("Unexpected expression {:?}", expression),
    });
}

#[test]
fn call() {
    let src = include_str!("../../sample_code/should_succeed/subterms/expressions/call.x.pht");
    expect_expression(src, |expression| {
        match &expression {
            Expression::Call(call) => {
                assert_eq!(3, call.args.len());
                match (&call.callee, call.args.deref()) {
                    (
                        Expression::Dot(callee),
                        [Expression::Identifier(arg0), Expression::Dot(arg1), Expression::Identifier(arg2)],
                    ) => {
                        assert_eq!(IdentifierName::Standard("b".to_string()), callee.right.name);
                        assert_eq!(IdentifierName::Standard("c".to_string()), arg0.name);
                        assert_eq!(IdentifierName::Standard("e".to_string()), arg1.right.name);
                        assert_eq!(IdentifierName::Standard("f".to_string()), arg2.name);
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        panic!("Unexpected expression {:?}", expression);
    });
}

#[test]
fn fun() {
    let src = include_str!("../../sample_code/should_succeed/subterms/expressions/fun.x.pht");
    expect_expression(src, |expression| match expression {
        Expression::Fun(fun) => {
            assert_eq!(IdentifierName::Standard("x".to_string()), fun.name.name);

            assert_eq!(2, fun.params.len());

            assert_eq!(
                IdentifierName::Standard("a".to_string()),
                fun.params[0].name.name
            );
            assert!(fun.params[0].is_dashed);
            assert_eq!(None, fun.params[0].label);

            assert_eq!(
                IdentifierName::Standard("b".to_string()),
                fun.params[1].name.name
            );
            assert!(!fun.params[1].is_dashed);
            assert_eq!(None, fun.params[1].label);
        }
        other => panic!("Unexpected expression {:?}", other),
    });
}

#[test]
fn labeled_fun() {
    let src =
        include_str!("../../sample_code/should_succeed/subterms/expressions/labeled_fun.x.pht");
    expect_expression(src, |expression| match expression {
        Expression::Fun(fun) => {
            assert_eq!(IdentifierName::Standard("x".to_string()), fun.name.name);

            assert_eq!(2, fun.params.len());

            assert_eq!(
                IdentifierName::Standard("a".to_string()),
                fun.params[0].name.name
            );
            assert!(fun.params[0].is_dashed);
            assert_eq!(Some(ParamLabel::Implicit), fun.params[0].label);

            assert_eq!(
                IdentifierName::Standard("b".to_string()),
                fun.params[1].name.name
            );
            assert!(!fun.params[1].is_dashed);
            assert_eq!(
                Some(&IdentifierName::Standard("bar".to_string())),
                fun.params[1].label.as_ref().and_then(|label| match label {
                    ParamLabel::Explicit(name) => Some(&name.name),
                    ParamLabel::Implicit => None,
                })
            );
        }
        other => panic!("Unexpected expression {:?}", other),
    });
}

#[test]
fn match_() {
    let src = include_str!("../../sample_code/should_succeed/subterms/expressions/match.x.pht");
    expect_expression(src, |expression| match expression {
        Expression::Match(_) => {}
        other => panic!("Unexpected expression {:?}", other),
    });
}

#[test]
fn forall() {
    let src = include_str!("../../sample_code/should_succeed/subterms/expressions/forall.x.pht");
    expect_expression(src, |expression| match expression {
        Expression::Forall(_) => {}
        other => panic!("Unexpected expression {:?}", other),
    });
}

// TODO: Test Check expressions
