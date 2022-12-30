use crate::{
    data::{unsimplified_ast::*, FileId},
    processing::{lex::lex, parse::parse},
};

fn expect_type_statement(src: &str, panicker: impl Fn(TypeStatement)) {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let file_item = parse(tokens, file_id).expect("Parsing failed");
    panicker(file_item);
}

fn expect_let_statement(src: &str, panicker: impl Fn(LetStatement)) {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let file_item = parse(tokens, file_id).expect("Parsing failed");
    panicker(file_item);
}

#[test]
fn empty_type() {
    let src =
        include_str!("../../sample_code/should_succeed/subterms/file_items/empty_type.fi.pht");
    expect_type_statement(src, |item| {
        let expected_name = IdentifierName::new("Empty".to_string());
        assert_eq!(&expected_name, &item.name.name);
        assert_eq!(0, item.variants.len());
    });
}

#[test]
fn nat() {
    let src = include_str!("../../sample_code/should_succeed/subterms/file_items/nat.fi.pht");
    expect_type_statement(src, |item| {
        let expected_name = IdentifierName::new("Nat".to_string());
        assert_eq!(&expected_name, &item.name.name);
        assert_eq!(2, item.variants.len());
    });
}

#[test]
fn let_() {
    let src = include_str!("../../sample_code/should_succeed/subterms/file_items/let.fi.pht");
    expect_let_statement(src, |item| {
        let expected_name = IdentifierName::new("x".to_string());
        assert_eq!(&expected_name, &item.name.name);
    });
}
