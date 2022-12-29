use super::*;

fn expect_unexpected_non_eoi_token_error(
    src: &str,
    expected_kind: TokenKind,
    expected_content: &str,
) {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let err = parse_file(tokens, file_id).expect_err("Parsing unexpectedly succeeded");
    match err {
        ParseError::UnexpectedNonEoiToken(token) => {
            assert_eq!(expected_kind, token.kind);
            assert_eq!(expected_content, &token.content);
        }
        _ => panic!("Unexpected error: {:#?}", err),
    }
}

#[test]
fn super_dot_super() {
    let src = include_str!(
        "../../../sample_code/should_fail/parse/component_kw_in_dot_rhs/super_dot_super.ph"
    );
    expect_unexpected_non_eoi_token_error(src, TokenKind::Super, "super");
}

#[test]
fn mod_dot_super() {
    let src = include_str!(
        "../../../sample_code/should_fail/parse/component_kw_in_dot_rhs/mod_dot_super.ph"
    );
    expect_unexpected_non_eoi_token_error(src, TokenKind::Super, "super");
}

#[test]
fn mod_dot_mod() {
    let src = include_str!(
        "../../../sample_code/should_fail/parse/component_kw_in_dot_rhs/mod_dot_mod.ph"
    );
    expect_unexpected_non_eoi_token_error(src, TokenKind::Mod, "mod");
}

#[test]
fn super_dot_mod() {
    let src = include_str!(
        "../../../sample_code/should_fail/parse/component_kw_in_dot_rhs/super_dot_mod.ph"
    );
    expect_unexpected_non_eoi_token_error(src, TokenKind::Mod, "mod");
}
