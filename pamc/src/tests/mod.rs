use crate::{
    data::{node_registry::NodeRegistry, symbol_provider::SymbolProvider, FileId},
    processing::{
        bind_type_independent::{bind_symbols_to_identifiers, BindError},
        lex::lex,
        parse::parse_file,
        register::register_file,
        validate_fun_recursion::validate_fun_recursion_in_file,
    },
};

mod should_succeed {
    use super::*;

    #[test]
    fn hello_world() {
        let src = include_str!("sample_code/should_succeed/hello_world.ph");
        expect_success(src);
    }

    #[test]
    fn optional_commas() {
        let src = include_str!("sample_code/should_succeed/optional_commas.ph");
        expect_success(src);
    }

    fn expect_success(src: &str) {
        let file_id = FileId(0);
        let tokens = lex(src).expect("Lexing failed");
        let file = parse_file(tokens, file_id).expect("Parsing failed");
        let mut registry = NodeRegistry::empty();
        let file_id = register_file(&mut registry, file);
        let file = registry.file(file_id);
        let mut provider = SymbolProvider::new();
        let symbol_db = bind_symbols_to_identifiers(&registry, vec![file_id], &mut provider)
            .expect("Binding failed");
        validate_fun_recursion_in_file(&symbol_db, file).expect("Fun recursion validation failed");
    }
}

mod should_fail {
    use super::*;
    use crate::{
        data::{registered_ast::*, token::TokenKind},
        processing::{parse::ParseError, validate_fun_recursion::IllegalFunRecursionError},
    };

    #[test]
    fn empty_type_params() {
        let src = include_str!("sample_code/should_fail/empty_parens/empty_type_params.ph");
        let file_id = FileId(0);
        let tokens = lex(src).expect("Lexing failed");
        let err = parse_file(tokens, file_id).expect_err("Parsing unexpectedly succeeded");
        match err {
            ParseError::UnexpectedToken(token) => {
                assert_eq!(token.kind, TokenKind::RParen);
            }
            _ => panic!("Unexpected error: {:#?}", err),
        }
    }

    #[test]
    fn empty_variant_params() {
        let src = include_str!("sample_code/should_fail/empty_parens/empty_variant_params.ph");
        let file_id = FileId(0);
        let tokens = lex(src).expect("Lexing failed");
        let err = parse_file(tokens, file_id).expect_err("Parsing unexpectedly succeeded");
        match err {
            ParseError::UnexpectedToken(token) => {
                assert_eq!(token.kind, TokenKind::RParen);
            }
            _ => panic!("Unexpected error: {:#?}", err),
        }
    }

    #[test]
    fn empty_fun_params() {
        let src = include_str!("sample_code/should_fail/empty_parens/empty_fun_params.ph");
        let file_id = FileId(0);
        let tokens = lex(src).expect("Lexing failed");
        let err = parse_file(tokens, file_id).expect_err("Parsing unexpectedly succeeded");
        match err {
            ParseError::UnexpectedToken(token) => {
                assert_eq!(token.kind, TokenKind::RParen);
            }
            _ => panic!("Unexpected error: {:#?}", err),
        }
    }

    #[test]
    fn empty_call_params() {
        let src = include_str!("sample_code/should_fail/empty_parens/empty_call_params.ph");
        let file_id = FileId(0);
        let tokens = lex(src).expect("Lexing failed");
        let err = parse_file(tokens, file_id).expect_err("Parsing unexpectedly succeeded");
        match err {
            ParseError::UnexpectedToken(token) => {
                assert_eq!(token.kind, TokenKind::RParen);
            }
            _ => panic!("Unexpected error: {:#?}", err),
        }
    }

    #[test]
    fn empty_forall_params() {
        let src = include_str!("sample_code/should_fail/empty_parens/empty_forall_params.ph");
        let file_id = FileId(0);
        let tokens = lex(src).expect("Lexing failed");
        let err = parse_file(tokens, file_id).expect_err("Parsing unexpectedly succeeded");
        match err {
            ParseError::UnexpectedToken(token) => {
                assert_eq!(token.kind, TokenKind::RParen);
            }
            _ => panic!("Unexpected error: {:#?}", err),
        }
    }

