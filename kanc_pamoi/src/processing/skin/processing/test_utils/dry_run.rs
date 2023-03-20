use crate::{
    data::node_registry::NodeRegistry,
    processing::{
        bind_type_independent::bind_files,
        generate_code::{targets::javascript::JavaScript, CompileTarget},
        lighten_ast::register_file_items,
        simplify_ast::simplify_file,
        skin::processing::{
            format::FormatErrorForCli, parse_cli_args::parse_args,
            read_compiler_options::read_compiler_options, read_kantu_files::read_kantu_files,
        },
        type_check::type_check_file_items,
        validate_fun_recursion::validate_fun_recursion_in_file_items,
        validate_type_positivity::validate_type_positivity_in_file_items,
        validate_variant_return_types::validate_variant_return_types_in_file_items,
    },
};

pub fn run_pipeline_without_writing_files(args: &[String]) -> Result<String, String> {
    let mut out = "".to_string();

    let options = parse_args(&args).fmt_err(())?;
    let options = read_compiler_options(&options).fmt_err(())?;
    let (files, file_tree, file_path_map) = read_kantu_files(&options).fmt_err(())?;
    let files = files
        .into_iter()
        .map(|file| simplify_file(file))
        .collect::<Result<Vec<_>, _>>()
        .fmt_err(&file_path_map)?;
    let file_items =
        bind_files(file_tree.root(), files, &file_tree).fmt_err((&file_path_map, &file_tree))?;
    let mut registry = NodeRegistry::empty();
    let file_item_list_id = register_file_items(&mut registry, file_items);

    let file_item_list_id =
        validate_variant_return_types_in_file_items(&registry, file_item_list_id)
            .fmt_err(&registry)?;
    let file_item_list_id = validate_fun_recursion_in_file_items(&mut registry, file_item_list_id)
        .fmt_err(&registry)?;
    let file_item_list_id =
        validate_type_positivity_in_file_items(&mut registry, file_item_list_id)
            .fmt_err(&registry)?;
    let warnings = type_check_file_items(&file_tree, &mut registry, file_item_list_id)
        .fmt_err((&options, &file_path_map, &file_tree, &registry))?;
    let _js_file =
        JavaScript::generate_code(&registry, file_item_list_id.raw()).fmt_err(&registry)?;

    if warnings.is_empty() {
        out.push_str("Compiled successfully.\n");
    } else {
        out.push_str("Compiled with warnings:\n");
        for warning in &warnings {
            out.push_str(&format!("{}\n", warning.format_for_cli(&registry)));
        }
    }

    out.push_str(&format!(
        "Skipped writing output files, but would have tried writing them to {}.\n",
        options.target_dir.display()
    ));

    Ok(out)
}

trait FormatErr<T> {
    type Ok;
    type Err;

    fn fmt_err(self, data: T) -> Result<Self::Ok, Self::Err>;
}

impl<O, E, T> FormatErr<T> for Result<O, E>
where
    E: FormatErrorForCli<T>,
{
    type Ok = O;
    type Err = String;

    fn fmt_err(self, data: T) -> Result<O, String> {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => Err(format!("Error: {}", err.format_for_cli(data))),
        }
    }
}
