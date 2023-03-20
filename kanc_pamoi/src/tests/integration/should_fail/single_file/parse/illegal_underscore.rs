use super::*;

fn expect_underscore_parse_error(src: &str) {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let err = parse_file(tokens, file_id).expect_err("Parsing unexpectedly succeeded");
    match err {
        ParseError::UnexpectedNonEoiToken(token) => {
            assert_eq!(token.kind, TokenKind::Underscore);
        }
        _ => panic!("Unexpected error: {:#?}", err),
    }
}

#[test]
fn type_() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/parse/illegal_underscore/type.k"
    );
    expect_underscore_parse_error(src);
}

#[test]
fn variant() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/parse/illegal_underscore/variant.k"
    );
    expect_underscore_parse_error(src);
}

#[test]
fn let_() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/parse/illegal_underscore/let.k"
    );
    expect_underscore_parse_error(src);
}