    #[test]
    fn empty_match_case_params() {
        let src = include_str!("sample_code/should_fail/empty_parens/empty_match_case_params.ph");
        let file_id = FileId(0);
        let tokens = lex(src).expect("Lexing failed");
        let err = parse_file(tokens, file_id).expect_err("Parsing unexpectedly succeeded");
        match err {
            ParseError::UnexpectedToken(token) => {
                assert_eq!(token.kind, TokenKind::RParen);
            }
            _ => panic!("Unexpected error: {:#?}", err),
        }
    }

    #[test]
    fn reference_let_in_body() {
        let src = include_str!("sample_code/should_fail/scope/ref_let_in_body.ph");
        let file_id = FileId(0);
        let tokens = lex(src).expect("Lexing failed");
        let file = parse_file(tokens, file_id).expect("Parsing failed");
        let mut registry = NodeRegistry::empty();
        let file_id = register_file(&mut registry, file);
        let mut provider = SymbolProvider::new();
        let err = bind_symbols_to_identifiers(&registry, vec![file_id], &mut provider)
            .expect_err("Binding unexpectedly succeeded");
        match err {
            BindError::NameNotFound(err) => {
                assert_eq!(err.name, standard_ident_name("a"), "Unexpected param name");
            }
            _ => panic!("Unexpected error: {:#?}", err),
        }
    }

    #[test]
    fn reference_type_in_param() {
        let src = include_str!("sample_code/should_fail/scope/ref_type_in_param.ph");
        let file_id = FileId(0);
        let tokens = lex(src).expect("Lexing failed");
        let file = parse_file(tokens, file_id).expect("Parsing failed");
        let mut registry = NodeRegistry::empty();
        let file_id = register_file(&mut registry, file);
        let mut provider = SymbolProvider::new();
        let err = bind_symbols_to_identifiers(&registry, vec![file_id], &mut provider)
            .expect_err("Binding unexpectedly succeeded");
        match err {
            BindError::NameNotFound(err) => {
                assert_eq!(err.name, standard_ident_name("U"), "Unexpected param name");
            }
            _ => panic!("Unexpected error: {:#?}", err),
        }
    }

    #[test]
    fn reference_fun_in_param() {
        let src = include_str!("sample_code/should_fail/scope/ref_fun_in_param.ph");
        let file_id = FileId(0);
        let tokens = lex(src).expect("Lexing failed");
        let file = parse_file(tokens, file_id).expect("Parsing failed");
        let mut registry = NodeRegistry::empty();
        let file_id = register_file(&mut registry, file);
        let mut provider = SymbolProvider::new();
        let err = bind_symbols_to_identifiers(&registry, vec![file_id], &mut provider)
            .expect_err("Binding unexpectedly succeeded");
        match err {
            BindError::NameNotFound(err) => {
                assert_eq!(err.name, standard_ident_name("g"), "Unexpected param name");
            }
            _ => panic!("Unexpected error: {:#?}", err),
        }
    }

    #[test]
    fn reference_fun_in_return_type() {
        let src = include_str!("sample_code/should_fail/scope/ref_fun_in_return_type.ph");
        let file_id = FileId(0);
        let tokens = lex(src).expect("Lexing failed");
        let file = parse_file(tokens, file_id).expect("Parsing failed");
        let mut registry = NodeRegistry::empty();
        let file_id = register_file(&mut registry, file);
        let mut provider = SymbolProvider::new();
        let err = bind_symbols_to_identifiers(&registry, vec![file_id], &mut provider)
            .expect_err("Binding unexpectedly succeeded");
        match err {
            BindError::NameNotFound(err) => {
                assert_eq!(err.name, standard_ident_name("g"), "Unexpected param name");
            }
            _ => panic!("Unexpected error: {:#?}", err),
        }
    }

