use crate::{
    data::{
        bind_error::BindError, fun_recursion_validation_result::IllegalFunRecursionError,
        node_registry::NodeRegistry, text_span::*,
        type_positivity_validation_result::TypePositivityError,
        variant_return_type_validation_result::IllegalVariantReturnTypeError,
    },
    processing::{
        generate_code::targets::javascript::CompileToJavaScriptError,
        simplify_ast::SimplifyAstError,
        type_check::{TypeCheckError, TypeCheckWarning},
    },
};

use super::super::data::prelude::*;

use std::path::Path;

pub trait FormatErrorForCli<T> {
    fn format_for_cli(&self, data: T) -> String;
}

impl FormatErrorForCli<()> for InvalidCliArgsError {
    fn format_for_cli(&self, (): ()) -> String {
        match self {
            InvalidCliArgsError::UnrecognizedFlag(flag) => {
                format!("[E0100] Unrecognized CLI flag: {}", flag)
            }
            InvalidCliArgsError::MissingFlagValue(flag) => {
                format!("[E0101] Expected value after flag: {}", flag)
            }
            InvalidCliArgsError::CannotFindImplicitPackYsclPath => {
                "[E0102] Cannot find pack.yscl in current working directory or any of its ancestors."
                    .to_string()
            }
            InvalidCliArgsError::CannotReadCwd(err) => {
                format!("[E0103] Cannot read current working directory: {:?}", err)
            }
            InvalidCliArgsError::CwdIsNotAbsolute(path) => {
                format!("[E0104] Current working directory is not absolute: {}. There probably isn't anything you can do about this error except open an issue at https://github.com/kantu-lang/kantu/issues/new.", path.display())
            }
        }
    }
}

impl FormatErrorForCli<()> for InvalidCompilerOptionsError {
    fn format_for_cli(&self, (): ()) -> String {
        match self {
            InvalidCompilerOptionsError::CannotReadPackYscl(path, err) => {
                format!(
                    "[E0200] Cannot read pack.yscl at {}. Error: {:?}",
                    path.display(),
                    err
                )
            }
            InvalidCompilerOptionsError::CannotParsePackYscl { src, err } => match err {
                yscl::prelude::ParseError::UnexpectedEoi => {
                    "[E0201] Could not parse pack.yscl: Unexpected end of input".to_string()
                }
                yscl::prelude::ParseError::UnexpectedChar(unexpected_ch, byte_index) => {
                    let byte_index = ByteIndex(*byte_index);
                    let TextCoord { line, col } =
                        TextCoord::new(src, byte_index).expect("Byte index should be valid.");
                    format!(
                        "[E0201] Could not parse pack.yscl: Unexpected {} on line {} col {} of pack.yscl.",
                        unexpected_ch, line, col
                    )
                }
                yscl::prelude::ParseError::DuplicateKey(duplicate_key, byte_index) => {
                    let byte_index = ByteIndex(*byte_index);
                    let TextCoord { line, col } =
                        TextCoord::new(src, byte_index).expect("Byte index should be valid.");
                    format!(
                        "[E0201] Could not parse pack.yscl: Duplicate key {:?} on line {} col {} of pack.yscl.",
                        duplicate_key, line, col
                    )
                }
            },
            InvalidCompilerOptionsError::MissingEntry { key } => {
                format!("[E0202] Missing entry {:?} in pack.yscl.", key)
            }
            InvalidCompilerOptionsError::ExpectedAtomButGotCollection { key, collection } => {
                format!(
                    "[E0203] Illegal type for entry {:?} in pack.yscl. Expected string, got {}.",
                    key,
                    match &collection {
                        yscl::prelude::Node::Atom(_) => unreachable!(),
                        yscl::prelude::Node::Map(_) => "map",
                        yscl::prelude::Node::List(_) => "list",
                    },
                )
            }
            InvalidCompilerOptionsError::IllegalKantuVersion(version) => {
                const SUPPORTED_VERSIONS: [&str; 1] = ["1.0.0"];
                format!(
                    "[E0204] This compiler does not support Kantu version {:?}. Supported versions are: {:?}",
                    version,
                    SUPPORTED_VERSIONS,
                )
            }
        }
    }
}

impl FormatErrorForCli<()> for ReadKantuFilesError {
    fn format_for_cli(&self, (): ()) -> String {
        match self {
            ReadKantuFilesError::CannotReadFile(path, err) => {
                format!(
                    "[E0300] Cannot read file at {}. Error: {:?}",
                    path.display(),
                    err
                )
            }

            ReadKantuFilesError::ModHasBothLeafAndModKFiles {
                leaf_path,
                mod_k_path,
            } => {
                format!(
                    "[E0301] Both {} and {} exist. The compiler doesn't know which file to use. Please delete one.",
                    leaf_path.display(),
                    mod_k_path.display(),
                )
            }

            ReadKantuFilesError::MultipleModsWithSameName {
                parent_mod_path,
                mod_name,
                first_bispan,
                second_bispan,
            } => {
                format!(
                    "[E0302] Multiple definitions of mod {} in {}. First definition: {}. Second definition: {}.",
                    mod_name.src_str(),
                    parent_mod_path.display(),
                    flc_display(parent_mod_path, first_bispan.start),
                    flc_display(parent_mod_path, second_bispan.start),
                )
            }

            ReadKantuFilesError::LexError { path, src, err } => {
                let TextCoord { line, col } =
                    TextCoord::new(src, err).expect("Byte index should be valid.");
                format!(
                    "[E0303] Lex error in {} at line {} col {}: {}",
                    path.display(),
                    line,
                    col,
                    err.message
                )
            }

            _ => unimplemented!(),
        }
    }
}

impl FormatErrorForCli<()> for SimplifyAstError {
    fn format_for_cli(&self, (): ()) -> String {
        unimplemented!()
    }
}

impl FormatErrorForCli<()> for BindError {
    fn format_for_cli(&self, (): ()) -> String {
        unimplemented!()
    }
}

impl<'a> FormatErrorForCli<&'a NodeRegistry> for IllegalVariantReturnTypeError {
    fn format_for_cli(&self, _registry: &NodeRegistry) -> String {
        unimplemented!()
    }
}

impl<'a> FormatErrorForCli<&'a NodeRegistry> for IllegalFunRecursionError {
    fn format_for_cli(&self, _registry: &NodeRegistry) -> String {
        unimplemented!()
    }
}

impl<'a> FormatErrorForCli<&'a NodeRegistry> for TypePositivityError {
    fn format_for_cli(&self, _registry: &NodeRegistry) -> String {
        unimplemented!()
    }
}

impl<'a> FormatErrorForCli<&'a NodeRegistry> for TypeCheckError {
    fn format_for_cli(&self, _registry: &NodeRegistry) -> String {
        unimplemented!()
    }
}

impl<'a> FormatErrorForCli<&'a NodeRegistry> for CompileToJavaScriptError {
    fn format_for_cli(&self, _registry: &NodeRegistry) -> String {
        unimplemented!()
    }
}

impl<'a> FormatErrorForCli<&'a NodeRegistry> for TypeCheckWarning {
    fn format_for_cli(&self, _registry: &NodeRegistry) -> String {
        unimplemented!()
    }
}

impl<'a> FormatErrorForCli<&'a NodeRegistry> for WriteTargetFilesError {
    fn format_for_cli(&self, _registry: &NodeRegistry) -> String {
        unimplemented!()
    }
}

fn flc_display(path: &Path, coord: TextCoord) -> impl std::fmt::Display {
    format!("{}:{}:{}", path.display(), coord.line, coord.col)
}
