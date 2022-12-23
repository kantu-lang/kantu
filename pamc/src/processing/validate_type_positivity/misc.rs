use super::*;

pub fn dummy_id<T>() -> NodeId<T> {
    NodeId::new(0)
}

pub fn get_possibly_empty_param_type_ids(
    registry: &NodeRegistry,
    id: Option<NonEmptyParamListId>,
) -> Vec<ExpressionId> {
    id.map(|id| get_param_type_ids(registry, id).into())
        .unwrap_or_else(|| vec![])
}

pub fn get_param_type_ids(
    registry: &NodeRegistry,
    id: NonEmptyParamListId,
) -> NonEmptyVec<ExpressionId> {
    match id {
        NonEmptyParamListId::Unlabeled(id) => get_unlabeled_param_ids(registry, id),
        NonEmptyParamListId::UniquelyLabeled(id) => get_labeled_param_ids(registry, id),
    }
}

pub fn get_unlabeled_param_ids(
    registry: &NodeRegistry,
    id: NonEmptyListId<NodeId<UnlabeledParam>>,
) -> NonEmptyVec<ExpressionId> {
    registry.get_list(id).to_mapped(|&param_id| {
        let param = registry.get(param_id);
        param.type_id
    })
}

pub fn get_labeled_param_ids(
    registry: &NodeRegistry,
    id: NonEmptyListId<NodeId<LabeledParam>>,
) -> NonEmptyVec<ExpressionId> {
    registry.get_list(id).to_mapped(|&param_id| {
        let param = registry.get(param_id);
        param.type_id
    })
}
