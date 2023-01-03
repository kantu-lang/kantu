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
        return Err(InvalidCliArgsError::CannotFindImplicitPackOmletPath);
    };
    Ok(CliOptions { pack_omlet_path })
}
