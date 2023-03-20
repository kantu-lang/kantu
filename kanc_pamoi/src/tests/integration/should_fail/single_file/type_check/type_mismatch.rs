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
                &FORMAT_OPTIONS_FOR_COMPARISON,
            );
            assert_eq_up_to_white_space(&actual_expression_src, expected_expression_src);

            let actual_expected_type_src = format_expression(
                &expand_expression(registry, expected_type_id.raw()),
                0,
                &FORMAT_OPTIONS_FOR_COMPARISON,
            );
            assert_eq_up_to_white_space(&actual_expected_type_src, expected_expected_type_src);

            let actual_actual_type_src = format_expression(
                &expand_expression(registry, actual_type_id.raw()),
                0,
                &FORMAT_OPTIONS_FOR_COMPARISON,
            );
            assert_eq_up_to_white_space(&actual_actual_type_src, expected_actual_type_src);
        }
        _ => panic!("Unexpected error: {:#?}", err),
    });
}

#[test]
fn adt() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/type_mismatch/adt.k"
    );
    expect_type_mismatch_error(src, "U2.u2", "U1", "U2");
}

#[test]
fn type_not_a_type() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/type_mismatch/type_not_a_type.k"
    );
    expect_type_mismatch_error(src, "Type", "Type", "Type1");
}

#[test]
fn ill_typed_param_type() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/type_mismatch/ill_typed_param_type.k"
    );
    expect_type_mismatch_error(
        src,
        "Eq.refl(Nat, x',)",
        "Eq(Nat, x, Nat.s(x',),)",
        "Eq(Nat, x', x',)",
    );
}

#[test]
fn ill_typed_match_case_output_evaluates_to_well_typed_term() {
    let src = include_str!(
            "../../../../sample_code/should_fail/single_file/type_check/type_mismatch/ill_typed_match_case_output_evaluates_to_well_typed_term.k"
        );
    expect_type_mismatch_error(src, "Nat.o", "Bool", "Nat");
}

#[test]
fn differing_generated_underscore_name_expressions() {
    let src = include_str!(
            "../../../../sample_code/should_fail/single_file/type_check/type_mismatch/differing_generated_underscore_name_expressions.k"
        );
    expect_type_mismatch_error(
        src,
        "ColorEq.refl(x,)",
        "ColorEq(x, match x { .c(:b, :r, :g) => Color.c(r: g, :g, :b,), },)",
        "ColorEq(x, x,)",
    );
}

#[test]
fn misordered_fun_params() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/type_mismatch/misordered_fun_params.k"
    );
    expect_type_mismatch_error(
        src,
        "foo",
        "forall(~y: Nat, ~x: Nat,) { Nat }",
        "forall(~x: Nat, ~y: Nat,) { Nat }",
    );
}

#[test]
fn wrong_empty() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/type_mismatch/wrong_empty.k"
    );
    expect_type_mismatch_error(src, "e1", "Empty2", "Empty1");
}
