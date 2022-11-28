use super::*;

fn expect_unexpected_ascii_digit_error(src: &str) {
    let err = lex(src).expect_err("Lexing unexpectedly succeeded");
    match err {
        LexError::UnexpectedAsciiDigit => {}
        _ => panic!("Unexpected error: {:#?}", err),
    }
}

#[test]
fn identifier_is_digit() {
    let src = include_str!("../sample_code/should_fail/lex/identifier_is_digit.ph");
    expect_unexpected_ascii_digit_error(src);
}

#[test]
fn identifier_starts_with_digit() {
    let src = include_str!("../sample_code/should_fail/lex/identifier_starts_with_digit.ph");
    expect_unexpected_ascii_digit_error(src);
}
