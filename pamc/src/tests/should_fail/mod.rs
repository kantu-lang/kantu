mod ast_simplification;
mod empty_params;
mod fun_recursion;
mod scope;
mod variant_return_type;

use crate::{
    data::{
        token::TokenKind,
        x_light_ast::*,
        x_node_registry::{ExpressionRef, NodeId, NodeRegistry},
        FileId,
    },
    processing::{
        check_variant_return_types::check_variant_return_types_for_file,
        lex::lex,
        parse::{parse_file, ParseError},
        simplify_ast::{simplify_file, SimplifyAstError},
        validate_fun_recursion::{validate_fun_recursion_in_file, IllegalFunRecursionError},
        x_bind_type_independent::{bind_files, BindError},
        x_lighten::lighten_file,
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
