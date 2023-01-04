use super::super::data::{
    error::InvalidCompilerOptionsError,
    options::{CliOptions, CompilerOptions},
};

use std::fs;

use yscl::prelude::parse_doc;

pub fn read_compiler_options(
    options: &CliOptions,
) -> Result<CompilerOptions, InvalidCompilerOptionsError> {
    let pack_omlet = fs::read_to_string(&options.pack_omlet_abs_path).map_err(|raw_err| {
        InvalidCompilerOptionsError::CannotReadPackOmlet(
            options.pack_omlet_abs_path.clone(),
            raw_err,
        )
    })?;
    let pack_omlet =
        parse_doc(&pack_omlet).map_err(InvalidCompilerOptionsError::CannotParsePackOmlet)?;
    println!("TODO: {:#?}", pack_omlet);
    unimplemented!()
}
