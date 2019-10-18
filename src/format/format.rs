//! Parse formats used in the `format` and `parse` methods.

use crate::format::{FormatItem, Padding, Specifier};
#[cfg(not(feature = "std"))]
use crate::no_std_prelude::*;
use crate::Language;

/// Parse the formatting string with the provided language.
pub(crate) fn parse_with_language<'a>(s: &'a str, language: Language) -> Vec<FormatItem<'a>> {
    let mut items = vec![];
    let mut literal_start = 0;
    let mut chars = s.char_indices().peekable();

    /// Push the provided specifier to the list of items.
    macro_rules! push_specifier {
        ($specifier:ident $($opts:tt)?) => {{
            paste::expr! {
                literal_start = index + 1;
                items.push(FormatItem::Specifier(Specifier::[<$specifier>] $($opts)?))
            }
        }};
    }

    while let Some((index, c)) = chars.next() {
        if c == '%' {
            // Avoid adding unnecessary empty strings.
            if literal_start != index {
                items.push(FormatItem::Literal(&s[literal_start..index]));
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
                Some((index, 'a')) => push_specifier!(a { language }),
                Some((index, 'A')) => push_specifier!(A { language }),
                Some((index, 'b')) => push_specifier!(b { language }),
                Some((index, 'B')) => push_specifier!(B { language }),
                Some((index, 'c')) => push_specifier!(c { language }),
                Some((index, 'C')) => push_specifier!(C { padding }),
                Some((index, 'd')) => push_specifier!(d { padding }),
                Some((index, 'D')) => push_specifier!(D),
                Some((index, 'e')) => push_specifier!(e { padding }),
                Some((index, 'F')) => push_specifier!(F),
                Some((index, 'g')) => push_specifier!(g { padding }),
                Some((index, 'G')) => push_specifier!(G { padding }),
                Some((index, 'H')) => push_specifier!(H { padding }),
                Some((index, 'I')) => push_specifier!(I { padding }),
                Some((index, 'j')) => push_specifier!(j { padding }),
                Some((index, 'm')) => push_specifier!(m { padding }),
                Some((index, 'M')) => push_specifier!(M { padding }),
                Some((index, 'p')) => push_specifier!(p),
                Some((index, 'P')) => push_specifier!(P),
                Some((index, 'r')) => push_specifier!(r),
                Some((index, 'R')) => push_specifier!(R),
                Some((index, 'S')) => push_specifier!(S { padding }),
                Some((index, 'T')) => push_specifier!(T),
                Some((index, 'u')) => push_specifier!(u),
                Some((index, 'U')) => push_specifier!(U { padding }),
                Some((index, 'V')) => push_specifier!(V { padding }),
                Some((index, 'w')) => push_specifier!(w),
                Some((index, 'W')) => push_specifier!(W { padding }),
                Some((index, 'y')) => push_specifier!(y { padding }),
                Some((index, 'Y')) => push_specifier!(Y { padding }),
                Some((index, 'z')) => push_specifier!(z),
                Some((index, '%')) => literal_start = index,
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
