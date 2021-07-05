use std::char;
use std::ops::{Index, RangeFrom};

use proc_macro::Span;

use crate::Error;

pub(crate) fn parse(token: &proc_macro::Literal) -> Result<(Span, Vec<u8>), Error> {
    let span = token.span();
    let repr = token.to_string();

    match repr.as_bytes() {
        [b'"', ..] => Ok((span, parse_lit_str_cooked(&repr[1..], span)?)),
        [b'b', b'"', rest @ ..] => Ok((span, parse_lit_byte_str_cooked(rest, span)?)),
        [b'r', rest @ ..] | [b'b', b'r', rest @ ..] => Ok((span, parse_lit_str_raw(rest))),
        _ => Err(Error::ExpectedString {
            span_start: Some(span),
            span_end: Some(span),
        }),
    }
}

fn byte(s: impl AsRef<[u8]>, idx: usize) -> u8 {
    s.as_ref().get(idx).copied().unwrap_or_default()
}

fn parse_lit_str_cooked(mut s: &str, span: Span) -> Result<Vec<u8>, Error> {
    let mut content = String::new();
    'outer: loop {
        let ch = match byte(s, 0) {
            b'"' => break,
            b'\\' => {
                let b = byte(s, 1);
                s = &s[2..];
                match b {
                    b'x' => {
                        let (byte, rest) = backslash_x(s, span)?;
                        s = rest;
                        assert!(byte <= 0x80, "Invalid \\x byte in string literal");
                        char::from_u32(u32::from(byte)).expect("byte was just validated")
                    }
                    b'u' => {
                        let (chr, rest) = backslash_u(s, span)?;
                        s = rest;
                        chr
                    }
                    b'n' => '\n',
                    b'r' => '\r',
                    b't' => '\t',
                    b'\\' => '\\',
                    b'0' => '\0',
                    b'\'' => '\'',
                    b'"' => '"',
                    b'\r' | b'\n' => loop {
                        let ch = s.chars().next().unwrap_or_default();
                        if ch.is_whitespace() {
                            s = &s[ch.len_utf8()..];
                        } else {
                            continue 'outer;
                        }
                    },
                    b => {
                        return Err(Error::Custom {
                            message: format!(
                                "unexpected byte {:?} after \\ character in byte literal",
                                b
                            )
                            .into(),
                            span_start: Some(span),
                            span_end: Some(span),
                        });
                    }
                }
            }
            b'\r' => {
                assert_eq!(byte(s, 1), b'\n', "Bare CR not allowed in string");
                s = &s[2..];
                '\n'
            }
            _ => {
                let ch = s.chars().next().unwrap_or_default();
                s = &s[ch.len_utf8()..];
                ch
            }
        };
        content.push(ch);
    }

    assert!(s.starts_with('"'));
    Ok(content.into_bytes())
}

fn parse_lit_str_raw(s: &[u8]) -> Vec<u8> {
    let mut pounds = 0;
    while byte(s, pounds) == b'#' {
        pounds += 1;
    }
    assert_eq!(byte(s, pounds), b'"');
    let close = s
        .iter()
        .rposition(|&b| b == b'"')
        .expect("had a string without trailing \"");
    for &end in &s[close + 1..close + 1 + pounds] {
        assert_eq!(end, b'#');
    }

    s[pounds + 1..close].to_owned()
}

