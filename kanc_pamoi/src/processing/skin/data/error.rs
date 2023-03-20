use crate::{
    data::{
        text_span::TextBispan, unsimplified_ast as unsimplified, unsimplified_ast::IdentifierName,
    },
    processing::{lex::LexError, parse::ParseError},
};

use std::path::PathBuf;

#[derive(Debug)]
pub enum InvalidCliArgsError {
    UnrecognizedFlag(String),
    MissingFlagValue(String),
    MutuallyExclusiveFlagsBothProvided(String, String),
    PackYsclPathDidNotEndWithPackYscl(PathBuf),
    SingleFilePathDidNotHaveKExtension(PathBuf),
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
    ExpectedBoolButGot {
        key: String,
        value: yscl::prelude::Node,
    },
    IllegalKantuVersion(String),
}

#[derive(Debug)]
pub enum ReadKantuFilesError {
    CannotReadFile(PathBuf, std::io::Error),
    ModHasBothLeafAndModKFiles {
        leaf_path: PathBuf,
        mod_k_path: PathBuf,
    },
    NonModDotKHasSubmodules {
        non_mod_dot_k_path: PathBuf,
        mod_statement: unsimplified::ModStatement,
        mod_statement_bispan: TextBispan,
    },
    MultipleModsWithSameName {
        parent_mod_path: PathBuf,
        mod_name: IdentifierName,
        first_bispan: TextBispan,
        second_bispan: TextBispan,
    },
    LexError {
        path: PathBuf,
        src: String,
        err: LexError,
    },
    ParseError {
        path: PathBuf,
        src: String,
        err: ParseError,
    },
}

#[derive(Debug)]
pub enum WriteTargetFilesError {
    CannotRemoveTargetDir(PathBuf, std::io::Error),
    TargetDirExistsButIsNotDir(PathBuf),
    CannotCreateTargetDir(PathBuf, std::io::Error),
    CannotWriteFile(PathBuf, std::io::Error),
}
