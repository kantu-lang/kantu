use crate::{
    data::{non_empty_vec::OptionalNonEmptyVecLen, unsimplified_ast::*, FileId},
    processing::{lex::lex, parse::parse},
};

fn expect_variant(src: &str, panicker: impl Fn(Variant)) {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let variant = parse(tokens, file_id).expect("Parsing failed");
    panicker(variant);
}

#[test]
fn nullary() {
    let src = include_str!("../../sample_code/should_succeed/subterms/variants/nullary.tv.pht");
    expect_variant(src, |variant| {
        let expected_name = IdentifierName::new("O".to_string());
        assert_eq!(&expected_name, &variant.name.name);
        assert_eq!(0, variant.params.len());
    });
}

#[test]
fn unary() {
    let src = include_str!("../../sample_code/should_succeed/subterms/variants/unary.tv.pht");
    expect_variant(src, |variant| {
        let expected_name = IdentifierName::new("S".to_string());
        assert_eq!(&expected_name, &variant.name.name);
        assert_eq!(1, variant.params.len());
    });
}

#[test]
fn binary() {
    let src = include_str!("../../sample_code/should_succeed/subterms/variants/binary.tv.pht");
    expect_variant(src, |variant| {
        let expected_name = IdentifierName::new("Some".to_string());
        assert_eq!(&expected_name, &variant.name.name);
        assert_eq!(2, variant.params.len());
    });
}
