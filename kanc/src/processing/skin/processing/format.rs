use crate::{
    data::{
        bind_error::BindError, file_id::*,
        fun_recursion_validation_result::IllegalFunRecursionError, node_registry::NodeRegistry,
        text_span::*, type_positivity_validation_result::TypePositivityError,
        variant_return_type_validation_result::IllegalVariantReturnTypeError,
    },
    processing::{
        format_unsimplified,
        generate_code::targets::javascript::CompileToJavaScriptError,
        lex::LexError,
        parse::ParseError,
        simplify_ast::SimplifyAstError,
        type_check::{TypeCheckError, TypeCheckWarning},
    },
};

use super::super::data::prelude::*;

use std::{
    fs,
    path::{Path, PathBuf},
};

use rustc_hash::FxHashMap;

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
                        "[E0201] Could not parse pack.yscl: Unexpected {unexpected_ch} on pack.yscl:{line}:{col}."
                    )
                }
                yscl::prelude::ParseError::DuplicateKey(duplicate_key, byte_index) => {
                    let byte_index = ByteIndex(*byte_index);
                    let TextCoord { line, col } =
                        TextCoord::new(src, byte_index).expect("Byte index should be valid.");
                    format!(
                        "[E0201] Could not parse pack.yscl: Duplicate key {duplicate_key:?} on pack.yscl:{line}:{col}.",
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

            ReadKantuFilesError::NonModDotKHasSubmodules {
                non_mod_dot_k_path,
                mod_statement: _,
                mod_statement_bispan,
            } => {
                let non_leaf_path = non_mod_dot_k_path.with_extension("").join("mod.k");
                let non_leaf_path = non_leaf_path.display();
                format!(
                    "[E0302] {} is a leaf module, but it declared a submodule at {}. Leaf modules cannot have submodules. To fix this, either delete the submodule declaration or rename {} to {non_leaf_path}",
                    non_mod_dot_k_path.display(),
                    flc_display(non_mod_dot_k_path, mod_statement_bispan.start),
                    non_mod_dot_k_path.display(),
                )
            }

            ReadKantuFilesError::MultipleModsWithSameName {
                parent_mod_path,
                mod_name,
                first_bispan,
                second_bispan,
            } => {
                format!(
                    "[E0303] Multiple definitions of mod {} in {}. First definition: {}. Second definition: {}.",
                    mod_name.src_str(),
                    parent_mod_path.display(),
                    flc_display(parent_mod_path, first_bispan.start),
                    flc_display(parent_mod_path, second_bispan.start),
                )
            }

            ReadKantuFilesError::LexError { path, src, err } => match err {
                LexError::UnexpectedEoi => {
                    "[E0304] Could not lex file: Unexpected end of input".to_string()
                }
                LexError::UnexpectedCharacter(unexpected_ch, byte_index) => {
                    let coord =
                        TextCoord::new(src, *byte_index).expect("Byte index should be valid.");
                    format!(
                        "[E0304] Could not lex file: Unexpected {unexpected_ch} on {}.",
                        flc_display(path, coord),
                    )
                }
            },

            ReadKantuFilesError::ParseError { path, src, err } => match err {
                ParseError::UnexpectedEoi => {
                    "[E0305] Could not parse file: Unexpected end of input".to_string()
                }
                ParseError::UnexpectedNonEoiToken(token) => {
                    let coord = TextCoord::new(src, token.start_index)
                        .expect("Byte index should be valid.");
                    format!(
                        "[E0305] Could not parse file: Unexpected token `{}` on {}.",
                        token.content,
                        flc_display(path, coord),
                    )
                }
            },
        }
    }
}

