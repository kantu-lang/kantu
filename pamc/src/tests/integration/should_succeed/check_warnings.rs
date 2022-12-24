use super::*;

fn expect_success_with_one_or_more_warnings(
    src: &str,
    expected_warnings: &[TypeCheckWarningExpectation],
) {
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
    let file = registry.get(file_id);
    let file_id = validate_variant_return_types_in_file(&registry, file)
        .expect("Variant return type validation failed");
    let file_id = validate_fun_recursion_in_file(&mut registry, file_id)
        .expect("Fun recursion validation failed");
    let file_id = validate_type_positivity_in_file(&mut registry, file_id)
        .expect("Type positivity validation failed");
    let warnings = type_check_files(&mut registry, &[file_id]).expect("Type checking failed");
    assert_expectations_match_actual_warnings(&registry, expected_warnings, &warnings);
    let _js_ast =
        JavaScript::generate_code(&registry, &[file_id.raw()]).expect("Code generation failed");
}

#[test]
fn type_assertion_goal_lhs() {
    use TypeCheckWarningExpectation::*;
    let src =
        include_str!("../../sample_code/should_succeed/should_succeed_with_warnings/check/type_assertion_goal_lhs.ph");
    let expected_warnings = vec![
        TypeAssertionGoalLhs {
            assertion_src: "goal: Nat",
        },
        TypeAssertionGoalLhs {
            assertion_src: "goal: ?",
        },
    ];
    expect_success_with_one_or_more_warnings(src, &expected_warnings);
}

// TODO: Fix
#[ignore]
#[test]
fn mismatched_nf_comparees() {
    use TypeCheckWarningExpectation::*;
    let src =
        include_str!("../../sample_code/should_succeed/should_succeed_with_warnings/check/mismatched_nf_comparees.ph");
    let expected_warnings = vec![
        NormalFormAssertionCompareeMismatch {
            original_left_src: "goal",
            rewritten_left_src: "Nat",
            original_right_src: "Eq(Nat, Nat.O, Nat.O,)",
            rewritten_right_src: "Eq(Nat, Nat.O, Nat.O,)",
        },
        NormalFormAssertionCompareeMismatch {
            original_left_src: "n",
            // TODO: Check
            rewritten_left_src: "Nat.S(n',)",
            original_right_src: "Nat.S(n,)",
            // TODO: Check
            rewritten_right_src: "Nat.S(Nat.S(n',),)",
        },
        NormalFormAssertionCompareeQuestionMark {
            original_left_src: "m",
            // TODO: Check
            rewritten_left_src: "Nat.O",
        },
        NormalFormAssertionCompareeQuestionMark {
            original_left_src: "m",
            // TODO: Check
            rewritten_left_src: "Nat.S(m',)",
        },
    ];
    expect_success_with_one_or_more_warnings(src, &expected_warnings);
}

// TODO: Delete
// fn check() {
//     use TypeCheckWarningExpectation::*;
//     let src =
//         include_str!("../../sample_code/should_succeed/should_succeed_with_warnings/check/type_assertion_goal_lhs.ph");
//     let expected_warnings = vec![
//         // type_assertion_goal_lhs
//         TypeAssertionGoalLhs {
//             assertion_src: "goal: Nat",
//         },
//         TypeAssertionGoalLhs {
//             assertion_src: "goal: ?",
//         },
//         // mismatched_comparees
//         NormalFormCompareeMismatch {
//             original_left_src: "goal",
//             rewritten_left_src: "Nat",
//             original_right_src: "Eq(Nat, Nat.O, Nat.O,)",
//             rewritten_right_src: "Eq(Nat, Nat.O, Nat.O,)",
//         },
//         NormalFormCompareeMismatch {
//             original_left_src: "n",
//             // TODO: Check
//             rewritten_left_src: "Nat.S(n')",
//             original_right_src: "Nat.S(n)",
//             // TODO: Check
//             rewritten_right_src: "Nat.S(Nat.S(n'))",
//         },
//         // question_mark_rhs
//         NormalFormCompareeQuestionMark {
//             original_left_src: "m",
//             // TODO: Check
//             rewritten_left_src: "Nat.O",
//         },
//         NormalFormCompareeQuestionMark {
//             original_left_src: "m",
//             // TODO: Check
//             rewritten_left_src: "Nat.S(m')",
//         },
//         // goal_checkee
//         NormalFormCompareeQuestionMark {
//             original_left_src: "goal",
//             rewritten_left_src: "Nat",
//         },
//         // symbolically_invalid_annotations1
//         // TODO
//     ];
//     expect_success_with_one_or_more_warnings(src, &expected_warnings);
// }
