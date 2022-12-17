use super::*;

use crate::data::unsimplified_ast as ust;

fn expect_simplification_error(src: &str, panicker: impl Fn(SimplifyAstError)) {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let file = parse_file(tokens, file_id).expect("Parsing failed");
    let err = simplify_file(file).expect_err("AST Simplification unexpectedly succeeded");
    panicker(err);
}

#[test]
fn illegal_dot_lhs() {
    let src =
        include_str!("../../sample_code/should_fail/ast_simplification/dot/illegal_dot_lhs.ph");
    expect_simplification_error(src, |err| match err {
        SimplifyAstError::IllegalDotLhs(lhs) => {
            assert!(
                matches!(lhs, ust::Expression::Match(_)),
                "Unexpected lhs {:?}",
                lhs
            );
        }
        other_err => panic!("Unexpected error: {:#?}", other_err),
    });
}

mod labeled_params {
    use super::*;

    #[test]
    fn explicitly_labeled_before_unlabeled_param() {
        let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_params/explicit_before_unlabeled.ph");
        expect_heterogeneous_params_error(src);
    }

    #[test]
    fn implicit_before_unlabeled() {
        let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_params/implicit_before_unlabeled.ph");
        expect_heterogeneous_params_error(src);
    }

    #[test]
    fn unlabeled_before_explicit() {
        let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_params/unlabeled_before_explicit.ph");
        expect_heterogeneous_params_error(src);
    }

    #[test]
    fn unlabeled_before_implicit() {
        let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_params/unlabeled_before_implicit.ph");
        expect_heterogeneous_params_error(src);
    }

    fn expect_heterogeneous_params_error(src: &str) {
        expect_simplification_error(src, |err| {
            if !matches!(&err, SimplifyAstError::HeterogeneousParams(_)) {
                panic!("Unexpected error: {:#?}", err);
            }
        });
    }

    // TODO
    #[ignore]
    #[test]
    fn explicit_underscore_label() {
        let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_params/explicit_underscore_label.ph");
        expect_underscore_label_params_error(src);
    }

    // TODO
    #[ignore]
    #[test]
    fn implicit_underscore_label() {
        let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_params/implicit_underscore_label.ph");
        expect_underscore_label_params_error(src);
    }

    fn expect_underscore_label_params_error(src: &str) {
        expect_simplification_error(src, |err| {
            if !matches!(&err, SimplifyAstError::UnderscoreParamLabel(_)) {
                panic!("Unexpected error: {:#?}", err);
            }
        });
    }
}
