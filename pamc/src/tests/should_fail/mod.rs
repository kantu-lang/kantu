mod ast_simplification;
mod empty_params;
mod fun_recursion;
mod scope;
mod type_check;
mod variant_return_type;

use crate::{
    data::{
        light_ast::*,
        node_registry::{ExpressionRef, NodeId, NodeRegistry},
        token::TokenKind,
        FileId,
    },
    processing::{
        bind_type_independent::{bind_files, BindError, OwnedSymbolSource},
        check_variant_return_types::check_variant_return_types_for_file,
        lex::lex,
        lighten_ast::lighten_file,
        parse::{parse_file, ParseError},
        simplify_ast::{simplify_file, SimplifyAstError},
        test_utils::{
            expand_lightened::{expand_expression, expand_match_case},
            format::{format_expression, format_match_case, FormatOptions},
        },
        type_check::{type_check_files, TypeCheckError},
        validate_fun_recursion::{validate_fun_recursion_in_file, IllegalFunRecursionError},
    },
};

fn standard_ident_name(name: &str) -> IdentifierName {
    IdentifierName::Standard(name.into())
}

fn component_identifier_names(
    registry: &NodeRegistry,
    name_id: NodeId<NameExpression>,
) -> Vec<IdentifierName> {
    let name = registry.name_expression(name_id);
    registry
        .identifier_list(name.component_list_id)
        .iter()
        .map(|component_id| registry.identifier(*component_id).name.clone())
        .collect()
}

fn assert_eq_up_to_white_space(left: &str, right: &str) {
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

// TODO: Add type checker tests