impl<'a> FormatErrorForCli<&'a FxHashMap<FileId, PathBuf>> for SimplifyAstError {
    fn format_for_cli(&self, file_path_map: &FxHashMap<FileId, PathBuf>) -> String {
        match self {
            SimplifyAstError::IllegalDotLhs(expr) => {
                let loc = format_span_start(expr.span(), file_path_map);
                let formatted_lhs =
                    format_unsimplified::format_expression_with_default_options(expr);
                format!("[E0400] Illegal LHS for dot expression. Currently, dot LHSs can only be identifiers or other dot expressions. At {loc} the following LHS has been found:\n{formatted_lhs}")
            }

            SimplifyAstError::HeterogeneousParams(params) => {
                let is_first_labeled = params[0].label.is_some();
                let (first_display_with_capitalized_article, second_display_with_lowercase_article) =
                    if is_first_labeled {
                        ("A labeled parameter", "an unlabeled parameter")
                    } else {
                        ("An unlabeled parameter", "a labeled parameter")
                    };
                let first_loc = format_span_start(params[0].span, file_path_map);
                let second_param = params
                    .iter()
                    .find(|param| param.label.is_some() != is_first_labeled)
                    .expect("There should be at least one labeled and one unlabeled parameter.");
                let second_loc = format_span_start(second_param.span, file_path_map);
                format!("[E0401] A parameter list must be either all unlabeled or all labeled. {first_display_with_capitalized_article} is declared at {first_loc} but a {second_display_with_lowercase_article} is declarated at {second_loc}.")
            }
            SimplifyAstError::UnderscoreParamLabel(param) => {
                let loc = format_span_start(param.span, file_path_map);
                format!("[E0402] A parameter label cannot be `_`.  There is a parameter labeled `_` at {loc}.")
            }
            SimplifyAstError::DuplicateParamLabel(param1, param2) => {
                let name = param1
                    .label_name()
                    .expect("Param 1 should have a label.")
                    .src_str();
                let loc1 = format_span_start(param1.span, file_path_map);
                let loc2 = format_span_start(param2.span, file_path_map);
                format!("[E0403] Multiple parameters have the label {name}. The first is at {loc1}. The second is at {loc2}.")
            }

            SimplifyAstError::HeterogeneousCallArgs(args) => {
                let is_first_labeled = args[0].label.is_some();
                let (first_display_with_capitalized_article, second_display_with_lowercase_article) =
                    if is_first_labeled {
                        ("A labeled argument", "an unlabeled argument")
                    } else {
                        ("An unlabeled argument", "a labeled argument")
                    };
                let first_loc = format_span_start(args[0].span, file_path_map);
                let second_arg = args
                    .iter()
                    .find(|arg| arg.label.is_some() != is_first_labeled)
                    .expect("There should be at least one labeled and one unlabeled argument.");
                let second_loc = format_span_start(second_arg.span, file_path_map);
                format!("[E0404] A call argument list must be either all unlabeled or all labeled. {first_display_with_capitalized_article} is declared at {first_loc} but {second_display_with_lowercase_article} is declarated at {second_loc}.")
            }
            SimplifyAstError::UnderscoreCallArgLabel(arg) => {
                let loc = format_span_start(arg.span, file_path_map);
                format!("[E0405] An argument label cannot be `_`.  There is an argument labeled `_` at {loc}.")
            }
            SimplifyAstError::DuplicateCallArgLabel(arg1, arg2) => {
                let name = arg1
                    .label_name()
                    .expect("Arg 1 should have a label.")
                    .src_str();
                let loc1 = format_span_start(arg1.span, file_path_map);
                let loc2 = format_span_start(arg2.span, file_path_map);
                format!("[E0406] Multiple arguments have the label {name}. The first is at {loc1}. The second is at {loc2}.")
            }

            SimplifyAstError::HeterogeneousMatchCaseParams(params) => {
                let is_first_labeled = params[0].label.is_some();
                let (first_display_with_capitalized_article, second_display_with_lowercase_article) =
                    if is_first_labeled {
                        ("A labeled parameter", "an unlabeled parameter")
                    } else {
                        ("An unlabeled parameter", "a labeled parameter")
                    };
                let first_loc = format_span_start(params[0].span, file_path_map);
                let second_param = params
                    .iter()
                    .find(|param| param.label.is_some() != is_first_labeled)
                    .expect("There should be at least one labeled and one unlabeled parameter.");
                let second_loc = format_span_start(second_param.span, file_path_map);
                format!("[E0407] A match case parameter list must be either all unlabeled or all labeled. {first_display_with_capitalized_article} is declared at {first_loc} but a {second_display_with_lowercase_article} is declarated at {second_loc}.")
            }
            SimplifyAstError::UnderscoreMatchCaseParamLabel(param) => {
                let loc = format_span_start(param.span, file_path_map);
                format!("[E0408] A match case parameter label cannot be `_`.  There is a match case parameter labeled `_` at {loc}.")
            }
            SimplifyAstError::DuplicateMatchCaseParamLabel(param1, param2) => {
                let name = param1
                    .label_name()
                    .expect("Param 1 should have a label.")
                    .src_str();
                let loc1 = format_span_start(param1.span, file_path_map);
                let loc2 = format_span_start(param2.span, file_path_map);
                format!("[E0409] Multiple match case parameters have the label {name}. The first is at {loc1}. The second is at {loc2}.")
            }
        }
    }
}

fn format_span_start(
    span: TextSpan,
    file_path_map: &FxHashMap<FileId, PathBuf>,
) -> impl std::fmt::Display {
    let path = file_path_map
        .get(&span.file_id)
        .expect("File ID should be valid.");
    let src =
        fs::read_to_string(path).expect("[E9000] File path held in file path map should be valid.");
    let start = TextCoord::new(&src, span.start).expect("Byte index should be valid.");
    flc_display(path, start)
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
