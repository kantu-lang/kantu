use super::*;

fn expect_underscore_label_params_error(src: &str, expected_kind: TokenKind) {
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
fn explicit_underscore_label() {
    let src = include_str!("../../../../sample_code/should_fail/single_file/parse/labeled_call_args/underscore_label/explicit_underscore_label.k");
    expect_underscore_label_params_error(src, TokenKind::Colon);
}

#[test]
fn implicit_underscore_label() {
    let src = include_str!("../../../../sample_code/should_fail/single_file/parse/labeled_call_args/underscore_label/implicit_underscore_label.k");
    expect_underscore_label_params_error(src, TokenKind::Underscore);
}

#[test]
fn second_label_is_underscore() {
    let src = include_str!("../../../../sample_code/should_fail/single_file/parse/labeled_call_args/underscore_label/second_label_is_underscore.k");
    expect_underscore_label_params_error(src, TokenKind::Colon);
}
