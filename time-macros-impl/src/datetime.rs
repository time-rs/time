use crate::{
    error::Error,
    helpers::{self, consume_char, days_in_year},
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
        let date = match Date::parse(chars) {
            Ok(result) => result,
            Err(err) => return Err(err),
        };

        consume_char(' ', chars)?;

        let time = match Time::parse(chars) {
            Ok(result) => result,
            Err(err) => return Err(err),
        };

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

    fn utc_date_time(&self) -> (Date, Time) {
        let offset = match self.offset {
            Some(offset) => offset,
            None => return (self.date, self.time),
        };

        let mut second = self.time.second as i8 - (offset.seconds % 60) as i8;
        let mut minute = self.time.minute as i8 - (offset.seconds / 60 % 60) as i8;
        let mut hour = self.time.hour as i8 - (offset.seconds / 3_600) as i8;

        let mut ordinal = self.date.ordinal;
        let mut year = self.date.year;

        if second >= 60 {
            second -= 60;
            minute += 1;
        } else if second < 0 {
            second += 60;
            minute -= 1;
        }
        if minute >= 60 {
            minute -= 60;
            hour += 1;
        } else if minute < 0 {
            minute += 60;
            hour -= 1;
        }
        if hour >= 24 {
            hour -= 24;
            ordinal += 1;
        } else if hour < 0 {
            hour += 24;
            ordinal -= 1;
        }
        if ordinal > days_in_year(year) {
            year += 1;
            ordinal = 1;
        } else if ordinal == 0 {
            year -= 1;
            ordinal = days_in_year(year);
        }

        (
            Date { year, ordinal },
            Time {
                hour: hour as u8,
                minute: minute as u8,
                second: second as u8,
                nanosecond: self.time.nanosecond,
            },
        )
    }
}

impl ToTokens for DateTime {
    fn to_internal_tokens(&self, tokens: &mut TokenStream) {
        let (utc_date, utc_time) = self.utc_date_time();

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
                        utc_date.to_internal_token_stream(),
                        TokenTree::Punct(Punct::new(',', Spacing::Alone)).into(),
                        utc_time.to_internal_token_stream(),
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
                    TokenTree::Ident(Ident::new("assume_utc", Span::call_site())),
                    TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
                    TokenTree::Punct(Punct::new('.', Spacing::Alone)),
                    TokenTree::Ident(Ident::new("to_offset", Span::call_site())),
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
