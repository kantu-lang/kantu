use crate::{
    data::{unsimplified_ast as unsimplified, unsimplified_ast::IdentifierName},
    processing::{lex::LexError, parse::ParseError},
};

use std::path::PathBuf;

#[derive(Debug)]
pub enum InvalidCliArgsError {
    UnrecognizedArg(String),
    ExpectedPathAfterFlag(String),
    InvalidPackOmletPath(PathBuf),
    CannotFindImplicitPackOmletPath,
    CannotReadCwd(std::io::Error),
    CwdIsNotAbsolute(PathBuf),
}

#[derive(Debug)]
pub enum InvalidCompilerOptionsError {
    CannotReadPackOmlet(PathBuf, std::io::Error),
    CannotParsePackOmlet(yscl::prelude::ParseError),
}

#[derive(Debug)]
pub enum ReadKantuFilesError {
    CannotGetPackOmletDirectory,
    CannotReadFile(PathBuf, std::io::Error),
    ModHasMultipleFiles(PathBuf, PathBuf),
    NonModDotKHasSubmodules(PathBuf, unsimplified::ModStatement),
    MultipleModsWithSameName(PathBuf, IdentifierName),
    LexError(PathBuf, LexError),
    ParseError(PathBuf, ParseError),
}

#[derive(Clone, Debug)]
pub enum WriteTargetFilesError {}
