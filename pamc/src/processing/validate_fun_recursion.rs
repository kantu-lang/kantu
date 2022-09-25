use crate::data::{node_registry::NodeId, registered_ast::*, symbol_database::SymbolDatabase};

#[derive(Clone, Debug)]
pub enum IllegalFunRecursionError {
    RecursiveReferenceWasNotDirectCall(NodeId<Identifier>),
    NonSubstructPassedToDecreasingParam(NodeId<Identifier>, NodeId<WrappedExpression>),
    CannotRecursivelyCallFunctionWithoutDecreasingParam(NodeId<Identifier>),
}

pub fn validate_fun_recursion_in_file(
    symbol_db: &SymbolDatabase,
    file: &File,
) -> Result<(), IllegalFunRecursionError> {
    for item in &file.items {
        match item {
            FileItem::Type(type_statement) => {
                validate_fun_recursion_in_type_statement(symbol_db, type_statement)?;
            }
            FileItem::Let(let_statement) => {
                validate_fun_recursion_in_let_statement(symbol_db, let_statement)?;
            }
        }
    }
    Ok(())
}

fn validate_fun_recursion_in_type_statement(
    symbol_db: &SymbolDatabase,
    type_statement: &TypeStatement,
) -> Result<(), IllegalFunRecursionError> {
    for variant in &type_statement.variants {
        validate_fun_recursion_in_variant(symbol_db, variant)?;
    }
    Ok(())
}

fn validate_fun_recursion_in_variant(
    symbol_db: &SymbolDatabase,
    variant: &Variant,
) -> Result<(), IllegalFunRecursionError> {
    for param in &variant.params {
        validate_fun_recursion_in_param(symbol_db, param)?;
    }
    Ok(())
}

fn validate_fun_recursion_in_param(
    symbol_db: &SymbolDatabase,
    param: &Param,
) -> Result<(), IllegalFunRecursionError> {
    let mut state = ValidateFunRecursionState::new(symbol_db);
    validate_fun_recursion_in_expression(&mut state, &param.type_)?;
    Ok(())
}

fn validate_fun_recursion_in_let_statement(
    symbol_db: &SymbolDatabase,
    let_statement: &LetStatement,
) -> Result<(), IllegalFunRecursionError> {
    let mut state = ValidateFunRecursionState::new(symbol_db);
    validate_fun_recursion_in_expression(&mut state, &let_statement.value)?;
    Ok(())
}

fn validate_fun_recursion_in_expression(
    state: &mut ValidateFunRecursionState,
    expression: &WrappedExpression,
) -> Result<(), IllegalFunRecursionError> {
    match &expression.expression {
        Expression::Identifier(identifier) => {
            if state.reference_restriction(identifier.id).is_some() {
                return Err(
                    IllegalFunRecursionError::RecursiveReferenceWasNotDirectCall(identifier.id),
                );
            }
            Ok(())
        }
        Expression::Dot(_) => Ok(()),
        Expression::Call(call) => {
            match &call.callee.expression {
                Expression::Identifier(callee_identifier) => {
                    if let Some(restriction) = state.reference_restriction(callee_identifier.id) {
                        match restriction {
                            ReferenceRestriction::MustCallWithSubstruct {
                                arg_position,
                                superstruct,
                                ..
                            } => {
                                if arg_position < call.args.len() {
                                    let expected_substruct = &call.args[arg_position];
                                    let err = Err(
                                        IllegalFunRecursionError::NonSubstructPassedToDecreasingParam(
                                            callee_identifier.id,
                                            expected_substruct.id,
                                        ),
                                    );
                                    match &expected_substruct.expression {
                                        Expression::Identifier(expected_substruct_identifier) => {
                                            if !state.is_substruct(
                                                expected_substruct_identifier.id,
                                                superstruct,
                                            ) {
                                                return err;
                                            }
                                        }
                                        _ => return err,
                                    }
                                }
                            }
                            ReferenceRestriction::CannotCall => return Err(IllegalFunRecursionError::CannotRecursivelyCallFunctionWithoutDecreasingParam(
                                callee_identifier.id,
                            )),
                        }
                    }
                }
                _ => {}
            }

            validate_fun_recursion_in_expression(state, &call.callee)?;
            for arg in &call.args {
                validate_fun_recursion_in_expression(state, arg)?;
            }
            Ok(())
        }
        Expression::Fun(fun) => {
            for param in &fun.params {
                validate_fun_recursion_in_expression(state, &param.type_)?;
            }
            validate_fun_recursion_in_expression(state, &fun.return_type)?;

            let decreasing_param_position_and_decreasing_param =
                fun.params.iter().enumerate().find(|(_i, p)| p.is_dashed);
            let reference_restriction = match decreasing_param_position_and_decreasing_param {
                Some((param_position, decreasing_param)) => {
                    ReferenceRestriction::MustCallWithSubstruct {
                        arg_position: param_position,
                        superstruct: decreasing_param.name.id,
                        substructs: vec![],
                    }
                }
                None => ReferenceRestriction::CannotCall,
            };

            state.push_reference_restriction(fun.name.id, reference_restriction);
            validate_fun_recursion_in_expression(state, &fun.body)?;
            state.pop_reference_restriction();

            Ok(())
        }
        Expression::Match(match_) => {
            validate_fun_recursion_in_expression(state, &match_.matchee)?;
            match &match_.matchee.expression {
                Expression::Identifier(matchee_identifier) => {
                    if let Some(substructs) = state.matchee_substructs_mut(matchee_identifier.id) {
                        for case in &match_.cases {
                            for case_param in &case.params {
                                substructs.push(case_param.id);
                            }
                        }
                    }
                }
                _ => {}
            }
            for case in &match_.cases {
                validate_fun_recursion_in_expression(state, &case.output)?;
            }
            Ok(())
        }
        Expression::Forall(forall) => {
            for param in &forall.params {
                validate_fun_recursion_in_expression(state, &param.type_)?;
            }
            validate_fun_recursion_in_expression(state, &forall.output)?;

            Ok(())
        }
    }
}

use state::*;
mod state {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct ValidateFunRecursionState<'a> {
        symbol_db: &'a SymbolDatabase,
    }

    impl ValidateFunRecursionState<'_> {
        pub fn new(symbol_db: &SymbolDatabase) -> Self {
            unimplemented!()
        }
    }

    impl ValidateFunRecursionState<'_> {
        pub fn reference_restriction(
            &self,
            id: NodeId<Identifier>,
        ) -> Option<ReferenceRestriction> {
            unimplemented!()
        }

        pub fn is_substruct(
            &self,
            possible_substruct: NodeId<Identifier>,
            possible_superstruct: NodeId<Identifier>,
        ) -> bool {
            unimplemented!()
        }

        pub fn push_reference_restriction(
            &mut self,
            id: NodeId<Identifier>,
            restriction: ReferenceRestriction,
        ) {
            unimplemented!()
        }

        pub fn pop_reference_restriction(&mut self) {
            unimplemented!()
        }

        pub fn matchee_substructs_mut(
            &mut self,
            matchee: NodeId<Identifier>,
        ) -> Option<&mut Vec<NodeId<Identifier>>> {
            unimplemented!()
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum ReferenceRestriction {
        MustCallWithSubstruct {
            superstruct: NodeId<Identifier>,
            arg_position: usize,
            substructs: Vec<NodeId<Identifier>>,
        },
        CannotCall,
    }
}
