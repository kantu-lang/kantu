use crate::{
    data::{unsimplified_ast::*, FileId},
    processing::{lex::lex, parse::parse},
};

fn expect_file_item(src: &str, panicker: impl Fn(FileItem)) {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let file_item = parse(tokens, file_id).expect("Parsing failed");
    panicker(file_item);
}

#[test]
fn empty_type() {
    let src =
        include_str!("../../sample_code/should_succeed/subterms/file_items/empty_type.fi.pht");
    expect_file_item(src, |file_item| match file_item {
        FileItem::Type(item) => {
            let expected_name = IdentifierName::Standard("Empty".to_string());
            assert_eq!(&expected_name, &item.name.name);
            assert_eq!(0, item.variants.len());
        }
        other => panic!("Unexpected file item: {:#?}", other),
    });
}

#[test]
fn nat() {
    let src = include_str!("../../sample_code/should_succeed/subterms/file_items/nat.fi.pht");
    expect_file_item(src, |file_item| match file_item {
        FileItem::Type(item) => {
            let expected_name = IdentifierName::Standard("Nat".to_string());
            assert_eq!(&expected_name, &item.name.name);
            assert_eq!(2, item.variants.len());
        }
        other => panic!("Unexpected file item: {:#?}", other),
    });
}

#[test]
fn let_() {
    let src = include_str!("../../sample_code/should_succeed/subterms/file_items/let.fi.pht");
    expect_file_item(src, |file_item| match file_item {
        FileItem::Let(item) => {
            let expected_name = IdentifierName::Standard("x".to_string());
            assert_eq!(&expected_name, &item.name.name);
        }
        other => panic!("Unexpected file item: {:#?}", other),
    });
}
