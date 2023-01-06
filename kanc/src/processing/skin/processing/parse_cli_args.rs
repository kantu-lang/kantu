use super::super::data::prelude::*;

use std::path::{Path, PathBuf};

use path_clean::PathClean;

pub mod flags {
    pub const PACK_YSCL: &str = "--pack";
    pub const SINGLE_FILE: &str = "--file";
}

pub fn parse_args(args: &[String]) -> Result<CliOptions, InvalidCliArgsError> {
    let mut remaining = args.iter().skip(1);
    let mut pack_yscl_path: Option<String> = None;
    let mut single_file_path: Option<String> = None;

    while let Some(arg) = remaining.next() {
        if arg == flags::PACK_YSCL {
            if let Some(path) = remaining.next() {
                pack_yscl_path = Some(path.clone());
            } else {
                return Err(InvalidCliArgsError::MissingFlagValue(
                    flags::PACK_YSCL.to_string(),
                ));
            }
        }
        if arg == flags::SINGLE_FILE {
            if let Some(path) = remaining.next() {
                single_file_path = Some(path.clone());
            } else {
                return Err(InvalidCliArgsError::MissingFlagValue(
                    flags::SINGLE_FILE.to_string(),
                ));
            }
        } else {
            return Err(InvalidCliArgsError::UnrecognizedFlag(arg.clone()));
        }
    }

    let abs_cwd = {
        let cwd = std::env::current_dir().map_err(InvalidCliArgsError::CannotReadCwd)?;
        if cwd.is_absolute() {
            cwd
        } else {
            return Err(InvalidCliArgsError::CwdIsNotAbsolute(cwd));
        }
    };

    let pack_abs_path = match (pack_yscl_path, single_file_path) {
        (Some(_), Some(_)) => {
            return Err(InvalidCliArgsError::MutuallyExclusiveFlagsBothProvided(
                flags::PACK_YSCL.to_string(),
                flags::SINGLE_FILE.to_string(),
            ));
        }

        (Some(pack_yscl_path), None) => {
            let p = PathBuf::from(pack_yscl_path);

            if !p.ends_with("pack.yscl") {
                return Err(InvalidCliArgsError::PackYsclPathDidNotEndWithPackYscl(p));
            }

            PackPath::PackYscl(
                if p.is_absolute() {
                    p.to_path_buf()
                } else {
                    abs_cwd.join(p)
                }
                .clean(),
            )
        }

        (None, Some(single_file_path)) => {
            let p = PathBuf::from(single_file_path);

            if p.extension() != Some("k".as_ref()) {
                return Err(InvalidCliArgsError::SingleFilePathDidNotHaveKExtension(p));
            }

            PackPath::SingleFile(
                if p.is_absolute() {
                    p.to_path_buf()
                } else {
                    abs_cwd.join(p)
                }
                .clean(),
            )
        }

        (None, None) => {
            if let Some(p) = get_default_pack_yscl_path(&abs_cwd) {
                PackPath::PackYscl(p)
            } else {
                return Err(InvalidCliArgsError::CannotFindImplicitPackYsclPath);
            }
        }
    };

    Ok(CliOptions { pack_abs_path })
}

fn get_default_pack_yscl_path(abs_cwd: &Path) -> Option<PathBuf> {
    let mut current = abs_cwd;
    loop {
        let pack_yscl_path = current.join("pack.yscl");
        if pack_yscl_path.is_file() {
            return Some(pack_yscl_path);
        }
        if let Some(parent) = current.parent() {
            current = parent;
        } else {
            return None;
        }
    }
}
