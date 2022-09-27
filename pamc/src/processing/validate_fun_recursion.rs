use crate::data::{
    node_registry::{NodeId, NodeRegistry},
    registered_ast::*,
    symbol_database::SymbolDatabase,
};

#[derive(Clone, Debug)]
pub enum IllegalFunRecursionError {
    RecursiveReferenceWasNotDirectCall {
        reference: NodeId<Identifier>,
    },
    NonSubstructPassedToDecreasingParam {
        callee: NodeId<Identifier>,
        arg: NodeId<WrappedExpression>,
    },
    RecursivelyCalledFunctionWithoutDecreasingParam {
        callee: NodeId<Identifier>,
    },
}

pub fn validate_fun_recursion_in_file(
    symbol_db: &SymbolDatabase,
    registry: &NodeRegistry,
    file: &File,
) -> Result<(), IllegalFunRecursionError> {
    for item_id in &file.item_ids {
        match item_id {
            FileItemNodeId::Type(type_id) => {
                let type_statement = registry.type_statement(*type_id);
                validate_fun_recursion_in_type_statement(symbol_db, registry, type_statement)?;
            }
            FileItemNodeId::Let(let_id) => {
                let let_statement = registry.let_statement(*let_id);
                validate_fun_recursion_in_let_statement(symbol_db, registry, let_statement)?;
            }
        }
    }
    Ok(())
}

fn validate_fun_recursion_in_type_statement(
    symbol_db: &SymbolDatabase,
    registry: &NodeRegistry,
    type_statement: &TypeStatement,
) -> Result<(), IllegalFunRecursionError> {
    for variant_id in &type_statement.variant_ids {
        let variant = registry.variant(*variant_id);
        validate_fun_recursion_in_variant(symbol_db, registry, variant)?;
    }
    Ok(())
}

fn validate_fun_recursion_in_variant(
    symbol_db: &SymbolDatabase,
    registry: &NodeRegistry,
    variant: &Variant,
) -> Result<(), IllegalFunRecursionError> {
    for param_id in &variant.param_ids {
        let param = registry.param(*param_id);
        validate_fun_recursion_in_param(symbol_db, registry, param)?;
    }
    Ok(())
}

fn validate_fun_recursion_in_param(
    symbol_db: &SymbolDatabase,
    registry: &NodeRegistry,
    param: &Param,
) -> Result<(), IllegalFunRecursionError> {
    let mut state = ValidationState::new(symbol_db);
    let type_ = registry.wrapped_expression(param.type_id);
    validate_fun_recursion_in_expression(&mut state, registry, type_)?;
    Ok(())
}

fn validate_fun_recursion_in_let_statement(
    symbol_db: &SymbolDatabase,
    registry: &NodeRegistry,
    let_statement: &LetStatement,
) -> Result<(), IllegalFunRecursionError> {
    let mut state = ValidationState::new(symbol_db);
    let value = registry.wrapped_expression(let_statement.value_id);
    validate_fun_recursion_in_expression(&mut state, registry, value)?;
    Ok(())
}

