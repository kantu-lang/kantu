use kanc::{
    data::node_registry::NodeRegistry,
    processing::{
        bind_type_independent::bind_files,
        generate_code::{
            targets::javascript::{
                format::{format_file as format_js_file, FormatOptions},
                JavaScript,
            },
            CompileTarget,
        },
        lighten_ast::register_file_items,
        simplify_ast::simplify_file,
        skin::processing::{
            format::FormatErrorForCli, parse_cli_args::parse_args,
            read_compiler_options::read_compiler_options, read_kantu_files::read_kantu_files,
            write_target_files::write_target_files,
        },
        type_check::type_check_file_items,
        validate_fun_recursion::validate_fun_recursion_in_file_items,
        validate_type_positivity::validate_type_positivity_in_file_items,
        validate_variant_return_types::validate_variant_return_types_in_file_items,
    },
};

use std::path::PathBuf;

fn main() -> Result<(), ()> {
    let args: Vec<String> = std::env::args().collect();
    let options = parse_args(&args).map_err(print_and_drop)?;
    let options = read_compiler_options(&options).map_err(print_and_drop)?;
    let (files, file_tree) = read_kantu_files(&options).map_err(print_and_drop)?;
    let files = files
        .into_iter()
        .map(|file| simplify_file(file))
        .collect::<Result<Vec<_>, _>>()
        .map_err(print_and_drop)?;
    let file_items = bind_files(file_tree.root(), files, &file_tree).map_err(print_and_drop)?;
    let mut registry = NodeRegistry::empty();
    let file_item_list_id = register_file_items(&mut registry, file_items);

    let file_item_list_id =
        validate_variant_return_types_in_file_items(&registry, file_item_list_id)
            .map_err(print_and_drop)?;
    let file_item_list_id = validate_fun_recursion_in_file_items(&mut registry, file_item_list_id)
        .map_err(print_and_drop)?;
    let file_item_list_id =
        validate_type_positivity_in_file_items(&mut registry, file_item_list_id)
            .map_err(print_and_drop)?;
    let warnings = type_check_file_items(&file_tree, &mut registry, file_item_list_id)
        .map_err(print_and_drop)?;
    let js_file =
        JavaScript::generate_code(&registry, file_item_list_id.raw()).map_err(print_and_drop)?;

    let write_result = write_target_files(
        &options,
        vec![(
            PathBuf::from("index.js"),
            format_js_file(&js_file, &FormatOptions { indentation: 4 }),
        )],
    );

    if warnings.is_empty() {
        println!("Compiled successfully.");
    } else {
        println!("Compiled with warnings:\n");
        for warning in &warnings {
            println!("{}\n", warning.format_for_cli());
        }
    }

    match write_result {
        Ok(()) => println!("Successfully wrote output files."),
        Err(err) => {
            println!("Failed to write output files:\n");
            println!("{}", err.format_for_cli());
            return Err(());
        }
    }

    Ok(())
}

fn print_and_drop<T: FormatErrorForCli>(err: T) {
    println!("{}", err.format_for_cli());
}
