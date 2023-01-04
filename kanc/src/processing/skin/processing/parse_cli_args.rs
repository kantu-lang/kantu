use super::super::data::{error::InvalidCliArgsError, options::CliOptions};

use std::path::{Path, PathBuf};

use path_clean::PathClean;

pub mod flags {
    pub const PACK_YSCL: &str = "--pack";
}

pub fn parse_args(args: &[String]) -> Result<CliOptions, InvalidCliArgsError> {
    let mut remaining = args.iter();
    let mut pack_yscl_path: Option<String> = None;

    while let Some(arg) = remaining.next() {
        if arg == flags::PACK_YSCL {
            if let Some(path) = remaining.next() {
                pack_yscl_path = Some(path.clone());
            } else {
                return Err(InvalidCliArgsError::ExpectedPathAfterFlag(
                    flags::PACK_YSCL.to_string(),
                ));
            }
        } else {
            return Err(InvalidCliArgsError::UnrecognizedArg(arg.clone()));
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

    let pack_yscl_abs_path = if let Some(p) = pack_yscl_path {
        let p = PathBuf::from(p);
        if p.is_absolute() {
            p.to_path_buf()
        } else {
            abs_cwd.join(p)
        }
        .clean()
    } else if let Some(p) = get_default_pack_yscl_path(&abs_cwd) {
        p
    } else {
        return Err(InvalidCliArgsError::CannotFindImplicitPackYsclPath);
    };
    Ok(CliOptions { pack_yscl_abs_path })
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
