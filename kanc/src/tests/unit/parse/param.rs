use crate::{
    data::{unsimplified_ast::*, FileId},
    processing::{lex::lex, parse::parse},
};

fn expect_param(src: &str, panicker: impl Fn(Param)) {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let param = parse(tokens, file_id).expect("Parsing failed");
    panicker(param);
}

#[test]
fn undashed() {
    let src = include_str!("../../sample_code/should_succeed/subterms/params/undashed.p.ksn");
    expect_param(src, |param| {
        let expected_name = IdentifierName::new("a".to_string());
        assert_eq!(&expected_name, &param.name.name);
        assert!(!param.is_dashed);
    });
}

#[test]
fn dashed() {
    let src = include_str!("../../sample_code/should_succeed/subterms/params/dashed.p.ksn");
    expect_param(src, |param| {
        let expected_name = IdentifierName::new("b".to_string());
        assert_eq!(&expected_name, &param.name.name);
        assert!(param.is_dashed);
    });
}

#[test]
fn underscore() {
    let src =
        include_str!("../../sample_code/should_succeed/subterms/params/undashed_underscore.p.ksn");
    expect_param(src, |param| {
        let expected_name = IdentifierName::Reserved(ReservedIdentifierName::Underscore);
        assert_eq!(&expected_name, &param.name.name);
        assert!(!param.is_dashed);
    });
}

#[test]
fn dashed_underscore() {
    let src =
        include_str!("../../sample_code/should_succeed/subterms/params/dashed_underscore.p.ksn");
    expect_param(src, |param| {
        let expected_name = IdentifierName::Reserved(ReservedIdentifierName::Underscore);
        assert_eq!(&expected_name, &param.name.name);
        assert!(param.is_dashed);
    });
}
