use crate::{
    data::{
        bind_error::BindError, bound_ast::ModScope, file_id::*, file_tree::*,
        fun_recursion_validation_result::IllegalFunRecursionError, node_registry::NodeRegistry,
        text_span::*, type_positivity_validation_result::TypePositivityError,
        unsimplified_ast as unsimplified,
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

type FilePathMap = FxHashMap<FileId, PathBuf>;

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
                format!("[E9901] Current working directory is not absolute: {}. There probably isn't anything you can do about this error except open an issue at https://github.com/kantu-lang/kantu/issues/new.", path.display())
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

impl<'a> FormatErrorForCli<&'a FilePathMap> for SimplifyAstError {
    fn format_for_cli(&self, file_path_map: &FilePathMap) -> String {
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

fn format_span_start(span: TextSpan, file_path_map: &FilePathMap) -> impl std::fmt::Display {
    let path = file_path_map
        .get(&span.file_id)
        .expect("File ID should be valid.");
    let src =
        fs::read_to_string(path).expect("[E9900] File path held in file path map should be valid.");
    let start = TextCoord::new(&src, span.start).expect("Byte index should be valid.");
    flc_display(path, start)
}

impl<'a> FormatErrorForCli<(&'a FilePathMap, &'a FileTree)> for BindError {
    fn format_for_cli(&self, (file_path_map, file_tree): (&FilePathMap, &FileTree)) -> String {
        use crate::data::bind_error::*;

        match self {
            BindError::NameNotFound(NameNotFoundError { name_components }) => {
                let name_display = name_components_display(name_components);
                let loc = format_span_start(name_components[0].span, file_path_map);
                format!(r#"[E0500] Could not find name `{name_display}` at {loc}."#)
            }

            BindError::NameIsPrivate(NameIsPrivateError {
                name_component,
                required_visibility,
                actual_visibility,
            }) => {
                let name_display = name_component.name.src_str();
                let loc = format_span_start(name_component.span, file_path_map);
                let required_vis_display = mod_scope_display(required_visibility.0, file_tree);
                let actual_vis_display = mod_scope_display(actual_visibility.0, file_tree);
                format!(
                    r#"[E0501] Could not access name `{name_display}` at {loc}. The required visibility is `{required_vis_display}`, but the actual visibility is `{actual_vis_display}`."#
                )
            }

            BindError::CannotLeakPrivateName(CannotLeakPrivateNameError {
                name_component,
                required_visibility,
                actual_visibility,
            }) => {
                let name_display = name_component.name.src_str();
                let loc = format_span_start(name_component.span, file_path_map);
                let required_vis_display = mod_scope_display(required_visibility.0, file_tree);
                let actual_vis_display = mod_scope_display(actual_visibility.0, file_tree);
                format!(
                    r#"[E0502] Could not leak name `{name_display}` at {loc}. The required visibility is `{required_vis_display}`, but the actual visibility is `{actual_vis_display}`."#
                )
            }

            BindError::NameClash(NameClashError { name, old, new }) => {
                fn symbol_source_display(
                    source: &OwnedSymbolSource,
                    file_path_map: &FilePathMap,
                ) -> impl std::fmt::Display {
                    match source {
                        OwnedSymbolSource::Builtin | OwnedSymbolSource::Mod(_) => {
                            "defined as a builtin".to_string()
                        }
                        OwnedSymbolSource::Identifier(ident) => {
                            let loc = format_span_start(ident.span, file_path_map);
                            format!("defined at {}", loc)
                        }
                        OwnedSymbolSource::WildcardImport(use_statement) => {
                            let loc = format_span_start(use_statement.span, file_path_map);
                            format!("defined by a wildcard `use` statement at {}", loc)
                        }
                    }
                }
                let name_display = name.src_str();
                let first_display = symbol_source_display(old, file_path_map);
                let second_display = symbol_source_display(new, file_path_map);
                format!(
                    r#"[E0503] There are conflicting definitions for `{name_display}`. The first is {first_display}. The second is {second_display}."#
                )
            }

            BindError::ExpectedTermButNameRefersToMod(ExpectedTermButNameRefersToModError {
                name_components,
            }) => {
                let name_display = name_components_display(name_components);
                let loc = format_span_start(name_components[0].span, file_path_map);
                format!(
                    r#"[E0504] Expected a term, but the name "{name_display}" at {loc} refers to a module."#
                )
            }

            BindError::ExpectedModButNameRefersToTerm(ExpectedModButNameRefersToTermError {
                name_components,
            }) => {
                let name_display = name_components_display(name_components);
                let loc = format_span_start(name_components[0].span, file_path_map);
                format!(
                    r#"[E0505] Expected a module, but the name "{name_display}" at {loc} refers to a term."#
                )
            }

            BindError::CannotUselesslyImportItemAsSelf(CannotUselesslyImportItemAsSelfError {
                use_statement,
            }) => {
                let loc = format_span_start(use_statement.span, file_path_map);
                format!(
                    r#"[E0506] A non-renamed non-wildcard `use` statement cannot import a "singleton dot chain". For example, `use foo;` is illegal, but `use foo.bar;` is legal. This is because `foo` has only 1 component--`foo`, while `foo.bar` has 2 components--`foo` and `bar`. A non-renamed non-wildcard `use` statement was found at {loc}."#
                )
            }

            BindError::ModFileNotFound(ModFileNotFoundError { mod_name }) => {
                let mod_name_display = mod_name.name.src_str();
                let loc = format_span_start(mod_name.span, file_path_map);
                format!(
                    r#"[E0507] Could not find a file for the module `{mod_name_display}` defined at {loc}."#
                )
            }

            BindError::VisibilityWasNotAtLeastAsPermissiveAsCurrentMod(
                VisibilityWasNotAtLeastAsPermissiveAsCurrentModError {
                    visibility_modifier,
                    actual_visibility,
                    defining_mod_id,
                },
            ) => {
                let loc = format_span_start(visibility_modifier.span, file_path_map);
                let actual_vis_display = mod_scope_display(actual_visibility.0, file_tree);
                let mod_vis_display = mod_scope_display(ModScope::Mod(*defining_mod_id), file_tree);
                format!(
                    r#"[E0508] The visibility modifier at {loc} has a visibility of `{actual_vis_display}`, which is insufficient for the defining module. The minimum visibility is `{mod_vis_display}`."#
                )
            }

            BindError::TransparencyWasNotAtLeastAsPermissiveAsCurrentMod(
                TransparencyWasNotAtLeastAsPermissiveAsCurrentModError {
                    transparency_modifier,
                    actual_transparency,
                    defining_mod_id,
                },
            ) => {
                let loc = format_span_start(transparency_modifier.span, file_path_map);
                let actual_transp_display = mod_scope_display(actual_transparency.0, file_tree);
                let mod_transp_display =
                    mod_scope_display(ModScope::Mod(*defining_mod_id), file_tree);
                format!(
                    r#"[E0509] The transparency modifier at {loc} has a transparency of `{actual_transp_display}`, which is insufficient for the defining module. The minimum transparency is `{mod_transp_display}`."#
                )
            }

            BindError::TransparencyWasNotAtLeastAsRestrictiveAsVisibility(
                TransparencyWasNotAtLeastAsRestrictiveAsVisibilityError {
                    transparency_modifier,
                    transparency,
                    visibility,
                },
            ) => {
                let loc = format_span_start(transparency_modifier.span, file_path_map);
                let transp_display = mod_scope_display(transparency.0, file_tree);
                let vis_display = mod_scope_display(visibility.0, file_tree);
                format!(
                    r#"[E0510] The transparency modifier at `{loc}` has a transparency of "{transp_display}", which is not a subset of the associated visibility (`{vis_display}`). An item's transparency must be a subset of its visibility."#
                )
            }
        }
    }
}

impl<'a> FormatErrorForCli<&'a NodeRegistry> for IllegalVariantReturnTypeError {
    fn format_for_cli(&self, _registry: &NodeRegistry) -> String {
        // TODO: Improve error message formatting.
        format!("[E06??] {:#?}", self)
    }
}

impl<'a> FormatErrorForCli<&'a NodeRegistry> for IllegalFunRecursionError {
    fn format_for_cli(&self, _registry: &NodeRegistry) -> String {
        // TODO: Improve error message formatting.
        format!("[E07??] {:#?}", self)
    }
}

impl<'a> FormatErrorForCli<&'a NodeRegistry> for TypePositivityError {
    fn format_for_cli(&self, _registry: &NodeRegistry) -> String {
        // TODO: Improve error message formatting.
        format!("[E08??] {:#?}", self)
    }
}

impl<'a> FormatErrorForCli<&'a NodeRegistry> for TypeCheckError {
    fn format_for_cli(&self, _registry: &NodeRegistry) -> String {
        // TODO: Improve error message formatting.
        format!("[E20??] {:#?}", self)
    }
}

impl<'a> FormatErrorForCli<&'a NodeRegistry> for CompileToJavaScriptError {
    fn format_for_cli(&self, _registry: &NodeRegistry) -> String {
        match *self {}
    }
}

impl<'a> FormatErrorForCli<&'a NodeRegistry> for TypeCheckWarning {
    fn format_for_cli(&self, _registry: &NodeRegistry) -> String {
        // TODO: Improve error message formatting.
        format!("[W20??] {:#?}", self)
    }
}

impl<'a> FormatErrorForCli<&'a NodeRegistry> for WriteTargetFilesError {
    fn format_for_cli(&self, _registry: &NodeRegistry) -> String {
        // TODO: Improve error message formatting.
        format!("[E80??] {:#?}", self)
    }
}

fn flc_display(path: &Path, coord: TextCoord) -> impl std::fmt::Display {
    format!("{}:{}:{}", path.display(), coord.line, coord.col)
}

fn name_components_display(name_components: &[unsimplified::Identifier]) -> impl std::fmt::Display {
    name_components
        .iter()
        .map(|component| component.name.src_str())
        .collect::<Vec<_>>()
        .join(".")
}

fn mod_scope_display(scope: ModScope, file_tree: &FileTree) -> impl std::fmt::Display {
    let vis_file_id = match scope {
        ModScope::Mod(id) => id,
        ModScope::Global => return "*".to_string(),
    };
    let edge_labels_descending = {
        let mut current = vis_file_id;
        let mut labels = vec![];
        while let Some((parent, label)) = file_tree.parent_and_label(current) {
            labels.push(label);
            current = parent;
        }
        labels.reverse();
        labels
    };

    let mut out = "pack".to_string();
    for label in edge_labels_descending {
        out.push('.');
        out.push_str(label.src_str());
    }
    out
}
