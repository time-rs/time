use crate::{
    helpers::{consume_char, consume_digits, consume_digits_with_length, consume_str},
    Error, ToTokens,
};
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use std::{cmp::Ordering, iter::Peekable, str::Chars};

enum Period {
    Am,
    Pm,
    _24,
}

pub(crate) struct Time {
    hour: u8,
    minute: u8,
    second: u8,
    nanosecond: u32,
}

impl Time {
    pub(crate) fn parse(chars: &mut Peekable<Chars<'_>>) -> Result<Self, Error> {
        let hour = consume_digits("hour", chars)?;
        consume_char(':', chars)?;
        let minute = consume_digits::<u8>("minute", chars)?;
        let mut second = 0;
        let mut nanosecond = 0;

        if consume_char(':', chars).is_ok() {
            second = consume_digits("second", chars)?;

            if consume_char('.', chars).is_ok() {
                let (raw_nanosecond, num_digits) =
                    consume_digits_with_length::<u32>("nanosecond", chars)?;

                nanosecond = match num_digits.cmp(&9) {
                    Ordering::Less => raw_nanosecond * 10_u32.pow(9 - num_digits as u32),
                    Ordering::Equal => raw_nanosecond,
                    Ordering::Greater => {
                        return Err(Error::InvalidComponent {
                            name: "nanosecond",
                            value: raw_nanosecond.to_string(),
                        })
                    }
                }
            }
        }

        let period = if consume_str(" am", chars).is_ok() || consume_str(" AM", chars).is_ok() {
            Period::Am
        } else if consume_str(" pm", chars).is_ok() || consume_str(" pm", chars).is_ok() {
            Period::Pm
        } else {
            Period::_24
        };

        let hour = match (hour, period) {
            (12, Period::Am) => 0,
            (12, Period::Pm) => 12,
            (hour, Period::Am) | (hour, Period::_24) => hour,
            (hour, Period::Pm) => hour + 12,
        };

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
                hour,
                minute,
                second,
                nanosecond,
            })
        }
    }
}

impl ToTokens for Time {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(
            [
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("time", Span::call_site())),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("Time", Span::call_site())),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("from_hms_nanos_unchecked", Span::call_site())),
                TokenTree::Group(Group::new(
                    Delimiter::Parenthesis,
                    [
                        TokenTree::Literal(Literal::u8_unsuffixed(self.hour)),
                        TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                        TokenTree::Literal(Literal::u8_unsuffixed(self.minute)),
                        TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                        TokenTree::Literal(Literal::u8_unsuffixed(self.second)),
                        TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                        TokenTree::Literal(Literal::u32_unsuffixed(self.nanosecond)),
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
