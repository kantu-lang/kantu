use crate::{
    data::{
        bind_error::BindError, fun_recursion_validation_result::IllegalFunRecursionError,
        type_positivity_validation_result::TypePositivityError,
        variant_return_type_validation_result::IllegalVariantReturnTypeError,
    },
    processing::{
        generate_code::targets::javascript::CompileToJavaScriptError,
        simplify_ast::SimplifyAstError, type_check::TypeCheckError,
    },
};

use super::super::data::error::{
    InvalidCliArgsError, InvalidCompilerOptionsError, ReadKantuFilesError,
};

pub trait FormatErrorForCli {
    fn format_for_cli(&self) -> String;
}

impl FormatErrorForCli for InvalidCliArgsError {
    fn format_for_cli(&self) -> String {
        unimplemented!()
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

impl FormatErrorForCli for IllegalVariantReturnTypeError {
    fn format_for_cli(&self) -> String {
        unimplemented!()
    }
}

impl FormatErrorForCli for IllegalFunRecursionError {
    fn format_for_cli(&self) -> String {
        unimplemented!()
    }
}

impl FormatErrorForCli for TypePositivityError {
    fn format_for_cli(&self) -> String {
        unimplemented!()
    }
}

impl FormatErrorForCli for TypeCheckError {
    fn format_for_cli(&self) -> String {
        unimplemented!()
    }
}

impl FormatErrorForCli for CompileToJavaScriptError {
    fn format_for_cli(&self) -> String {
        match *self {}
    }
}
