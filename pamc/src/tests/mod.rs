use crate::{
    data::{node_registry::NodeRegistry, FileId},
    processing::{
        bind_type_independent::bind_symbols_to_identifiers, lex::lex, parse::parse_file,
        register::register_file, validate_fun_recursion::validate_fun_recursion_in_file,
    },
};

mod should_succeed {
    use super::*;

    #[test]
    fn hello_world() {
        let src = include_str!("sample_code/hello_world.ph");
        expect_success(src);
    }

    fn expect_success(src: &str) {
        let file_id = FileId(0);
        let tokens = lex(src).expect("Lexing failed");
        let file = parse_file(tokens, file_id).expect("Parsing failed");
        let mut registry = NodeRegistry::empty();
        let file_id = register_file(&mut registry, file);
        let file = registry.file(file_id);
        let symbol_db =
            bind_symbols_to_identifiers(&registry, vec![file_id]).expect("Binding failed");
        validate_fun_recursion_in_file(&symbol_db, file).expect("Fun recursion validation failed");
    }
}

mod should_fail {
    use super::*;
    use crate::{
        data::registered_ast::*, processing::validate_fun_recursion::IllegalFunRecursionError,
    };

    #[test]
    fn rec_fun_same_param() {
        let src = include_str!("sample_code/rec_fun_same_param.ph");
        let file_id = FileId(0);
        let tokens = lex(src).expect("Lexing failed");
        let file = parse_file(tokens, file_id).expect("Parsing failed");
        let mut registry = NodeRegistry::empty();
        let file_id = register_file(&mut registry, file);
        let file = registry.file(file_id);
        let symbol_db =
            bind_symbols_to_identifiers(&registry, vec![file_id]).expect("Binding failed");
        let err = validate_fun_recursion_in_file(&symbol_db, file)
            .expect_err("Fun recursion validation unexpectedly succeeded");
        match err {
            IllegalFunRecursionError::NonSubstructPassedToDecreasingParam {
                callee: callee_id,
                arg: arg_id,
            } => {
                let callee = registry.identifier(callee_id);
                let arg = &registry.wrapped_expression(arg_id).expression;
                assert_eq!(
                    callee.name,
                    standard_ident_name("x_"),
                    "Unexpected param name"
                );
                assert!(
                    matches!(arg, Expression::Identifier(identifier) if identifier.name == standard_ident_name("a")),
                    "Unexpected arg: {:#?}",
                    arg
                );
            }
            _ => panic!("Unexpected error: {:#?}", err),
        }
    }

    fn standard_ident_name(name: &str) -> IdentifierName {
        IdentifierName::Standard(name.into())
    }
}
