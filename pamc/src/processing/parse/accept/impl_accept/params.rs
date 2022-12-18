use super::*;

impl Accept for UnfinishedParams {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match item {
            FinishedStackItem::Token(token) => match token.kind {
                // TODO: Delete
                // TokenKind::Tilde => {
                //     if self.pending_dash.is_some() {
                //         // A tilde can never come after a dash.
                //         AcceptResult::Error(ParseError::unexpected_token(token))
                //     } else if self.pending_tilde.is_some() {
                //         // Double tildes are forbidden.
                //         AcceptResult::Error(ParseError::unexpected_token(token))
                //     } else {
                //         self.pending_tilde = Some(token);
                //         AcceptResult::ContinueToNextToken
                //     }
                // }
                // TokenKind::Dash => {
                //     if self.maximum_dashed_params_allowed == 0 || self.pending_dash.is_some() {
                //         AcceptResult::Error(ParseError::unexpected_token(token))
                //     } else {
                //         self.maximum_dashed_params_allowed -= 1;
                //         self.pending_dash = Some(token);
                //         AcceptResult::ContinueToNextToken
                //     }
                // }
                // TokenKind::StandardIdentifier => {
                //     let name_or_label = Identifier {
                //         span: span_single(file_id, &token),
                //         name: IdentifierName::Standard(token.content.clone()),
                //     };
                //     self.push_unfinished_param(name_or_label, token)
                // }
                // TokenKind::Underscore => {
                //     let name_or_label = Identifier {
                //         span: span_single(file_id, &token),
                //         name: IdentifierName::Reserved(ReservedIdentifierName::Underscore),
                //     };
                //     self.push_unfinished_param(name_or_label, token)
                // }
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

// TODO: Delete
// impl UnfinishedParams {
//     fn push_unfinished_param(
//         &mut self,
//         name_or_label: Identifier,
//         name_or_label_token: Token,
//     ) -> AcceptResult {
//         let pending_tilde = self.pending_tilde.take();
//         let pending_dash = self.pending_dash.take();
//         let is_tilded = pending_tilde.is_some();
//         let is_dashed = pending_dash.is_some();
//         AcceptResult::Push(UnfinishedStackItem::Param(
//             UnfinishedParam::FirstIdentifier {
//                 first_token: pending_tilde
//                     .unwrap_or_else(|| pending_dash.unwrap_or_else(|| name_or_label_token)),
//                 is_tilded,
//                 is_dashed,
//                 is_dash_allowed: self.maximum_dashed_params_allowed > 0 || is_dashed,
//                 name_or_label,
//             },
//         ))
//     }
// }
