use crate::{error::Error, helpers::consume_char, Date, Time, ToTokens};
use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use std::{iter::Peekable, str::Chars};

pub(crate) struct DateTime {
    date: Date,
    time: Time,
}

impl DateTime {
    pub(crate) fn parse(chars: &mut Peekable<Chars<'_>>) -> Result<Self, Error> {
        let date = match Date::parse(chars) {
            Ok(result) => result,
            Err(err) => return Err(err),
        };

        consume_char(' ', chars)?;

        let time = match Time::parse(chars) {
            Ok(result) => result,
            Err(err) => return Err(err),
        };

        if let Some(&char) = chars.peek() {
            return Err(Error::UnexpectedCharacter(char));
        }

        Ok(Self { date, time })
    }
}

impl ToTokens for DateTime {
    fn to_tokens(&self, tokens: &mut TokenStream) {
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
            .collect::<TokenStream>(),
        )
    }
}
