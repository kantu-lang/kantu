use crate::data::{
    node_registry::NodeId,
    registered_ast::*,
    symbol_database::{Symbol, SymbolDatabase},
};

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
            let is_call_restricted = match &call.callee.expression {
                Expression::Identifier(callee_identifier) => {
                    if let Some(restriction) = state.reference_restriction(callee_identifier.id) {
                        match restriction {
                            ReferenceRestriction::MustCallWithSubstruct {
                                arg_position,
                                superstruct,
                                ..
                            } => {
                                if *arg_position < call.args.len() {
                                    let expected_substruct = &call.args[*arg_position];
                                    let err = Err(
                                        IllegalFunRecursionError::NonSubstructPassedToDecreasingParam(
                                            callee_identifier.id,
                                            expected_substruct.id,
                                        ),
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
                            ReferenceRestriction::CannotCall {..}=> return Err(IllegalFunRecursionError::CannotRecursivelyCallFunctionWithoutDecreasingParam(
                                callee_identifier.id,
                            )),
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
                validate_fun_recursion_in_expression(state, &call.callee)?;
            }

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
                Some((param_position, decreasing_param)) => state
                    .create_must_call_with_substruct_restriction(
                        fun.name.id,
                        param_position,
                        decreasing_param.name.id,
                    ),
                None => state.create_cannot_call_restriction(fun.name.id),
            };

            state.push_reference_restriction(reference_restriction);
            validate_fun_recursion_in_expression(state, &fun.body)?;
            state.pop_reference_restriction();

            Ok(())
        }
        Expression::Match(match_) => {
            validate_fun_recursion_in_expression(state, &match_.matchee)?;
            match &match_.matchee.expression {
                Expression::Identifier(matchee_identifier) => {
                    if let Some(mut substructs) =
                        state.matchee_substructs_mut(matchee_identifier.id)
                    {
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

    #[derive(Clone, Debug)]
    pub struct ValidateFunRecursionState<'a> {
        symbol_db: &'a SymbolDatabase,
        internal: SymbolicState,
    }

    impl ValidateFunRecursionState<'_> {
        pub fn new(symbol_db: &SymbolDatabase) -> ValidateFunRecursionState {
            ValidateFunRecursionState {
                symbol_db,
                internal: SymbolicState::empty(),
            }
        }
    }

    impl ValidateFunRecursionState<'_> {
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

    pub trait Push<T> {
        fn push(&mut self, value: T);
    }

    impl ValidateFunRecursionState<'_> {
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

    #[derive(Clone, Debug)]
    struct SymbolicState {
        restrictions: Vec<ReferenceRestriction>,
    }

    impl SymbolicState {
        fn empty() -> Self {
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
