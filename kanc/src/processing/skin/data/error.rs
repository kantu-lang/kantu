use crate::{
    data::{unsimplified_ast as unsimplified, unsimplified_ast::IdentifierName},
    processing::{lex::LexError, parse::ParseError},
};

use std::path::PathBuf;

#[derive(Debug)]
pub enum InvalidCliArgsError {
    UnrecognizedFlag(String),
    MissingFlagValue(String),
    CannotFindImplicitPackYsclPath,
    CannotReadCwd(std::io::Error),
    CwdIsNotAbsolute(PathBuf),
}

#[derive(Debug)]
pub enum InvalidCompilerOptionsError {
    CannotReadPackYscl(PathBuf, std::io::Error),
    CannotParsePackYscl {
        src: String,
        err: yscl::prelude::ParseError,
    },
    MissingEntry {
        key: String,
    },
    ExpectedAtomButGotCollection {
        key: String,
        collection: yscl::prelude::Node,
    },
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
