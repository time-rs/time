use std::io::prelude::*;
use super::{Tm, ParseError};
use std::io::{Cursor, SeekFrom};

use ParseError::*;
use super::NSEC_PER_SEC;

/// Parses the time from the string according to the format string.
pub fn strptime(s: &str, format: &str) -> Result<Tm, ParseError> {
    let mut rdr = Cursor::new(format.as_bytes());
    let mut tm = Tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_utcoff: 0,
        tm_nsec: 0,
    };
    let mut pos = 0;
    let len = s.len();
    let mut result = Err(InvalidTime);

    while pos < len {
        let range = s.char_range_at(pos);
        let ch = range.ch;
        let next = range.next;

        let mut buf = &mut [0];
        let c = match rdr.read(buf) {
            Ok(..) => buf[0] as char,
            Err(..) => break
        };
        match c {
            '%' => {
                let ch = match rdr.read(buf) {
                    Ok(..) => buf[0] as char,
                    Err(..) => break
                };
                match parse_type(s, pos, ch, &mut tm) {
                    Ok(next) => pos = next,
                    Err(e) => { result = Err(e); break; }
                }
            },
            c => {
                if c != ch { break }
                pos = next;
            }
        }
    }

    if pos == len && rdr.seek(SeekFrom::Current(0)).unwrap() == format.len() as u64 {
        Ok(Tm {
            tm_sec: tm.tm_sec,
            tm_min: tm.tm_min,
            tm_hour: tm.tm_hour,
            tm_mday: tm.tm_mday,
            tm_mon: tm.tm_mon,
            tm_year: tm.tm_year,
            tm_wday: tm.tm_wday,
            tm_yday: tm.tm_yday,
            tm_isdst: tm.tm_isdst,
            tm_utcoff: tm.tm_utcoff,
            tm_nsec: tm.tm_nsec,
        })
    } else { result }
}

fn match_str(s: &str, pos: usize, needle: &str) -> bool {
    s[pos..].starts_with(needle)
}

fn match_strs(ss: &str, pos: usize, strs: &[(&str, i32)])
  -> Option<(i32, usize)> {
    for &(needle, value) in strs.iter() {
        if match_str(ss, pos, needle) {
            return Some((value, pos + needle.len()));
        }
    }

    None
}

fn match_digits(ss: &str, pos: usize, digits: usize, ws: bool)
  -> Option<(i32, usize)> {
    let mut pos = pos;
    let len = ss.len();
    let mut value = 0;

    let mut i = 0;
    while i < digits {
        if pos >= len {
            return None;
        }
        let range = ss.char_range_at(pos);
        pos = range.next;

        match range.ch {
          '0' ... '9' => {
            value = value * 10 + (range.ch as i32 - '0' as i32);
          }
          ' ' if ws => (),
          _ => return None
        }
        i += 1;
    }

    Some((value, pos))
}

fn match_fractional_seconds(ss: &str, pos: usize) -> (i32, usize) {
    let len = ss.len();
    let mut value = 0;
    let mut multiplier = NSEC_PER_SEC / 10;
    let mut pos = pos;

    while pos < len {
        let range = ss.char_range_at(pos);

        match range.ch {
            '0' ... '9' => {
                pos = range.next;
                // This will drop digits after the nanoseconds place
                let digit = range.ch as i32 - '0' as i32;
                value += digit * multiplier;
                multiplier /= 10;
            }
            _ => break
        }
    }

    (value, pos)
}

fn match_digits_in_range(ss: &str, pos: usize, digits: usize, ws: bool,
                         min: i32, max: i32) -> Option<(i32, usize)> {
    match match_digits(ss, pos, digits, ws) {
      Some((val, pos)) if val >= min && val <= max => {
        Some((val, pos))
      }
      _ => None
    }
}

fn parse_char(s: &str, pos: usize, c: char) -> Result<usize, ParseError> {
    let range = s.char_range_at(pos);

    if c == range.ch {
        Ok(range.next)
    } else {
        Err(UnexpectedCharacter(c, range.ch))
    }
}

