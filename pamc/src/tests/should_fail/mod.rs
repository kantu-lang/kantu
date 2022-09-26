mod empty_params;
mod fun_recursion;
mod scope;
mod variant_return_type;

use crate::{
    data::{node_registry::NodeRegistry, symbol_provider::SymbolProvider, FileId},
    processing::{
        bind_type_independent::{bind_symbols_to_identifiers, BindError},
        extract_variant_type_args::{
            extract_variant_type_args_for_file, IllegalVariantReturnTypeError,
        },
        lex::lex,
        parse::parse_file,
        register::register_file,
        validate_fun_recursion::validate_fun_recursion_in_file,
    },
};

use crate::{
    data::{registered_ast::*, token::TokenKind},
    processing::{parse::ParseError, validate_fun_recursion::IllegalFunRecursionError},
};

fn standard_ident_name(name: &str) -> IdentifierName {
    IdentifierName::Standard(name.into())
}
