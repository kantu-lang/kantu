use super::*;

fn expect_universe_inconsistency_error(src: &str) {
    expect_type_check_error(src, |_registry, err| match err {
        _ => panic!("UNIMPLEMENTED. The error is: {:#?}", err),
    });
}

#[ignore]
#[test]
fn currys_paradox() {
    let src = include_str!("../../../../sample_code/should_fail/single_file/type_check/universe_inconsistency/currys_paradox.k");
    expect_universe_inconsistency_error(src);
}

#[ignore]
#[test]
fn russells_paradox() {
    let src = include_str!("../../../../sample_code/should_fail/single_file/type_check/universe_inconsistency/russells_paradox.k");
    expect_universe_inconsistency_error(src);
}
