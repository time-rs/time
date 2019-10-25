//! Parse formats used in the `format` and `parse` methods.

use crate::format::{FormatItem, Padding, Specifier};
#[cfg(not(feature = "std"))]
use crate::no_std_prelude::*;
use crate::Language;

/// Parse the formatting string with the provided language.
#[inline]
pub(crate) fn parse_with_language<'a>(s: &'a str, language: Language) -> Vec<FormatItem<'a>> {
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
                Some((i, 'a')) => push_specifier!(i, Specifier::a { language }),
                Some((i, 'A')) => push_specifier!(i, Specifier::A { language }),
                Some((i, 'b')) => push_specifier!(i, Specifier::b { language }),
                Some((i, 'B')) => push_specifier!(i, Specifier::B { language }),
                Some((i, 'c')) => push_specifier!(i, Specifier::c { language }),
                Some((i, 'C')) => push_specifier!(i, Specifier::C { padding }),
                Some((i, 'd')) => push_specifier!(i, Specifier::d { padding }),
                Some((i, 'D')) => push_specifier!(i, Specifier::D),
                Some((i, 'e')) => push_specifier!(i, Specifier::e { padding }),
                Some((i, 'F')) => push_specifier!(i, Specifier::F),
                Some((i, 'g')) => push_specifier!(i, Specifier::g { padding }),
                Some((i, 'G')) => push_specifier!(i, Specifier::G { padding }),
                Some((i, 'H')) => push_specifier!(i, Specifier::H { padding }),
                Some((i, 'I')) => push_specifier!(i, Specifier::I { padding }),
                Some((i, 'j')) => push_specifier!(i, Specifier::j { padding }),
                Some((i, 'm')) => push_specifier!(i, Specifier::m { padding }),
                Some((i, 'M')) => push_specifier!(i, Specifier::M { padding }),
                Some((i, 'p')) => push_specifier!(i, Specifier::p),
                Some((i, 'P')) => push_specifier!(i, Specifier::P),
                Some((i, 'r')) => push_specifier!(i, Specifier::r),
                Some((i, 'R')) => push_specifier!(i, Specifier::R),
                Some((i, 'S')) => push_specifier!(i, Specifier::S { padding }),
                Some((i, 'T')) => push_specifier!(i, Specifier::T),
                Some((i, 'u')) => push_specifier!(i, Specifier::u),
                Some((i, 'V')) => push_specifier!(i, Specifier::V { padding }),
                Some((i, 'w')) => push_specifier!(i, Specifier::w),
                Some((i, 'y')) => push_specifier!(i, Specifier::y { padding }),
                Some((i, 'Y')) => push_specifier!(i, Specifier::Y { padding }),
                Some((i, 'z')) => push_specifier!(i, Specifier::z),
                Some((i, '%')) => literal_start = i,
                Some((_, c)) => panic!("Invalid specifier `{}`", c),
                None => panic!(
                    "Cannot end formatting with `%`. If you want a literal `%`, you must use `%%`."
                ),
            }
        }
    }

    if literal_start < s.len() {
        items.push(FormatItem::Literal(&s[literal_start..]));
    }

    items
}
