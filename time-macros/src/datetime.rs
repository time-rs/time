use crate::{
    error::Error,
    helpers::{self, consume_char},
    Date, Offset, Time, ToTokens,
};
use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use std::{iter::Peekable, str::Chars};

#[derive(Clone, Copy)]
pub(crate) struct DateTime {
    date: Date,
    time: Time,
    offset: Option<Offset>,
}

impl DateTime {
    pub(crate) fn parse(chars: &mut Peekable<Chars<'_>>) -> Result<Self, Error> {
        let date = Date::parse(chars)?;
        consume_char(' ', chars)?;
        let time = Time::parse(chars)?;

        let offset = if chars.peek() == Some(&' ') {
            consume_char(' ', chars)?;
            Some(Offset::parse(chars)?)
        } else {
            None
        };

        if let Some(&char) = chars.peek() {
            return Err(Error::UnexpectedCharacter(char));
        }

        Ok(Self { date, time, offset })
    }
}

impl ToTokens for DateTime {
    fn to_internal_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(
            [
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
                        self.date.to_internal_token_stream(),
                        TokenTree::Punct(Punct::new(',', Spacing::Alone)).into(),
                        self.time.to_internal_token_stream(),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                )),
            ]
            .iter()
            .cloned()
            .collect::<TokenStream>(),
        );

        if let Some(offset) = self.offset {
            tokens.extend(
                [
                    TokenTree::Punct(Punct::new('.', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("assume_offset", Span::call_site())),
                    TokenTree::Group(Group::new(
                        Delimiter::Parenthesis,
                        offset.to_internal_token_stream(),
                    )),
                ]
                .iter()
                .cloned()
                .collect::<TokenStream>(),
            );
        }
    }

    fn to_external_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(helpers::const_block(
            self.to_internal_token_stream(),
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
