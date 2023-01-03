use super::*;

fn expect_unexpected_non_eoi_token_error(src: &str, expected_kind: TokenKind) {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let err = parse_file(tokens, file_id).expect_err("Parsing unexpectedly succeeded");
    match err {
        ParseError::UnexpectedNonEoiToken(token) => {
            assert_eq!(token.kind, expected_kind);
        }
        _ => panic!("Unexpected error: {:#?}", err),
    }
}

#[test]
fn question_checkee() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/parse/check/question_checkee.ph"
    );
    expect_unexpected_non_eoi_token_error(src, TokenKind::Question);
}

#[test]
fn question_output() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/parse/check/question_output.ph"
    );
    expect_unexpected_non_eoi_token_error(src, TokenKind::Question);
}
