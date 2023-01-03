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
            format::{FormatErrorForCli, FormatErrorForWithRegistry},
            parse_cli_args::parse_args,
            read_compiler_options::read_compiler_options,
            read_kantu_files::read_kantu_files,
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
    let options = parse_args(&args).print_err()?;
    let options = read_compiler_options(&options).print_err()?;
    let (files, file_tree) = read_kantu_files(&options).print_err()?;
    let files = files
        .into_iter()
        .map(|file| simplify_file(file))
        .collect::<Result<Vec<_>, _>>()
        .print_err()?;
    let file_items = bind_files(file_tree.root(), files, &file_tree).print_err()?;
    let mut registry = NodeRegistry::empty();
    let file_item_list_id = register_file_items(&mut registry, file_items);

    let file_item_list_id =
        validate_variant_return_types_in_file_items(&registry, file_item_list_id)
            .print_err_with_registry(&registry)?;
    let file_item_list_id = validate_fun_recursion_in_file_items(&mut registry, file_item_list_id)
        .print_err_with_registry(&registry)?;
    let file_item_list_id =
        validate_type_positivity_in_file_items(&mut registry, file_item_list_id)
            .print_err_with_registry(&registry)?;
    let warnings = type_check_file_items(&file_tree, &mut registry, file_item_list_id)
        .print_err_with_registry(&registry)?;
    let js_file = JavaScript::generate_code(&registry, file_item_list_id.raw())
        .print_err_with_registry(&registry)?;

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
            println!("{}\n", warning.format_for_cli_with_registry(&registry));
        }
    }

    match write_result {
        Ok(()) => println!("Successfully wrote output files."),
        Err(err) => {
            println!("Failed to write output files:\n");
            println!("{}", err.format_for_cli_with_registry(&registry));
            return Err(());
        }
    }

    Ok(())
}

trait PrintErr {
    type Ok;
    type Err;

    fn print_err(self) -> Result<Self::Ok, Self::Err>;
}

impl<O, E> PrintErr for Result<O, E>
where
    E: FormatErrorForCli,
{
    type Ok = O;
    type Err = ();

    fn print_err(self) -> Result<O, ()> {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => {
                println!("{}", err.format_for_cli());
                Err(())
            }
        }
    }
}

trait PrintErrWithRegistry {
    type Ok;
    type Err;

    fn print_err_with_registry(self, registry: &NodeRegistry) -> Result<Self::Ok, Self::Err>;
}

impl<O, E> PrintErrWithRegistry for Result<O, E>
where
    E: FormatErrorForWithRegistry,
{
    type Ok = O;
    type Err = ();

    fn print_err_with_registry(self, registry: &NodeRegistry) -> Result<O, ()> {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => {
                println!("{}", err.format_for_cli_with_registry(registry));
                Err(())
            }
        }
    }
}
