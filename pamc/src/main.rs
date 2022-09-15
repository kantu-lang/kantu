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
    let lex_result = pamc::lex::lex(&file_content);
    match lex_result {
        Ok(tokens) => {
            println!("Lex success!");
            for t in tokens
                .iter()
                .filter(|t| t.kind != pamc::lex::TokenKind::Whitespace)
            {
                println!("{}        ({:?})", t.content, t.kind);
            }
            println!("\n\n\n\n\n\n\n\n\n\n\n\n\n");

            let parse_result = pamc::parse::parse_file(tokens);
            println!("Parse result: {:?}", parse_result);
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }
}
