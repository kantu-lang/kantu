use super::*;

/// The job of `panicker` is to panic if the error is different than the expected
/// error.
fn expect_type_check_error(src: &str, panicker: impl Fn(&NodeRegistry, TypeCheckError)) {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let file = parse_file(tokens, file_id).expect("Parsing failed");
    let file = simplify_file(file).expect("AST Simplification failed");
    let file = bind_files(vec![file])
        .expect("Binding failed")
        .into_iter()
        .next()
        .unwrap();
    let mut registry = NodeRegistry::empty();
    let file_id = lighten_file(&mut registry, file);
    let file = registry.file(file_id);

    check_variant_return_types_for_file(&registry, file)
        .expect("Variant return type validation failed");
    validate_fun_recursion_in_file(&registry, file).expect("Fun recursion validation failed");
    let err = type_check_files(&mut registry, &[file_id]).expect_err("Type checking failed");
    panicker(&registry, err);
}

mod illegal_type {
    use super::*;

    fn expect_illegal_type_error(src: &str, expected_illegal_type_src: &str) {
        expect_type_check_error(src, |registry, err| match err {
            TypeCheckError::IllegalTypeExpression(id) => {
                let actual_src = format_expression(
                    &expand_expression(registry, id),
                    0,
                    &FormatOptions {
                        ident_size_in_spaces: 4,
                        print_db_indices: false,
                        print_fun_body_status: false,
                    },
                );
                assert_eq_up_to_white_space(&actual_src, expected_illegal_type_src);
            }
            _ => panic!("Unexpected error: {:#?}", err),
        });
    }

    #[test]
    fn forall_output() {
        let src =
            include_str!("../sample_code/should_fail/type_check/illegal_type/forall_output.ph");
        expect_illegal_type_error(src, "U.U");
    }

    #[test]
    fn forall_param() {
        let src =
            include_str!("../sample_code/should_fail/type_check/illegal_type/forall_param.ph");
        expect_illegal_type_error(src, "U.U");
    }

    #[test]
    fn fun_param() {
        let src = include_str!("../sample_code/should_fail/type_check/illegal_type/fun_param.ph");
        expect_illegal_type_error(src, "U.U");
    }

    #[test]
    fn fun_return() {
        let src = include_str!("../sample_code/should_fail/type_check/illegal_type/fun_return.ph");
        expect_illegal_type_error(src, "U.U");
    }

    #[test]
    fn type_param() {
        let src = include_str!("../sample_code/should_fail/type_check/illegal_type/type_param.ph");
        expect_illegal_type_error(src, "U.U");
    }

    #[test]
    fn variant_param() {
        let src =
            include_str!("../sample_code/should_fail/type_check/illegal_type/variant_param.ph");
        expect_illegal_type_error(src, "U.U");
    }
}

mod illegal_callee {
    use super::*;

    fn expect_illegal_callee_error(src: &str, expected_illegal_callee_src: &str) {
        expect_type_check_error(src, |registry, err| match err {
            TypeCheckError::IllegalCallee(id) => {
                let actual_src = format_expression(
                    &expand_expression(registry, id),
                    0,
                    &FormatOptions {
                        ident_size_in_spaces: 4,
                        print_db_indices: false,
                        print_fun_body_status: false,
                    },
                );
                assert_eq_up_to_white_space(&actual_src, expected_illegal_callee_src);
            }
            _ => panic!("Unexpected error: {:#?}", err),
        });
    }

    #[test]
    fn forall() {
        let src = include_str!("../sample_code/should_fail/type_check/illegal_callee/forall.ph");
        expect_illegal_callee_error(src, "forall(T: Type) { Type }");
    }

    #[test]
    fn non_nullary_adt_instance() {
        let src = include_str!(
            "../sample_code/should_fail/type_check/illegal_callee/non_nullary_adt_instance.ph"
        );
        expect_illegal_callee_error(src, "Bar.B(U.U,)");
    }

    #[test]
    fn nullary_adt_instance() {
        let src = include_str!(
            "../sample_code/should_fail/type_check/illegal_callee/nullary_adt_instance.ph"
        );
        expect_illegal_callee_error(src, "U.U");
    }

