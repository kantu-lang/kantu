mod ast_simplification;
mod empty_params;
mod fun_recursion;
mod scope;
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

// TODO: Add type checker tests
