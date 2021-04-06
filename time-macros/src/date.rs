use std::iter::Peekable;
use std::str::Chars;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
#[allow(unused_imports)]
use standback::prelude::*;

use crate::helpers::{
    self, consume_char, consume_digits, days_in_year, days_in_year_month, weeks_in_year, ymd_to_yo,
    ywd_to_yo,
};
use crate::{Error, ToTokens};

#[cfg(feature = "large-dates")]
const MAX_YEAR: i32 = 999_999;
#[cfg(not(feature = "large-dates"))]
const MAX_YEAR: i32 = 9_999;

#[derive(Clone, Copy)]
pub(crate) struct Date {
    pub(crate) year: i32,
    pub(crate) ordinal: u16,
}

impl Date {
    pub(crate) fn parse(chars: &mut Peekable<Chars<'_>>) -> Result<Self, Error> {
        let (year_sign, explicit_sign) = if chars.next_if_eq(&'-').is_some() {
            (-1, true)
        } else {
            (1, chars.next_if_eq(&'+').is_some())
        };
        let year = year_sign * consume_digits::<i32>("year", chars)?;
        if year.abs() > MAX_YEAR {
            return Err(Error::InvalidComponent {
                name: "year",
                value: year.to_string(),
            });
        }
        if !explicit_sign && year.abs() >= 10_000 {
            return Err(Error::Custom(
                "years with more than four digits must have an explicit sign".into(),
            ));
        }

        consume_char('-', chars)?;

        // year-week-day
        if chars.next_if_eq(&'W').is_some() {
            let week = consume_digits::<u8>("week", chars)?;
            consume_char('-', chars)?;
            let day = consume_digits::<u8>("day", chars)?;

            if week > weeks_in_year(year) {
                return Err(Error::InvalidComponent {
                    name: "week",
                    value: week.to_string(),
                });
            }
            if day == 0 || day > 7 {
                return Err(Error::InvalidComponent {
                    name: "day",
                    value: day.to_string(),
                });
            }

            let (year, ordinal) = ywd_to_yo(year, week, day);

            return Ok(Self { year, ordinal });
        }

        // We don't yet know whether it's year-month-day or year-ordinal.
        let month_or_ordinal = consume_digits::<u16>("month or ordinal", chars)?;

        // year-month-day
        if chars.next_if_eq(&'-').is_some() {
            let month = month_or_ordinal;
            let day = consume_digits::<u8>("day", chars)?;

            if month == 0 || month > 12 {
                return Err(Error::InvalidComponent {
                    name: "month",
                    value: month.to_string(),
                });
            }
            let month = month as _;
            if day == 0 || day > days_in_year_month(year, month) {
                return Err(Error::InvalidComponent {
                    name: "day",
                    value: day.to_string(),
                });
            }

            let (year, ordinal) = ymd_to_yo(year, month, day);

            Ok(Self { year, ordinal })
        }
        // year-ordinal
        else {
            let ordinal = month_or_ordinal;

            if ordinal == 0 || ordinal > days_in_year(year) {
                return Err(Error::InvalidComponent {
                    name: "ordinal",
                    value: ordinal.to_string(),
                });
            }

            Ok(Self { year, ordinal })
        }
    }
}

impl ToTokens for Date {
    fn to_internal_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(
            [
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("time", Span::call_site())),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("Date", Span::call_site())),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new(
                    "__from_ordinal_date_unchecked",
                    Span::call_site(),
                )),
                TokenTree::Group(Group::new(
                    Delimiter::Parenthesis,
                    [
                        TokenTree::Literal(Literal::i32_unsuffixed(self.year)),
                        TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                        TokenTree::Literal(Literal::u16_unsuffixed(self.ordinal)),
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

    fn to_external_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(helpers::const_block(
            self.to_internal_token_stream(),
            [
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("time", Span::call_site())),
                TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Ident(Ident::new("Date", Span::call_site())),
            ]
            .iter()
            .cloned()
            .collect(),
        ));
    }
}
