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
fn empty_use_vis() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/parse/visibility/empty_use_vis.k"
    );
    expect_unexpected_non_eoi_token_error(src, TokenKind::RParen, ")");
}

#[test]
fn empty_mod_vis() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/parse/visibility/empty_mod_vis.k"
    );
    expect_unexpected_non_eoi_token_error(src, TokenKind::RParen, ")");
}

#[test]
fn empty_type_vis() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/parse/visibility/empty_type_vis.k"
    );
    expect_unexpected_non_eoi_token_error(src, TokenKind::RParen, ")");
}

#[test]
fn empty_let_vis() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/parse/visibility/empty_let_vis.k"
    );
    expect_unexpected_non_eoi_token_error(src, TokenKind::RParen, ")");
}

#[test]
fn super1() {
    let src =
        include_str!("../../../../sample_code/should_fail/single_file/parse/visibility/super1.k");
    expect_unexpected_non_eoi_token_error(src, TokenKind::StandardIdentifier, "super1");
}

#[test]
fn super9() {
    let src =
        include_str!("../../../../sample_code/should_fail/single_file/parse/visibility/super9.k");
    expect_unexpected_non_eoi_token_error(src, TokenKind::StandardIdentifier, "super9");
}
