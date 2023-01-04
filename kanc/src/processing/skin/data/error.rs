use crate::{
    data::{unsimplified_ast as unsimplified, unsimplified_ast::IdentifierName},
    processing::{lex::LexError, parse::ParseError},
};

use std::path::PathBuf;

#[derive(Debug)]
pub enum InvalidCliArgsError {
    UnrecognizedArg(String),
    ExpectedPathAfterFlag(String),
    CannotFindImplicitPackYsclPath,
    CannotReadCwd(std::io::Error),
    CwdIsNotAbsolute(PathBuf),
}

#[derive(Debug)]
pub enum InvalidCompilerOptionsError {
    CannotReadPackYscl(PathBuf, std::io::Error),
    CannotParsePackYscl(yscl::prelude::ParseError),
    MissingEntry(String),
    ExpectedAtomButGotCollection(String),
    IllegalKantuVersion(String),
}

#[derive(Debug)]
pub enum ReadKantuFilesError {
    CannotGetPackYsclDirectory,
    CannotReadFile(PathBuf, std::io::Error),
    ModHasMultipleFiles(PathBuf, PathBuf),
    NonModDotKHasSubmodules(PathBuf, unsimplified::ModStatement),
    MultipleModsWithSameName(PathBuf, IdentifierName),
    LexError(PathBuf, LexError),
    ParseError(PathBuf, ParseError),
}

#[derive(Debug)]
pub enum WriteTargetFilesError {
    CannotRemoveTargetDir(PathBuf, std::io::Error),
    TargetDirExistsButIsNotDir(PathBuf),
    CannotCreateTargetDir(PathBuf, std::io::Error),
    CannotWriteFile(PathBuf, std::io::Error),
}
