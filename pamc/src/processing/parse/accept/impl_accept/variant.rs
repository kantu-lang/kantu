use super::*;

impl Accept for UnfinishedVariant {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedVariant::EmptyString => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Dot => {
                        *self = UnfinishedVariant::Dot(token.clone());
                        AcceptResult::ContinueToNextToken
                    }
                    _ => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
            UnfinishedVariant::Dot(dot) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::StandardIdentifier => {
                        let name = Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::Standard(token.content.clone()),
                        };
                        *self = UnfinishedVariant::Name(dot.clone(), name);
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
            UnfinishedVariant::Name(dot, name) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::LParen => {
                        AcceptResult::Push(UnfinishedStackItem::Params(UnfinishedParams {
                            first_token: token.clone(),
                            maximum_dashed_params_allowed: 0,
                            pending_tilde: None,
                            pending_dash: None,
                            params: vec![],
                        }))
                    }
                    TokenKind::Colon => {
                        *self = UnfinishedVariant::Params(dot.clone(), name.clone(), None);
                        AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                            UnfinishedDelimitedExpression::Empty,
                        ))
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                FinishedStackItem::Params(_, params) => {
                    *self = UnfinishedVariant::Params(dot.clone(), name.clone(), Some(params));
                    AcceptResult::ContinueToNextToken
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
            UnfinishedVariant::Params(dot, name, params) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Colon => {
                        AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                            UnfinishedDelimitedExpression::Empty,
                        ))
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                FinishedStackItem::DelimitedExpression(_, expression, end_delimiter) => {
                    AcceptResult::PopAndContinueReducing(FinishedStackItem::Variant(
                        dot.clone(),
                        Variant {
                            span: span_single(file_id, &dot).inclusive_merge(expression.span()),
                            name: name.clone(),
                            params: params.clone(),
                            return_type: expression,
                        },
                        end_delimiter,
                    ))
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
        }
    }
}
