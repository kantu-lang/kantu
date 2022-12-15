use super::*;

fn expect_unexpected_token_error(src: &str, expected_kind: TokenKind) {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let err = parse_file(tokens, file_id).expect_err("Parsing unexpectedly succeeded");
    match err {
        ParseError::UnexpectedToken(token) => {
            assert_eq!(token.kind, expected_kind);
        }
        _ => panic!("Unexpected error: {:#?}", err),
    }
}

#[test]
fn dashed_label() {
    let src = include_str!("../../../sample_code/should_fail/parse/labeled_params/dashed_label.ph");
    expect_unexpected_token_error(src, TokenKind::Tilde);
}

#[test]
fn dash_before_tilde() {
    let src =
        include_str!("../../../sample_code/should_fail/parse/labeled_params/dash_before_tilde.ph");
    expect_unexpected_token_error(src, TokenKind::Tilde);
}
