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

    validate_variant_return_types_in_file(&registry, file)
        .expect("Variant return type validation failed");
    validate_fun_recursion_in_file(&mut registry, file_id)
        .expect("Fun recursion validation failed");
    let err = type_check_files(&mut registry, &[file_id])
        .expect_err("Type checking unexpected succeeded");
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

mod type_mismatch {
    use super::*;

    fn expect_type_mismatch_error(
        src: &str,
        expected_expression_src: &str,
        expected_expected_type_src: &str,
        expected_actual_type_src: &str,
    ) {
        expect_type_check_error(src, |registry, err| match err {
            TypeCheckError::TypeMismatch {
                expression_id,
                expected_type_id,
                actual_type_id,
            } => {
                let actual_expression_src = format_expression(
                    &expand_expression(registry, expression_id),
                    0,
                    &FormatOptions {
                        ident_size_in_spaces: 4,
                        print_db_indices: false,
                        print_fun_body_status: false,
                    },
                );
                assert_eq_up_to_white_space(&actual_expression_src, expected_expression_src);

                let actual_expected_type_src = format_expression(
                    &expand_expression(registry, expected_type_id.raw()),
                    0,
                    &FormatOptions {
                        ident_size_in_spaces: 4,
                        print_db_indices: false,
                        print_fun_body_status: false,
                    },
                );
                assert_eq_up_to_white_space(&actual_expected_type_src, expected_expected_type_src);

                let actual_actual_type_src = format_expression(
                    &expand_expression(registry, actual_type_id.raw()),
                    0,
                    &FormatOptions {
                        ident_size_in_spaces: 4,
                        print_db_indices: false,
                        print_fun_body_status: false,
                    },
                );
                assert_eq_up_to_white_space(&actual_actual_type_src, expected_actual_type_src);
            }
            _ => panic!("Unexpected error: {:#?}", err),
        });
    }

    #[test]
    fn adt() {
        let src = include_str!("../sample_code/should_fail/type_check/type_mismatch/adt.ph");
        expect_type_mismatch_error(src, "U2.U2", "U1", "U2");
    }

    #[test]
    fn type_not_a_type() {
        let src =
            include_str!("../sample_code/should_fail/type_check/type_mismatch/type_not_a_type.ph");
        expect_type_mismatch_error(src, "Type", "Type", "Type1");
    }

    #[test]
    fn ill_typed_param_type() {
        let src = include_str!(
            "../sample_code/should_fail/type_check/type_mismatch/ill_typed_param_type.ph"
        );
        expect_type_mismatch_error(
            src,
            "Eq.Refl(Nat, x',)",
            "Eq(Nat, x, Nat.S(x',),)",
            "Eq(Nat, x', x',)",
        );
    }

    // TODO: Fix
    #[ignore]
    #[test]
    fn ill_typed_match_case_output_evaluates_to_well_typed_term() {
        let src = include_str!(
            "../sample_code/should_fail/type_check/type_mismatch/ill_typed_match_case_output_evaluates_to_well_typed_term.ph"
        );
        expect_type_mismatch_error(src, "Nat.O", "Bool", "Nat");
    }
}

mod non_adt_matchee {
    use super::*;

    fn expect_non_adt_matchee_error(
        src: &str,
        expected_matchee_src: &str,
        expected_type_src: &str,
    ) {
        expect_type_check_error(src, |registry, err| match err {
            TypeCheckError::NonAdtMatchee {
                matchee_id,
                type_id,
            } => {
                let actual_matchee_src = format_expression(
                    &expand_expression(registry, matchee_id),
                    0,
                    &FormatOptions {
                        ident_size_in_spaces: 4,
                        print_db_indices: false,
                        print_fun_body_status: false,
                    },
                );
                assert_eq_up_to_white_space(&actual_matchee_src, expected_matchee_src);

                let actual_type_src = format_expression(
                    &expand_expression(registry, type_id.raw()),
                    0,
                    &FormatOptions {
                        ident_size_in_spaces: 4,
                        print_db_indices: false,
                        print_fun_body_status: false,
                    },
                );
                assert_eq_up_to_white_space(&actual_type_src, expected_type_src);
            }
            _ => panic!("Unexpected error: {:#?}", err),
        });
    }

    #[test]
    fn type0() {
        let src = include_str!("../sample_code/should_fail/type_check/non_adt_matchee/type0.ph");
        expect_non_adt_matchee_error(src, "U", "Type");
    }

