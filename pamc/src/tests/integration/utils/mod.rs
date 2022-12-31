pub use crate::data::{
    bind_error::*,
    file_tree::FileTree,
    fun_recursion_validation_result::IllegalFunRecursionError,
    light_ast::*,
    node_registry::{ExpressionRef, NodeId, NodeRegistry},
    simplified_ast,
    token::TokenKind,
    type_positivity_validation_result::TypePositivityError,
    FileId,
};
pub use crate::processing::{
    bind_type_independent::bind_files,
    bind_type_independent::{BindError, OwnedSymbolSource},
    generate_code::{targets::javascript::JavaScript, CompileTarget},
    lex::lex,
    lex::LexError,
    lighten_ast::register_file_items,
    parse::parse_file,
    parse::ParseError,
    simplify_ast::simplify_file,
    simplify_ast::SimplifyAstError,
    test_utils::{
        expand_lightened::{expand_expression, expand_match, expand_match_case},
        format::{format_expression, format_match, format_match_case, FormatOptions},
    },
    type_check::TypeCheckError,
    type_check::{
        type_check_file_items, NormalFormAssertionWarning, TypeAssertionWarning,
        TypeCheckFailureReason, TypeCheckWarning,
    },
    validate_fun_recursion::validate_fun_recursion_in_file_items,
    validate_type_positivity::validate_type_positivity_in_file_items,
    validate_variant_return_types::validate_variant_return_types_in_file_items,
};

#[macro_use]
pub mod macros;

mod multi_file;
pub use multi_file::*;

mod warning_comparison;
pub use warning_comparison::*;

pub fn component_identifier_names(
    registry: &NodeRegistry,
    name_id: NodeId<NameExpression>,
) -> Vec<IdentifierName> {
    let name = registry.get(name_id);
    registry
        .get_list(name.component_list_id)
        .iter()
        .map(|component_id| registry.get(*component_id).name.clone())
        .collect()
}

pub fn assert_eq_up_to_white_space(left: &str, right: &str) {
    let mut left_non_whitespace = left.chars().enumerate().filter(|(_, c)| !c.is_whitespace());
    let left_non_whitespace_len = left_non_whitespace.clone().count();
    let mut right_non_whitespace = right
        .chars()
        .enumerate()
        .filter(|(_, c)| !c.is_whitespace());
    let right_non_whitespace_len = right_non_whitespace.clone().count();

    loop {
        let left_char = left_non_whitespace.next();
        let right_char = right_non_whitespace.next();

        match (left_char, right_char) {
            (Some((left_original_index, left_char)), Some((right_original_index, right_char))) => {
                assert_eq!(
                    left_char, right_char,
                    "Strings differ (after removing whitespace): left_index = {}; right_index = {};\nleft = {:?};\nright = {:?};\nleft_remaining = {:?};\nright_remaining = {:?}",
                    left_original_index, right_original_index, left, right, &left[left_original_index..], &right[right_original_index..]
                );
            }
            (None, None) => {
                break;
            }
            (Some((left_original_index, _)), None) => {
                panic!(
                    "Strings differ in length after removing whitespace: left_len = {}; right_len = {};\nleft = {:?};\nright = {:?};\nleft_remaining = {:?};\nright_remaining = {:?}",
                    left_non_whitespace_len,
                    right_non_whitespace_len,
                    left,
                    right,
                    &left[left_original_index..],
                    "",
                );
            }
            (None, Some((right_original_index, _))) => {
                panic!(
                    "Strings differ in length after removing whitespace: left_len = {}; right_len = {};\nleft = {:?};\nright = {:?};\nleft_remaining = {:?};\nright_remaining = {:?}",
                    left_non_whitespace_len,
                    right_non_whitespace_len,
                    left,
                    right,
                    "",
                    &right[right_original_index..],
                );
            }
        }
    }
}
