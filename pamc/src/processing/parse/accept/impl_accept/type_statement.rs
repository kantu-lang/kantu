use super::*;

impl Accept for UnfinishedTypeStatement {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedTypeStatement::Keyword(type_kw) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::StandardIdentifier => {
                        let name = Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::Standard(token.content.clone()),
                        };
                        *self = UnfinishedTypeStatement::Name(type_kw.clone(), name);
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                other_item => unexpected_finished_item(&other_item),
            },
            UnfinishedTypeStatement::Name(type_kw, name) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::LParen => {
                        AcceptResult::Push(UnfinishedStackItem::Params(UnfinishedParams {
                            first_token: token,
                            maximum_dashed_params_allowed: 0,
                            pending_tilde: None,
                            pending_dash: None,
                            params: vec![],
                        }))
                    }
                    TokenKind::LCurly => {
                        *self = UnfinishedTypeStatement::Variants(
                            type_kw.clone(),
                            name.clone(),
                            None,
                            vec![],
                        );
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                FinishedStackItem::Params(_, params) => {
                    *self = UnfinishedTypeStatement::Params(
                        type_kw.clone(),
                        name.clone(),
                        Some(params),
                    );
                    AcceptResult::ContinueToNextToken
                }
                other_item => unexpected_finished_item(&other_item),
            },
            UnfinishedTypeStatement::Params(type_kw, name, params) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::LCurly => {
                        *self = UnfinishedTypeStatement::Variants(
                            type_kw.clone(),
                            name.clone(),
                            params.clone(),
                            vec![],
                        );
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                other_item => unexpected_finished_item(&other_item),
            },
            UnfinishedTypeStatement::Variants(type_kw, name, params, variants) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Dot => AcceptResult::Push(UnfinishedStackItem::Variant(
                        UnfinishedVariant::Dot(token),
                    )),
                    TokenKind::RCurly => {
                        AcceptResult::PopAndContinueReducing(FinishedStackItem::Type(
                            type_kw.clone(),
                            TypeStatement {
                                span: span_range_including_end(file_id, &type_kw, &token),
                                name: name.clone(),
                                params: params.clone(),
                                variants: variants.clone(),
                            },
                        ))
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                FinishedStackItem::Variant(_, variant, end_delimiter) => {
                    variants.push(variant);
                    match end_delimiter.raw().kind {
                        TokenKind::Comma => AcceptResult::ContinueToNextToken,
                        TokenKind::RCurly => {
                            AcceptResult::PopAndContinueReducing(FinishedStackItem::Type(
                                type_kw.clone(),
                                TypeStatement {
                                    span: span_range_including_end(
                                        file_id,
                                        &type_kw,
                                        end_delimiter.raw(),
                                    ),
                                    name: name.clone(),
                                    params: params.clone(),
                                    variants: variants.clone(),
                                },
                            ))
                        }
                        _other_end_delimiter => AcceptResult::Error(ParseError::UnexpectedToken(
                            end_delimiter.into_raw(),
                        )),
                    }
                }
                other_item => unexpected_finished_item(&other_item),
            },
        }
    }
}