fn validate_fun_recursion_in_expression(
    state: &mut ValidationState,
    registry: &NodeRegistry,
    expression: &WrappedExpression,
) -> Result<(), IllegalFunRecursionError> {
    match &expression.expression {
        Expression::Identifier(identifier) => {
            if state.reference_restriction(identifier.id).is_some() {
                return Err(
                    IllegalFunRecursionError::RecursiveReferenceWasNotDirectCall {
                        reference: identifier.id,
                    },
                );
            }
            Ok(())
        }
        Expression::Dot(_) => Ok(()),
        Expression::Call(call) => {
            let callee = registry.wrapped_expression(call.callee_id);
            let is_call_restricted = match &callee.expression {
                Expression::Identifier(callee_identifier) => {
                    if let Some(restriction) = state.reference_restriction(callee_identifier.id) {
                        match restriction {
                            ReferenceRestriction::MustCallWithSubstruct {
                                arg_position,
                                superstruct,
                                ..
                            } => {
                                if *arg_position < call.arg_ids.len() {
                                    let expected_substruct =registry.wrapped_expression(call.arg_ids[*arg_position]);
                                    let err = Err(
                                        IllegalFunRecursionError::NonSubstructPassedToDecreasingParam {
                                          callee: callee_identifier.id,
                                          arg: expected_substruct.id,
                                        },
                                    );
                                    match &expected_substruct.expression {
                                        Expression::Identifier(expected_substruct_identifier) => {
                                            if !state.is_substruct_of_restricted_superstruct(
                                                expected_substruct_identifier.id,
                                               *superstruct,
                                            ) {
                                                return err;
                                            }
                                        }
                                        _ => return err,
                                    }
                                }
                            }
                            ReferenceRestriction::CannotCall {..}=> return Err(IllegalFunRecursionError::RecursivelyCalledFunctionWithoutDecreasingParam {
                                callee: callee_identifier.id,
                            }),
                        }
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            };

            // If the call is restricted (i.e., in the form
            // `f(x, y, ...z)`, where `f` is a restricted recursive function,
            // then we need to skip the callee validation (otherwise `f` will trigger
            // an error, since it is a recursive reference).
            if !is_call_restricted {
                validate_fun_recursion_in_expression(state, registry, callee)?;
            }

            for arg_id in &call.arg_ids {
                let arg = registry.wrapped_expression(*arg_id);
                validate_fun_recursion_in_expression(state, registry, arg)?;
            }
            Ok(())
        }
        Expression::Fun(fun) => {
            for param_id in &fun.param_ids {
                let param = registry.param(*param_id);
                let param_type = registry.wrapped_expression(param.type_id);
                validate_fun_recursion_in_expression(state, registry, param_type)?;
            }
            let return_type = registry.wrapped_expression(fun.return_type_id);
            validate_fun_recursion_in_expression(state, registry, return_type)?;

            let decreasing_param_position_and_decreasing_param =
                fun.param_ids.iter().enumerate().find(|(_i, param_id)| {
                    let param = registry.param(**param_id);
                    param.is_dashed
                });
            let reference_restriction = match decreasing_param_position_and_decreasing_param {
                Some((param_position, decreasing_param_id)) => {
                    let decreasing_param = registry.param(*decreasing_param_id);
                    state.create_must_call_with_substruct_restriction(
                        fun.name_id,
                        param_position,
                        decreasing_param.name_id,
                    )
                }
                None => state.create_cannot_call_restriction(fun.name_id),
            };

            state.push_reference_restriction(reference_restriction);
            let body = registry.wrapped_expression(fun.body_id);
            validate_fun_recursion_in_expression(state, registry, body)?;
            state.pop_reference_restriction();

            Ok(())
        }
        Expression::Match(match_) => {
            let matchee = registry.wrapped_expression(match_.matchee_id);
            validate_fun_recursion_in_expression(state, registry, matchee)?;
            match &matchee.expression {
                Expression::Identifier(matchee_identifier) => {
                    if let Some(mut substructs) =
                        state.matchee_substructs_mut(matchee_identifier.id)
                    {
                        for case_id in &match_.case_ids {
                            let case = registry.match_case(*case_id);
                            for case_param_id in &case.param_ids {
                                let case_param = registry.identifier(*case_param_id);
                                substructs.push(case_param.id);
                            }
                        }
                    }
                }
                _ => {}
            }
            for case_id in &match_.case_ids {
                let case = registry.match_case(*case_id);
                let case_output = registry.wrapped_expression(case.output_id);
                validate_fun_recursion_in_expression(state, registry, case_output)?;
            }
            Ok(())
        }
        Expression::Forall(forall) => {
            for param_id in &forall.param_ids {
                let param = registry.param(*param_id);
                let param_type = registry.wrapped_expression(param.type_id);
                validate_fun_recursion_in_expression(state, registry, param_type)?;
            }
            let output = registry.wrapped_expression(forall.output_id);
            validate_fun_recursion_in_expression(state, registry, output)?;

            Ok(())
        }
    }
}

use state::*;
mod state {
    use super::*;
    use crate::data::symbol_database::Symbol;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum ReferenceRestriction {
        MustCallWithSubstruct {
            restricted_referent: Symbol,
            superstruct: Symbol,
            arg_position: usize,
            substructs: Vec<Symbol>,
        },
        CannotCall {
            restricted_referent: Symbol,
        },
    }

    // The "real" logic is all implemented in `SymbolicState`--this
    // is just a thin wrapper that converts identifier node IDs to
    // symbols, so the consumer won't have to deal with symbols.
    #[derive(Clone, Debug)]
    pub struct ValidationState<'a> {
        symbol_db: &'a SymbolDatabase,
        internal: SymbolicState,
    }

    impl ValidationState<'_> {
        pub fn new(symbol_db: &SymbolDatabase) -> ValidationState {
            ValidationState {
                symbol_db,
                internal: SymbolicState::empty(),
            }
        }
    }

    impl ValidationState<'_> {
        pub fn reference_restriction(
            &self,
            referent: NodeId<Identifier>,
        ) -> Option<&ReferenceRestriction> {
            let referent_symbol = self.symbol_db.identifier_symbols.get(referent);
            self.internal.reference_restriction(referent_symbol)
        }

        /// Panics if `possible_superstruct` is not a superstruct of any
        /// reference restriction.
        pub fn is_substruct_of_restricted_superstruct(
            &self,
            possible_substruct: NodeId<Identifier>,
            possible_superstruct: Symbol,
        ) -> bool {
            let possible_substruct_symbol =
                self.symbol_db.identifier_symbols.get(possible_substruct);
            self.internal.is_substruct_of_restricted_superstruct(
                possible_substruct_symbol,
                possible_superstruct,
            )
        }

        pub fn push_reference_restriction(&mut self, restriction: ReferenceRestriction) {
            self.internal.push_reference_restriction(restriction)
        }

        pub fn pop_reference_restriction(&mut self) {
            self.internal.pop_reference_restriction()
        }
    }

    impl ValidationState<'_> {
        pub fn create_must_call_with_substruct_restriction(
            &self,
            fun_name: NodeId<Identifier>,
            param_position: usize,
            param_name: NodeId<Identifier>,
        ) -> ReferenceRestriction {
            let fun_symbol = self.symbol_db.identifier_symbols.get(fun_name);
            let param_symbol = self.symbol_db.identifier_symbols.get(param_name);
            ReferenceRestriction::MustCallWithSubstruct {
                restricted_referent: fun_symbol,
                arg_position: param_position,
                superstruct: param_symbol,
                substructs: vec![],
            }
        }

        pub fn create_cannot_call_restriction(
            &self,
            fun_name: NodeId<Identifier>,
        ) -> ReferenceRestriction {
            let fun_symbol = self.symbol_db.identifier_symbols.get(fun_name);
            ReferenceRestriction::CannotCall {
                restricted_referent: fun_symbol,
            }
        }
    }

    pub trait Push<T> {
        fn push(&mut self, value: T);
    }

    impl ValidationState<'_> {
        pub fn matchee_substructs_mut(
            &mut self,
            matchee: NodeId<Identifier>,
        ) -> Option<impl Push<NodeId<Identifier>> + '_> {
            struct PushAndConvertToSymbol<'a> {
                symbol_db: &'a SymbolDatabase,
                symbols: &'a mut Vec<Symbol>,
            }

            impl Push<NodeId<Identifier>> for PushAndConvertToSymbol<'_> {
                fn push(&mut self, item: NodeId<Identifier>) {
                    self.symbols
                        .push(self.symbol_db.identifier_symbols.get(item))
                }
            }

            let matchee_symbol = self.symbol_db.identifier_symbols.get(matchee);
            if let Some(substruct_symbols) = self.internal.matchee_substructs_mut(matchee_symbol) {
                Some(PushAndConvertToSymbol {
                    symbol_db: &self.symbol_db,
                    symbols: substruct_symbols,
                })
            } else {
                None
            }
        }
    }

    // This is where the "real" logic is implemented.
    use symbolic_state::*;
    mod symbolic_state {
        use super::*;

        #[derive(Clone, Debug)]
        pub struct SymbolicState {
            restrictions: Vec<ReferenceRestriction>,
        }

        impl SymbolicState {
            pub fn empty() -> Self {
                Self {
                    restrictions: Vec::new(),
                }
            }
        }

        impl SymbolicState {
            pub fn reference_restriction(&self, referent: Symbol) -> Option<&ReferenceRestriction> {
                for restriction in self.restrictions.iter().rev() {
                    if restriction.restricted_referent() == referent {
                        return Some(restriction);
                    }
                }
                None
            }

            /// Panics if `possible_superstruct` is not a superstruct of any
            /// reference restriction.
            pub fn is_substruct_of_restricted_superstruct(
                &self,
                possible_substruct: Symbol,
                possible_superstruct: Symbol,
            ) -> bool {
                for restriction in self.restrictions.iter().rev() {
                    match restriction {
                        ReferenceRestriction::MustCallWithSubstruct {
                            superstruct,
                            substructs,
                            ..
                        } => {
                            if *superstruct == possible_superstruct {
                                return substructs.contains(&possible_substruct);
                            }
                        }
                        ReferenceRestriction::CannotCall { .. } => {}
                    }
                }

                panic!(
                    "No superstruct restriction found for {:?}",
                    possible_superstruct
                );
            }

            pub fn push_reference_restriction(&mut self, restriction: ReferenceRestriction) {
                self.restrictions.push(restriction);
            }

            pub fn pop_reference_restriction(&mut self) {
                self.restrictions
                    .pop()
                    .expect("Error: Tried to pop an empty reference restriction stack.");
            }

            pub fn matchee_substructs_mut(&mut self, matchee: Symbol) -> Option<&mut Vec<Symbol>> {
                for restriction in self.restrictions.iter_mut().rev() {
                    match restriction {
                        ReferenceRestriction::MustCallWithSubstruct {
                            superstruct,
                            substructs,
                            ..
                        } => {
                            if *superstruct == matchee || substructs.contains(&matchee) {
                                return Some(substructs);
                            }
                        }
                        ReferenceRestriction::CannotCall { .. } => {}
                    }
                }
                None
            }
        }

        impl ReferenceRestriction {
            fn restricted_referent(&self) -> Symbol {
                match self {
                    ReferenceRestriction::MustCallWithSubstruct {
                        restricted_referent,
                        ..
                    } => *restricted_referent,
                    ReferenceRestriction::CannotCall {
                        restricted_referent,
                    } => *restricted_referent,
                }
            }
        }
    }
}
