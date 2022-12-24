use crate::{
    data::{
        light_ast::*,
        node_registry::{NodeId, NodeRegistry},
        FileId,
    },
    processing::{
        bind_type_independent::bind_files,
        generate_code::{targets::javascript::JavaScript, CompileTarget},
        lex::lex,
        lighten_ast::lighten_file,
        parse::parse_file,
        simplify_ast::simplify_file,
        type_check::{
            type_check_files, LhsIsGoalKw, NormalFormAssertionWarning, TypeAssertionWarning,
            TypeCheckFailureReason, TypeCheckWarning,
        },
        validate_fun_recursion::validate_fun_recursion_in_file,
        validate_type_positivity::validate_type_positivity_in_file,
        validate_variant_return_types::validate_variant_return_types_in_file,
    },
};

mod check_warnings;
mod no_warnings;

use warning_comparison::*;
mod warning_comparison;