    #[test]
    fn rec_fun_same_param() {
        let src = include_str!("sample_code/should_fail/illegal_recursion/rec_fun_same_param.ph");
        let file_id = FileId(0);
        let tokens = lex(src).expect("Lexing failed");
        let file = parse_file(tokens, file_id).expect("Parsing failed");
        let mut registry = NodeRegistry::empty();
        let file_id = register_file(&mut registry, file);
        let file = registry.file(file_id);
        let mut provider = SymbolProvider::new();
        let symbol_db = bind_symbols_to_identifiers(&registry, vec![file_id], &mut provider)
            .expect("Binding failed");
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
                    standard_ident_name("x"),
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

    #[test]
    fn rec_fun_non_substruct() {
        let src =
            include_str!("sample_code/should_fail/illegal_recursion/rec_fun_non_substruct.ph");
        let file_id = FileId(0);
        let tokens = lex(src).expect("Lexing failed");
        let file = parse_file(tokens, file_id).expect("Parsing failed");
        let mut registry = NodeRegistry::empty();
        let file_id = register_file(&mut registry, file);
        let file = registry.file(file_id);
        let mut provider = SymbolProvider::new();
        let symbol_db = bind_symbols_to_identifiers(&registry, vec![file_id], &mut provider)
            .expect("Binding failed");
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
                    standard_ident_name("x"),
                    "Unexpected param name"
                );
                assert!(
                    matches!(arg, Expression::Identifier(identifier) if identifier.name == standard_ident_name("b")),
                    "Unexpected arg: {:#?}",
                    arg
                );
            }
            _ => panic!("Unexpected error: {:#?}", err),
        }
    }

    #[test]
    fn rec_fun_non_ident() {
        let src = include_str!("sample_code/should_fail/illegal_recursion/rec_fun_non_ident.ph");
        let file_id = FileId(0);
        let tokens = lex(src).expect("Lexing failed");
        let file = parse_file(tokens, file_id).expect("Parsing failed");
        let mut registry = NodeRegistry::empty();
        let file_id = register_file(&mut registry, file);
        let file = registry.file(file_id);
        let mut provider = SymbolProvider::new();
        let symbol_db = bind_symbols_to_identifiers(&registry, vec![file_id], &mut provider)
            .expect("Binding failed");
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
                    standard_ident_name("x"),
                    "Unexpected param name"
                );
                assert!(
                    matches!(arg, Expression::Dot(dot) if dot.right.name == standard_ident_name("O")),
                    "Unexpected arg: {:#?}",
                    arg
                );
            }
            _ => panic!("Unexpected error: {:#?}", err),
        }
    }

    #[test]
    fn rec_fun_no_decreasing_param() {
        let src = include_str!(
            "sample_code/should_fail/illegal_recursion/rec_fun_no_decreasing_param.ph"
        );
        let file_id = FileId(0);
        let tokens = lex(src).expect("Lexing failed");
        let file = parse_file(tokens, file_id).expect("Parsing failed");
        let mut registry = NodeRegistry::empty();
        let file_id = register_file(&mut registry, file);
        let file = registry.file(file_id);
        let mut provider = SymbolProvider::new();
        let symbol_db = bind_symbols_to_identifiers(&registry, vec![file_id], &mut provider)
            .expect("Binding failed");
        let err = validate_fun_recursion_in_file(&symbol_db, file)
            .expect_err("Fun recursion validation unexpectedly succeeded");
        match err {
            IllegalFunRecursionError::RecursivelyCalledFunctionWithoutDecreasingParam {
                callee: callee_id,
            } => {
                let callee = registry.identifier(callee_id);
                assert_eq!(
                    callee.name,
                    standard_ident_name("x"),
                    "Unexpected param name"
                );
            }
            _ => panic!("Unexpected error: {:#?}", err),
        }
    }

    fn standard_ident_name(name: &str) -> IdentifierName {
        IdentifierName::Standard(name.into())
    }
}
