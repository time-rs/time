//! Parse formats used in the `format` and `parse` methods.

use crate::format::{FormatItem, Specifier};
use crate::Language;
use alloc::{string::String, vec::Vec};
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

    let mut chars = s.chars();

    /// Push the provided specifier to the list of items. If an asterisk is
    /// present, the language will be provided to the specifier.
    macro_rules! push_specifier {
        ($specifier:ident) => {
            paste::expr! {
                items.push(FormatItem::Specifier(Specifier::[<$specifier>]))
            }
        };

        ($specifier:ident *) => {
            paste::expr! {
                items.push(FormatItem::Specifier(Specifier::[<$specifier>] { language }))
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

            match chars.next() {
                Some('a') => push_specifier!(a*),
                Some('A') => push_specifier!(A*),
                Some('b') => push_specifier!(b*),
                Some('B') => push_specifier!(B*),
                Some('c') => push_specifier!(c*),
                Some('C') => push_specifier!(C),
                Some('d') => push_specifier!(d),
                Some('D') => push_specifier!(D),
                Some('e') => push_specifier!(e),
                Some('F') => push_specifier!(F),
                Some('g') => push_specifier!(g),
                Some('G') => push_specifier!(G),
                Some('H') => push_specifier!(H),
                Some('I') => push_specifier!(I),
                Some('j') => push_specifier!(j),
                Some('m') => push_specifier!(m),
                Some('M') => push_specifier!(M),
                Some('p') => push_specifier!(p),
                Some('P') => push_specifier!(P),
                Some('r') => push_specifier!(r),
                Some('R') => push_specifier!(R),
                Some('S') => push_specifier!(S),
                Some('T') => push_specifier!(T),
                Some('u') => push_specifier!(u),
                Some('U') => push_specifier!(U),
                Some('V') => push_specifier!(V),
                Some('w') => push_specifier!(w),
                Some('W') => push_specifier!(W),
                Some('y') => push_specifier!(y),
                Some('Y') => push_specifier!(Y),
                Some('z') => push_specifier!(z),
                Some(c) => panic!("Invalid specifier `{}`", c),
                None => panic!(
                    "Cannot end formatting with `%`. If you want a literal `%`, you must use `%%`."
                ),
            }
        } else {
            buf.push(c);
        }
    }

    items
}
