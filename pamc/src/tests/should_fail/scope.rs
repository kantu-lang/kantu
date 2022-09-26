use super::*;

#[test]
fn reference_let_in_body() {
    let src = include_str!("../sample_code/should_fail/scope/ref_let_in_body.ph");
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let file = parse_file(tokens, file_id).expect("Parsing failed");
    let mut registry = NodeRegistry::empty();
    let file_id = register_file(&mut registry, file);
    let mut provider = SymbolProvider::new();
    let err = bind_symbols_to_identifiers(&registry, vec![file_id], &mut provider)
        .expect_err("Binding unexpectedly succeeded");
    match err {
        BindError::NameNotFound(err) => {
            assert_eq!(err.name, standard_ident_name("a"), "Unexpected param name");
        }
        _ => panic!("Unexpected error: {:#?}", err),
    }
}

#[test]
fn reference_type_in_param() {
    let src = include_str!("../sample_code/should_fail/scope/ref_type_in_param.ph");
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let file = parse_file(tokens, file_id).expect("Parsing failed");
    let mut registry = NodeRegistry::empty();
    let file_id = register_file(&mut registry, file);
    let mut provider = SymbolProvider::new();
    let err = bind_symbols_to_identifiers(&registry, vec![file_id], &mut provider)
        .expect_err("Binding unexpectedly succeeded");
    match err {
        BindError::NameNotFound(err) => {
            assert_eq!(err.name, standard_ident_name("U"), "Unexpected param name");
        }
        _ => panic!("Unexpected error: {:#?}", err),
    }
}

#[test]
fn reference_fun_in_param() {
    let src = include_str!("../sample_code/should_fail/scope/ref_fun_in_param.ph");
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let file = parse_file(tokens, file_id).expect("Parsing failed");
    let mut registry = NodeRegistry::empty();
    let file_id = register_file(&mut registry, file);
    let mut provider = SymbolProvider::new();
    let err = bind_symbols_to_identifiers(&registry, vec![file_id], &mut provider)
        .expect_err("Binding unexpectedly succeeded");
    match err {
        BindError::NameNotFound(err) => {
            assert_eq!(err.name, standard_ident_name("g"), "Unexpected param name");
        }
        _ => panic!("Unexpected error: {:#?}", err),
    }
}

#[test]
fn reference_fun_in_return_type() {
    let src = include_str!("../sample_code/should_fail/scope/ref_fun_in_return_type.ph");
    let file_id = FileId(0);
    let tokens = lex(src).expect("Lexing failed");
    let file = parse_file(tokens, file_id).expect("Parsing failed");
    let mut registry = NodeRegistry::empty();
    let file_id = register_file(&mut registry, file);
    let mut provider = SymbolProvider::new();
    let err = bind_symbols_to_identifiers(&registry, vec![file_id], &mut provider)
        .expect_err("Binding unexpectedly succeeded");
    match err {
        BindError::NameNotFound(err) => {
            assert_eq!(err.name, standard_ident_name("g"), "Unexpected param name");
        }
        _ => panic!("Unexpected error: {:#?}", err),
    }
}