    #[test]
    fn type1() {
        let src = include_str!("../sample_code/should_fail/type_check/non_adt_matchee/type1.ph");
        expect_non_adt_matchee_error(src, "Type", "Type1");
    }
}

mod duplicate_match_case {
    use super::*;

    fn expect_duplicate_match_case_error(
        src: &str,
        expected_existing_match_case_src: &str,
        expected_new_match_case_src: &str,
    ) {
        expect_type_check_error(src, |registry, err| match err {
            TypeCheckError::DuplicateMatchCase {
                existing_match_case_id,
                new_match_case_id,
            } => {
                let actual_existing_match_case_src = format_match_case(
                    &expand_match_case(registry, existing_match_case_id),
                    0,
                    &FormatOptions {
                        ident_size_in_spaces: 4,
                        print_db_indices: false,
                        print_fun_body_status: false,
                    },
                );
                assert_eq_up_to_white_space(
                    &actual_existing_match_case_src,
                    expected_existing_match_case_src,
                );

                let actual_new_match_case_src = format_match_case(
                    &expand_match_case(registry, new_match_case_id),
                    0,
                    &FormatOptions {
                        ident_size_in_spaces: 4,
                        print_db_indices: false,
                        print_fun_body_status: false,
                    },
                );
                assert_eq_up_to_white_space(
                    &actual_new_match_case_src,
                    expected_new_match_case_src,
                );
            }
            _ => panic!("Unexpected error: {:#?}", err),
        });
    }

    #[test]
    fn duplicate_match_case() {
        let src = include_str!("../sample_code/should_fail/type_check/duplicate_match_case.ph");
        expect_duplicate_match_case_error(src, "U => Bool.True,", "U => Bool.False,");
    }
}

mod missing_match_case {
    use super::*;

    fn expect_missing_match_case_error(src: &str, expected_variant_name: &IdentifierName) {
        expect_type_check_error(src, |registry, err| match err {
            TypeCheckError::MissingMatchCase { variant_name_id } => {
                let actual_variant_name = &registry.identifier(variant_name_id).name;
                assert_eq!(expected_variant_name, actual_variant_name);
            }
            _ => panic!("Unexpected error: {:#?}", err),
        });
    }

    #[test]
    fn missing_match_case() {
        let src = include_str!("../sample_code/should_fail/type_check/missing_match_case.ph");
        expect_missing_match_case_error(src, &standard_ident_name("False"));
    }
}

mod extraneous_match_case {
    use super::*;

    fn expect_extraneous_match_case_error(src: &str, expected_extraneous_match_case_src: &str) {
        expect_type_check_error(src, |registry, err| match err {
            TypeCheckError::ExtraneousMatchCase { case_id } => {
                let actual_extraneous_match_case_src = format_match_case(
                    &expand_match_case(registry, case_id),
                    0,
                    &FormatOptions {
                        ident_size_in_spaces: 4,
                        print_db_indices: false,
                        print_fun_body_status: false,
                    },
                );
                assert_eq_up_to_white_space(
                    &actual_extraneous_match_case_src,
                    expected_extraneous_match_case_src,
                );
            }
            _ => panic!("Unexpected error: {:#?}", err),
        });
    }

    #[test]
    fn extraneous_match_case() {
        let src = include_str!("../sample_code/should_fail/type_check/extraneous_match_case.ph");
        expect_extraneous_match_case_error(src, "Maybe => Nat.S(Nat.S(Nat.O,),),");
    }
}

mod ambiguous_output_type {
    use super::*;

    fn expect_ambiguous_output_type_error(src: &str, expected_ambiguous_match_case_src: &str) {
        expect_type_check_error(src, |registry, err| match err {
            TypeCheckError::AmbiguousOutputType { case_id } => {
                let actual_ambiguous_match_case_src = format_match_case(
                    &expand_match_case(registry, case_id),
                    0,
                    &FormatOptions {
                        ident_size_in_spaces: 4,
                        print_db_indices: false,
                        print_fun_body_status: false,
                    },
                );
                assert_eq_up_to_white_space(
                    &actual_ambiguous_match_case_src,
                    expected_ambiguous_match_case_src,
                );
            }
            _ => panic!("Unexpected error: {:#?}", err),
        });
    }

    #[test]
    fn ambiguous_output_type() {
        let src = include_str!("../sample_code/should_fail/type_check/ambiguous_output_type.ph");
        expect_ambiguous_output_type_error(src, "S(problem) => Eq.Refl(Nat, Nat.S(problem,),),");
    }
}
