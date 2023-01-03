use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum InvalidCliArgsError {
    UnrecognizedArg(String),
    ExpectedPathAfterFlag(String),
    InvalidPackOmletPath(PathBuf),
    CannotFindImplicitPackOmletPath,
}

#[derive(Clone, Debug)]
pub enum InvalidCompilerOptionsError {}

#[derive(Clone, Debug)]
pub enum ReadKantuFilesError {}

#[derive(Clone, Debug)]
pub enum WriteTargetFilesError {}
