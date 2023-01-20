use super::*;

fn expect_unexpected_character_error(src: &str, expected_unexpected_character: char) {
    let err = lex(src).expect_err("Lexing unexpectedly succeeded");
    match err {
        LexError::UnexpectedCharacter(c, _) => {
            assert_eq!(expected_unexpected_character, c);
        }
        _ => panic!("Unexpected error: {:#?}", err),
    }
}

#[test]
fn identifier_is_digit() {
    let src =
        include_str!("../../../sample_code/should_fail/single_file/lex/identifier_is_digit.k");
    expect_unexpected_character_error(src, '0');
}

#[test]
fn identifier_starts_with_digit() {
    let src = include_str!(
        "../../../sample_code/should_fail/single_file/lex/identifier_starts_with_digit.k"
    );
    expect_unexpected_character_error(src, '9');
}

#[test]
fn identifier_is_single_quote() {
    let src = include_str!(
        "../../../sample_code/should_fail/single_file/lex/identifier_is_single_quote.k"
    );
    expect_unexpected_character_error(src, '\'');
}

#[test]
fn identifier_starts_with_single_quote() {
    let src = include_str!(
        "../../../sample_code/should_fail/single_file/lex/identifier_starts_with_single_quote.k"
    );
    expect_unexpected_character_error(src, '\'');
}

#[test]
fn identifier_contains_non_ascii() {
    let src = include_str!(
        "../../../sample_code/should_fail/single_file/lex/identifier_contains_non_ascii.k"
    );
    expect_unexpected_character_error(src, 'á');
}
