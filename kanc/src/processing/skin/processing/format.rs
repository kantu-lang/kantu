use crate::{
    data::{
        bind_error::BindError, fun_recursion_validation_result::IllegalFunRecursionError,
        node_registry::NodeRegistry, type_positivity_validation_result::TypePositivityError,
        variant_return_type_validation_result::IllegalVariantReturnTypeError,
    },
    processing::{
        generate_code::targets::javascript::CompileToJavaScriptError,
        simplify_ast::SimplifyAstError,
        type_check::{TypeCheckError, TypeCheckWarning},
    },
};

use super::super::data::error::{
    InvalidCliArgsError, InvalidCompilerOptionsError, ReadKantuFilesError, WriteTargetFilesError,
};

pub trait FormatErrorForCli {
    fn format_for_cli(&self) -> String;
}

pub trait FormatErrorForWithRegistry {
    fn format_for_cli_with_registry(&self, registry: &NodeRegistry) -> String;
}

impl FormatErrorForCli for InvalidCliArgsError {
    fn format_for_cli(&self) -> String {
        match self {
            InvalidCliArgsError::UnrecognizedArg(arg) => {
                format!("Unrecognized CLI argument: {}", arg)
            }
            InvalidCliArgsError::ExpectedPathAfterFlag(flag) => {
                format!("Expected path after flag: {}", flag)
            }
            InvalidCliArgsError::CannotFindImplicitPackYsclPath => {
                "Cannot find pack.yscl in current working directory or any of its ancestors."
                    .to_string()
            }
            InvalidCliArgsError::CannotReadCwd(err) => {
                format!("Cannot read current working directory: {:?}", err)
            }
            InvalidCliArgsError::CwdIsNotAbsolute(path) => {
                format!("Current working directory is not absolute: {}. There probably isn't anything you can do about this error except open an issue at https://github.com/kantu-lang/kantu/issues/new.", path.display())
            }
        }
    }
}

impl FormatErrorForCli for InvalidCompilerOptionsError {
    fn format_for_cli(&self) -> String {
        match self {
            InvalidCompilerOptionsError::CannotReadPackYscl(path, err) => {
                format!(
                    "Cannot read pack.yscl at {}. Error: {:?}",
                    path.display(),
                    err
                )
            }
            InvalidCompilerOptionsError::CannotParsePackYscl(src, err) => match err {
                yscl::prelude::ParseError::UnexpectedEoi => {
                    "Could not parse pack.yscl: Unexpected end of input".to_string()
                }
                yscl::prelude::ParseError::UnexpectedChar(unexpected_ch, byte_index) => {
                    let (line, col) = get_line_and_col(src, *byte_index);
                    format!(
                        "Unexpected {} on line {} col {} of pack.yscl.",
                        unexpected_ch, line, col
                    )
                }
                yscl::prelude::ParseError::DuplicateKey(duplicate_key, byte_index) => {
                    let (line, col) = get_line_and_col(src, *byte_index);
                    format!(
                        "Duplicate key {:?} on line {} col {} of pack.yscl.",
                        duplicate_key, line, col
                    )
                }
            },
            InvalidCompilerOptionsError::MissingEntry(entry) => {
                format!("Missing entry {:?} in pack.yscl.", entry)
            }
            InvalidCompilerOptionsError::ExpectedAtomButGotCollection(entry) => {
                format!(
                    "Expected atom but got collection for entry {:?} in pack.yscl.",
                    entry
                )
            }
            InvalidCompilerOptionsError::IllegalKantuVersion(version) => {
                format!("This compiler does not support Kantu version {:?}", version)
            }
        }
    }
}

fn get_line_and_col(src: &str, byte_index: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 0;
    for (i, c) in src.char_indices() {
        if i == byte_index {
            break;
        }
        if c == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    (line, col)
}
impl FormatErrorForCli for ReadKantuFilesError {
    fn format_for_cli(&self) -> String {
        unimplemented!()
    }
}

impl FormatErrorForCli for SimplifyAstError {
    fn format_for_cli(&self) -> String {
        unimplemented!()
    }
}

impl FormatErrorForCli for BindError {
    fn format_for_cli(&self) -> String {
        unimplemented!()
    }
}

impl FormatErrorForWithRegistry for IllegalVariantReturnTypeError {
    fn format_for_cli_with_registry(&self, _registry: &NodeRegistry) -> String {
        unimplemented!()
    }
}

impl FormatErrorForWithRegistry for IllegalFunRecursionError {
    fn format_for_cli_with_registry(&self, _registry: &NodeRegistry) -> String {
        unimplemented!()
    }
}

impl FormatErrorForWithRegistry for TypePositivityError {
    fn format_for_cli_with_registry(&self, _registry: &NodeRegistry) -> String {
        unimplemented!()
    }
}

impl FormatErrorForWithRegistry for TypeCheckError {
    fn format_for_cli_with_registry(&self, _registry: &NodeRegistry) -> String {
        unimplemented!()
    }
}

impl FormatErrorForWithRegistry for CompileToJavaScriptError {
    fn format_for_cli_with_registry(&self, _registry: &NodeRegistry) -> String {
        match *self {}
    }
}

impl FormatErrorForWithRegistry for TypeCheckWarning {
    fn format_for_cli_with_registry(&self, _registry: &NodeRegistry) -> String {
        unimplemented!()
    }
}

impl FormatErrorForWithRegistry for WriteTargetFilesError {
    fn format_for_cli_with_registry(&self, _registry: &NodeRegistry) -> String {
        unimplemented!()
    }
}
