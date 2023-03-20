use super::*;

fn expect_rparen_parse_error(src: &str) {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let err = parse_file(tokens, file_id).expect_err("Parsing unexpectedly succeeded");
    match err {
        ParseError::UnexpectedNonEoiToken(token) => {
            assert_eq!(token.kind, TokenKind::RParen);
        }
        _ => panic!("Unexpected error: {:#?}", err),
    }
}

#[test]
fn empty_type_params() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/parse/empty_parens/empty_type_params.k"
    );
    expect_rparen_parse_error(src);
}

#[test]
fn empty_variant_params() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/parse/empty_parens/empty_variant_params.k"
    );
    expect_rparen_parse_error(src);
}

#[test]
fn empty_fun_params() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/parse/empty_parens/empty_fun_params.k"
    );
    expect_rparen_parse_error(src);
}

#[test]
fn empty_call_params() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/parse/empty_parens/empty_call_params.k"
    );
    expect_rparen_parse_error(src);
}

#[test]
fn empty_forall_params() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/parse/empty_parens/empty_forall_params.k"
    );
    expect_rparen_parse_error(src);
}

#[test]
fn empty_match_case_params() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/parse/empty_parens/empty_match_case_params.k"
    );
    expect_rparen_parse_error(src);
}
