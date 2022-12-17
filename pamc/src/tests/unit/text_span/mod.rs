use crate::{
    data::{non_empty_vec::NonEmptyVec, unsimplified_ast::*, FileId},
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

#[test]
fn hello_world() {
    let src = include_str!(
        "../../sample_code/should_succeed/should_succeed_without_warnings/hello_world.ph"
    );
    verify_that_spans_are_correct(src);
}

#[test]
fn ill_typed_until_substituted() {
    let src = include_str!("../../sample_code/should_succeed/should_succeed_without_warnings/ill_typed_until_substituted.ph");
    verify_that_spans_are_correct(src);
}

#[test]
fn plus_commutative() {
    let src = include_str!(
        "../../sample_code/should_succeed/should_succeed_without_warnings/plus_commutative.ph"
    );
    verify_that_spans_are_correct(src);
}

#[test]
fn forall() {
    let src =
        include_str!("../../sample_code/should_succeed/should_succeed_without_warnings/forall.ph");
    verify_that_spans_are_correct(src);
}

#[test]
fn exists() {
    let src =
        include_str!("../../sample_code/should_succeed/should_succeed_without_warnings/exists.ph");
    verify_that_spans_are_correct(src);
}

#[test]
fn check() {
    let src =
        include_str!("../../sample_code/should_succeed/should_succeed_with_warnings/check.ph");
    verify_that_spans_are_correct(src);
}

#[test]
fn comment() {
    let src =
        include_str!("../../sample_code/should_succeed/should_succeed_without_warnings/comment.ph");
    verify_that_spans_are_correct(src);
}

#[test]
fn match_explosion() {
    let src = include_str!(
        "../../sample_code/should_succeed/should_succeed_without_warnings/match_explosion.ph"
    );
    verify_that_spans_are_correct(src);
}

#[test]
fn underscore() {
    let src = include_str!(
        "../../sample_code/should_succeed/should_succeed_without_warnings/underscore.ph"
    );
    verify_that_spans_are_correct(src);
}

#[test]
fn optional_commas() {
    let src = include_str!(
        "../../sample_code/should_succeed/should_succeed_without_warnings/optional_commas.ph"
    );
    verify_that_spans_are_correct(src);
}
