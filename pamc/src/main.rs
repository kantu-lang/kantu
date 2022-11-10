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

                                    let mut registry =
                                        pamc::data::x_node_registry::NodeRegistry::empty();
                                    let lightened_file = pamc::processing::x_lighten::lighten_file(
                                        &mut registry,
                                        files[0].clone(),
                                    );
                                    println!("Lightened file!");
                                    print_separator();

                                    use pamc::processing::{
                                        x_expand_lightened::expand_expression, x_type_check::*,
                                    };

                                    let type_check_result =
                                        type_check_files(&mut registry, &[lightened_file]);
                                    match type_check_result {
                                        Ok(_) => {
                                            println!("Type check success!");
                                        }
                                        Err(err) => {
                                            println!("Type check error: {:?}", err);
                                            if let TypeCheckError::IllegalTypeExpression(
                                                expression_id,
                                            ) = &err
                                            {
                                                println!(
                                                    "Illegal type expression: {:#?}",
                                                    expand_expression(&registry, *expression_id,)
                                                );
                                            }
                                            if let TypeCheckError::TypeMismatch {
                                                expression_id,
                                                expected_type_id,
                                                actual_type_id,
                                            } = &err
                                            {
                                                println!(
                                                    "TYPE_MISMATCH.expression: {:#?}",
                                                    expand_expression(&registry, *expression_id)
                                                );
                                                println!(
                                                    "TYPE_MISMATCH.expected_type: {:#?}",
                                                    expand_expression(
                                                        &registry,
                                                        expected_type_id.raw()
                                                    )
                                                );
                                                println!(
                                                    "TYPE_MISMATCH.actual_type: {:#?}",
                                                    expand_expression(
                                                        &registry,
                                                        actual_type_id.raw()
                                                    )
                                                );
                                            }
                                            if let TypeCheckError::WrongNumberOfArguments {
                                                call_id,
                                                expected,
                                                actual,
                                            } = &err
                                            {
                                                println!(
                                                    "TYPE_MISMATCH.call: {:#?}",
                                                    expand_expression(
                                                        &registry,
                                                        pamc::data::x_light_ast::ExpressionId::Call(
                                                            *call_id
                                                        )
                                                    )
                                                );
                                                println!(
                                                    "TYPE_MISMATCH.expected_arity: {}",
                                                    expected
                                                );
                                                println!("TYPE_MISMATCH.actual_arity: {}", actual);
                                            }
                                        }
                                    }
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
