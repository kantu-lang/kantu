use super::*;

impl Accept for UnfinishedParams {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match item {
            FinishedStackItem::Token(token) => match token.kind {
                TokenKind::Tilde => {
                    if self.pending_dash.is_some() {
                        // A tilde can never come after a dash.
                        AcceptResult::Error(ParseError::UnexpectedToken(token))
                    } else if self.pending_tilde.is_some() {
                        // Double tildes are forbidden.
                        AcceptResult::Error(ParseError::UnexpectedToken(token))
                    } else {
                        self.pending_tilde = Some(token);
                        AcceptResult::ContinueToNextToken
                    }
                }
                TokenKind::Dash => {
                    if self.maximum_dashed_params_allowed == 0 || self.pending_dash.is_some() {
                        AcceptResult::Error(ParseError::UnexpectedToken(token))
                    } else {
                        self.maximum_dashed_params_allowed -= 1;
                        self.pending_dash = Some(token);
                        AcceptResult::ContinueToNextToken
                    }
                }
                TokenKind::StandardIdentifier => {
                    let name = Identifier {
                        span: span_single(file_id, &token),
                        name: IdentifierName::Standard(token.content.clone()),
                    };
                    let is_tilded = self.pending_tilde.is_some();
                    let is_dashed = self.pending_dash.is_some();
                    let pending_dash = self.pending_dash.take();
                    AcceptResult::Push(UnfinishedStackItem::Param(
                        UnfinishedParam::NoExplicitLabel {
                            first_token: pending_dash.unwrap_or(token),
                            is_tilded,
                            is_dashed,
                            is_dash_allowed: self.maximum_dashed_params_allowed > 0 || is_dashed,
                            name_or_label: name,
                        },
                    ))
                }
                TokenKind::Underscore => {
                    let name = Identifier {
                        span: span_single(file_id, &token),
                        name: IdentifierName::Reserved(ReservedIdentifierName::Underscore),
                    };
                    let is_tilded = self.pending_tilde.is_some();
                    let is_dashed = self.pending_dash.is_some();
                    let pending_dash = self.pending_dash.take();
                    AcceptResult::Push(UnfinishedStackItem::Param(
                        UnfinishedParam::NoExplicitLabel {
                            first_token: pending_dash.unwrap_or(token),
                            is_tilded,
                            is_dashed,
                            is_dash_allowed: self.maximum_dashed_params_allowed > 0 || is_dashed,
                            name_or_label: name,
                        },
                    ))
                }
                TokenKind::RParen => {
                    if self.pending_dash.is_some() {
                        AcceptResult::Error(ParseError::UnexpectedToken(token))
                    } else {
                        match NonEmptyVec::try_from(self.params.clone()) {
                            Ok(params) => AcceptResult::PopAndContinueReducing(
                                FinishedStackItem::Params(self.first_token.clone(), params),
                            ),
                            Err(_) => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                        }
                    }
                }
                _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
            },
            FinishedStackItem::Param(_, param, end_delimiter) => {
                let params = NonEmptyVec::from_pushed(self.params.clone(), param);
                self.params = params.to_vec();
                match end_delimiter.raw().kind {
                    TokenKind::Comma => AcceptResult::ContinueToNextToken,
                    TokenKind::RParen => AcceptResult::PopAndContinueReducing(
                        FinishedStackItem::Params(self.first_token.clone(), params),
                    ),
                    _other_end_delimiter => {
                        AcceptResult::Error(ParseError::UnexpectedToken(end_delimiter.into_raw()))
                    }
                }
            }
            other_item => unexpected_finished_item(&other_item),
        }
    }
}