fn parse_type(s: &str, pos: usize, ch: char, tm: &mut Tm)
  -> Result<usize, ParseError> {
    match ch {
      'A' => match match_strs(s, pos, &[
          ("Sunday", 0),
          ("Monday", 1),
          ("Tuesday", 2),
          ("Wednesday", 3),
          ("Thursday", 4),
          ("Friday", 5),
          ("Saturday", 6)
        ]) {
              Some(item) => { let (v, pos) = item; tm.tm_wday = v; Ok(pos) }
              None => Err(InvalidDay)
      },
      'a' => match match_strs(s, pos, &[
          ("Sun", 0),
          ("Mon", 1),
          ("Tue", 2),
          ("Wed", 3),
          ("Thu", 4),
          ("Fri", 5),
          ("Sat", 6)
      ]) {
        Some(item) => { let (v, pos) = item; tm.tm_wday = v; Ok(pos) }
        None => Err(InvalidDay)
      },
      'B' => match match_strs(s, pos, &[
          ("January", 0),
          ("February", 1),
          ("March", 2),
          ("April", 3),
          ("May", 4),
          ("June", 5),
          ("July", 6),
          ("August", 7),
          ("September", 8),
          ("October", 9),
          ("November", 10),
          ("December", 11)
      ]) {
        Some(item) => { let (v, pos) = item; tm.tm_mon = v; Ok(pos) }
        None => Err(InvalidMonth)
      },
      'b' | 'h' => match match_strs(s, pos, &[
          ("Jan", 0),
          ("Feb", 1),
          ("Mar", 2),
          ("Apr", 3),
          ("May", 4),
          ("Jun", 5),
          ("Jul", 6),
          ("Aug", 7),
          ("Sep", 8),
          ("Oct", 9),
          ("Nov", 10),
          ("Dec", 11)
      ]) {
        Some(item) => { let (v, pos) = item; tm.tm_mon = v; Ok(pos) }
        None => Err(InvalidMonth)
      },
      'C' => match match_digits_in_range(s, pos, 2, false, 0,
                                         99) {
        Some(item) => {
            let (v, pos) = item;
              tm.tm_year += (v * 100) - 1900;
              Ok(pos)
          }
        None => Err(InvalidYear)
      },
      'c' => {
        parse_type(s, pos, 'a', &mut *tm)
            .and_then(|pos| parse_char(s, pos, ' '))
            .and_then(|pos| parse_type(s, pos, 'b', &mut *tm))
            .and_then(|pos| parse_char(s, pos, ' '))
            .and_then(|pos| parse_type(s, pos, 'e', &mut *tm))
            .and_then(|pos| parse_char(s, pos, ' '))
            .and_then(|pos| parse_type(s, pos, 'T', &mut *tm))
            .and_then(|pos| parse_char(s, pos, ' '))
            .and_then(|pos| parse_type(s, pos, 'Y', &mut *tm))
      }
      'D' | 'x' => {
        parse_type(s, pos, 'm', &mut *tm)
            .and_then(|pos| parse_char(s, pos, '/'))
            .and_then(|pos| parse_type(s, pos, 'd', &mut *tm))
            .and_then(|pos| parse_char(s, pos, '/'))
            .and_then(|pos| parse_type(s, pos, 'y', &mut *tm))
      }
      'd' => match match_digits_in_range(s, pos, 2, false, 1,
                                         31) {
        Some(item) => { let (v, pos) = item; tm.tm_mday = v; Ok(pos) }
        None => Err(InvalidDayOfMonth)
      },
      'e' => match match_digits_in_range(s, pos, 2, true, 1,
                                         31) {
        Some(item) => { let (v, pos) = item; tm.tm_mday = v; Ok(pos) }
        None => Err(InvalidDayOfMonth)
      },
      'f' => {
        let (val, pos) = match_fractional_seconds(s, pos);
        tm.tm_nsec = val;
        Ok(pos)
      }
      'F' => {
        parse_type(s, pos, 'Y', &mut *tm)
            .and_then(|pos| parse_char(s, pos, '-'))
            .and_then(|pos| parse_type(s, pos, 'm', &mut *tm))
            .and_then(|pos| parse_char(s, pos, '-'))
            .and_then(|pos| parse_type(s, pos, 'd', &mut *tm))
      }
      'H' => {
        match match_digits_in_range(s, pos, 2, false, 0, 23) {
          Some(item) => { let (v, pos) = item; tm.tm_hour = v; Ok(pos) }
          None => Err(InvalidHour)
        }
      }
      'I' => {
        match match_digits_in_range(s, pos, 2, false, 1, 12) {
          Some(item) => {
              let (v, pos) = item;
              tm.tm_hour = if v == 12 { 0 } else { v };
              Ok(pos)
          }
          None => Err(InvalidHour)
        }
      }
      'j' => {
        match match_digits_in_range(s, pos, 3, false, 1, 366) {
          Some(item) => {
            let (v, pos) = item;
            tm.tm_yday = v - 1;
            Ok(pos)
          }
          None => Err(InvalidDayOfYear)
        }
      }
      'k' => {
        match match_digits_in_range(s, pos, 2, true, 0, 23) {
          Some(item) => { let (v, pos) = item; tm.tm_hour = v; Ok(pos) }
          None => Err(InvalidHour)
        }
      }
      'l' => {
        match match_digits_in_range(s, pos, 2, true, 1, 12) {
          Some(item) => {
              let (v, pos) = item;
              tm.tm_hour = if v == 12 { 0 } else { v };
              Ok(pos)
          }
          None => Err(InvalidHour)
        }
      }
      'M' => {
        match match_digits_in_range(s, pos, 2, false, 0, 59) {
          Some(item) => { let (v, pos) = item; tm.tm_min = v; Ok(pos) }
          None => Err(InvalidMinute)
        }
      }
      'm' => {
        match match_digits_in_range(s, pos, 2, false, 1, 12) {
          Some(item) => {
            let (v, pos) = item;
            tm.tm_mon = v - 1;
            Ok(pos)
          }
          None => Err(InvalidMonth)
        }
      }
      'n' => parse_char(s, pos, '\n'),
      'P' => match match_strs(s, pos,
                              &[("am", 0), ("pm", 12)]) {

        Some(item) => { let (v, pos) = item; tm.tm_hour += v; Ok(pos) }
        None => Err(InvalidHour)
      },
      'p' => match match_strs(s, pos,
                              &[("AM", 0), ("PM", 12)]) {

        Some(item) => { let (v, pos) = item; tm.tm_hour += v; Ok(pos) }
        None => Err(InvalidHour)
      },
      'R' => {
        parse_type(s, pos, 'H', &mut *tm)
            .and_then(|pos| parse_char(s, pos, ':'))
            .and_then(|pos| parse_type(s, pos, 'M', &mut *tm))
      }
      'r' => {
        parse_type(s, pos, 'I', &mut *tm)
            .and_then(|pos| parse_char(s, pos, ':'))
            .and_then(|pos| parse_type(s, pos, 'M', &mut *tm))
            .and_then(|pos| parse_char(s, pos, ':'))
            .and_then(|pos| parse_type(s, pos, 'S', &mut *tm))
            .and_then(|pos| parse_char(s, pos, ' '))
            .and_then(|pos| parse_type(s, pos, 'p', &mut *tm))
      }
      'S' => {
        match match_digits_in_range(s, pos, 2, false, 0, 60) {
          Some(item) => {
            let (v, pos) = item;
            tm.tm_sec = v;
            Ok(pos)
          }
          None => Err(InvalidSecond)
        }
      }
      //'s' {}
      'T' | 'X' => {
        parse_type(s, pos, 'H', &mut *tm)
            .and_then(|pos| parse_char(s, pos, ':'))
            .and_then(|pos| parse_type(s, pos, 'M', &mut *tm))
            .and_then(|pos| parse_char(s, pos, ':'))
            .and_then(|pos| parse_type(s, pos, 'S', &mut *tm))
      }
      't' => parse_char(s, pos, '\t'),
      'u' => {
        match match_digits_in_range(s, pos, 1, false, 1, 7) {
          Some(item) => {
            let (v, pos) = item;
            tm.tm_wday = if v == 7 { 0 } else { v };
            Ok(pos)
          }
          None => Err(InvalidDayOfWeek)
        }
      }
      'v' => {
        parse_type(s, pos, 'e', &mut *tm)
            .and_then(|pos|  parse_char(s, pos, '-'))
            .and_then(|pos| parse_type(s, pos, 'b', &mut *tm))
            .and_then(|pos| parse_char(s, pos, '-'))
            .and_then(|pos| parse_type(s, pos, 'Y', &mut *tm))
      }
      //'W' {}
      'w' => {
        match match_digits_in_range(s, pos, 1, false, 0, 6) {
          Some(item) => { let (v, pos) = item; tm.tm_wday = v; Ok(pos) }
          None => Err(InvalidDayOfWeek)
        }
      }
      'Y' => {
        match match_digits(s, pos, 4, false) {
          Some(item) => {
            let (v, pos) = item;
            tm.tm_year = v - 1900;
            Ok(pos)
          }
          None => Err(InvalidYear)
        }
      }
      'y' => {
        match match_digits_in_range(s, pos, 2, false, 0, 99) {
          Some(item) => {
            let (v, pos) = item;
            tm.tm_year = v;
            Ok(pos)
          }
          None => Err(InvalidYear)
        }
      }
      'Z' => {
        if match_str(s, pos, "UTC") || match_str(s, pos, "GMT") {
            tm.tm_utcoff = 0;
            Ok(pos + 3)
        } else {
            // It's odd, but to maintain compatibility with c's
            // strptime we ignore the timezone.
            let mut pos = pos;
            let len = s.len();
            while pos < len {
                let range = s.char_range_at(pos);
                pos = range.next;
                if range.ch == ' ' { break; }
            }

            Ok(pos)
        }
      }
      'z' => {
        let range = s.char_range_at(pos);

        if range.ch == '+' || range.ch == '-' {
            let sign = if range.ch == '+' { 1 } else { -1 };

            match match_digits(s, range.next, 4, false) {
              Some(item) => {
                let (v, pos) = item;
                if v == 0 {
                    tm.tm_utcoff = 0;
                } else {
                    let hours = v / 100;
                    let minutes = v - hours * 100;
                    tm.tm_utcoff = sign * (hours * 60 * 60 + minutes * 60);
                }
                Ok(pos)
              }
              None => Err(InvalidZoneOffset)
            }
        } else {
            Err(InvalidZoneOffset)
        }
      }
      '%' => parse_char(s, pos, '%'),
      ch => Err(InvalidFormatSpecifier(ch))
    }
}

