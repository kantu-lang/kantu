use super::*;

pub(in crate::processing::type_check) fn get_type_of_fun_dirty(
    state: &mut State,
    fun_id: NodeId<Fun>,
) -> Result<NormalFormId, Tainted<TypeCheckError>> {
    let fun = state.registry.get(fun_id).clone();
    // We call this "param arity" instead of simply "arity"
    // to convey the fact that it does **not** include the recursive
    // function.
    // For example, `fun f(a: A, b: B) -> C { ... }` has param arity 2,
    // even though `f` is also added to the context as a third entry
    // (to enable recursion).
    let param_arity = fun.param_list_id.len();
    let normalized_param_list_id =
        normalize_params_and_leave_params_in_context_dirty(state, fun.param_list_id)??;
    {
        let return_type_type_id = get_type_of_expression_dirty(state, None, fun.return_type_id)?;
        if !is_term_equal_to_type0_or_type1(state, return_type_type_id) {
            return tainted_err(TypeCheckError::ExpectedTermOfTypeType0OrType1 {
                expression_id: fun.return_type_id,
                non_type0_or_type1_type_id: return_type_type_id,
            });
        }
    }
    let normalized_return_type_id = evaluate_well_typed_expression(state, fun.return_type_id);

    let fun_type_id = NormalFormId::unchecked_new(ExpressionId::Forall(
        state
            .registry
            .add_and_overwrite_id(Forall {
                id: dummy_id(),
                span: None,
                param_list_id: normalized_param_list_id,
                output_id: normalized_return_type_id.raw(),
            })
            .without_spans(state.registry),
    ));

    let shifted_fun_type_id = fun_type_id.upshift(param_arity, state.registry);
    state.context.push(ContextEntry {
        type_id: shifted_fun_type_id,
        definition: ContextEntryDefinition::Uninterpreted,
    })?;

    // We need to upshift the return type by one level before comparing it
    // to the body type, to account for the fact that the function has been
    // added to the context.
    let normalized_return_type_id_relative_to_body = {
        let shifted_return_type_id = fun.return_type_id.upshift(1, state.registry);
        evaluate_well_typed_expression(state, shifted_return_type_id)
    };
    // Shadow the old variable to prevent it from being accidentally used.
    #[allow(unused_variables)]
    let normalized_return_type_id = ();

    let normalized_body_type_id = get_type_of_expression_dirty(
        state,
        Some(normalized_return_type_id_relative_to_body),
        fun.body_id,
    )?;

    let equality_status = get_rewritten_term_equality_status(
        state,
        normalized_body_type_id,
        normalized_return_type_id_relative_to_body,
    );

    match equality_status {
        RewrittenTermEqualityStatus::Equal => (),
        RewrittenTermEqualityStatus::Exploded => {
            return tainted_err(TypeCheckError::UnreachableExpression(fun.body_id));
        }
        RewrittenTermEqualityStatus::NotEqual => {
            return tainted_err(TypeCheckError::TypeMismatch {
                expression_id: fun.body_id,
                expected_type_id: normalized_return_type_id_relative_to_body,
                actual_type_id: normalized_body_type_id,
            });
        }
    }

    state.context.pop_n(param_arity + 1);
    Ok(fun_type_id)
}
