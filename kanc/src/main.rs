use kanc::{
    data::{file_tree::FileTree, node_registry::NodeRegistry, FileId},
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
        lighten_ast::register_file_items,
        parse::parse_file,
        simplify_ast::simplify_file,
        type_check::type_check_file_items,
        validate_fun_recursion::validate_fun_recursion_in_file_items,
        validate_type_positivity::validate_type_positivity_in_file_items,
        validate_variant_return_types::validate_variant_return_types_in_file_items,
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
    let file_items =
        bind_files(file_id, vec![file], &FileTree::from_root(file_id)).expect("Binding failed");
    let mut registry = NodeRegistry::empty();
    let file_item_list_id = register_file_items(&mut registry, file_items);
    let file_item_list_id =
        validate_variant_return_types_in_file_items(&registry, file_item_list_id)
            .expect("Variant return type validation failed");
    let file_item_list_id = validate_fun_recursion_in_file_items(&mut registry, file_item_list_id)
        .expect("Fun recursion validation failed");
    let file_item_list_id =
        validate_type_positivity_in_file_items(&mut registry, file_item_list_id)
            .expect("Type positivity validation failed");
    type_check_file_items(
        &FileTree::from_root(file_id),
        &mut registry,
        file_item_list_id,
    )
    .expect("Type checking failed");
    let js_ast = JavaScript::generate_code(&registry, file_item_list_id.raw())
        .expect("Code generation failed");
    println!(
        "Compilation pipeline completed successfully!\n\n{}",
        format_file(&js_ast, &FormatOptions { indentation: 4 })
    );
}
