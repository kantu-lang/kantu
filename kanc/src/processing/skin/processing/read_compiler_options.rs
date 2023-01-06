use super::super::data::{
    error::InvalidCompilerOptionsError,
    options::{CliOptions, CompilerOptions, KantuVersion},
};

use std::fs;

use yscl::{prelude::parse_doc, tree as yt};

mod pack_keys {
    pub const VERSION: &str = "kantu_version";
}

pub fn read_compiler_options(
    options: &CliOptions,
) -> Result<CompilerOptions, InvalidCompilerOptionsError> {
    let pack_yscl_src = fs::read_to_string(&options.pack_yscl_abs_path).map_err(|raw_err| {
        InvalidCompilerOptionsError::CannotReadPackYscl(options.pack_yscl_abs_path.clone(), raw_err)
    })?;
    let pack_yscl = parse_doc(&pack_yscl_src).map_err(|raw_err| {
        InvalidCompilerOptionsError::CannotParsePackYscl {
            src: pack_yscl_src,
            err: raw_err,
        }
    })?;
    build_options(options, &pack_yscl)
}

fn build_options(
    options: &CliOptions,
    pack: &yt::Map,
) -> Result<CompilerOptions, InvalidCompilerOptionsError> {
    let kantu_version = get_str_entry(pack, pack_keys::VERSION)?;
    let Some(kantu_version) = KantuVersion::new(&kantu_version) else {
        return Err(InvalidCompilerOptionsError::IllegalKantuVersion(kantu_version));
    };
    let target_dir = options
        .pack_yscl_abs_path
        .parent()
        .expect("pack.yscl path should have parent")
        .join("target")
        .to_path_buf();
    Ok(CompilerOptions {
        pack_yscl_abs_path: options.pack_yscl_abs_path.clone(),
        kantu_version,
        target_dir,
    })
}

fn get_str_entry(pack: &yt::Map, key: &str) -> Result<String, InvalidCompilerOptionsError> {
    let value = pack
        .get(key)
        .ok_or_else(|| InvalidCompilerOptionsError::MissingEntry {
            key: key.to_string(),
        })?
        .as_ref();
    match value {
        yt::NodeRef::Atom(atom) => Ok(atom.value.clone()),
        other => Err(InvalidCompilerOptionsError::ExpectedAtomButGotCollection {
            key: key.to_owned(),
            collection: other.to_owned(),
        }),
    }
}
