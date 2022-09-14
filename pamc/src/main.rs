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
            println!("Success!");
            for t in &tokens {
                if t.kind != pamc::lex::TokenKind::Whitespace {
                    println!("{}        ({:?})", t.content, t.kind);
                }
            }
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }
}
