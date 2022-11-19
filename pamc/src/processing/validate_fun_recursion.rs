use crate::data::{
    x_light_ast::*,
    x_node_registry::{NodeId, NodeRegistry},
};

#[derive(Clone, Debug)]
pub enum IllegalFunRecursionError {
    RecursiveReferenceWasNotDirectCall {
        reference: NodeId<NameExpression>,
    },
    NonSubstructPassedToDecreasingParam {
        callee: NodeId<NameExpression>,
        arg: ExpressionId,
    },
    RecursivelyCalledFunctionWithoutDecreasingParam {
        callee: NodeId<NameExpression>,
    },
}

pub fn validate_fun_recursion_in_file(
    registry: &NodeRegistry,
    file: &File,
) -> Result<(), IllegalFunRecursionError> {
    let mut context = Context::new(registry);
    let item_ids = registry.file_item_list(file.item_list_id);
    for item_id in item_ids {
        match item_id {
            FileItemNodeId::Type(type_id) => {
                let type_statement = registry.type_statement(*type_id);
                validate_fun_recursion_in_type_statement(&mut context, registry, type_statement)?;
            }
            FileItemNodeId::Let(let_id) => {
                let let_statement = registry.let_statement(*let_id);
                validate_fun_recursion_in_let_statement(&mut context, registry, let_statement)?;
            }
        }
    }
    Ok(())
}

fn validate_fun_recursion_in_type_statement(
    context: &mut Context,
    registry: &NodeRegistry,
    type_statement: &TypeStatement,
) -> Result<(), IllegalFunRecursionError> {
    let variant_ids = registry.variant_list(type_statement.variant_list_id);
    for variant_id in variant_ids {
        let variant = registry.variant(*variant_id);
        validate_fun_recursion_in_variant(context, registry, variant)?;
    }
    Ok(())
}

fn validate_fun_recursion_in_variant(
    context: &mut Context,
    registry: &NodeRegistry,
    variant: &Variant,
) -> Result<(), IllegalFunRecursionError> {
    let param_ids = registry.param_list(variant.param_list_id);
    for param_id in param_ids {
        let param = registry.param(*param_id);
        validate_fun_recursion_in_param(context, registry, param)?;
    }
    Ok(())
}

fn validate_fun_recursion_in_param(
    context: &mut Context,
    registry: &NodeRegistry,
    param: &Param,
) -> Result<(), IllegalFunRecursionError> {
    validate_fun_recursion_in_expression(context, registry, param.type_id)?;
    Ok(())
}

fn validate_fun_recursion_in_let_statement(
    context: &mut Context,
    registry: &NodeRegistry,
    let_statement: &LetStatement,
) -> Result<(), IllegalFunRecursionError> {
    validate_fun_recursion_in_expression(context, registry, let_statement.value_id)?;
    Ok(())
}

