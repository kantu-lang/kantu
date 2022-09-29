use crate::{
    data::{node_registry::NodeRegistry, FileId},
    processing::{
        bind_type_independent::bind_symbols_to_identifiers,
        check_variant_return_types::check_variant_return_types_for_file, lex::lex,
        parse::parse_file, register::register_file,
        validate_fun_recursion::validate_fun_recursion_in_file,
    },
};

#[test]
fn hello_world() {
    let src = include_str!("sample_code/should_succeed/hello_world.ph");
    expect_success(src);
}

#[test]
fn optional_commas() {
    let src = include_str!("sample_code/should_succeed/optional_commas.ph");
    expect_success(src);
}

fn expect_success(src: &str) {
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let file = parse_file(tokens, file_id).expect("Parsing failed");
    let mut registry = NodeRegistry::empty();
    let file_id = register_file(&mut registry, file);
    let file = registry.file(file_id);
    let symbol_db = bind_symbols_to_identifiers(&registry, vec![file_id]).expect("Binding failed");
    let _variant_return_type_map = check_variant_return_types_for_file(&symbol_db, &registry, file)
        .expect("Extracting variant type args failed");
    validate_fun_recursion_in_file(&symbol_db, &registry, file)
        .expect("Fun recursion validation failed");
}
