use super::*;

impl Accept for UnfinishedParams {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match item {
            FinishedStackItem::Token(token) => match token.kind {
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
                    let is_dashed = self.pending_dash.is_some();
                    let pending_dash = self.pending_dash.take();
                    AcceptResult::Push(UnfinishedStackItem::Param(UnfinishedParam::Name {
                        first_token: pending_dash.unwrap_or(token),
                        is_dashed,
                        name,
                    }))
                }
                TokenKind::Underscore => {
                    let name = Identifier {
                        span: span_single(file_id, &token),
                        name: IdentifierName::Reserved(ReservedIdentifierName::Underscore),
                    };
                    let is_dashed = self.pending_dash.is_some();
                    let pending_dash = self.pending_dash.take();
                    AcceptResult::Push(UnfinishedStackItem::Param(UnfinishedParam::Name {
                        first_token: pending_dash.unwrap_or(token),
                        is_dashed,
                        name,
                    }))
                }
                TokenKind::RParen => {
                    if self.params.is_empty() || self.pending_dash.is_some() {
                        AcceptResult::Error(ParseError::UnexpectedToken(token))
                    } else {
                        AcceptResult::PopAndContinueReducing(FinishedStackItem::Params(
                            self.first_token.clone(),
                            self.params.clone(),
                        ))
                    }
                }
                _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
            },
            FinishedStackItem::Param(_, param, end_delimiter) => {
                self.params.push(param);
                match end_delimiter.raw().kind {
                    TokenKind::Comma => AcceptResult::ContinueToNextToken,
                    TokenKind::RParen => AcceptResult::PopAndContinueReducing(
                        FinishedStackItem::Params(self.first_token.clone(), self.params.clone()),
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
