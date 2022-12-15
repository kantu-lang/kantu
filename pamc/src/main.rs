use pamc::{
    data::{node_registry::NodeRegistry, FileId},
    processing::{
        bind_type_independent::bind_files,
        generate_code::{
            targets::javascript::{
                format::{format_file, FormatOptions},
                JavaScript,
            },
            CompileTarget,
        },
        lex::lex,
        lighten_ast::lighten_file,
        parse::parse_file,
        simplify_ast::simplify_file,
        type_check::type_check_files,
        validate_fun_recursion::validate_fun_recursion_in_file,
        validate_variant_return_types::validate_variant_return_types_in_file,
    },
};

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

    let file_id = FileId(0);
    let tokens = lex(&file_content).expect("Lexing failed");
    let file = parse_file(tokens, file_id).expect("Parsing failed");
    let file = simplify_file(file).expect("AST Simplification failed");
    let file = bind_files(vec![file])
        .expect("Binding failed")
        .into_iter()
        .next()
        .unwrap();
    let mut registry = NodeRegistry::empty();
    let file_id = lighten_file(&mut registry, file);
    let file = registry.get(file_id);
    let file_id = validate_variant_return_types_in_file(&registry, file)
        .expect("Variant return type validation failed");
    let file_id = validate_fun_recursion_in_file(&mut registry, file_id)
        .expect("Fun recursion validation failed");
    type_check_files(&mut registry, &[file_id]).expect("Type checking failed");
    let js_ast =
        JavaScript::generate_code(&registry, &[file_id.raw()]).expect("Code generation failed");
    println!(
        "Compilation pipeline completed successfully!\n\n{}",
        format_file(&js_ast[0], &FormatOptions { indentation: 4 })
    );
}
