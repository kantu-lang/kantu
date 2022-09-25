use crate::{
    data::{
        node_registry::{NodeId, NodeRegistry},
        registered_ast,
        symbol_database::SymbolSource,
        unregistered_ast::IdentifierName,
        unregistered_ast::ReservedIdentifierName,
        FileId,
    },
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
    let type_title_case_id =
        registry.add_identifier_and_overwrite_its_id(registered_ast::Identifier {
            id: NodeId::new(0),
            start: None,
            name: IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase),
        });
    let type_title_case_identifier = registry.identifier(type_title_case_id).clone();
    let builtin_identifiers = vec![(
        type_title_case_identifier,
        SymbolSource::BuiltinTypeTitleCase,
    )];
    let file_id = register_file(&mut registry, file);
    let file = registry.file(file_id);
    let symbol_db = bind_symbols_to_identifiers(&registry, vec![file_id], &builtin_identifiers)
        .expect("Binding failed");
    validate_fun_recursion_in_file(&symbol_db, file).expect("Fun recursion validation failed");
}
