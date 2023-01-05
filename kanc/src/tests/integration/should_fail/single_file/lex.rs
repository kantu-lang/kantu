use super::*;

fn expect_unexpected_ascii_digit_error(src: &str) {
    let err = lex(src).expect_err("Lexing unexpectedly succeeded");
    match err {
        LexError::UnexpectedCharacter(c, _) => {
            assert!(c.is_ascii_digit());
        }
        _ => panic!("Unexpected error: {:#?}", err),
    }
}

#[test]
fn identifier_is_digit() {
    let src =
        include_str!("../../../sample_code/should_fail/single_file/lex/identifier_is_digit.k");
    expect_unexpected_ascii_digit_error(src);
}

#[test]
fn identifier_starts_with_digit() {
    let src = include_str!(
        "../../../sample_code/should_fail/single_file/lex/identifier_starts_with_digit.k"
    );
    expect_unexpected_ascii_digit_error(src);
}
