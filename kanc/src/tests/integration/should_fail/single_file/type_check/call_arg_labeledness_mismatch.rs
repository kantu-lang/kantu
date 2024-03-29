use super::*;

fn expect_call_arg_labeledness_mismatch_type_error(src: &str, expected_call_src: &str) {
    expect_type_check_error(src, |registry, err| match err {
        TypeCheckError::CallLabelednessMismatch { call_id } => {
            let actual_call_src = format_expression(
                &expand_expression(registry, ExpressionId::Call(call_id)),
                0,
                &FORMAT_OPTIONS_FOR_COMPARISON,
            );
            assert_eq_up_to_white_space(&actual_call_src, expected_call_src);
        }
        _ => {
            panic!("Unexpected error: {:#?}", err)
        }
    });
}

#[test]
fn labeled_fun_unlabeled_args() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/call_arg_labeledness_mismatch/labeled_fun_unlabeled_args.k"
    );
    expect_call_arg_labeledness_mismatch_type_error(src, "plus(o, o,)");
}

#[test]
fn labeled_variant_unlabeled_args() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/call_arg_labeledness_mismatch/labeled_variant_unlabeled_args.k"
    );
    expect_call_arg_labeledness_mismatch_type_error(src, "Nat.s(Nat.o,)");
}

#[test]
fn labeled_type_unlabeled_args() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/call_arg_labeledness_mismatch/labeled_type_unlabeled_args.k"
    );
    expect_call_arg_labeledness_mismatch_type_error(src, "List(Nat,)");
}

#[test]
fn unlabeled_fun_labeled_args() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/call_arg_labeledness_mismatch/unlabeled_fun_labeled_args.k"
    );
    expect_call_arg_labeledness_mismatch_type_error(src, "plus(left: o, right: o,)");
}

#[test]
fn unlabeled_variant_labeled_args() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/call_arg_labeledness_mismatch/unlabeled_variant_labeled_args.k"
    );
    expect_call_arg_labeledness_mismatch_type_error(src, "Nat.s(pred: o,)");
}

#[test]
fn unlabeled_type_labeled_args() {
    let src = include_str!(
        "../../../../sample_code/should_fail/single_file/type_check/call_arg_labeledness_mismatch/unlabeled_type_labeled_args.k"
    );
    expect_call_arg_labeledness_mismatch_type_error(src, "List(Item: Nat,)");
}
