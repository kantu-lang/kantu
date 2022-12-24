use super::*;

mod ambiguous_output_type;
mod call_arg_labeledness_mismatch;
mod cannot_infer_type_of_empty_match;
mod duplicate_match_case;
mod extraneous_match_case;
mod illegal_callee;
mod illegal_type;
mod labeled_call_args;
mod labeled_match_case_params;
mod missing_match_case;
mod non_adt_matchee;
mod type_mismatch;
mod wrong_number_of_args;
mod wrong_number_of_case_params;

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
    let file = registry.get(file_id);

    let file_id = validate_variant_return_types_in_file(&registry, file)
        .expect("Variant return type validation failed");
    let file_id = validate_fun_recursion_in_file(&mut registry, file_id)
        .expect("Fun recursion validation failed");
    let file_id = validate_type_positivity_in_file(&mut registry, file_id)
        .expect("Type positivity validation failed");
    let err = type_check_files(&mut registry, &[file_id])
        .expect_err("Type checking unexpected succeeded");
    panicker(&registry, err);
}
