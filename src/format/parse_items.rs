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

    /// Push the provided specifier to the list of items.
    macro_rules! push_specifier {
        ($specifier:ident $($opts:tt)?) => {{
            paste::expr! {
                literal_start = i + 1;
                items.push(FormatItem::Specifier(Specifier::[<$specifier>] $($opts)?))
            }
        }};
    }

    while let Some((i, c)) = chars.next() {
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
                Some((i, 'a')) => push_specifier!(a { language }),
                Some((i, 'A')) => push_specifier!(A { language }),
                Some((i, 'b')) => push_specifier!(b { language }),
                Some((i, 'B')) => push_specifier!(B { language }),
                Some((i, 'c')) => push_specifier!(c { language }),
                Some((i, 'C')) => push_specifier!(C { padding }),
                Some((i, 'd')) => push_specifier!(d { padding }),
                Some((i, 'D')) => push_specifier!(D),
                Some((i, 'e')) => push_specifier!(e { padding }),
                Some((i, 'F')) => push_specifier!(F),
                Some((i, 'g')) => push_specifier!(g { padding }),
                Some((i, 'G')) => push_specifier!(G { padding }),
                Some((i, 'H')) => push_specifier!(H { padding }),
                Some((i, 'I')) => push_specifier!(I { padding }),
                Some((i, 'j')) => push_specifier!(j { padding }),
                Some((i, 'm')) => push_specifier!(m { padding }),
                Some((i, 'M')) => push_specifier!(M { padding }),
                Some((i, 'p')) => push_specifier!(p),
                Some((i, 'P')) => push_specifier!(P),
                Some((i, 'r')) => push_specifier!(r),
                Some((i, 'R')) => push_specifier!(R),
                Some((i, 'S')) => push_specifier!(S { padding }),
                Some((i, 'T')) => push_specifier!(T),
                Some((i, 'u')) => push_specifier!(u),
                Some((i, 'U')) => push_specifier!(U { padding }),
                Some((i, 'V')) => push_specifier!(V { padding }),
                Some((i, 'w')) => push_specifier!(w),
                Some((i, 'W')) => push_specifier!(W { padding }),
                Some((i, 'y')) => push_specifier!(y { padding }),
                Some((i, 'Y')) => push_specifier!(Y { padding }),
                Some((i, 'z')) => push_specifier!(z),
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
