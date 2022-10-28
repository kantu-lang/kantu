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

                            let bind_result =
                                pamc::processing::x_bind_type_independent::bind_files(vec![file]);
                            match bind_result {
                                Ok((files, _symbol_provider, _dot_targets)) => {
                                    println!("Bind success!");
                                    println!("{:#?}", &files[0]);

                                    let _lightened_file = pamc::processing::x_lighten::lighten_file(
                                        &mut pamc::data::x_node_registry::NodeRegistry::empty(),
                                        files[0].clone(),
                                    );
                                    println!("Lightened file!");
                                }
                                Err(err) => {
                                    println!("Bind error: {:?}", err);
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
