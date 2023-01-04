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
                format!("Current working directory is not absolute: {:?}. There probably isn't anything you can do about this error except open an issue at https://github.com/kantu-lang/kantu/issues/new.", path)
            }
        }
    }
}

impl FormatErrorForCli for InvalidCompilerOptionsError {
    fn format_for_cli(&self) -> String {
        unimplemented!()
    }
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
