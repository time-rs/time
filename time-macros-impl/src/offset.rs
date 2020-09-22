use crate::{
    helpers::{consume_char, consume_digits, consume_str},
    Error, ToTokens,
};
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use std::{iter::Peekable, str::Chars};

pub(crate) struct Offset {
    seconds: i32,
}

impl Offset {
    pub(crate) fn parse(chars: &mut Peekable<Chars<'_>>) -> Result<Self, Error> {
        if consume_str("utc", chars).is_ok() || consume_str("UTC", chars).is_ok() {
            return Ok(Self { seconds: 0 });
        }

        let sign = match chars.next() {
            Some('+') => 1,
            Some('-') => -1,
            Some(char) => return Err(Error::UnexpectedCharacter(char)),
            None => return Err(Error::MissingComponent { name: "sign" }),
        };

        let hour = consume_digits::<i32>("hour", chars)?;
        let mut minute = 0;
        let mut second = 0;

        if consume_char(':', chars).is_ok() {
            minute = consume_digits::<i32>("minute", chars)?;

            if consume_char(':', chars).is_ok() {
                second = consume_digits::<i32>("second", chars)?;
            }
        }

        if hour >= 24 {
            Err(Error::InvalidComponent {
                name: "hour",
                value: hour.to_string(),
            })
        } else if minute >= 60 {
            Err(Error::InvalidComponent {
                name: "minute",
                value: minute.to_string(),
            })
        } else if second >= 60 {
            Err(Error::InvalidComponent {
                name: "second",
                value: second.to_string(),
            })
        } else {
            Ok(Self {
                seconds: sign * (hour * 3_600 + minute * 60 + second),
            })
        }
    }
}

impl ToTokens for Offset {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(
            [
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("time", Span::call_site())),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("UtcOffset", Span::call_site())),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("seconds_unchecked", Span::call_site())),
                TokenTree::Group(Group::new(
                    Delimiter::Parenthesis,
                    TokenStream::from(TokenTree::Literal(Literal::i32_unsuffixed(self.seconds))),
                )),
            ]
            .iter()
            .cloned()
            .collect::<TokenStream>(),
        )
    }
}