fn validate_fun_recursion_in_expression(
    context: &mut Context,
    registry: &NodeRegistry,
    expression_id: ExpressionId,
) -> Result<(), IllegalFunRecursionError> {
    match expression_id {
        ExpressionId::Name(name_id) => {
            if context.reference_restriction(name_id).is_some() {
                return Err(
                    IllegalFunRecursionError::RecursiveReferenceWasNotDirectCall {
                        reference: name_id,
                    },
                );
            }
            Ok(())
        }
        ExpressionId::Call(call_id) => {
            let call = registry.call(call_id);
            let is_call_restricted = match call.callee_id {
                ExpressionId::Name(callee_name_id) => {
                    let callee_name = registry.name_expression(callee_name_id);
                    if let Some(restriction) = context.reference_restriction(callee_name_id) {
                        match restriction {
                            ReferenceRestriction::MustCallWithSubstruct {
                                arg_position,
                                superstruct_db_level,
                                ..
                            } => {
                                let arg_ids = registry.expression_list(call.arg_list_id);
                                if arg_position < arg_ids.len() {
                                    let expected_substruct_id = arg_ids[arg_position];
                                    let err = Err(
                                        IllegalFunRecursionError::NonSubstructPassedToDecreasingParam {
                                          callee: callee_name_id,
                                          arg: expected_substruct_id,
                                        },
                                    );
                                    match expected_substruct_id {
                                        ExpressionId::Name(expected_substruct_name_id) => {
                                            if !context.is_substruct_of_restricted_superstruct(
                                                expected_substruct_name_id,
                                                superstruct_db_level,
                                            ) {
                                                return err;
                                            }
                                        }
                                        _ => return err,
                                    }
                                }
                            }
                            ReferenceRestriction::CannotCall {..}=> return Err(IllegalFunRecursionError::RecursivelyCalledFunctionWithoutDecreasingParam {
                                callee: callee_name.id,
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
                validate_fun_recursion_in_expression(context, registry, call.callee_id)?;
            }

            let arg_ids = registry.expression_list(call.arg_list_id);
            for arg_id in arg_ids {
                validate_fun_recursion_in_expression(context, registry, *arg_id)?;
            }
            Ok(())
        }
        ExpressionId::Fun(fun_id) => {
            let fun = registry.fun(fun_id);
            let param_ids = registry.param_list(fun.param_list_id);
            for param_id in param_ids {
                let param = registry.param(*param_id);
                validate_fun_recursion_in_expression(context, registry, param.type_id)?;
            }
            validate_fun_recursion_in_expression(context, registry, fun.return_type_id)?;

            let decreasing_param_position_and_decreasing_param =
                param_ids.iter().enumerate().find(|(_i, param_id)| {
                    let param = registry.param(**param_id);
                    param.is_dashed
                });
            let reference_restriction = match decreasing_param_position_and_decreasing_param {
                Some((param_position, decreasing_param_id)) => {
                    let decreasing_param = registry.param(*decreasing_param_id);
                    context.create_must_call_with_substruct_restriction(
                        fun.name_id,
                        param_position,
                        decreasing_param.name_id,
                    )
                }
                None => context.create_cannot_call_restriction(fun.name_id),
            };

            context.push_reference_restriction(reference_restriction);
            validate_fun_recursion_in_expression(context, registry, fun.body_id)?;
            context.pop_reference_restriction();

            Ok(())
        }
        ExpressionId::Match(match_id) => {
            let match_ = registry.match_(match_id);
            validate_fun_recursion_in_expression(context, registry, match_.matchee_id)?;
            let case_ids = registry.match_case_list(match_.case_list_id);
            match match_.matchee_id {
                ExpressionId::Name(matchee_name_id) => {
                    if let Some(mut substructs) = context.matchee_substructs_mut(matchee_name_id) {
                        for case_id in case_ids {
                            let case = registry.match_case(*case_id);
                            let param_ids = registry.identifier_list(case.param_list_id);
                            for case_param_id in param_ids {
                                let case_param = registry.identifier(*case_param_id);
                                substructs.push(case_param.id);
                            }
                        }
                    }
                }
                _ => {}
            }
            for case_id in case_ids {
                let case = registry.match_case(*case_id);
                validate_fun_recursion_in_expression(context, registry, case.output_id)?;
            }
            Ok(())
        }
        ExpressionId::Forall(forall_id) => {
            let forall = registry.forall(forall_id);
            let param_ids = registry.param_list(forall.param_list_id);
            for param_id in param_ids {
                let param = registry.param(*param_id);
                validate_fun_recursion_in_expression(context, registry, param.type_id)?;
            }
            validate_fun_recursion_in_expression(context, registry, forall.output_id)?;

            Ok(())
        }
    }
}

// TODO: Delete
// use state::*;
// mod state {
//     use super::*;
//     use crate::data::symbol_database::Symbol;

//     #[derive(Clone, Debug, PartialEq, Eq)]
//     pub enum ReferenceRestriction {
//         MustCallWithSubstruct {
//             restricted_referent: Symbol,
//             superstruct: Symbol,
//             arg_position: usize,
//             substructs: Vec<Symbol>,
//         },
//         CannotCall {
//             restricted_referent: Symbol,
//         },
//     }

//     // The "real" logic is all implemented in `SymbolicState`--this
//     // is just a thin wrapper that converts identifier node IDs to
//     // symbols, so the consumer won't have to deal with symbols.
//     #[derive(Clone, Debug)]
//     pub struct ValidationState<'a> {
//         registry: &'a NodeRegistry,
//         symbol_db: &'a SymbolDatabase,
//         internal: SymbolicState,
//     }

//     impl<'a> ValidationState<'a> {
//         pub fn new(
//             registry: &'a NodeRegistry,
//             symbol_db: &'a SymbolDatabase,
//         ) -> ValidationState<'a> {
//             ValidationState {
//                 registry,
//                 symbol_db,
//                 internal: SymbolicState::empty(),
//             }
//         }
//     }

//     impl ValidationState<'_> {
//         pub fn reference_restriction(
//             &self,
//             referent_id: NodeId<NameExpression>,
//         ) -> Option<&ReferenceRestriction> {
//             let referent_symbol = self
//                 .symbol_db
//                 .identifier_symbols
//                 .get_using_rightmost((referent_id, self.registry));
//             self.internal.reference_restriction(referent_symbol)
//         }

//         /// Panics if `possible_superstruct` is not a superstruct of any
//         /// reference restriction.
//         pub fn is_substruct_of_restricted_superstruct(
//             &self,
//             possible_substruct: NodeId<NameExpression>,
//             possible_superstruct: Symbol,
//         ) -> bool {
//             let possible_substruct_symbol = self
//                 .symbol_db
//                 .identifier_symbols
//                 .get_using_rightmost((possible_substruct, self.registry));
//             self.internal.is_substruct_of_restricted_superstruct(
//                 possible_substruct_symbol,
//                 possible_superstruct,
//             )
//         }

//         pub fn push_reference_restriction(&mut self, restriction: ReferenceRestriction) {
//             self.internal.push_reference_restriction(restriction)
//         }

//         pub fn pop_reference_restriction(&mut self) {
//             self.internal.pop_reference_restriction()
//         }
//     }

//     impl ValidationState<'_> {
//         pub fn create_must_call_with_substruct_restriction(
//             &self,
//             fun_name: NodeId<Identifier>,
//             param_position: usize,
//             param_name: NodeId<Identifier>,
//         ) -> ReferenceRestriction {
//             let fun_symbol = self.symbol_db.identifier_symbols.get(fun_name);
//             let param_symbol = self.symbol_db.identifier_symbols.get(param_name);
//             ReferenceRestriction::MustCallWithSubstruct {
//                 restricted_referent: fun_symbol,
//                 arg_position: param_position,
//                 superstruct: param_symbol,
//                 substructs: vec![],
//             }
//         }

//         pub fn create_cannot_call_restriction(
//             &self,
//             fun_name: NodeId<Identifier>,
//         ) -> ReferenceRestriction {
//             let fun_symbol = self.symbol_db.identifier_symbols.get(fun_name);
//             ReferenceRestriction::CannotCall {
//                 restricted_referent: fun_symbol,
//             }
//         }
//     }

//     pub trait Push<T> {
//         fn push(&mut self, value: T);
//     }

//     impl ValidationState<'_> {
//         pub fn matchee_substructs_mut(
//             &mut self,
//             matchee: NodeId<NameExpression>,
//         ) -> Option<impl Push<NodeId<Identifier>> + '_> {
//             struct PushAndConvertToSymbol<'a> {
//                 symbol_db: &'a SymbolDatabase,
//                 symbols: &'a mut Vec<Symbol>,
//             }

//             impl Push<NodeId<Identifier>> for PushAndConvertToSymbol<'_> {
//                 fn push(&mut self, item: NodeId<Identifier>) {
//                     self.symbols
//                         .push(self.symbol_db.identifier_symbols.get(item))
//                 }
//             }

//             let matchee_symbol = self
//                 .symbol_db
//                 .identifier_symbols
//                 .get_using_rightmost((matchee, self.registry));
//             if let Some(substruct_symbols) = self.internal.matchee_substructs_mut(matchee_symbol) {
//                 Some(PushAndConvertToSymbol {
//                     symbol_db: &self.symbol_db,
//                     symbols: substruct_symbols,
//                 })
//             } else {
//                 None
//             }
//         }
//     }

//     // This is where the "real" logic is implemented.
//     use symbolic_state::*;
//     mod symbolic_state {
//         use super::*;

//         #[derive(Clone, Debug)]
//         pub struct SymbolicState {
//             restrictions: Vec<ReferenceRestriction>,
//         }

//         impl SymbolicState {
//             pub fn empty() -> Self {
//                 Self {
//                     restrictions: Vec::new(),
//                 }
//             }
//         }

//         impl SymbolicState {
//             pub fn reference_restriction(&self, referent: Symbol) -> Option<&ReferenceRestriction> {
//                 for restriction in self.restrictions.iter().rev() {
//                     if restriction.restricted_referent() == referent {
//                         return Some(restriction);
//                     }
//                 }
//                 None
//             }

//             /// Panics if `possible_superstruct` is not a superstruct of any
//             /// reference restriction.
//             pub fn is_substruct_of_restricted_superstruct(
//                 &self,
//                 possible_substruct: Symbol,
//                 possible_superstruct: Symbol,
//             ) -> bool {
//                 for restriction in self.restrictions.iter().rev() {
//                     match restriction {
//                         ReferenceRestriction::MustCallWithSubstruct {
//                             superstruct,
//                             substructs,
//                             ..
//                         } => {
//                             if *superstruct == possible_superstruct {
//                                 return substructs.contains(&possible_substruct);
//                             }
//                         }
//                         ReferenceRestriction::CannotCall { .. } => {}
//                     }
//                 }

//                 panic!(
//                     "No superstruct restriction found for {:?}",
//                     possible_superstruct
//                 );
//             }

//             pub fn push_reference_restriction(&mut self, restriction: ReferenceRestriction) {
//                 self.restrictions.push(restriction);
//             }

//             pub fn pop_reference_restriction(&mut self) {
//                 self.restrictions
//                     .pop()
//                     .expect("Error: Tried to pop an empty reference restriction stack.");
//             }

//             pub fn matchee_substructs_mut(&mut self, matchee: Symbol) -> Option<&mut Vec<Symbol>> {
//                 for restriction in self.restrictions.iter_mut().rev() {
//                     match restriction {
//                         ReferenceRestriction::MustCallWithSubstruct {
//                             superstruct,
//                             substructs,
//                             ..
//                         } => {
//                             if *superstruct == matchee || substructs.contains(&matchee) {
//                                 return Some(substructs);
//                             }
//                         }
//                         ReferenceRestriction::CannotCall { .. } => {}
//                     }
//                 }
//                 None
//             }
//         }

//         impl ReferenceRestriction {
//             fn restricted_referent(&self) -> Symbol {
//                 match self {
//                     ReferenceRestriction::MustCallWithSubstruct {
//                         restricted_referent,
//                         ..
//                     } => *restricted_referent,
//                     ReferenceRestriction::CannotCall {
//                         restricted_referent,
//                     } => *restricted_referent,
//                 }
//             }
//         }
//     }
// }

#[derive(Clone, Debug)]
struct Context<'a> {
    registry: &'a NodeRegistry,
    raw: RawContext,
}

impl<'a> Context<'a> {
    fn new(registry: &'a NodeRegistry) -> Self {
        Self {
            registry,
            raw: RawContext::new(),
        }
    }
}

impl Context<'_> {
    fn reference_restriction(
        &self,
        name_id: NodeId<NameExpression>,
    ) -> Option<ReferenceRestriction> {
        let index = self.registry.name_expression(name_id).db_index;
        let level = self.raw.index_to_level(index);
        self.raw.reference_restriction(level)
    }

    fn is_substruct_of_restricted_superstruct(
        &self,
        possible_substruct_id: NodeId<NameExpression>,
        possible_superstruct_level: DbLevel,
    ) -> bool {
        unimplemented!()
    }
}

#[derive(Clone, Debug)]
struct RawContext {
    stack: Vec<ContextEntry>,
}

#[derive(Clone, Copy, Debug)]
enum ContextEntry {
    Substructure { superstruct_db_level: DbLevel },
    Fun(ReferenceRestriction),
    NoInformation,
}

#[derive(Clone, Copy, Debug)]
enum ReferenceRestriction {
    MustCallWithSubstruct {
        superstruct_db_level: DbLevel,
        arg_position: usize,
    },
    CannotCall,
}

impl RawContext {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }
}

impl RawContext {
    fn len(&self) -> usize {
        self.stack.len()
    }

    fn level_to_index(&self, level: DbLevel) -> DbIndex {
        DbIndex(self.len() - level.0 - 1)
    }

    fn index_to_level(&self, index: DbIndex) -> DbLevel {
        DbLevel(self.len() - index.0 - 1)
    }
}

impl RawContext {
    /// Panics if `n > self.len()`.
    fn pop_n(&mut self, n: usize) {
        if n > self.len() {
            panic!(
                "Tried to pop {} elements from a context with only {} elements",
                n,
                self.len()
            );
        }
        self.stack.truncate(self.len() - n);
    }

    fn push(&mut self, entry: ContextEntry) {
        self.stack.push(entry);
    }
}

impl RawContext {
    fn reference_restriction(&self, level: DbLevel) -> Option<ReferenceRestriction> {
        let entry = self.stack[level.0];
        match entry {
            ContextEntry::Substructure { .. } => None,
            ContextEntry::Fun(restriction) => Some(restriction),
            ContextEntry::NoInformation => None,
        }
    }
}
