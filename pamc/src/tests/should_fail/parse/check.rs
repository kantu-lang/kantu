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
fn question_checkee() {
    let src = include_str!("../../sample_code/should_fail/parse/check/question_checkee.ph");
    expect_unexpected_token_error(src, TokenKind::Question);
}

#[test]
fn question_output() {
    let src = include_str!("../../sample_code/should_fail/parse/check/question_output.ph");
    expect_unexpected_token_error(src, TokenKind::Question);
}

#[test]
fn goal_value() {
    let src = include_str!("../../sample_code/should_fail/parse/check/goal_value.ph");
    expect_unexpected_token_error(src, TokenKind::Equal);
}

#[test]
fn goal_value_question() {
    let src = include_str!("../../sample_code/should_fail/parse/check/goal_value_question.ph");
    expect_unexpected_token_error(src, TokenKind::Equal);
}
