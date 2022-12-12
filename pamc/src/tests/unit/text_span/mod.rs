use crate::{
    data::{unsimplified_ast::*, FileId},
    processing::{
        lex::lex,
        parse::{parse, Parse, ParseError},
    },
};

mod check_spans;
use check_spans::*;

mod replace_spans_and_file_ids_with_dummies;
use replace_spans_and_file_ids_with_dummies::*;

fn verify_that_spans_are_correct(src: &str) {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let file = parse(tokens, file_id).expect("Parsing failed");
    check_spans_in_file(src, &file);
}

// TODO: Fix
#[ignore]
#[test]
fn hello_world() {
    let src = include_str!("../../sample_code/should_succeed/hello_world.ph");
    verify_that_spans_are_correct(src);
}
