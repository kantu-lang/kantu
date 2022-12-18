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

    mod heterogeneous {
        use super::*;

        #[test]
        fn explicitly_labeled_before_unlabeled_param() {
            let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_params/heterogeneous/explicit_before_unlabeled.ph");
            expect_heterogeneous_params_error(src);
        }

        #[test]
        fn implicit_before_unlabeled() {
            let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_params/heterogeneous/implicit_before_unlabeled.ph");
            expect_heterogeneous_params_error(src);
        }

        #[test]
        fn unlabeled_before_explicit() {
            let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_params/heterogeneous/unlabeled_before_explicit.ph");
            expect_heterogeneous_params_error(src);
        }

        #[test]
        fn unlabeled_before_implicit() {
            let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_params/heterogeneous/unlabeled_before_implicit.ph");
            expect_heterogeneous_params_error(src);
        }

        fn expect_heterogeneous_params_error(src: &str) {
            expect_simplification_error(src, |err| {
                if !matches!(&err, SimplifyAstError::HeterogeneousParams(_)) {
                    panic!("Unexpected error: {:#?}", err);
                }
            });
        }
    }

    mod underscore_label {
        use super::*;

        #[test]
        fn explicit_underscore_label() {
            let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_params/underscore_label/explicit_underscore_label.ph");
            expect_underscore_label_params_error(src);
        }

        #[test]
        fn implicit_underscore_label() {
            let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_params/underscore_label/implicit_underscore_label.ph");
            expect_underscore_label_params_error(src);
        }

        #[test]
        fn second_label_is_underscore() {
            let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_params/underscore_label/second_label_is_underscore.ph");
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

    mod duplicate_labels {
        use super::*;

        #[test]
        fn explicit() {
            let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_params/duplicate_labels/explicit.ph");
            expect_duplicate_label_params_error(src, "X");
        }

        #[test]
        fn implicit() {
            let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_params/duplicate_labels/implicit.ph");
            expect_duplicate_label_params_error(src, "z");
        }

        #[test]
        fn implicit_explicit() {
            let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_params/duplicate_labels/implicit_explicit.ph");
            expect_duplicate_label_params_error(src, "x");
        }

        #[test]
        fn explicit_implicit() {
            let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_params/duplicate_labels/explicit_implicit.ph");
            expect_duplicate_label_params_error(src, "y");
        }

        fn expect_duplicate_label_params_error(src: &str, label: &str) {
            expect_simplification_error(src, |err| {
                if let SimplifyAstError::DuplicateParamLabel(param1, param2) = &err {
                    let label = standard_ident_name(label);
                    let label = Some(&label);
                    assert_eq!(label, param1.label_name());
                    assert_eq!(label, param2.label_name());
                } else {
                    panic!("Unexpected error: {:#?}", err);
                }
            });
        }
    }
}

mod labeled_match_case_params {
    use super::*;

    mod heterogeneous {
        use super::*;

        #[test]
        fn explicitly_labeled_before_unlabeled_param() {
            let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_match_case_params/heterogeneous/explicit_before_unlabeled.ph");
            expect_heterogeneous_params_error(src);
        }

        #[test]
        fn implicit_before_unlabeled() {
            let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_match_case_params/heterogeneous/implicit_before_unlabeled.ph");
            expect_heterogeneous_params_error(src);
        }

        #[test]
        fn unlabeled_before_explicit() {
            let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_match_case_params/heterogeneous/unlabeled_before_explicit.ph");
            expect_heterogeneous_params_error(src);
        }

        #[test]
        fn unlabeled_before_implicit() {
            let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_match_case_params/heterogeneous/unlabeled_before_implicit.ph");
            expect_heterogeneous_params_error(src);
        }

        fn expect_heterogeneous_params_error(src: &str) {
            expect_simplification_error(src, |err| {
                if !matches!(&err, SimplifyAstError::HeterogeneousMatchCaseParams(_)) {
                    panic!("Unexpected error: {:#?}", err);
                }
            });
        }
    }

    mod underscore_label {
        use super::*;

        #[test]
        fn explicit_underscore_label() {
            let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_match_case_params/underscore_label/explicit_underscore_label.ph");
            expect_underscore_label_params_error(src);
        }

        #[test]
        fn implicit_underscore_label() {
            let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_match_case_params/underscore_label/implicit_underscore_label.ph");
            expect_underscore_label_params_error(src);
        }

        #[test]
        fn second_label_is_underscore() {
            let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_match_case_params/underscore_label/second_label_is_underscore.ph");
            expect_underscore_label_params_error(src);
        }

        fn expect_underscore_label_params_error(src: &str) {
            expect_simplification_error(src, |err| {
                if !matches!(&err, SimplifyAstError::UnderscoreMatchCaseParamLabel(_)) {
                    panic!("Unexpected error: {:#?}", err);
                }
            });
        }
    }

    mod duplicate_labels {
        use super::*;

        #[test]
        fn explicit() {
            let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_match_case_params/duplicate_labels/explicit.ph");
            expect_duplicate_label_params_error(src, "pred");
        }

        #[test]
        fn implicit() {
            let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_match_case_params/duplicate_labels/implicit.ph");
            expect_duplicate_label_params_error(src, "pred");
        }

        #[test]
        fn implicit_explicit() {
            let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_match_case_params/duplicate_labels/implicit_explicit.ph");
            expect_duplicate_label_params_error(src, "pred");
        }

        #[test]
        fn explicit_implicit() {
            let src = include_str!("../../sample_code/should_fail/ast_simplification/labeled_match_case_params/duplicate_labels/explicit_implicit.ph");
            expect_duplicate_label_params_error(src, "pred");
        }

        fn expect_duplicate_label_params_error(src: &str, label: &str) {
            expect_simplification_error(src, |err| {
                if let SimplifyAstError::DuplicateMatchCaseParamLabel(param1, param2) = &err {
                    let label = standard_ident_name(label);
                    let label = Some(&label);
                    assert_eq!(label, param1.label_name());
                    assert_eq!(label, param2.label_name());
                } else {
                    panic!("Unexpected error: {:#?}", err);
                }
            });
        }
    }
}
