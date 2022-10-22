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

                    let simplification_result = pamc::processing::simplify_ast::simplify_file(file);
                    match simplification_result {
                        Ok(file) => {
                            println!("Simplification success!");
                            println!("{:#?}", file);
                            print_separator();

                            let mut registry = pamc::data::node_registry::NodeRegistry::empty();
                            let file_node_id =
                                pamc::processing::register::register_file(&mut registry, file);
                            println!("Registered file.");
                            println!("{:#?}", registry.file(file_node_id));
                            print_separator();

                            let bind_result =
                        pamc::processing::bind_type_independent::bind_symbols_to_identifiers(
                            &registry,
                            vec![file_node_id],
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
                                        let symbol =
                                            symbol_db.identifier_symbols.get(identifier.id);
                                        if !symbol_db.symbol_sources.contains_key(&symbol) {
                                            identifiers_with_undefined_symbols.push(identifier);
                                        }
                                    }
                                    println!(
                                        "Identifiers with undefined symbols: {:#?}",
                                        identifiers_with_undefined_symbols
                                    );
                                    print_separator();

                                    let variant_return_type_validation_result = pamc::processing::check_variant_return_types::check_variant_return_types_for_file(&symbol_db, &registry, registry.file(file_node_id));
                                    match variant_return_type_validation_result {
                                        Ok(variant_db) => {
                                            println!("Variant return type validation success!");

                                            let rec_validation_result = pamc::processing::validate_fun_recursion::validate_fun_recursion_in_file(
                                        &symbol_db,
                                        &registry,
                                        registry.file(file_node_id),
                                    );
                                            match rec_validation_result {
                                                Ok(()) => {
                                                    // TODO: Restore type checking once we finish
                                                    // implementing the type checker.

                                                    //     println!("Recursion validation success!");
                                                    //     let mut symbol_db = symbol_db;
                                                    //     let type_check_result =
                                                    // pamc::processing::type_check::type_check_file(
                                                    //     &mut registry,
                                                    //     &mut symbol_db,
                                                    //     &variant_db,
                                                    //     file_node_id,
                                                    // );
                                                    //     match type_check_result {
                                                    //         Ok(type_map) => {
                                                    //             println!("Type check success!");
                                                    //             println!("{:#?}", type_map);
                                                    //         }
                                                    //         Err(err) => {
                                                    //             println!(
                                                    //                 "Type check error: {:#?}",
                                                    //                 err
                                                    //             );
                                                    //         }
                                                    //     }

                                                    use pamc::processing::generate_code::CompileTarget;
                                                    let code_gen_result = pamc::processing::generate_code::targets::javascript::JavaScript::generate_code(
                                                        &registry,
                                                        &symbol_db,
                                                        &variant_db,
                                                        &[file_node_id],
                                                    );
                                                    match code_gen_result {
                                                        Ok(js_ast) => {
                                                            println!("Code generation success!");
                                                            println!("{:#?}", js_ast);
                                                        }
                                                        Err(err) => {
                                                            println!(
                                                                "Code generation error: {:#?}",
                                                                err
                                                            );
                                                        }
                                                    }
                                                }
                                                Err(err) => {
                                                    println!(
                                                        "Recursion validation error: {:#?}",
                                                        err
                                                    );
                                                }
                                            }
                                        }
                                        Err(error) => {
                                            println!(
                                                "Variant return type validation error: {:#?}",
                                                error
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    println!("Bind error: {:#?}", e);
                                }
                            }
                        }
                        Err(err) => {
                            println!("Simplification error: {:#?}", err);
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
