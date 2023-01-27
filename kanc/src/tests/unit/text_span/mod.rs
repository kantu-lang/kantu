use crate::{
    data::{file_id::*, unsimplified_ast::*},
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
    let src =
        include_str!("../../sample_code/should_succeed/single_file/no_warnings/hello_world.k");
    verify_that_spans_are_correct(src);
}

#[test]
fn ill_typed_until_substituted() {
    let src = include_str!(
        "../../sample_code/should_succeed/single_file/no_warnings/ill_typed_until_substituted.k"
    );
    verify_that_spans_are_correct(src);
}

#[test]
fn plus_commutative() {
    let src =
        include_str!("../../sample_code/should_succeed/single_file/no_warnings/plus_commutative.k");
    verify_that_spans_are_correct(src);
}

#[test]
fn forall() {
    let src = include_str!("../../sample_code/should_succeed/single_file/no_warnings/forall.k");
    verify_that_spans_are_correct(src);
}

#[test]
fn exists() {
    let src = include_str!("../../sample_code/should_succeed/single_file/no_warnings/exists.k");
    verify_that_spans_are_correct(src);
}

#[test]
fn check() {
    let src = include_str!(
        "../../sample_code/should_succeed/single_file/with_warnings/check/many_warnings.k"
    );
    verify_that_spans_are_correct(src);
}

#[test]
fn comment() {
    let src = include_str!("../../sample_code/should_succeed/single_file/no_warnings/comment.k");
    verify_that_spans_are_correct(src);
}

#[test]
fn match_explosion() {
    let src =
        include_str!("../../sample_code/should_succeed/single_file/no_warnings/match_explosion.k");
    verify_that_spans_are_correct(src);
}

#[test]
fn underscore() {
    let src = include_str!("../../sample_code/should_succeed/single_file/no_warnings/underscore.k");
    verify_that_spans_are_correct(src);
}

#[test]
fn optional_commas() {
    let src =
        include_str!("../../sample_code/should_succeed/single_file/no_warnings/optional_commas.k");
    verify_that_spans_are_correct(src);
}

#[test]
fn labeled_params() {
    let src =
        include_str!("../../sample_code/should_succeed/single_file/no_warnings/labeled_params.k");
    verify_that_spans_are_correct(src);
}

#[test]
fn labeled_call_args() {
    let src = include_str!(
        "../../sample_code/should_succeed/single_file/no_warnings/labeled_call_args.k"
    );
    verify_that_spans_are_correct(src);
}

#[test]
fn visibility_and_transparency_modifiers() {
    let src = include_str!(
        "../../sample_code/should_succeed/single_file/should_parse/visibility_and_transparency_modifiers.k"
    );
    verify_that_spans_are_correct(src);
}

#[test]
fn use_as_is() {
    let src = include_str!("../../sample_code/should_succeed/single_file/should_parse/use/as_is.k");
    verify_that_spans_are_correct(src);
}

#[test]
fn use_alternate_name() {
    let src = include_str!(
        "../../sample_code/should_succeed/single_file/should_parse/use/alternate_name.k"
    );
    verify_that_spans_are_correct(src);
}

#[test]
fn use_wildcard() {
    let src =
        include_str!("../../sample_code/should_succeed/single_file/should_parse/use/wildcard.k");
    verify_that_spans_are_correct(src);
}

#[test]
fn component_kw_in_dot_lhs() {
    let src = include_str!(
        "../../sample_code/should_succeed/single_file/should_parse/component_kw_in_dot_lhs.k"
    );
    verify_that_spans_are_correct(src);
}
