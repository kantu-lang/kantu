use crate::{
    data::{file_graph::FileGraph, light_ast::*, node_registry::NodeRegistry, FileId},
    processing::{
        bind_type_independent::bind_files,
        generate_code::{targets::javascript::JavaScript, CompileTarget},
        lex::lex,
        lighten_ast::register_file_items,
        parse::parse_file,
        simplify_ast::simplify_file,
        type_check::{
            type_check_file_items, NormalFormAssertionWarning, TypeAssertionWarning,
            TypeCheckFailureReason, TypeCheckWarning,
        },
        validate_fun_recursion::validate_fun_recursion_in_file_items,
        validate_type_positivity::validate_type_positivity_in_file_items,
        validate_variant_return_types::validate_variant_return_types_in_file_items,
    },
};

mod no_warnings;

mod check_warnings;
mod todo_warnings;

use warning_comparison::*;
mod warning_comparison;
