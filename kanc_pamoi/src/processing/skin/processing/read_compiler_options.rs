use super::super::data::prelude::*;

use std::{fs, path::Path};

use yscl::{prelude::parse_doc, tree as yt};

mod pack_keys {
    pub const VERSION: &str = "kantu_version";
    pub const SHOW_DB_INDICES: &str = "show_db_indices";
}

pub fn read_compiler_options(
    options: &CliOptions,
) -> Result<CompilerOptions, InvalidCompilerOptionsError> {
    match &options.pack_abs_path {
        PackPath::SingleFile(single_file_abs_path) => {
            read_compiler_options_from_single_file_path(single_file_abs_path)
        }
        PackPath::PackYscl(pack_yscl_abs_path) => {
            read_compiler_options_from_pack_yscl_path(pack_yscl_abs_path)
        }
    }
}

fn read_compiler_options_from_single_file_path(
    single_file_abs_path: &Path,
) -> Result<CompilerOptions, InvalidCompilerOptionsError> {
    get_default_options(single_file_abs_path)
}

fn get_default_options(
    single_file_abs_path: &Path,
) -> Result<CompilerOptions, InvalidCompilerOptionsError> {
    Ok(CompilerOptions {
        pack_abs_path: PackPath::SingleFile(single_file_abs_path.to_owned()),
        kantu_version: KantuVersion::V1_0_0,
        target_dir: single_file_abs_path
            .with_file_name("target")
            .with_extension(""),
        show_db_indices: true,
    })
}

fn read_compiler_options_from_pack_yscl_path(
    pack_yscl_abs_path: &Path,
) -> Result<CompilerOptions, InvalidCompilerOptionsError> {
    let pack_yscl_src = fs::read_to_string(pack_yscl_abs_path).map_err(|raw_err| {
        InvalidCompilerOptionsError::CannotReadPackYscl(pack_yscl_abs_path.to_owned(), raw_err)
    })?;
    let pack_yscl = parse_doc(&pack_yscl_src).map_err(|raw_err| {
        InvalidCompilerOptionsError::CannotParsePackYscl {
            src: pack_yscl_src,
            err: raw_err,
        }
    })?;
    build_options(pack_yscl_abs_path, &pack_yscl)
}

fn build_options(
    pack_yscl_abs_path: &Path,
    pack: &yt::Map,
) -> Result<CompilerOptions, InvalidCompilerOptionsError> {
    let kantu_version = get_required_str_entry(pack, pack_keys::VERSION)?;
    let Some(kantu_version) = KantuVersion::new(&kantu_version) else {
        return Err(InvalidCompilerOptionsError::IllegalKantuVersion(kantu_version));
    };

    let target_dir = pack_yscl_abs_path
        .parent()
        .expect("pack.yscl path should have parent")
        .join("target")
        .to_path_buf();

    let show_db_indices_value = pack.get(pack_keys::SHOW_DB_INDICES);
    let show_db_indices = match show_db_indices_value {
        Some(yt::Node::Atom(val)) => {
            if val.value == "true" {
                true
            } else if val.value == "false" {
                false
            } else {
                return Err(InvalidCompilerOptionsError::ExpectedBoolButGot {
                    key: pack_keys::SHOW_DB_INDICES.to_string(),
                    value: yt::Node::Atom(val.clone()),
                });
            }
        }
        Some(val) => {
            return Err(InvalidCompilerOptionsError::ExpectedBoolButGot {
                key: pack_keys::SHOW_DB_INDICES.to_string(),
                value: val.clone(),
            });
        }
        None => false,
    };
    Ok(CompilerOptions {
        pack_abs_path: PackPath::PackYscl(pack_yscl_abs_path.to_owned()),
        kantu_version,
        target_dir,
        show_db_indices,
    })
}

fn get_required_str_entry(
    pack: &yt::Map,
    key: &str,
) -> Result<String, InvalidCompilerOptionsError> {
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
