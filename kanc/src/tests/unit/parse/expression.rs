use std::ops::Deref;

use crate::{
    data::{
        non_empty_vec::{OptionalNonEmptyToPossiblyEmpty, OptionalNonEmptyVecLen},
        unsimplified_ast::*,
        FileId,
    },
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
            assert_eq!(&IdentifierName::new("a".to_string()), &name.name);
        }
        _ => panic!("Unexpected expression {:?}", expression),
    });
}

#[test]
fn dot2() {
    let src = include_str!("../../sample_code/should_succeed/subterms/expressions/dot2.x.pht");
    expect_expression(src, |expression| match &expression {
        Expression::Dot(dot) => {
            assert_eq!(&IdentifierName::new("b".to_string()), &dot.right.name);
            match &dot.left {
                Expression::Identifier(name) => {
                    assert_eq!(&IdentifierName::new("a".to_string()), &name.name);
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
            assert_eq!(&IdentifierName::new("c".to_string()), &dot.right.name);
            match &dot.left {
                Expression::Dot(left) => {
                    assert_eq!(&IdentifierName::new("b".to_string()), &left.right.name);
                    match &left.left {
                        Expression::Identifier(name) => {
                            assert_eq!(&IdentifierName::new("a".to_string()), &name.name);
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
                assert_eq!(None, call.args.iter().find(|arg| arg.label.is_some()));

                match (
                    &call.callee,
                    call.args
                        .iter()
                        .map(|arg| &arg.value)
                        .collect::<Vec<_>>()
                        .deref(),
                ) {
                    (
                        Expression::Dot(callee),
                        [Expression::Identifier(arg0), Expression::Dot(arg1), Expression::Identifier(arg2)],
                    ) => {
                        assert_eq!(IdentifierName::new("b".to_string()), callee.right.name);
                        assert_eq!(IdentifierName::new("c".to_string()), arg0.name);
                        assert_eq!(IdentifierName::new("e".to_string()), arg1.right.name);
                        assert_eq!(IdentifierName::new("f".to_string()), arg2.name);
                        return;
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
fn labeled_call() {
    let src =
        include_str!("../../sample_code/should_succeed/subterms/expressions/labeled_call.x.pht");
    expect_expression(src, |expression| {
        match &expression {
            Expression::Call(call) => {
                assert_eq!(4, call.args.len());

                let arg0 = &call.args[0];
                let arg1 = &call.args[1];
                let arg2 = &call.args[2];
                let arg3 = &call.args[3];

                match &call.callee {
                    Expression::Dot(callee) => {
                        assert_eq!(IdentifierName::new("b".to_string()), callee.right.name);

                        assert_eq!(Some(ParamLabel::Implicit), arg0.label);
                        assert_eq!(
                            Some(&IdentifierName::new("c".to_string())),
                            arg0.label_name()
                        );

                        assert!(matches!(arg1.label, Some(ParamLabel::Explicit(_))));
                        assert_eq!(
                            Some(&IdentifierName::new("e".to_string())),
                            arg1.label_name()
                        );

                        assert_eq!(Some(ParamLabel::Implicit), arg0.label);
                        assert_eq!(
                            Some(&IdentifierName::new("f".to_string())),
                            arg2.label_name()
                        );

                        assert!(matches!(arg3.label, Some(ParamLabel::Explicit(_))));
                        assert_eq!(
                            Some(&IdentifierName::new("g".to_string())),
                            arg3.label_name()
                        );

                        return;
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
            assert_eq!(IdentifierName::new("x".to_string()), fun.name.name);

            assert_eq!(2, fun.params.len());

            assert_eq!(
                IdentifierName::new("a".to_string()),
                fun.params[0].name.name
            );
            assert!(fun.params[0].is_dashed);
            assert_eq!(None, fun.params[0].label);

            assert_eq!(
                IdentifierName::new("b".to_string()),
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
            assert_eq!(IdentifierName::new("x".to_string()), fun.name.name);

            assert_eq!(2, fun.params.len());

            assert_eq!(
                IdentifierName::new("a".to_string()),
                fun.params[0].name.name
            );
            assert!(fun.params[0].is_dashed);
            assert_eq!(Some(ParamLabel::Implicit), fun.params[0].label);

            assert_eq!(
                IdentifierName::new("b".to_string()),
                fun.params[1].name.name
            );
            assert!(!fun.params[1].is_dashed);
            assert_eq!(
                Some(&IdentifierName::new("bar".to_string())),
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
fn labeled_match() {
    let src =
        include_str!("../../sample_code/should_succeed/subterms/expressions/labeled_match.x.pht");
    expect_expression(src, |expression| match expression {
        Expression::Match(match_) => {
            assert_eq!(15, match_.cases.len());

            let case_a = &match_.cases[0];
            assert_eq!(0, case_a.params.len());
            assert!(case_a.triple_dot.is_none());

            let case_b = &match_.cases[1];
            assert_eq!(1, case_b.params.len());
            assert!(case_b.triple_dot.is_none());

            let case_c = &match_.cases[2];
            assert_eq!(1, case_c.params.len());
            assert!(case_c.triple_dot.is_none());

            let case_d = &match_.cases[3];
            assert_eq!(2, case_d.params.len());
            assert!(matches!(
                case_d.params.to_possibly_empty()[0].label,
                Some(ParamLabel::Implicit)
            ));
            assert_eq!(
                Some(&IdentifierName::new("x'".to_string())),
                case_d.params.to_possibly_empty()[0].label_name()
            );
            assert_eq!(
                IdentifierName::new("x'".to_string()),
                case_d.params.to_possibly_empty()[0].name.name
            );
            assert!(matches!(
                case_d.params.to_possibly_empty()[1].label,
                Some(ParamLabel::Implicit)
            ));
            assert_eq!(
                Some(&IdentifierName::new("y'".to_string())),
                case_d.params.to_possibly_empty()[1].label_name()
            );
            assert_eq!(
                IdentifierName::new("y'".to_string()),
                case_d.params.to_possibly_empty()[1].name.name
            );
            assert!(case_d.triple_dot.is_none());

            let case_e = &match_.cases[4];
            assert_eq!(1, case_e.params.len());
            assert!(case_e.triple_dot.is_none());

            let case_f = &match_.cases[5];
            assert_eq!(2, case_f.params.len());
            assert!(matches!(
                case_f.params.to_possibly_empty()[0].label,
                Some(ParamLabel::Explicit(_))
            ));
            assert_eq!(
                Some(&IdentifierName::new("foo".to_string())),
                case_f.params.to_possibly_empty()[0].label_name()
            );
            assert_eq!(
                IdentifierName::new("x'".to_string()),
                case_f.params.to_possibly_empty()[0].name.name
            );
            assert!(matches!(
                case_f.params.to_possibly_empty()[1].label,
                Some(ParamLabel::Explicit(_))
            ));
            assert_eq!(
                Some(&IdentifierName::new("bar".to_string())),
                case_f.params.to_possibly_empty()[1].label_name()
            );
            assert_eq!(
                IdentifierName::new("y'".to_string()),
                case_f.params.to_possibly_empty()[1].name.name
            );
            assert!(case_f.triple_dot.is_none());

            let case_g = &match_.cases[6];
            assert_eq!(2, case_g.params.len());
            assert!(case_g.triple_dot.is_none());

            let case_h = &match_.cases[7];
            assert_eq!(2, case_h.params.len());
            assert!(case_h.triple_dot.is_none());

            let case_i = &match_.cases[8];
            assert_eq!(1, case_i.params.len());
            assert!(case_i.triple_dot.is_none());

            let case_j = &match_.cases[9];
            assert_eq!(2, case_j.params.len());
            assert!(case_j.triple_dot.is_none());

            let case_k = &match_.cases[10];
            assert_eq!(2, case_k.params.len());
            assert!(case_k.triple_dot.is_none());

            let case_l = &match_.cases[11];
            assert_eq!(2, case_l.params.len());
            assert!(matches!(
                case_l.params.to_possibly_empty()[0].label,
                Some(ParamLabel::Explicit(_))
            ));
            assert_eq!(
                Some(&IdentifierName::new("foo".to_string())),
                case_l.params.to_possibly_empty()[0].label_name()
            );
            assert_eq!(
                IdentifierName::Reserved(ReservedIdentifierName::Underscore),
                case_l.params.to_possibly_empty()[0].name.name
            );
            assert!(matches!(
                case_l.params.to_possibly_empty()[1].label,
                Some(ParamLabel::Implicit)
            ));
            assert_eq!(
                Some(&IdentifierName::new("z".to_string())),
                case_l.params.to_possibly_empty()[1].label_name()
            );
            assert_eq!(
                IdentifierName::new("z".to_string()),
                case_l.params.to_possibly_empty()[1].name.name
            );
            assert!(case_l.triple_dot.is_none());

            let case_m = &match_.cases[12];
            assert_eq!(0, case_m.params.len());
            assert!(case_m.triple_dot.is_some());

            let case_n = &match_.cases[13];
            assert_eq!(1, case_n.params.len());
            assert!(case_n.triple_dot.is_some());

            let case_o = &match_.cases[14];
            assert_eq!(2, case_o.params.len());
            assert!(case_o.triple_dot.is_some());
        }
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

#[test]
fn check() {
    let src = include_str!("../../sample_code/should_succeed/subterms/expressions/check.x.pht");
    expect_expression(src, |expression| match expression {
        Expression::Check(check) => {
            assert_eq!(3, check.assertions.len());
            assert!(matches!(&check.output, Expression::Call(_)));
        }
        other => panic!("Unexpected expression {:?}", other),
    });
}
