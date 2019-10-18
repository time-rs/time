//! Parse formats used in the `format` and `parse` methods.

use crate::format::{FormatItem, Padding, Specifier};
#[cfg(not(feature = "std"))]
use crate::no_std_prelude::*;
use crate::Language;
use core::mem;

/// Size of the buffer to store
const BUFFER_SIZE: usize = 512;

/// Empty the buffer, returning the old contents.
fn empty_buf(buf: &mut String) -> String {
    mem::replace(buf, String::with_capacity(BUFFER_SIZE))
}

// TODO This can likely be optimized and reduce allocations by storing the start
// and end positions in the full string, rather than copying the characters.
// This would completely eliminate the buffer.
/// Parse the formatting string with the provided language.
pub(crate) fn parse_with_language(s: &str, language: Language) -> Vec<FormatItem> {
    let mut items = vec![];
    let mut buf = String::with_capacity(BUFFER_SIZE);

    let mut chars = s.chars().peekable();

    /// Push the provided specifier to the list of items. If an asterisk is
    /// present, the language will be provided to the specifier. If a pound
    /// symbol is present, the padding will be provided.
    macro_rules! push_specifier {
        ($specifier:ident $($opts:tt)?) => {
            paste::expr! {
                items.push(FormatItem::Specifier(Specifier::[<$specifier>] $($opts)?))
            }
        };
    }

    while let Some(c) = chars.next() {
        if c == '%' {
            // Avoid adding unnecessary empty strings.
            if !buf.is_empty() {
                let buffer_contents = empty_buf(&mut buf);
                items.push(FormatItem::Literal(buffer_contents));
            }

            // Call `chars.next()` if a modifier is present, moving the iterator
            // past the character.
            let padding = match chars.peek() {
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
                Some('a') => push_specifier!(a { language }),
                Some('A') => push_specifier!(A { language }),
                Some('b') => push_specifier!(b { language }),
                Some('B') => push_specifier!(B { language }),
                Some('c') => push_specifier!(c { language }),
                Some('C') => push_specifier!(C { padding }),
                Some('d') => push_specifier!(d { padding }),
                Some('D') => push_specifier!(D),
                Some('e') => push_specifier!(e { padding }),
                Some('F') => push_specifier!(F),
                Some('g') => push_specifier!(g { padding }),
                Some('G') => push_specifier!(G { padding }),
                Some('H') => push_specifier!(H { padding }),
                Some('I') => push_specifier!(I { padding }),
                Some('j') => push_specifier!(j { padding }),
                Some('m') => push_specifier!(m { padding }),
                Some('M') => push_specifier!(M { padding }),
                Some('p') => push_specifier!(p),
                Some('P') => push_specifier!(P),
                Some('r') => push_specifier!(r),
                Some('R') => push_specifier!(R),
                Some('S') => push_specifier!(S { padding }),
                Some('T') => push_specifier!(T),
                Some('u') => push_specifier!(u),
                Some('U') => push_specifier!(U { padding }),
                Some('V') => push_specifier!(V { padding }),
                Some('w') => push_specifier!(w),
                Some('W') => push_specifier!(W { padding }),
                Some('y') => push_specifier!(y { padding }),
                Some('Y') => push_specifier!(Y { padding }),
                Some('z') => push_specifier!(z),
                Some('%') => buf.push('%'),
                Some(c) => panic!("Invalid specifier `{}`", c),
                None => panic!(
                    "Cannot end formatting with `%`. If you want a literal `%`, you must use `%%`."
                ),
            }
        } else {
            buf.push(c);
        }
    }

    if !buf.is_empty() {
        let buffer_contents = empty_buf(&mut buf);
        items.push(FormatItem::Literal(buffer_contents));
    }

    items
}