fn parse_lit_byte_str_cooked(mut v: &[u8], span: Span) -> Result<Vec<u8>, Error> {
    let mut out = Vec::new();
    'outer: loop {
        let byte = match byte(v, 0) {
            b'"' => break,
            b'\\' => {
                let b = byte(v, 1);
                v = &v[2..];
                match b {
                    b'x' => {
                        let (byte, rest) = backslash_x(v, span)?;
                        v = rest;
                        byte
                    }
                    b'n' => b'\n',
                    b'r' => b'\r',
                    b't' => b'\t',
                    b'\\' => b'\\',
                    b'0' => b'\0',
                    b'\'' => b'\'',
                    b'"' => b'"',
                    b'\r' | b'\n' => loop {
                        let byte = byte(v, 0);
                        let ch = char::from_u32(u32::from(byte)).expect("invalid byte");
                        if ch.is_whitespace() {
                            v = &v[1..];
                        } else {
                            continue 'outer;
                        }
                    },
                    b => {
                        return Err(Error::Custom {
                            message: format!(
                                "unexpected byte {:?} after \\ character in byte literal",
                                b
                            )
                            .into(),
                            span_start: Some(span),
                            span_end: Some(span),
                        });
                    }
                }
            }
            b'\r' => {
                assert_eq!(byte(v, 1), b'\n', "Bare CR not allowed in string");
                v = &v[2..];
                b'\n'
            }
            b => {
                v = &v[1..];
                b
            }
        };
        out.push(byte);
    }

    assert_eq!(byte(v, 0), b'"');
    Ok(out)
}

fn backslash_x<S>(s: &S, span: Span) -> Result<(u8, &S), Error>
where
    S: Index<RangeFrom<usize>, Output = S> + AsRef<[u8]> + ?Sized,
{
    let mut ch = 0;
    let b0 = byte(s, 0);
    let b1 = byte(s, 1);
    ch += 0x10
        * match b0 {
            b'0'..=b'9' => b0 - b'0',
            b'a'..=b'f' => 10 + (b0 - b'a'),
            b'A'..=b'F' => 10 + (b0 - b'A'),
            _ => {
                return Err(Error::Custom {
                    message: "unexpected non-hex character after \\x".into(),
                    span_start: Some(span),
                    span_end: Some(span),
                });
            }
        };
    ch += match b1 {
        b'0'..=b'9' => b1 - b'0',
        b'a'..=b'f' => 10 + (b1 - b'a'),
        b'A'..=b'F' => 10 + (b1 - b'A'),
        _ => {
            return Err(Error::Custom {
                message: "unexpected non-hex character after \\x".into(),
                span_start: Some(span),
                span_end: Some(span),
            });
        }
    };
    Ok((ch, &s[2..]))
}

fn backslash_u(mut s: &str, span: Span) -> Result<(char, &str), Error> {
    if byte(s, 0) != b'{' {
        return Err(Error::Custom {
            message: "expected { after \\u".into(),
            span_start: Some(span),
            span_end: Some(span),
        });
    }
    s = &s[1..];

    let mut ch = 0;
    let mut digits = 0;
    loop {
        let b = byte(s, 0);
        let digit = match b {
            b'0'..=b'9' => b - b'0',
            b'a'..=b'f' => 10 + b - b'a',
            b'A'..=b'F' => 10 + b - b'A',
            b'_' if digits > 0 => {
                s = &s[1..];
                continue;
            }
            b'}' if digits == 0 => {
                return Err(Error::Custom {
                    message: "invalid empty unicode escape".into(),
                    span_start: Some(span),
                    span_end: Some(span),
                });
            }
            b'}' => break,
            _ => {
                return Err(Error::Custom {
                    message: "unexpected non-hex character after \\u".into(),
                    span_start: Some(span),
                    span_end: Some(span),
                });
            }
        };
        if digits == 6 {
            return Err(Error::Custom {
                message: "overlong unicode escape (must have at most 6 hex digits)".into(),
                span_start: Some(span),
                span_end: Some(span),
            });
        }
        ch *= 0x10;
        ch += u32::from(digit);
        digits += 1;
        s = &s[1..];
    }
    assert_eq!(byte(s, 0), b'}');
    s = &s[1..];

    if let Some(ch) = char::from_u32(ch) {
        Ok((ch, s))
    } else {
        Err(Error::Custom {
            message: format!("character code {:x} is not a valid unicode character", ch).into(),
            span_start: Some(span),
            span_end: Some(span),
        })
    }
}
