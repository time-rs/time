use std::iter::Peekable;
use std::str::FromStr;

use proc_macro::{
    token_stream, Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree,
};

use crate::Error;

/// Simulate a const block, ensuring that the value will be computed at compile-time.
pub(crate) fn const_block(value: TokenStream, type_: TokenStream) -> TokenStream {
    TokenStream::from(TokenTree::Group(Group::new(
        Delimiter::Brace,
        [
            TokenStream::from(TokenTree::Ident(Ident::new("const", Span::call_site()))),
            TokenStream::from(TokenTree::Ident(Ident::new("VALUE", Span::call_site()))),
            TokenStream::from(TokenTree::Punct(Punct::new(':', Spacing::Alone))),
            type_,
            TokenStream::from(TokenTree::Punct(Punct::new('=', Spacing::Alone))),
            value,
            TokenStream::from(TokenTree::Punct(Punct::new(';', Spacing::Alone))),
            TokenStream::from(TokenTree::Ident(Ident::new("VALUE", Span::call_site()))),
        ]
        .iter()
        .cloned()
        .collect(),
    )))
}

pub(crate) fn get_string_literal(tokens: TokenStream) -> Result<String, Error> {
    let mut tokens = tokens.into_iter();

    match tokens.next() {
        Some(TokenTree::Literal(literal)) => {
            let s = literal.to_string();
            if s.starts_with('"') && s.ends_with('"') {
                tokens
                    .next()
                    .map_or(Ok(s[1..s.len() - 1].to_owned()), |tree| {
                        Err(Error::UnexpectedToken { tree })
                    })
            } else {
                Err(Error::ExpectedString)
            }
        }
        _ => Err(Error::ExpectedString),
    }
}

pub(crate) fn consume_number<T: FromStr>(
    component_name: &'static str,
    chars: &mut Peekable<token_stream::IntoIter>,
) -> Result<T, Error> {
    let digits = match chars.next() {
        Some(TokenTree::Literal(literal)) => literal.to_string(),
        Some(tree) => return Err(Error::UnexpectedToken { tree }),
        None => return Err(Error::UnexpectedEndOfInput),
    };

    if digits.is_empty() {
        Err(Error::MissingComponent {
            name: component_name,
        })
    } else if let Ok(value) = digits.replace('_', "").parse() {
        Ok(value)
    } else {
        Err(Error::InvalidComponent {
            name: component_name,
            value: digits,
        })
    }
}

pub(crate) fn consume_ident(
    s: &str,
    chars: &mut Peekable<token_stream::IntoIter>,
) -> Result<(), Error> {
    match chars.peek() {
        Some(TokenTree::Ident(char)) if s == char.to_string() => {
            drop(chars.next());
            Ok(())
        }
        Some(tree) => Err(Error::UnexpectedToken { tree: tree.clone() }),
        None => Err(Error::UnexpectedEndOfInput),
    }
}

pub(crate) fn consume_punct(
    c: char,
    chars: &mut Peekable<token_stream::IntoIter>,
) -> Result<(), Error> {
    match chars.peek() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == c => {
            drop(chars.next());
            Ok(())
        }
        Some(tree) => Err(Error::UnexpectedToken { tree: tree.clone() }),
        None => Err(Error::UnexpectedEndOfInput),
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
}

fn jan_weekday(year: i32, ordinal: i32) -> u8 {
    let adj_year = year - 1;
    ((ordinal + adj_year + adj_year / 4 - adj_year / 100 + adj_year / 400 + 6).rem_euclid(7)) as _
}

pub(crate) fn days_in_year(year: i32) -> u16 {
    365 + is_leap_year(year) as u16
}

pub(crate) fn days_in_year_month(year: i32, month: u8) -> u8 {
    [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31][month as usize - 1]
        + (month == 2 && is_leap_year(year)) as u8
}

pub(crate) fn weeks_in_year(year: i32) -> u8 {
    52 + (jan_weekday(year, 1) + is_leap_year(year) as u8 == 3) as u8
}

pub(crate) fn ywd_to_yo(year: i32, week: u8, iso_weekday_number: u8) -> (i32, u16) {
    let (ordinal, overflow) = (u16::from(week) * 7 + u16::from(iso_weekday_number))
        .overflowing_sub(u16::from(jan_weekday(year, 4)) + 4);

    if overflow || ordinal == 0 {
        return (year - 1, (ordinal.wrapping_add(days_in_year(year - 1))));
    }

    let days_in_cur_year = days_in_year(year);
    if ordinal > days_in_cur_year {
        (year + 1, ordinal - days_in_cur_year)
    } else {
        (year, ordinal)
    }
}

pub(crate) fn ymd_to_yo(year: i32, month: u8, day: u8) -> (i32, u16) {
    let ordinal = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334][month as usize - 1]
        + (month > 2 && is_leap_year(year)) as u16;

    (year, ordinal + u16::from(day))
}
