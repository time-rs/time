use std::iter::Peekable;

use proc_macro::{
    token_stream, Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree,
};

use crate::error::Error;
use crate::{helpers, Date, Offset, Time, ToTokens};

pub(crate) struct DateTime {
    date: Date,
    time: Time,
    offset: Option<Offset>,
}

impl DateTime {
    pub(crate) fn parse(chars: &mut Peekable<token_stream::IntoIter>) -> Result<Self, Error> {
        let date = Date::parse(chars)?;
        let time = Time::parse(chars)?;
        let offset = match Offset::parse(chars) {
            Ok(offset) => Some(offset),
            Err(Error::UnexpectedEndOfInput)
            | Err(Error::MissingComponent { name: "sign", .. }) => None,
            Err(err) => return Err(err),
        };

        if let Some(token) = chars.peek() {
            return Err(Error::UnexpectedToken {
                tree: token.clone(),
            });
        }

        Ok(Self { date, time, offset })
    }
}

impl ToTokens for DateTime {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(helpers::const_block(
            {
                let mut tokens = [
                    TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                    TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("time", Span::call_site())),
                    TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                    TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("PrimitiveDateTime", Span::call_site())),
                    TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                    TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("new", Span::call_site())),
                    TokenTree::Group(Group::new(
                        Delimiter::Parenthesis,
                        [
                            self.date.to_token_stream(),
                            TokenTree::Punct(Punct::new(',', Spacing::Alone)).into(),
                            self.time.to_token_stream(),
                        ]
                        .iter()
                        .cloned()
                        .collect(),
                    )),
                ]
                .iter()
                .cloned()
                .collect::<TokenStream>();

                if let Some(offset) = self.offset {
                    tokens.extend(
                        [
                            TokenTree::Punct(Punct::new('.', Spacing::Alone)),
                            TokenTree::Ident(Ident::new("assume_offset", Span::call_site())),
                            TokenTree::Group(Group::new(
                                Delimiter::Parenthesis,
                                offset.to_token_stream(),
                            )),
                        ]
                        .iter()
                        .cloned()
                        .collect::<TokenStream>(),
                    );
                }

                tokens
            },
            [
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("time", Span::call_site())),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new(
                    match self.offset {
                        Some(_) => "OffsetDateTime",
                        None => "PrimitiveDateTime",
                    },
                    Span::call_site(),
                )),
            ]
            .iter()
            .cloned()
            .collect(),
        ));
    }
}
