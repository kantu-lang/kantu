use super::super::data::{
    error::InvalidCompilerOptionsError,
    options::{CliOptions, CompilerOptions},
};

use std::fs;

use yscl::prelude::parse_doc;

pub fn read_compiler_options(
    options: &CliOptions,
) -> Result<CompilerOptions, InvalidCompilerOptionsError> {
    let pack_yscl = fs::read_to_string(&options.pack_yscl_abs_path).map_err(|raw_err| {
        InvalidCompilerOptionsError::CannotReadPackYscl(options.pack_yscl_abs_path.clone(), raw_err)
    })?;
    let pack_yscl =
        parse_doc(&pack_yscl).map_err(InvalidCompilerOptionsError::CannotParsePackYscl)?;
    println!("TODO: {:#?}", pack_yscl);
    unimplemented!()
}
