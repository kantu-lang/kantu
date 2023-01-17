use super::*;

fn expect_universe_inconsistency_error(src: &str) {
    expect_type_check_error(src, |_registry, err| match err {
        _ => panic!("UNIMPLEMENTED. The error is: {:#?}", err),
    });
}

// TODO: Fix
#[ignore]
#[test]
fn currys_paradox() {
    let src = include_str!("../../../../sample_code/should_fail/single_file/type_check/universe_inconsistency/currys_paradox.k");
    expect_universe_inconsistency_error(src);
}

// TODO: Uncomment `let false = ...` in sample code
// and reignore this test.
#[test]
fn russells_paradox() {
    let src = include_str!("../../../../sample_code/should_fail/single_file/type_check/universe_inconsistency/russells_paradox.k");
    // expect_universe_inconsistency_error(src);
    expect_type_check_error(src, |registry, err| match err {
        TypeCheckError::TypeMismatch {
            expression_id,
            expected_type_id,
            actual_type_id,
        } => {
            use crate::processing::test_utils::{expand_lightened::*, format::*};
            println!(
                "\n\nEXPRESSION: {}",
                format_expression_with_default_options(&expand_expression(registry, expression_id))
            );
            println!(
                "\n\nEXPECTED_TYPE: {}",
                format_expression_with_default_options(&expand_expression(
                    registry,
                    expected_type_id.raw()
                ))
            );
            println!(
                "\n\nACTUAL_TYPE: {}",
                format_expression_with_default_options(&expand_expression(
                    registry,
                    actual_type_id.raw()
                ))
            );
            panic!("\n\nUNIMPLEMENTED. The error is: {:#?}", err);
        }
        other => panic!("UNIMPLEMENTED. The error is: {:#?}", other),
    });
}
