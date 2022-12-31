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
fn dot_dot() {
    let src = include_str!("../../../../sample_code/should_fail/single_file/parse/use/dot_dot.ph");
    expect_unexpected_non_eoi_token_error(src, TokenKind::Dot, ".");
}

#[test]
fn dot_as() {
    let src = include_str!("../../../../sample_code/should_fail/single_file/parse/use/dot_as.ph");
    expect_unexpected_non_eoi_token_error(src, TokenKind::As, "as");
}

#[test]
fn dot_semicolon() {
    let src =
        include_str!("../../../../sample_code/should_fail/single_file/parse/use/dot_semicolon.ph");
    expect_unexpected_non_eoi_token_error(src, TokenKind::Semicolon, ";");
}

#[test]
fn dotless_star() {
    let src =
        include_str!("../../../../sample_code/should_fail/single_file/parse/use/dotless_star.ph");
    expect_unexpected_non_eoi_token_error(src, TokenKind::Star, "*");
}

#[test]
fn dotless_ident() {
    let src =
        include_str!("../../../../sample_code/should_fail/single_file/parse/use/dotless_ident.ph");
    expect_unexpected_non_eoi_token_error(src, TokenKind::StandardIdentifier, "baz");
}
