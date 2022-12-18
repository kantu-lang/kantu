use super::*;

impl Accept for UnfinishedParams {
    fn accept(&mut self, item: FinishedStackItem, _: FileId) -> AcceptResult {
        match item {
            FinishedStackItem::Token(token) => match token.kind {
                TokenKind::Tilde
                | TokenKind::Dash
                | TokenKind::StandardIdentifier
                | TokenKind::Underscore => AcceptResult::PushAndContinueReducingWithNewTop(
                    UnfinishedStackItem::Param(UnfinishedParam::NoIdentifier {
                        // Even if `token.kind == TokenKind::Tilde`, we
                        // do NOT pass in `Some(token)` (and instead pass in `None`).
                        // This is because we're repushing the token onto the stack,
                        // so passing in `Some(token)` would be double counting it.
                        pending_tilde: None,
                        // The same goes for `pending_dash`.
                        pending_dash: None,
                        is_dash_allowed: self.maximum_dashed_params_allowed > 0,
                    }),
                    FinishedStackItem::Token(token),
                ),
                TokenKind::RParen => {
                    if self.pending_dash.is_some() {
                        AcceptResult::Error(ParseError::unexpected_token(token))
                    } else {
                        match NonEmptyVec::try_from(self.params.clone()) {
                            Ok(params) => AcceptResult::PopAndContinueReducing(
                                FinishedStackItem::Params(self.first_token.clone(), params),
                            ),
                            Err(_) => AcceptResult::Error(ParseError::unexpected_token(token)),
                        }
                    }
                }
                _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
            },
            FinishedStackItem::Param(_, param, end_delimiter) => match end_delimiter.raw().kind {
                TokenKind::Comma => {
                    if param.is_dashed {
                        self.maximum_dashed_params_allowed -= 1;
                    }
                    self.params.push(param);
                    AcceptResult::ContinueToNextToken
                }
                TokenKind::RParen => {
                    let params = NonEmptyVec::from_pushed(self.params.clone(), param);
                    AcceptResult::PopAndContinueReducing(FinishedStackItem::Params(
                        self.first_token.clone(),
                        params,
                    ))
                }
                _other_end_delimiter => {
                    AcceptResult::Error(ParseError::unexpected_token(end_delimiter.into_raw()))
                }
            },
            other_item => wrapped_unexpected_finished_item_err(&other_item),
        }
    }
}
