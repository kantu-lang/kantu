use super::super::data::{error::InvalidCliArgsError, options::CliOptions};

use std::path::PathBuf;

pub mod flags {
    pub const PACK_OMLET: &str = "--pack";
}

pub fn parse_args(args: &[String]) -> Result<CliOptions, InvalidCliArgsError> {
    let mut remaining = args.iter();
    let mut provided_pack_omlet_path: Option<PathBuf> = None;
    while let Some(arg) = remaining.next() {
        if arg == flags::PACK_OMLET {
            if let Some(path) = remaining.next() {
                provided_pack_omlet_path = Some(path.clone().into());
            } else {
                return Err(InvalidCliArgsError::ExpectedPathAfterFlag(
                    flags::PACK_OMLET.to_string(),
                ));
            }
        } else {
            return Err(InvalidCliArgsError::UnrecognizedArg(arg.clone()));
        }
    }
    let pack_omlet_path = if let Some(p) = provided_pack_omlet_path {
        if p.is_file() {
            p
        } else {
            return Err(InvalidCliArgsError::InvalidPackOmletPath(p));
        }
    } else {
        let cwd = match std::env::current_dir() {
            Ok(cwd) => cwd,
            Err(_) => {
                return Err(InvalidCliArgsError::NoExplicitPackOmletPathProvidedAndCwdCannotBeRead)
            }
        };
        if let Some(p) = get_default_pack_omlet_path(cwd) {
            p
        } else {
            return Err(InvalidCliArgsError::CannotFindImplicitPackOmletPath);
        }
    };
    Ok(CliOptions {
        unvalidated_pack_omlet_path: pack_omlet_path,
    })
}

fn get_default_pack_omlet_path(cwd: PathBuf) -> Option<PathBuf> {
    unimplemented!()
}
