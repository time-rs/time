use crate::{peeking_take_while::PeekableExt, Error};
use proc_macro::{TokenStream, TokenTree};
#[allow(unused_imports)]
use standback::prelude::*; // rem_euclid (1.38)
use std::{
    iter::Peekable,
    str::{Chars, FromStr},
};

pub(crate) fn get_string_literal(tokens: TokenStream) -> Result<String, Error> {
    let mut tokens = tokens.into_iter();

    match tokens.next() {
        Some(TokenTree::Literal(literal)) => {
            let s = literal.to_string();
            if !s.starts_with('"') || !s.ends_with('"') {
                Err(Error::ExpectedString)
            } else if let Some(tree) = tokens.next() {
                Err(Error::UnexpectedToken { tree })
            } else {
                Ok(s[1..(s.len() - 1)].to_owned())
            }
        }
        _ => Err(Error::ExpectedString),
    }
}

pub(crate) fn consume_digits<T: FromStr>(
    component_name: &'static str,
    chars: &mut Peekable<Chars<'_>>,
) -> Result<T, Error> {
    Ok(consume_digits_with_length(component_name, chars)?.0)
}

pub(crate) fn consume_digits_with_length<T: FromStr>(
    component_name: &'static str,
    chars: &mut Peekable<Chars<'_>>,
) -> Result<(T, usize), Error> {
    let digits = chars
        .peeking_take_while(|&c| c.is_ascii_digit() || c == '_')
        .collect::<String>();

    // Internal underscores are allowed.
    if digits.starts_with('_') || digits.ends_with('_') {
        return Err(Error::UnexpectedCharacter('_'));
    }
    let digits = digits.replace('_', "");

    let num_digits = digits.len();

    if digits == "" {
        Err(Error::MissingComponent {
            name: component_name,
        })
    } else {
        match digits.parse() {
            Ok(value) => Ok((value, num_digits)),
            Err(_) => Err(Error::InvalidComponent {
                name: component_name,
                value: digits,
            }),
        }
    }
}

pub(crate) fn consume_char(c: char, chars: &mut Peekable<Chars<'_>>) -> Result<(), Error> {
    match chars.peek() {
        Some(&char) if c == char => {
            let _ = chars.next();
            Ok(())
        }
        Some(&char) => Err(Error::UnexpectedCharacter(char)),
        None => Err(Error::UnexpectedEndOfInput),
    }
}

pub(crate) fn consume_str(s: &str, chars: &mut Peekable<Chars<'_>>) -> Result<(), Error> {
    // If the first character matches, but additional characters don't, we need
    // to be able to reset the iterator to its original state. If this isn't
    // done, a failure would have side effects, which is undesirable.
    let old_chars = chars.clone();

    for c1 in s.chars() {
        match chars.peek() {
            Some(&c2) if c1 == c2 => {
                let _ = chars.next();
            }
            Some(&char) => {
                *chars = old_chars;
                return Err(Error::UnexpectedCharacter(char));
            }
            None => {
                *chars = old_chars;
                return Err(Error::UnexpectedEndOfInput);
            }
        }
    }

    Ok(())
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
}

#[allow(unstable_name_collisions)]
fn jan_weekday(year: i32, ordinal: i32) -> u8 {
    let adj_year = year - 1;
    ((ordinal + adj_year + adj_year / 4 - adj_year / 100 + adj_year / 400 + 6).rem_euclid(7)) as u8
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
