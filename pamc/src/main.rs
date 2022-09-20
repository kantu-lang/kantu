fn main() {
    let args: Vec<String> = std::env::args().collect();
    let in_flag_index = if let Some(i) = args.iter().position(|arg| arg == "--in") {
        i
    } else {
        panic!("Cannot find --in flag.")
    };
    if in_flag_index >= args.len() - 1 {
        panic!("There needs to be an argument after the --in flag.")
    }
    let in_file_path = &args[in_flag_index + 1];
    let file_content = if let Ok(content) = std::fs::read_to_string(in_file_path) {
        content
    } else {
        panic!(
            "Error reading {}. Perhaps the path is invalid.",
            in_file_path
        );
    };
    let lex_result = pamc::processing::lex::lex(&file_content);
    match lex_result {
        Ok(tokens) => {
            println!("Lex success!");
            for t in tokens
                .iter()
                .filter(|t| t.kind != pamc::data::token::TokenKind::Whitespace)
            {
                println!("{}        ({:?})", t.content, t.kind);
            }
            print_separator();

            let parse_result = pamc::processing::parse::parse_file(tokens, pamc::data::FileId(0));
            match parse_result {
                Ok(file) => {
                    println!("Parse success!");
                    println!("{:#?}", file);
                    print_separator();

                    let mut registry = pamc::data::node_registry::NodeRegistry::empty();
                    let type_title_case_id = registry.add_identifier_and_overwrite_its_id(
                        pamc::data::registered_ast::Identifier {
                            id: pamc::data::node_registry::NodeId::new(0),
                            start: None,
                            name: pamc::data::unregistered_ast::IdentifierName::Reserved(
                                pamc::data::unregistered_ast::ReservedIdentifierName::TypeTitleCase,
                            ),
                        },
                    );
                    let type_title_case_identifier =
                        registry.identifier(type_title_case_id).clone();
                    let file_node_id =
                        pamc::processing::register::register_file(&mut registry, file);
                    let file = registry.file(file_node_id);
                    println!("Registered file: {:#?}", file);

                    let bind_result =
                        pamc::processing::bind_type_independent::bind_symbols_to_identifiers(
                            &registry,
                            vec![file_node_id],
                            &[(
                                type_title_case_identifier,
                                pamc::data::symbol_database::SymbolSource::BuiltinTypeTitleCase,
                            )],
                        );
                    match bind_result {
                        Ok(symbol_db) => {
                            println!("Bind success!");
                            println!("{:#?}", symbol_db.identifier_symbols);
                            print_separator();
                            let all_identifiers = registry.TODO_identifiers().to_vec();
                            let mut unbound_identifiers = vec![];
                            let mut bound_identifiers = vec![];
                            for identifier in all_identifiers {
                                if !symbol_db.identifier_symbols.contains(identifier.id) {
                                    unbound_identifiers.push(identifier);
                                } else {
                                    bound_identifiers.push(identifier);
                                }
                            }
                            println!("Unbound identifiers: {:#?}", unbound_identifiers);
                            print_separator();
                            let mut identifiers_with_undefined_symbols = vec![];
                            for identifier in bound_identifiers {
                                let symbol = symbol_db.identifier_symbols.get(identifier.id);
                                if !symbol_db.symbol_sources.contains_key(&symbol) {
                                    identifiers_with_undefined_symbols.push(identifier);
                                }
                            }
                            println!(
                                "Identifiers with undefined symbols: {:#?}",
                                identifiers_with_undefined_symbols
                            );
                            print_separator();

                            let type_check_result = pamc::processing::type_check::type_check_file(
                                &registry,
                                &symbol_db.identifier_symbols,
                                file,
                            );
                            match type_check_result {
                                Ok(type_map) => {
                                    println!("Type check success!");
                                    println!("{:#?}", type_map);
                                }
                                Err(err) => {
                                    println!("Type check error: {:#?}", err);
                                }
                            }
                        }
                        Err(e) => {
                            println!("Bind error: {:#?}", e);
                        }
                    }
                }
                Err(err) => {
                    println!("Parse error: {:?}", err);
                }
            }
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }
}

fn print_separator() {
    println!("\n\n\n\n\n\n\n\n");
    for _ in 0..64 {
        for _ in 0..64 {
            print!("*");
        }
        println!();
    }
    println!("\n\n\n\n\n\n\n\n");
}