    #[test]
    fn nullary_type() {
        let src =
            include_str!("../sample_code/should_fail/type_check/illegal_callee/nullary_type.ph");
        expect_illegal_callee_error(src, "U");
    }

    #[test]
    fn type0() {
        let src = include_str!("../sample_code/should_fail/type_check/illegal_callee/type0.ph");
        expect_illegal_callee_error(src, "Type");
    }
}

mod wrong_number_of_arguments {
    use super::*;

    fn expect_wrong_number_of_arguments_error(
        src: &str,
        expected_illegal_call_src: &str,
        expected_expected_arity: usize,
    ) {
        expect_type_check_error(src, |registry, err| match err {
            TypeCheckError::WrongNumberOfArguments {
                call_id,
                expected: actual_expected_arity,
                ..
            } => {
                let actual_src = format_expression(
                    &expand_expression(registry, ExpressionId::Call(call_id)),
                    0,
                    &FormatOptions {
                        ident_size_in_spaces: 4,
                        print_db_indices: false,
                        print_fun_body_status: false,
                    },
                );
                assert_eq_up_to_white_space(&actual_src, expected_illegal_call_src);
                assert_eq!(expected_expected_arity, actual_expected_arity);
            }
            _ => panic!("Unexpected error: {:#?}", err),
        });
    }

    #[test]
    fn forall() {
        let src =
            include_str!("../sample_code/should_fail/type_check/wrong_number_of_arguments/fun.ph");
        expect_wrong_number_of_arguments_error(src, "bar_(U.U, U.U,)", 1);
    }

    #[test]
    fn type_() {
        let src =
            include_str!("../sample_code/should_fail/type_check/wrong_number_of_arguments/type.ph");
        expect_wrong_number_of_arguments_error(src, "V(U.U, U.U,)", 1);
    }

    #[test]
    fn variant() {
        let src = include_str!(
            "../sample_code/should_fail/type_check/wrong_number_of_arguments/variant.ph"
        );
        expect_wrong_number_of_arguments_error(src, "Bar.B(Empty, Empty,)", 1);
    }
}

mod wrong_number_of_case_params {
    use super::*;

    fn expect_wrong_number_of_case_params_error(
        src: &str,
        expected_illegal_match_case_src: &str,
        expected_expected_arity: usize,
    ) {
        expect_type_check_error(src, |registry, err| match err {
            TypeCheckError::WrongNumberOfCaseParams {
                case_id,
                expected: actual_expected_arity,
                ..
            } => {
                let actual_src = format_match_case(
                    &expand_match_case(registry, case_id),
                    0,
                    &FormatOptions {
                        ident_size_in_spaces: 4,
                        print_db_indices: false,
                        print_fun_body_status: false,
                    },
                );
                assert_eq_up_to_white_space(&actual_src, expected_illegal_match_case_src);
                assert_eq!(expected_expected_arity, actual_expected_arity);
            }
            _ => panic!("Unexpected error: {:#?}", err),
        });
    }

    #[test]
    fn expected_0_actually_1() {
        let src = include_str!("../sample_code/should_fail/type_check/wrong_number_of_case_params/expected_0_actually_1.ph");
        expect_wrong_number_of_case_params_error(src, "O(n) => n,", 0);
    }

    #[test]
    fn expected_1_actually_0() {
        let src = include_str!("../sample_code/should_fail/type_check/wrong_number_of_case_params/expected_1_actually_0.ph");
        expect_wrong_number_of_case_params_error(src, "S => Nat.O,", 1);
    }

    #[test]
    fn expected_1_actually_2() {
        let src = include_str!("../sample_code/should_fail/type_check/wrong_number_of_case_params/expected_1_actually_2.ph");
        expect_wrong_number_of_case_params_error(src, "S(n, m) => n,", 1);
    }

    #[test]
    fn expected_2_actually_1() {
        let src = include_str!("../sample_code/should_fail/type_check/wrong_number_of_case_params/expected_2_actually_1.ph");
        expect_wrong_number_of_case_params_error(src, "Refl(O) => O,", 2);
    }
}

// TODO: Add other tests
