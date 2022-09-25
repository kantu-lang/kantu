use crate::{
    data::{node_registry::NodeRegistry, FileId},
    processing::{
        bind_type_independent::bind_symbols_to_identifiers, lex::lex, parse::parse_file,
        register::register_file, validate_fun_recursion::validate_fun_recursion_in_file,
    },
};

#[test]
fn hello_world() {
    let file_id = FileId(0);
    let src = include_str!("sample_code/hello_world.ph");
    let tokens = lex(src).expect("Lexing failed");
    let file = parse_file(tokens, file_id).expect("Parsing failed");
    let mut registry = NodeRegistry::empty();
    let file_id = register_file(&mut registry, file);
    let file = registry.file(file_id);
    let symbol_db = bind_symbols_to_identifiers(&registry, vec![file_id]).expect("Binding failed");
    validate_fun_recursion_in_file(&symbol_db, file).expect("Fun recursion validation failed");
}
