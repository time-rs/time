//! Parse formats used in the `format` and `parse` methods.

use crate::format::{FormatItem, Padding, Specifier};
#[cfg(no_std)]
use crate::internal_prelude::*;

/// Parse the formatting string. Panics if not valid.
#[inline(always)]
pub(crate) fn parse_fmt_string<'a>(s: &'a str) -> Vec<FormatItem<'a>> {
    match try_parse_fmt_string(s) {
        Ok(items) => items,
        Err(err) => panic!("{}", err),
    }
}

/// Attempt to parse the formatting string.
#[inline]
pub(crate) fn try_parse_fmt_string<'a>(s: &'a str) -> Result<Vec<FormatItem<'a>>, String> {
    let mut items = vec![];
    let mut literal_start = 0;
    let mut chars = s.char_indices().peekable();

    while let Some((i, c)) = chars.next() {
        /// Push the provided specifier to the list of items.
        macro_rules! push_specifier {
            ($i:ident, $specifier:expr) => {{
                literal_start = $i + 1;
                items.push(FormatItem::Specifier($specifier))
            }};
        }

        if c == '%' {
            // Avoid adding unnecessary empty strings.
            if literal_start != i {
                items.push(FormatItem::Literal(&s[literal_start..i]));
            }

            // Call `chars.next()` if a modifier is present, moving the iterator
            // past the character.
            let padding = match chars.peek().map(|v| v.1) {
                Some('-') => {
                    let _ = chars.next();
                    Padding::None
                }
                Some('_') => {
                    let _ = chars.next();
                    Padding::Space
                }
                Some('0') => {
                    let _ = chars.next();
                    Padding::Zero
                }
                _ => Padding::Default,
            };

            match chars.next() {
                Some((i, 'a')) => push_specifier!(i, Specifier::a),
                Some((i, 'A')) => push_specifier!(i, Specifier::A),
                Some((i, 'b')) => push_specifier!(i, Specifier::b),
                Some((i, 'B')) => push_specifier!(i, Specifier::B),
                Some((i, 'c')) => push_specifier!(i, Specifier::c),
                Some((i, 'C')) => push_specifier!(i, Specifier::C { padding }),
                Some((i, 'd')) => push_specifier!(i, Specifier::d { padding }),
                Some((i, 'D')) => push_specifier!(i, Specifier::D),
                Some((i, 'F')) => push_specifier!(i, Specifier::F),
                Some((i, 'g')) => push_specifier!(i, Specifier::g { padding }),
                Some((i, 'G')) => push_specifier!(i, Specifier::G { padding }),
                Some((i, 'H')) => push_specifier!(i, Specifier::H { padding }),
                Some((i, 'I')) => push_specifier!(i, Specifier::I { padding }),
                Some((i, 'j')) => push_specifier!(i, Specifier::j { padding }),
                Some((i, 'm')) => push_specifier!(i, Specifier::m { padding }),
                Some((i, 'M')) => push_specifier!(i, Specifier::M { padding }),
                Some((i, 'N')) => push_specifier!(i, Specifier::N),
                Some((i, 'p')) => push_specifier!(i, Specifier::p),
                Some((i, 'P')) => push_specifier!(i, Specifier::P),
                Some((i, 'r')) => push_specifier!(i, Specifier::r),
                Some((i, 'R')) => push_specifier!(i, Specifier::R),
                Some((i, 'S')) => push_specifier!(i, Specifier::S { padding }),
                Some((i, 'T')) => push_specifier!(i, Specifier::T),
                Some((i, 'u')) => push_specifier!(i, Specifier::u),
                Some((i, 'U')) => push_specifier!(i, Specifier::U { padding }),
                Some((i, 'V')) => push_specifier!(i, Specifier::V { padding }),
                Some((i, 'w')) => push_specifier!(i, Specifier::w),
                Some((i, 'W')) => push_specifier!(i, Specifier::W { padding }),
                Some((i, 'y')) => push_specifier!(i, Specifier::y { padding }),
                Some((i, 'Y')) => push_specifier!(i, Specifier::Y { padding }),
                Some((i, 'z')) => push_specifier!(i, Specifier::z),
                Some((i, '%')) => literal_start = i,
                Some((_, c)) => return Err(format!("Invalid specifier `{}`", c)),
                None => {
                    return Err(String::from(
                        "Cannot end formatting with `%`. If you want a literal `%`, you must use \
                         `%%`.",
                    ))
                }
            }
        }
    }

    if literal_start < s.len() {
        items.push(FormatItem::Literal(&s[literal_start..]));
    }

    Ok(items)
}
